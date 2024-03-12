pub mod image_cache {
    use std::{fmt, fs};
    use std::fs::{File, OpenOptions};
    use std::path::{Path, PathBuf};
    use regex::Regex;
    use std::collections::HashMap;
    use std::time::SystemTime;
    use std::io::{Error, ErrorKind};
    use std::num::ParseIntError;
    use num::ToPrimitive;
    use std::ffi::OsString;
    use crate::geometry::geometry_core::LatLon;

    #[derive(Debug, Clone)]
    struct FileAttr {
        name: OsString,
        last_modified: SystemTime,
    }

    impl FileAttr {
        fn new(path: &PathBuf) -> Result<FileAttr, std::io::Error> {
            let name = path.file_name().unwrap().to_os_string();
            let path = path.clone().into_os_string();
            if FileAttr::is_feasible(&name) {
                let mut f = File::open(path)?;
                let metadata = f.metadata()?;
                Ok(FileAttr{name: name.clone(), last_modified: metadata.modified()?})
            } else {
                print!("None");
                Err(Error::from(ErrorKind::Other))
            }
        }

        fn is_feasible(name: &OsString) -> bool {
            // Format is,
            //   latitude position in tile domain, longitude position in tile domain, zoom level
            let re = Regex::new(r"^([0-9a-fA-F]{16})([0-9a-fA-F]{16})([0-9]{2})$").unwrap();
            re.is_match(name.to_str().unwrap())
        }

        fn parse(&self) -> Result<(i64, i64, i64), ParseIntError> {
            let re = Regex::new(r"^(\xFFFFFFFF)(\xFFFFFFFF\)(d{2})$").unwrap();
            let pm = re.captures(&self.name.to_str().unwrap()).unwrap();
            let x = match i64::from_str_radix(&pm[1], 16) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let y = match i64::from_str_radix(&pm[2], 16) {
                Ok(y) => y,
                Err(err) => return Err(err),
            };
            let z = match i64::from_str_radix(&pm[3], 10) {
                Ok(z) => z,
                Err(err) => return Err(err),
            };
            Ok((x, y, z))
        }
    }

    impl fmt::Display for FileAttr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(key:{:?}, Modified:{:?})", self.name, self.last_modified)
        }
    }
    // #Cache
    //
    // Cache is handling image cache from the map site like GSI, Open Street Map
    // The format of cache file is basically 8digit hexadecimal 8 digit hexadecimal 2 digit decimal
    // as a file name.
    // First one represents latitude (in tile position) second one is longitude (in tile position) and zoom level
    // Cache has lifetime and the file is expired, its cache file will be removed.
    // Also cache limits maximum number of entries, if number of entries are exceeded, oldest one will be deleted.
    // This class is supporting only single cache directory flat tree (no subdirectories)
    pub struct Cache {
        list: HashMap<OsString, FileAttr>, // File name (Key), file attribute
        timetable: Vec<(SystemTime, OsString)>,
        life: u64,  // In sec
        path: PathBuf,
        size: Option<usize>,
    }

    impl Cache {
        pub fn new(p: &String, life: u64) -> Result<Cache,Error> {
            let mut list: HashMap<OsString, FileAttr> = HashMap::new();
            let mut timetable: Vec<(SystemTime, OsString)> = Vec::new();
            let path = Path::new(p);
            Self::set_path_sub(p, true, &mut list, &mut timetable)?;

            Ok(Cache{path: path.to_path_buf(), list: list, life: life, size: None, timetable: timetable})
        }

        fn quick_find(self: &Self, t: &SystemTime) -> Option<usize> {
            self.quick_find_sub(t, 0, self.timetable.len() - 1)
        }

        fn quick_find_sub(self: &Self, t: &SystemTime, s: usize, e:usize) -> Option<usize> {
            if (s == e) {
                if (self.timetable[s].0 == *t) {
                    return Some(s)
                }
                return None
            }
            let st = self.timetable[s].0;
            let et = self.timetable[e].0;
            if ((t > &st) || (t < &et)) {return None}
            let m = (s + e) / 2;
            if &self.timetable[m].0 < t {
                return self.quick_find_sub(t, m, e)
            }
            return self.quick_find_sub(t, s, m)
        }

        fn quick_insert(t: &(SystemTime, OsString),
                        list: &mut HashMap<OsString, FileAttr>,
                        timetable: &mut Vec<(SystemTime, OsString)>)  -> bool {
            let e = timetable.len();
            Cache::quick_insert_sub(t, list, timetable, 0, e)
        }

        fn quick_insert_sub(t: &(SystemTime, OsString),
                            list: &mut HashMap<OsString, FileAttr>,
                            timetable: &mut Vec<(SystemTime, OsString)>,
                            s: usize, e:usize) -> bool {
            if (s == e) {
                timetable.insert(s.to_usize().unwrap(), t.clone());
                println!("{:?}", timetable);
                return true
            }
            let st = timetable[s].0;
            let et = timetable[e].0;
            if ((t.0 > st) || (t.0 < et)) {
                return false
            }
            let m = (s + e) / 2;
            if timetable[m].0 < t.0 {
                return Self::quick_insert_sub(t, list, timetable, m, e)
            }
            return Self::quick_insert_sub(t, list, timetable, s, m)
        }

        // Add entry considering live time if no key found, if key is existing update it
        fn add(self: &mut Self, path: &PathBuf, attr: FileAttr) {
            let file = path.file_name().unwrap().to_os_string();
            // Possible Cases
            // 1: No key enough space -> simply add.
            // 2: No key no space -> delete oldest one and add new one
            // 3: Has key -> update hash and timetable (remove old one and insert top to the table
            if (self.list.len() < self.size.unwrap()) || (self.size.is_none()) {
                let a = self.list.get(&file);
                if a.is_some() {
                    // Delete existing timetable entry
                    self.quick_find(&a.unwrap().last_modified);
                }
                Cache::quick_insert(&(SystemTime::now(), file.clone()), &mut self.list, &mut self.timetable);
                self.list.insert(file.clone(), attr);
            }

            // Add entry
            match FileAttr::new(path) {
                Ok(x) => {
                    self.list.insert(file.clone(), x)
                },
                Err(e) => return
            };
        }

        pub fn is_exist(self: &Self, path: &String) -> bool {
            let p = Path::new(path);
            let p = p.to_path_buf().into_os_string();
            !self.list.get(&p).is_none()
        }

        // Get File attribute corresponding to path from cache
        pub fn get_full_path(self: &Self, name: &OsString) -> Option<OsString>{
            match self.list.get(name) {
                Some(f) => {
                    let mut p = self.path.clone();
                    p.push(&f.name);
                    Some(OsString::from(p))
                },
                None => None,
            }
        }

        pub fn get_attr(self: &Self, path: &OsString) -> Option<&FileAttr> {
            self.list.get(path)
        }

        // Clean up cache according to its lifetime.
        pub fn refresh(self: &mut Self) {
            let mut keys: Vec<PathBuf> = vec![];
            for (path, attr) in &self.list {
                let attr = self.list.get(path).unwrap();
                let l = SystemTime::now().duration_since(attr.last_modified).unwrap().as_secs();
                if (l > self.life) {
                    // This entry is expired, remove from cache
                    keys.push(path.clone().into());
                }
            }
            for p in keys {
                self.list.remove(&p.into_os_string());
            }
        }

        // Change cache directory, drop all of existing cache file information and recreate meta information.
        pub fn set_path(self: &mut Self,
                        path: &String,
                        create: bool) -> Result<bool, std::io::Error> {
            match Self::set_path_sub(path, create, &mut self.list, &mut self.timetable) {
                Ok(r) => {
                    self.path = PathBuf::from(path);
                    Ok(r)
                },
                Err(e) => Err(e),
            }
        }

        pub fn set_path_sub(path: &String,
                            create: bool,
                            list: &mut HashMap<OsString, FileAttr>,
                            timetable: &mut Vec<(SystemTime, OsString)>) -> Result<bool, Error> {
            let p = Path::new(path);
            let mut p = p.to_path_buf();
            // Directory Check
            if !p.exists() {
                if create {
                    fs::create_dir_all(path)?;
                } else {
                    return Err(Error::from(ErrorKind::PermissionDenied))
                }
            } else if p.is_file() {
                return Err(Error::from(ErrorKind::Other))
            }

            // Cleanup
            list.drain();
            timetable.clear();

            // Parse new cache directory
            for entry in fs::read_dir(p)? {
                let entry = entry?;
                if entry.path().is_file() {
                    let file = entry.file_name();
                    let attr = match FileAttr::new(&entry.path()) {
                        Ok(x) => {
                            println!("Cache File: {:?}", file);
                            x
                        },
                        Err(e) => {
                            println!("{:?} is not cache file {:?}", file, e);
                            continue
                        }
                    };
                    Self::quick_insert(&(attr.last_modified, file), list, timetable);
                    list.insert(entry.file_name(), attr);
                } else {
                    // Subdirectory is not supported
                    println!("{:?} is sub directory?", entry.path());
                }
            }
            Ok(true)
        }
    }
    impl fmt::Display for Cache {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(List: {:?}, Time Table {:?}, Life Time:{:?}sec, Path:{:?}, Size: {:?})", self. list, self.timetable, self.life, self.path, self.size)
        }
    }
}

