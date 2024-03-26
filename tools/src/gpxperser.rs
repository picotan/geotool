pub mod gpx_parser {
    use std::f64::NAN;
    use std::ffi::OsString;
    use std::fs::File;
    use std::io::BufReader;
    use chrono::{DateTime, NaiveDateTime};
    use num::complex::ComplexFloat;
    use xml::attribute::OwnedAttribute;
    use xml::EventReader;
    use xml::reader::XmlEvent;
    use crate::gpx::gpx::*;
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
        pub fn open(self: &mut Self) -> Option<Track> {
            let mut route = self.process_gpx();
            let mut track = Track::new();
            match route {
                Some(x) => {
                    track.routes = x;
                    Some(track)
                },
                _ => {
                    None
                }
            }
        }
        pub fn process_gpx(self: &mut Self) -> Option<Vec<TrackRoute>> {
            let mut trk: Vec<TrackRoute> = Vec::new();

            loop {
                match self.parser.next() {
                    Ok(XmlEvent::StartElement { name, .. }) => {
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
                    Err(e) => {
                        eprintln!("Error on parse gpx: {e}");
                        return None;
                    }
                    // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                    _ => {}
                }
            };
            Some(trk)
        }
        fn process_route(self: &mut Self) -> Option<(TrackRoute)> {
            let mut route = TrackRoute::new();
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
                                "name" => { route.name = String::from(&message); }
                                "comment" => { route.comment = String::from(&message); }
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
                    Err(e) => {
                        eprintln!("Error on parse route: {e}");
                        return None;
                    }
                    _ => {
                    }
                }
            }
            Some(route.clone())
        }

        fn process_route_extensions(self: &mut Self, t: &mut TrackRoute) {
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
                    Err(e) => {
                        eprintln!("Error on parse route extensions: {e}");
                        return;
                    }
                    _ => {}
                }
            }
        }

        fn add_route_extensions(self: &mut Self, t: &mut TrackRoute, n: String) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::Characters(str)) => {
                        println!("Characters: {n} - {str}");
                        match n.as_str() {
                            "name" => { t.name = String::from(str); }
                            "comment" => { t.comment = String::from(str); }
                            _ => {}
                        }
                    }
                    Ok(XmlEvent::EndElement { name }) => {
                        if (name.local_name == n) {
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error on add route extensions: {e}");
                        return;
                    }
                    _ => {}
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
                                "name" => { segment.name = String::from(&message); }
                                "comment" => { segment.comment = String::from(&message); }
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
                    Ok(XmlEvent::Whitespace(_)) => {}
                    Err(e) => {
                        eprintln!("Error on parse segment: {e}");
                        return None;
                    }
                    _ => { println!("Unknown"); }
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
                    Err(e) => {
                        eprintln!("Error on point extensions: {e}");
                        return;
                    }
                    _ => {}
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
                    Err(e) => {
                        eprintln!("Error on track point extensions: {e}");
                        return;
                    }
                    _ => {}
                }
            }
        }

        fn apply_point(self: &mut Self, tag: &str, p: &mut TrackPoint) {
            loop {
                match self.parser.next() {
                    Ok(XmlEvent::Characters(str)) => {
                        match tag {
                            "name" => { p.name = String::from(str); }
                            "comment" => { p.comment = String::from(str); }
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
                            "weather" => { p.weather = Weather((str.parse::<u8>().unwrap()))}
                            "icon" => {

                            }
                            "time" => {
                                let a =  NaiveDateTime::parse_from_str(str.as_str(), "%Y-%m-%dT%H:%M:0%SZ");
                                let b =  NaiveDateTime::parse_from_str(str.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ");
                                let c =  NaiveDateTime::parse_from_str(str.as_str(), "%Y-%m-%dT%H:%M:%SZ");
                                let mut error: bool = true;
                                match a {
                                    Ok(x) => {
                                        p.time = DateTime::from_naive_utc_and_offset(x, Utc);
                                        error = false;
                                        continue;
                                    }
                                    _ => {}
                                }
                                match b {
                                    Ok(x) => {
                                        p.time = DateTime::from_naive_utc_and_offset(x, Utc);
                                        error = false;
                                        continue;
                                    }
                                    _ => {}
                                }
                                match c {
                                    Ok(x) => {
                                        p.time = DateTime::from_naive_utc_and_offset(x, Utc);
                                        error = false;
                                        continue;
                                    }
                                    _ => {}
                                }
                                if (error == true) {
                                    eprintln!("Error on parse time format");
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
                    Ok(XmlEvent::Whitespace (_)) => {}
                    _ => { println!("Someting wrong"); }
                }
            }
        }
    }
}