
//
// Geometry calculate tools
//
// -- Coordinate system --
// [LatLon] <Whole> Lat: -90.0 ~ + 90.0 Lon: -180.0 ~ +180.0
// [Coord] <Whole> v: 0.0 ~ 1.0 u: 0.0 ~ 1.0
// [Tile] <Whole> tx: 0 ~ 2^z, tx: 0 ~ 2^z <Relative> x: 0.0 ~ 1.0, y: 0.0 ~ 1.0 <Scale> z: 0 ~ 23
// [Pixel] <Whole> x: 0 ~ 2^z - 1, y: 0 ~ 2^z - 1
//
pub mod geometry_core {
    use std::f64::consts::PI;
    use std::fmt;
    use num::complex::ComplexFloat;
    use num::integer::sqrt;

    #[derive(Debug, Clone, Copy)]
    pub struct LatLon {
        pub id: u64,
        pub lat: f64,
        pub lon: f64,
    }

    #[derive(Debug)]
    pub struct MapCoord {
        pub u: f64,
        pub v: f64
    }

    #[derive(Debug)]
    pub struct TileCoord {
        pub tx: i64,
        pub ty: i64,
        pub x: f64,
        pub y: f64,
        pub z: u32
    }

    #[derive(Clone, Debug)]
    pub struct Area {
        pub north_west: LatLon,
        pub south_east: LatLon,
    }

    impl Area {
        pub fn invalid() -> Area {
            Self {
                north_west: LatLon::new(-90f64, 180f64),
                south_east: LatLon::new( 90f64,  -180f64)
            }
        }

        pub fn enter(self: &mut Self, location: &LatLon) {
            if (location.lat > self.north_west.lat) {self.north_west.lat = location.lat}
            if (location.lon < self.north_west.lon) {self.north_west.lon = location.lon}
            if (location.lat < self.south_east.lat) {self.south_east.lat = location.lat}
            if (location.lon > self.south_east.lon) {self.south_east.lon = location.lon}
        }

        pub fn add(self: &mut Self, area: &Area) {
            if (area.north_west.lat > self.north_west.lat) {self.north_west.lat = area.north_west.lat}
            if (area.north_west.lon < self.north_west.lon) {self.north_west.lon = area.north_west.lon}
            if (area.south_east.lat < self.south_east.lat) {self.south_east.lat = area.south_east.lat}
            if (area.south_east.lon > self.south_east.lon) {self.south_east.lon = area.south_east.lon}
        }

        pub fn is_in(self: &Self, area: &Area) -> bool {
            if ((area.north_west.lat > self.north_west.lat) && (area.south_east.lat < self.south_east.lat)
                && (area.north_west.lon < self.north_west.lon) && (area.south_east.lon > self.south_east.lon)) {
                return true;
            }

            return false;
        }
    }

    const RADIUS: f64 = 6378137f64; // Earth Radius

    impl LatLon {
        pub fn new(lat: f64, lon: f64) -> LatLon{
            let la: u64 = ((lat + 180f64) * 1000000f64).trunc() as u64;
            let lo: u64 = ((lon + 90f64) * 1000000f64).trunc() as u64;
            LatLon{id: la * 0x100000000 + lo, lat: lat, lon: lon}
        }

        pub fn latlon(coord: MapCoord) -> LatLon {
            LatLon::new(( (1f64 - coord.v)).tanh().asin() * 180f64 / PI, coord.u * 360f64 - 180f64)
        }

        pub fn coord(&self) -> MapCoord {
            MapCoord{u: (self.lon + 180f64) / 360f64, v: 1f64 - (self.lat * PI / 180f64).sin().atanh()}
        }

        pub fn radian(&self) -> (f64, f64) {
            (self.lat * PI / 180f64, self.lon * PI / 180f64)
        }

        pub fn position(&self) -> (f64, f64, f64) {
            let t = self.radian();
            (t.0.cos() * t.1.cos(), t.0.sin(), t.0.cos() * t.1.sin())
        }

        pub fn distance(self: &Self, target: &LatLon) -> f64 {
            let phi1 = self.lat * PI / 180.0_f64;
            let phi2 = target.lat * PI / 180.0_f64;
            let lambda1 = self.lon * PI / 180.0_f64;
            let lambda2 = target.lon * PI / 180.0_f64;
            let dx = phi2.cos() * lambda2.cos() - phi1.cos() * lambda1.cos();
            let dy = phi2.cos() * lambda2.sin() - phi1.cos() * lambda1.sin();
            let dz = phi2.sin() - phi1.sin();
            return  (dx * dx + dy * dy + dz * dz).sqrt() * RADIUS;
        }

        pub fn direction(self, point: &LatLon) -> f64 {
            let lat = point.lat - self.lat;
            let lon = point.lon - self.lon;
            return (lat / lon).atan();
        }
    }

    impl fmt::Display for LatLon {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(Latitude:{}, Longitude:{})", self.lat, self.lon)
        }
    }

    impl MapCoord {
    }

    impl fmt::Display for MapCoord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(u:{}, v:{})", self.u, self.v)
        }
    }

    impl TileCoord {
        pub fn latlon_from_tile(&self) -> LatLon {
            let s: f64 = 1f64 / (2i32.pow(self.z) as f64);
            let coord = MapCoord{u: self.tx as f64 * s + self.x * s, v: self.ty as f64 * s + self.y * s};
            println!("UV = {:?}", coord);
            LatLon::latlon(coord)
        }

        pub fn tile_from_latlon(l: &LatLon, z: u32) -> TileCoord {
            let c = l.coord();
            let s: f64 = 1f64 / (2i32.pow(z)) as f64;
            TileCoord{tx: (c.u / s).trunc() as i64, ty: (c.v / s).trunc() as i64, x: (c.u % s) / s, y: (c.v % s) / s, z: z}
        }
    }

    impl fmt::Display for TileCoord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Tile(X:{}, Y:{}), SubCord(x:{}, y:{}) Zoom: {}", self.tx, self.ty, self.x, self.y, self.z)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Geometry {
        pub location: LatLon,
        pub alt: f64
    }

    impl Geometry {
        pub fn horizontal_distance(&self, p: &Geometry) -> f64 {
            let a = self.location.position();
            let b = p.location.position();
            // Simplified because (a.0 * a.0 + a.1 * a.1 + a.2 * a.2).sqrt() * (b.0 * b.0 + b.1 * b.1 * b.1 + b.2 * b.2).sqrt() ~= 1
            let cos = a.0 * b.0 + a.1 * b.1 + a.2 * b.2;
            cos.acos() * RADIUS
        }

        pub fn distance(&self, p: &Geometry) -> f64 {
            let d = self.horizontal_distance(p);
            let h = (self.alt - p.alt).abs();
            let min = self.alt.min(p.alt);
            (d * d + h * h).sqrt()
        }

        pub fn per_mill(&self, p: &Geometry) -> f64 {
            let d = self.horizontal_distance(p);
            if d == 0f64 {return 0f64};
            let h = (self.alt - p.alt).abs();
            1000f64 * h / d
        }
    }
}
