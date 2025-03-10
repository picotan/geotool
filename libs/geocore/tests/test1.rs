
use std::ffi::OsString;
use std::hint::assert_unchecked;
use geocore::geometry::geometry_core::{LatLon, Geometry, TileCoord};
use geocore::gpx_parser::gpx_parser::Parser;
use geocore::gpx_writer::gpx_writer::Writer;

#[test]
fn main() {

    let z = 8;
    let a = LatLon::new(0f64, 0f64);
    let l1 = LatLon::new(35.543296, 139.641466);
    // L1 - L2 about 1km
    let l2 = LatLon::new(35.535823, 139.634943);

    // L1 - L3 about 10km
    let l3 = LatLon::new(35.457174, 139.606361);
    // L1 - L4 about 5.2km
    let l4 = LatLon::new(35.571471, 139.688416);
    //    let l1 = LatLon{lat: -33.870416, lon: 18.369141};
    // 3.27km
    let la = LatLon::new(35.128122, 135.799255);
    let lb = LatLon::new(35.108745, 135.826206);
    let c = TileCoord::tile_from_latlon(&l1, z);

    let track = match Parser::new(&OsString::from("samples/gpx/Garmin.gpx")) {
        Some(mut p) => p.open_parser().unwrap(),
        None => panic!("No file found"),
    };
    let mut writer = Writer::new("/tmp/a.gpx").unwrap();
    writer.write_gpx(&track);

    println!("Home tile: {:?}", c);
    println!("Distance <{:?}>m", lb.distance(&la));
    println!("Lat Lon: {:?}", c.latlon_from_tile());
    println!("Distance {}", Geometry{ location:lb, alt:100f64}.distance(&Geometry{ location:la,alt:0f64}));
    //    println!("{:?}", track);
    assert_eq!(2, 1+1);
}

fn max_tiles(z: u32) -> i32 {
    2i32.pow(z)
}

fn coord_to_pixel(x: f64, y: f64, z: u32) -> (i64, i64) {
    let total = (max_tiles(z) * 256) as f64;
    ((x * total).trunc() as i64, (y * total).trunc() as i64)
}

