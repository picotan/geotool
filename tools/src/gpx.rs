#![recursion_limit = "256"]


pub mod gpx {
    use std::fmt;
    use std::fmt::Formatter;
    use std::ffi::OsString;
    use crate::geometry::geometry_core::{Geometry, LatLon, Area};
    use chrono::prelude::{DateTime, Utc};
    use num::complex::ComplexFloat;
    use bitfield::BitRangeMut;
    use bitfield::{bitfield_bitrange, bitfield_debug, bitfield_fields};

    #[derive(Clone,Eq, PartialEq)]
    pub struct Weather(pub u8);
    bitfield_bitrange! {struct Weather(u8)}
    impl Weather {
        bitfield_fields! {
                u8;
                state, _: 3, 0;
                strong, _: 7, 4;
        }

        pub const NONE: u8 = 0;

        pub const SUNNY: u8 = 1;
        pub const CLOUDY: u8 = 2;
        pub const RAIN: u8 = 3;
        pub const SNOW: u8 = 4;
        pub const HAIL: u8 = 5;
        pub const SLEET: u8 = 6;
        pub const FOG: u8 = 7;

        pub const LITE: u8 = 1;
        pub const MEDIUM: u8 = 0;
        pub const HEAVY: u8 = 2;
        pub const FEEBLE: u8 = 3;
        pub const STORMY: u8 = 4;

        pub fn set_strong(&mut self, strong: u8) {
            self.0 = strong;
        }
    }

