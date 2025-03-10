use std::ffi::OsString;
use cachedb::cachedb::image_cache::Cache;

#[test]
fn test() {
    let cache = match Cache::new(&String::from("/tmp/test"), 64) {
        Ok(x) => {
            x
        },
        Err(x) => {
            println!("{:?}", x);
            panic!()
        },
    };
    println!("Path for {:?}", cache.get_full_path(&OsString::from("12345678abcde0980123456789ABCDEF02")));
}