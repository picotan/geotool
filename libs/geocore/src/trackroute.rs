use std::fmt;
use crate::gpx::gpx::{TrackPoint, TrackSegment};

#[derive(Clone)]
pub struct TrackRoute {
    pub segments: Vec<TrackSegment>,
    pub name: String,
    pub comment: String,
    highest: f64,
    lowest: f64,
    distance: f64,
}

impl TrackRoute {
    pub fn new() -> TrackRoute {
        Self {segments: Vec::new(),
            name: String::new(),
            comment: String::new(),
            highest: 0f64,
            lowest: 0f64,
            distance: 0f64,
        }
    }

    #[unsafe(no_mangle)]
    pub extern fn TrackRoute() -> TrackRoute {
        TrackRoute::new()
    }

    #[unsafe(no_mangle)]
    pub extern fn add_segment(self: &mut Self, segment: &TrackSegment) {
        if segment.lowest < self.lowest {
            self.lowest = segment.lowest;
        }

        if segment.highest < self.highest {
            self.highest = segment.highest;
        }
        self.segments.push(segment.clone());
        self.distance += segment.distance;
    }

    #[unsafe(no_mangle)]
    pub extern fn split_segment_at(self: &mut Self, point: &TrackPoint) -> bool {
        return true
    }
}

impl fmt::Display for TrackRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(").unwrap();
        if self.name != "" {write!(f, "name: {:?}", self.name).unwrap();}
        if self.comment != "" {write!(f, "comment: {:?}", self.comment).unwrap();}
        for i in &self.segments {
            write!(f, "{:?}", i).unwrap();
        }
        if !self.highest.is_nan() {write!(f, "time: {}", self.highest).unwrap();}
        if !self.lowest.is_nan() {write!(f, "time: {}", self.lowest).unwrap();}
        if !self.distance.is_nan() {write!(f, "time: {}", self.distance).unwrap();}
        writeln!(f, ")")
    }
}
impl fmt::Debug for TrackRoute {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TrackRoute")
            .field("name", &self.name)
            .field("comment", &self.comment)
            .field("highest", &self.highest)
            .field("lowest", &self.lowest)
            .field("distance", &self.distance)
            .finish()
    }
}