    impl std::fmt::Debug for Weather {
        bitfield_debug! {
                struct Weather;
                state, _: 3, 0;
                strong, _: 7, 4;
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Wind {
        pub direction: f64,
        pub strong: u64
    }

    // impl fmt::Debug for Wind {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         for (direction, strong) in self {
    //             writeln!(f, "Wind from {}:", direction)?;
    //         }
    //         Ok(())
    //     }
    // }
    #[derive(Clone, Eq, PartialEq, Copy)]
    pub enum PointType {
        None = 0,
        Hut = 1,
        Cliff = 2,
        Node = 3,
        Avalanche = 4,
        Flower = 5,
        Danger = 6,
        View = 7,
        Summit = 8,
        Water = 9,
        Station = 10,
        BusStop = 11,
        Note = 12,
        Waterfall = 13,
        Animal = 14,
        Entrance = 15,
    }


    impl PointType {
        pub fn as_str(&self) -> &str {
            match self {
                PointType::None => {"None"},
                PointType::Hut => {"Hut"},
                PointType::Cliff => {"Cliff"},
                PointType::Node => {"Node"},
                PointType::Avalanche => {"Avalanche"},
                PointType::Flower => {"Flowers"},
                PointType::Danger => {"Danger"},
                PointType::View => {"Good View"},
                PointType::Summit => {"Summit"},
                PointType::Water => {"Water Source"},
                PointType::Station => {"Train Station"},
                PointType::BusStop => {"Bus Stop"},
                PointType::Note => {"Note"},
                PointType::Waterfall => {"Waterfall"},
                PointType::Animal => {"Animal"},
                PointType::Entrance => {"Entrance/Exit"}
            }
        }
    }

    #[derive(Clone)]
    pub struct TrackPoint {
        pub location: LatLon,
        pub altitude: f64,
        pub time: DateTime<Utc>,
        pub heading: f64,           // Compass
        pub pressure: f64,          // Air Pressure
        pub temperature: f64,
        pub wind: Wind,
        pub heart_rate: f64,
        pub luminance: f64,
        pub radiation: f64,
        pub distance: f64,
        pub energy: f64,
        pub cadence: f64,
        pub pace: f64,
        pub vertical_speed: f64,
        pub weather: Weather,
        pub point_type: Vec::<PointType>,
        pub comment: String,
        pub name: String,
    }

    impl TrackPoint {
        pub fn new(lat: f64, lon: f64) -> TrackPoint {
            Self {
                location: LatLon::new(lat, lon),
                altitude: f64::NAN,
                time: DateTime::<Utc>::MIN_UTC,
                heading: f64::NAN,
                pressure: f64::NAN,
                temperature: f64::NAN,
                heart_rate: f64::NAN,
                wind: Wind{direction: f64::NAN, strong: u64::MIN},
                luminance: f64::NAN,
                radiation: f64::NAN,
                distance: f64::NAN,
                energy: f64::NAN,
                cadence: f64::NAN,
                pace: f64::NAN,
                vertical_speed: f64::NAN,
                weather: Weather(Weather::NONE),
                point_type: vec![PointType::None],
                comment: String::new(),
                name: String::new(),
            }
        }

        pub fn set_name(self: &mut Self, name: String) {
            self.name = name;
        }

        pub fn set_comment(self: &mut Self, comment: String) {
            self.comment = comment;
        }

        pub fn weather(str: u8, state: u8) -> Weather {
            return Weather((state << 4) | str);
        }

        pub fn weather_str(&self) -> OsString {
            let strong = match self.weather.strong() {
                Weather::LITE => {"Lite"}
                Weather::MEDIUM => {""}
                Weather::HEAVY => {"Heavy"}
                Weather::STORMY => {"Stomy"}
                Weather::FEEBLE => {"Feeble"}
                _ => {""}
            };

            let state = match self.weather.state() {
                Weather::RAIN => {"Rain"}
                Weather::SUNNY => {"Sunny"}
                Weather::SNOW => {"Snow"}
                Weather::CLOUDY => {"Cloudy"}
                Weather::SLEET => {"Sleet"}
                Weather::HAIL => {"Hail"}
                Weather::FOG => {"Fog"}
                _ => {""}
            };

            return OsString::from(format!("{strong} {state}"));
        }

        pub fn type_str(&self) -> Vec<OsString> {
            let mut list: Vec<OsString> = vec![];
            for pt in &self.point_type {
                list.push(OsString::from(pt.as_str()));
            }
            return list.clone();
        }

        pub fn has_extension(&self) -> bool {
            if (!self.heading.is_nan()) {return true}
            if (!self.pressure.is_nan()) {return true}
            if (!self.temperature.is_nan()) {return true}
            if (!self.heart_rate.is_nan()) {return true}
            if ((!self.wind.direction.is_nan()) && (self.wind.strong == u64::MIN)) {return true}
            if (!self.luminance.is_nan()) {return true}
            if (!self.radiation.is_nan()) {return true}
            if (!self.distance.is_nan()) {return true}
            if (!self.energy.is_nan()) {return true}
            if (!self.cadence.is_nan()) {return true}
            if (!self.pace.is_nan()) {return true}
            if (!self.vertical_speed.is_nan()) {return true}
            if (self.weather != Weather(Weather::NONE)) {return true}
            if (self.point_type.len() != 0) {return true}
            if (!self.comment.is_empty()) {return true}
            if (!self.name.is_empty()) {return true}
            return false;
        }
    }

    impl fmt::Display for crate::gpx::gpx::TrackPoint {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(location: {:?}", self.location).unwrap();
            if (!self.altitude.is_nan()) {write!(f, "altitude: {}m", self.altitude).unwrap();}
            if (self.name != "") {write!(f, "name: {:?}", self.name).unwrap();}
            if (self.comment != "") {write!(f, "comment: {:?}", self.comment).unwrap();}
            if (!self.time.eq(&DateTime::<Utc>::MIN_UTC)) {write!(f, "time: {:?}", self.time).unwrap();}
            if (!self.heading.is_nan()) {write!(f, "heading: {}", self.heading).unwrap();}
            if (!self.pressure.is_nan()) {write!(f, "pressure: {}hp", self.pressure).unwrap();}
            if (!self.temperature.is_nan()) {write!(f, "temperature: {}", self.temperature).unwrap();}
            if (!self.heart_rate.is_nan()) {write!(f, "heart nrate: {}bpm", self.heart_rate).unwrap();}
            if (!self.luminance.is_nan()) {write!(f, "luminance: {}lx", self.luminance).unwrap();}
            if (!self.radiation.is_nan()) {write!(f, "radiation: {}", self.radiation).unwrap();}
            if (self.weather != Weather(Weather::NONE)) {write!(f, "time: {:?}", &self.weather_str()).unwrap();}
            if (self.point_type.len() != 0) {write!(f, "time: {:?}", &self.type_str()).unwrap();}
            writeln!(f, ")")
        }
    }

    impl fmt::Debug for crate::gpx::gpx::TrackPoint {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("TrackPoint")
                .field("altitude", &self.altitude)
                .finish()
        }
    }

    #[derive(Clone, Debug)]
    pub struct PointAttr {
        pub point: TrackPoint,
        geometry: Geometry,
        distance: f64,
        direction: f64,
    }

    #[derive(Clone, Debug)]
    pub struct TrackSegment {
        pub points: Vec<PointAttr>,
        pub name: String,
        pub comment: String,
        highest: f64,
        lowest: f64,
        distance: f64,
        area: Area,
    }

    impl TrackSegment {
        pub fn new() -> TrackSegment {
            Self {
                points: Vec::new(),
                name: String::new(),
                comment: String::new(),
                highest: f64::NAN,
                lowest: f64::NAN,
                distance: 0f64,
                area:Area::invalid(),
            }
        }

