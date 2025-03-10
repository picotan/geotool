use std::fmt;
use crate::trackroute::{TrackRoute};

#[derive(Clone)]
pub struct Track {
    pub routes: Vec<TrackRoute>,
    pub name: String,
    pub comment: String,
}

impl Track {
    pub fn new() -> Track {
        Self {
            routes: Vec::new(),
            name: String::new(),
            comment: String::new(),
        }
    }
    #[unsafe(no_mangle)]
    pub extern fn Track() -> Track {
        Track::new()
    }
}

impl fmt::Debug for Track {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Track")
            .field("name", &self.name)
            .field("comment", &self.comment)
            .finish()
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(").unwrap();
        if self.name != "" {write!(f, "name: {:?}", self.name).unwrap();}
        if self.comment != "" {write!(f, "comment: {:?}", self.comment).unwrap();}
        for i in &self.routes {
            write!(f, "{:?}", i).unwrap();
        }
        writeln!(f, ")")
    }
}