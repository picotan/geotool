pub mod gpx_writer {
    use std::fs::File;
    use std::io;
    use std::io::{Write};
    use chrono::{DateTime, Utc};
    use crate::gpx::gpx::*;
    use xmlwriter::XmlWriter;
    use xmlwriter::Options;
    use num::complex::ComplexFloat;
    use crate::gpx::gpx;

    pub struct GpxWriter {
        file: File,
    }

    impl GpxWriter {
        pub fn new(name: &str) -> io::Result<Self> {
            match File::create(name) {
                Ok(x) => {Ok(Self { file: x })},
                Err(e) => {Err(e)},
            }
        }

        pub fn write(self: &mut Self, track: &Track) {
            let opt = Options {
                use_single_quote: true,
                ..Options::default()
            };
            let mut writer= XmlWriter::new(opt);

            writer.start_element("gpx");
            writer.write_attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
            writer.write_attribute("xsi:schemaLocation", "http://www.topografix.com/GPX/1/1");
            writer.write_attribute("xmlns:gpxdata", "http://www.topografix.com/GPX/1/0");
            writer.write_attribute("xmlns", "http://www.topografix.com/GPX/1/1");
            writer.write_attribute("creator", "geotool");

            for route in &track.routes {
                self.write_route(&mut writer, &route);
            }
            if !track.comment.is_empty() || !track.name.is_empty() {
                writer.start_element("extensions");
                if !track.name.is_empty()  {
                    writer.start_element("name");
                    writer.write_text(&track.name);
                    writer.end_element();
                }
                if !track.comment.is_empty()  {
                    writer.start_element("comment");
                    writer.write_text(&track.comment);
                    writer.end_element();
                }
                writer.end_element();
            }
            writer.end_element();
            let a = writer.end_document();
            self.file.write(a.as_ref());
        }

        fn write_route(self: &mut Self, writer: &mut XmlWriter, route: &TrackRoute) {
            writer.start_element("trk");
            if !route.name.is_empty() {
                writer.start_element("name");
                writer.write_text(&route.name);
                writer.end_element();
            }
            if !route.segments.is_empty() {
                self.write_segments(writer, &route.segments);
            }
            writer.end_element();
        }

        fn write_segments(self: &mut Self, writer: &mut XmlWriter, segments: &Vec<TrackSegment>) {
            for seg in segments {
                writer.start_element("trkseg");
                if !seg.name.is_empty() {
                    writer.start_element("name");
                    writer.write_text(&seg.name);
                    writer.end_element();
                }
                if !seg.comment.is_empty() {
                    writer.start_element("extensions");
                    if !seg.comment.is_empty() {
                        writer.start_element("comment");
                        writer.write_text(&seg.comment);
                        writer.end_element();
                    }
                    writer.end_element();
                }
                if !seg.points.is_empty() { self.write_points(writer, &seg.points) }
                writer.end_element();
            }
        }
        fn write_points(self: &mut Self, writer: &mut XmlWriter, points: &Vec<PointAttr>) {
            for p in points {
                writer.start_element("trkpt");
                writer.write_attribute("lat", &p.point.location.lat);
                writer.write_attribute("lon", &p.point.location.lon);
                if !p.point.altitude.is_nan() {
                    writer.start_element("ele");
                    writer.write_text(&p.point.altitude.to_string());
                    writer.end_element();
                }
                if p.point.time != DateTime::<Utc>::MIN_UTC {
                    writer.start_element("time");
                    writer.write_text(&p.point.time.to_string());
                    writer.end_element();
                }
                if p.point.has_extension() {
                    writer.start_element("extensions");
                    if !p.point.comment.is_empty() {
                        writer.start_element("comment");
                        writer.write_text(&p.point.comment);
                        writer.end_element();
                    }
                    if !p.point.name.is_empty() {
                        writer.start_element("name");
                        writer.write_text(&p.point.name);
                        writer.end_element();
                    }
                    if !p.point.heading.is_nan() {
                        writer.start_element("heading");
                        writer.write_text(&p.point.heading.to_string());
                        writer.end_element();
                    }
                    if !p.point.pressure.is_nan() {
                        writer.start_element("pressure");
                        writer.write_text(&p.point.pressure.to_string());
                        writer.end_element();
                    }
                    if !p.point.temperature.is_nan() {
                        writer.start_element("temperature");
                        writer.write_text(&p.point.temperature.to_string());
                        writer.end_element();
                    }
                    if !p.point.heart_rate.is_nan() {
                        writer.start_element("heart_rate");
                        writer.write_text(&p.point.heart_rate.to_string());
                        writer.end_element();
                    }
                    if !p.point.luminance.is_nan() {
                        writer.start_element("luminance");
                        writer.write_text(&p.point.luminance.to_string());
                        writer.end_element();
                    }
                    if !p.point.radiation.is_nan() {
                        writer.start_element("radiation");
                        writer.write_text(&p.point.radiation.to_string());
                        writer.end_element();
                    }
                    if !p.point.energy.is_nan() {
                        writer.start_element("energy");
                        writer.write_text(&p.point.energy.to_string());
                        writer.end_element();
                    }
                    if !p.point.cadence.is_nan() {
                        writer.start_element("cadence");
                        writer.write_text(&p.point.cadence.to_string());
                        writer.end_element();
                    }
                    if !p.point.pace.is_nan() {
                        writer.start_element("pace");
                        writer.write_text(&p.point.pace.to_string());
                        writer.end_element();
                    }
                    if !p.point.vertical_speed.is_nan() {
                        writer.start_element("vertical_speed");
                        writer.write_text(&p.point.vertical_speed.to_string());
                        writer.end_element();
                    }
                    if p.point.weather != gpx::Weather(Weather::NONE) {
                        writer.start_element("vertical_speed");
                        writer.write_text(&p.point.vertical_speed.to_string());
                        writer.end_element();
                    }
                    if ((!p.point.wind.direction.is_nan()) && (p.point.wind.strong == u64::MIN)) {
                        writer.start_element("wind");
                        writer.write_attribute("direction", &p.point.wind.direction);
                        writer.write_attribute("strong", &p.point.wind.strong);
                        writer.end_element();
                    }
                    if (!p.point.point_type.is_empty()) {
                        writer.start_element("type");
                        for t in &p.point.point_type {
                            writer.start_element("value");
                            writer.write_text(&(*t as i64).to_string());
                            writer.end_element();
                        }
                        writer.end_element();
                    }
                    writer.end_element();
                }
                writer.end_element();
            }
        }
    }
}