        pub fn add_point(self: &mut Self, p: TrackPoint) {
            if (self.highest < p.altitude) {
                self.highest = p.altitude
            }
            if (self.lowest > p.altitude) {
                self.lowest = p.altitude
            }
            let g = Geometry{location: p.location.clone(), alt: p.altitude};
            let d = if self.points.len() == 0 {
                0f64
            } else {
                self.points[self.points.len() - 1].geometry.distance(&g)
            };
            self.distance += d;
            self.area.enter(&p.location);
            let point = PointAttr{point: p.clone(), geometry: g, distance: d, direction: 0f64};
            self.points.push(point);
        }

        pub fn append(self: &mut Self, seg: &mut TrackSegment) {
            if (self.lowest > seg.lowest) {self.lowest = seg.lowest}
            if (self.highest < seg.highest) {self.highest = seg.highest}
            self.points.append(&mut seg.points);
            self.area.add(&seg.area);
        }

        pub fn split(self: &mut Self, i: usize) -> Option<(TrackSegment, TrackSegment)> {
            if (i != 0) && (i != (self.points.len() -1)) {
                let mut a = self.clone();
                let mut b = self.clone();
                b.points = a.points.split_off(i);
                a.reparse();
                b.reparse();
                Some((a, b))
            } else {
                None
            }
        }

        pub fn cut_in(self: &mut Self, i: usize, segment: &mut TrackSegment) {
            if (self.highest < segment.highest) {
                self.highest = segment.highest
            }
            if (self.lowest > segment.lowest) {
                self.lowest = self.lowest
            }
            if (self.points.len() == 0) {
                self.append(segment);
                self.distance = segment.distance
            } else if (i == 0) {
                let l = self.points.len();
                self.points[l - 1].distance
                    = self.points[l - 1].geometry.distance(&segment.points[0].geometry);
                self.distance += segment.distance + self.points[l - 1].distance;

                self.points[l - 1].direction
                    = self.points[l - 1].point.location.direction(&segment.points[0].point.location);

                let mut a = segment.points.clone();
                a.append(&mut self.points);
                self.points = a;
            } else {
                segment.points[0].distance = segment.points[0].geometry.distance(&self.points[i - 1].geometry);
                self.distance -= self.points[i].distance;
                self.points[i].distance
                    = self.points[i].geometry.distance(&segment.points[segment.points.len() - 1].geometry);
                let mut b = self.points.split_off(i);
                self.distance += self.points[i].distance;

                segment.points[0].direction
                    = segment.points[0].point.location.direction(&self.points[i - 1].point.location);
                self.points[i].direction
                    = self.points[i].point.location.distance(&segment.points[segment.points.len() - 1].point.location);
                self.points.append(&mut segment.points);
                self.points.append(&mut b);
            }
            self.area.add(&segment.area);
        }

        /// Follow and update point's following attributes
        ///     distance
        ///     direction of compass
        ///     maximum altitude
        ///     minimum altitude
        ///
        fn reparse(self: &mut Self) {
            self.distance = 0f64;
            self.highest = f64::NAN;
            self.lowest = f64::NAN;
            self.area = Area::invalid();

            // Reparse segment
            for i in 0..self.points.len() {
                // These 2 declarations are used in only else clause but borrow checker point of view, need to declare them here...
                let prev_g = self.points[i - 1].geometry.clone();
                let prev_l = self.points[i - 1].point.location.clone();

                let mut p = &mut self.points[usize::from(i)];
                if !f64::is_nan(p.point.altitude) && f64::is_nan(self.highest) {
                    self.highest = p.point.altitude
                } else if !f64::is_nan(p.point.altitude) && (self.highest <  p.point.altitude) {
                    self.highest = p.point.altitude
                }
                if !f64::is_nan(p.point.altitude) && f64::is_nan(self.lowest) {
                    self.lowest = p.point.altitude
                } else if f64::is_nan(p.point.altitude) && (self.lowest >  p.point.altitude) {
                    self.lowest = p.point.altitude
                }
                if !f64::is_nan(p.distance) {
                    self.distance += p.distance
                } else if (i != 0) {
                    p.distance = p.geometry.distance(&prev_g);
                    self.distance += p.distance;

                    p.direction = p.point.location.direction(&prev_l);
                }
                self.area.enter(&p.point.location);
            }
        }

