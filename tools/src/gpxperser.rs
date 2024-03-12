pub mod gpx_parser {
    use std::f64::NAN;
    use std::ffi::OsString;
    use std::fs::File;
    use std::io::BufReader;
    use std::time::SystemTime;
    use chrono::{DateTime, NaiveDateTime};
    use num::complex::ComplexFloat;
    use xml::attribute::OwnedAttribute;
    use xml::EventReader;
    use xml::name::OwnedName;
    use xml::reader::XmlEvent;
    use xml::reader::XmlEvent::ProcessingInstruction;
    use crate::geometry::geometry_core::{Geometry, LatLon};
    use crate::gpx::Gpx::*;
    use chrono::prelude::Utc;

    pub struct GPXParser {
        file: OsString,
        parser: EventReader<BufReader<File>>,
    }

    impl GPXParser {

        pub fn new(name: &OsString) -> Option<GPXParser> {
            let file = File::open(name).unwrap();
            let file = BufReader::new(file); // Buffering is important for performance
            Some(GPXParser{file: name.clone(), parser: EventReader::new(file)})
        }
        pub fn open(self: &mut Self) -> Option<Vec<Track>> {
            let mut trk: Vec<Track> = Vec::new();

            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                        match name.local_name.as_str() {
                            "trk" => { trk.push(self.process_route().unwrap()); }
                            "extensions" => {
                            }
                            "name" => {
                            }
                            _ => {
                            }
                        }
                    }
                    Ok(XmlEvent::EndDocument) => {
                        println!("End of document");
                        break;
                    }
                    _ => {}
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return None;
                    }
                    // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                    _ => {}
                }
            };
            Some(trk)
        }
        fn process_route(self: &mut Self) -> Option<(Track)> {
            let mut route = Track::new();
            let mut tag: String = String::new();
            let mut message = String::new();

            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement { name, .. }) => {
                        match name.local_name.as_str() {
                            "trkseg" => {
                                route.add_segment(&self.process_segment().unwrap());
                            }
                            "extensions" => {
                                self.process_route_extensions(&mut route);
                            }
                            _ => {
                                tag = name.local_name.clone();
                            }
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
//                        println!("{:spaces$}-{name}", "", spaces = depth * 2);
                        if (name.local_name == "trk") {
                            break;
                        }
                        let name = name.local_name;
                        if (tag == name) {
                            match name.as_str() {
                                "name" => { route.name = OsString::from(&message); }
                                "comment" => { route.comment = OsString::from(&message); }
                                _ => {

                                }
                            }
                        }
                    }
                    Ok(XmlEvent::Characters (str)) => {
                        message = str.clone();
                    }
                    Ok(XmlEvent::EndDocument) => {
                        eprintln!("Unexpected 'end of document'");
                        return None;
                    }
                    Ok(XmlEvent::Whitespace(str)) => {
                        println!("-> {str}");
                    }
                    _ => {
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return None;
                    }
                }
            }
            Some(route.clone())
        }

        fn process_route_extensions(self: &mut Self, t: &mut Track) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement { name, .. }) => {
                        self.add_route_extensions(t, name.local_name);
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name == "extensions") {
                            return;
                        }
                    }
                    _ => {}
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return;
                    }
                }
            }
        }

        fn add_route_extensions(self: &mut Self, t: &mut Track, n: String) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::Characters(str)) => {
                        println!("Characters: {n} - {str}");
                        match n.as_str() {
                            "name" => { t.name = OsString::from(str); }
                            "comment" => { t.comment = OsString::from(str); }
                            _ => {}
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name == n) {
                            return;
                        }
                    }
                    _ => {}
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return;
                    }
                }
            }
        }

        fn process_segment(self: &mut Self) -> Option<(TrackSegment)> {
            let mut segment = TrackSegment::new();
            let mut tag = String::new();
            let mut message = String::new();
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                        match name.local_name.as_str() {
                            "trkpt" => {
                                segment.add_point(self.process_point(attributes).unwrap());
                            }
                            _ => {
                                tag = name.local_name.clone();
                            }
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name == "trkseg") {
                            break;
                        }
                        let name = name.local_name;
                        if (tag == name) {
                            match name.as_str() {
                                "name" => { segment.name = OsString::from(&message); }
                                "comment" => { segment.comment = OsString::from(&message); }
                                _ => {

                                }
                            }
                        }

                    }
                    Ok(XmlEvent::Characters (str)) => { message = str.clone(); }
                    Ok(XmlEvent::StartDocument { .. }) => {
                        eprintln!("Unexpected 'start of document'");
                        return None;
                    }
                    Ok(XmlEvent::EndDocument) => {
                        eprintln!("Unexpected 'end of document'");
                        return None;
                    }
                    Ok(XmlEvent::Whitespace(str)) => {}
                    _ => { println!("Unknown"); }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return None;
                    }
                }
            }
            Some(segment.clone())
        }

        fn process_point(self: &mut Self, attr: Vec<OwnedAttribute>) -> Option<TrackPoint> {
            let mut lat: f64 = NAN;
            let mut lon: f64 = NAN;
            for a in attr {
                match a.name.local_name.as_str() {
                    "lat" => {
                        if (lat.is_nan()) { lat = a.value.parse().unwrap();}
                        else {return None;}
                    }
                    "lon" => {
                        if (lon.is_nan()) { lon = a.value.parse().unwrap();}
                        else {return None;}
                    }
                    _ => {return None; }
                }
            }
            let location: LatLon;
            if (lat.is_nan() || lon.is_nan()) {return None;}
            let mut point = TrackPoint::new(lat, lon);

            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement { name, .. }) => {
                        match name.local_name.as_str() {
                            "extensions" => { self.process_point_extensions(&mut point); }
                            _ => { self.apply_point(name.local_name.as_str(), &mut point);}
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
//                        println!("{:spaces$}-{name}", "", spaces = depth * 2);
                        if (name.local_name == "trkpt") {break;}
                    }
                    _ => {}
                }
            }
            Some(point)
        }

        fn process_point_extensions(self: &mut Self, p: &mut TrackPoint) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement {name, ..}) => {
                        match name.local_name.as_str() {
                            "TrackPointExtension" => {
                                // Garmin extension
                                self.process_track_point_extensions(p)
                            }
                            _ => {self.apply_point(name.local_name.as_str(), p);}
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name == "extensions") {
                            return;
                        }
                    }
                    _ => {}
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return;
                    }
                }
            }
        }
        fn process_track_point_extensions(self: &mut Self, p: &mut TrackPoint) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement {name, ..}) => {
                        self.apply_point(name.local_name.as_str(), p)
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name == "TrackPointExtension") {
                            return;
                        }
                    }
                    _ => {}
                    Err(e) => {
                        eprintln!("Error: {e}");
                        return;
                    }
                }
            }
        }

        fn apply_point(self: &mut Self, tag: &str, p: &mut TrackPoint) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::Characters(str)) => {
                        match tag {
                            "name" => { p.name = OsString::from(str); }
                            "comment" => { p.comment = OsString::from(str); }
                            "ele" => { p.altitude =  str.parse::<f64>().unwrap(); }
                            "alt" => { p.altitude = str.parse::<f64>().unwrap(); }
                            "altitude" => { p.altitude = str.parse::<f64>().unwrap(); }
                            "heading" => { p.heading = str.parse::<f64>().unwrap(); }
                            "pressure" => { p.pressure = str.parse::<f64>().unwrap(); }
                            "seaLevelPressure" => { p.pressure = str.parse::<f64>().unwrap(); }
                            "distance" => { p.distance = str.parse::<f64>().unwrap(); }
                            "energy" => { p.energy = str.parse::<f64>().unwrap(); }
                            "atemp" => { p.temperature = str.parse::<f64>().unwrap(); }
                            "temp" => { p.temperature = str.parse::<f64>().unwrap(); }
                            "temperature" => { p.temperature = str.parse::<f64>().unwrap(); }
                            "hr" => { p.heart_rate = str.parse::<f64>().unwrap(); }
                            "heart_rate" => { p.heart_rate = str.parse::<f64>().unwrap(); }
                            "luminance" => { p.luminance = str.parse::<f64>().unwrap(); }
                            "radiation" => { p.radiation = str.parse::<f64>().unwrap(); }
                            "speed" => {p.pace = str.parse::<f64>().unwrap()}
                            "verticalSpeed" => {p.vertical_speed = str.parse::<f64>().unwrap()}
                            "type" => {

                            }
                            "weather" => {

                            }
                            "icon" => {

                            }
                            "time" => {
                                let a =  NaiveDateTime::parse_from_str(str.as_str(), "%Y-%m-%dT%H:%M:0%SZ");
                                let b =  NaiveDateTime::parse_from_str(str.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ");
                                let c =  NaiveDateTime::parse_from_str(str.as_str(), "%Y-%m-%dT%H:%M:%SZ");
                                match a {
                                    Ok(x) => {
                                        p.time = DateTime::from_naive_utc_and_offset(x, Utc);
                                        continue;
                                    }
                                    Err(x) => {}
                                }
                                match b {
                                    Ok(x) => {
                                        p.time = DateTime::from_naive_utc_and_offset(x, Utc);
                                        continue;
                                    }
                                    Err(x) => {}
                                }
                                match c {
                                    Ok(x) => {
                                        p.time = DateTime::from_naive_utc_and_offset(x, Utc);
                                        continue;
                                    }
                                    Err(x) => {}
                                }

                            }
                            "cad" => {p.cadence = str.parse::<f64>().unwrap()}
                            _ => { println!("{tag} is not supported"); }
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name.as_str() == tag) {
                            return;
                        } else {
                            eprintln!( "Type mismatch {name}");
                        }
                    }
                    Ok(XmlEvent::Whitespace (str)) => {}
                    _ => { eprintln!("Someting wrong"); }
                }
            }
        }
    }
}