        pub fn insert_at(self: &mut Self, p: TrackPoint, i: usize) {
            if (self.points.len() < i) {
                panic!("{i} is bigger than currently segment has")
            }
            if (self.highest < p.altitude) {
                self.highest = p.altitude
            }
            if (self.lowest > p.altitude) {
                self.lowest = p.altitude
            }
            let mut pt = PointAttr{point: p.clone(), distance: 0f64, geometry: Geometry{location: p.location.clone(), alt: p.altitude.clone()}, direction:0f64};
            if (self.points.len() != 0) {
                // Recalculate direction and distance
                if (i != (self.points.len() - 1)) {
                    // Not last
                    let mut n = &mut self.points[i + 1];
                    n.direction = n.geometry.location.distance(&p.location);
                    n.distance = n.geometry.distance(&pt.geometry);
                }
                if (i != 0) {
                    let n = &self.points[i - 1];
                    pt.direction = n.geometry.location.distance(&p.location);
                    pt.distance = n.geometry.distance(&pt.geometry);

                }
            }
            self.area.enter(&p.location);
            self.points.insert(i, pt);
        }

        pub fn remove_at(self: &mut Self, p: TrackPoint, i: usize) {
            if (self.points.len() < i) {
                panic!("{i} is bigger than currently segment has")
            }

            // Distance re-calculation
            if (self.points.len() > 1) {
                self.distance -= self.points[i + 1].distance;
                if (i == 0) {
                    self.points[i + 1].distance = 0f64;
                } else if (self.points.len() > 2) && (i != (self.points.len() - 1)) {
                    self.points[i + 1].distance = self.points[i + 1].geometry.distance(&self.points[i - 1].geometry);
                }
            }

            self.points.remove(i);

            if (p.altitude <= self.lowest) || (p.altitude >= self.highest) {
                // Recalc
                self.update_minmax();
            }
            if (p.location.lat > self.area.north_west.lat) {}
        }

        fn update_minmax(self: &mut Self) {

            for p in &self.points {
                if (p.point.altitude < self.lowest) {self.lowest = p.point.altitude;}
                if (p.point.altitude > self.highest) {self.highest = p.point.altitude;}
            }
        }
    }

    impl fmt::Display for crate::gpx::gpx::TrackSegment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(").unwrap();
            if (self.name != "") {write!(f, "name: {:?}", self.name).unwrap();}
            if (self.comment != "") {write!(f, "comment: {:?}", self.comment).unwrap();}
            for i in &self.points {
                write!(f, "{:?}", i).unwrap();
            }
            if (!self.highest.is_nan()) {write!(f, "time: {}", self.highest).unwrap();}
            if (!self.lowest.is_nan()) {write!(f, "time: {}", self.lowest).unwrap();}
            writeln!(f, ")")
        }
    }

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

        pub fn add_segment(self: &mut Self, segment: &TrackSegment) {
            if (segment.lowest < self.lowest) {
                self.lowest = segment.lowest;
            }

            if (segment.highest < self.highest) {
                self.highest = segment.highest;
            }
            self.segments.push(segment.clone());
            self.distance += segment.distance;
        }
    }

    impl fmt::Display for crate::gpx::gpx::TrackRoute {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(").unwrap();
            if (self.name != "") {write!(f, "name: {:?}", self.name).unwrap();}
            if (self.comment != "") {write!(f, "comment: {:?}", self.comment).unwrap();}
            for i in &self.segments {
                write!(f, "{:?}", i).unwrap();
            }
            if (!self.highest.is_nan()) {write!(f, "time: {}", self.highest).unwrap();}
            if (!self.lowest.is_nan()) {write!(f, "time: {}", self.lowest).unwrap();}
            if (!self.distance.is_nan()) {write!(f, "time: {}", self.distance).unwrap();}
            writeln!(f, ")")
        }
    }
    impl fmt::Debug for crate::gpx::gpx::TrackRoute {
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

    #[derive(Clone)]
    pub struct Track {
        pub routes: Vec<TrackRoute>,
        pub name: String,
        pub comment: String,
    }

    impl Track {
        pub fn new() -> Self {
            Self {
                routes: Vec::new(),
                name: String::new(),
                comment: String::new(),
            }
        }
    }

    impl fmt::Debug for crate::gpx::gpx::Track {
        fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Track")
                .field("name", &self.name)
                .field("comment", &self.comment)
                .finish()
        }
    }

    impl fmt::Display for crate::gpx::gpx::Track {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(").unwrap();
            if (self.name != "") {write!(f, "name: {:?}", self.name).unwrap();}
            if (self.comment != "") {write!(f, "comment: {:?}", self.comment).unwrap();}
            for i in &self.routes {
                write!(f, "{:?}", i).unwrap();
            }
            writeln!(f, ")")
        }
    }


}