
pub mod map {
    use std::ffi::OsString;
    use crate::geometry::geometry_core::{*};

    struct Map {
        center: LatLon,
        zoom_min: u64,
        zoom_max: u64,
        zoom: u32,
        sub_scale: f64,
        window_size: (u64, u64),    // in pixel
    }

    impl Map {
        #[unsafe(no_mangle)]
        pub extern fn get_file(geo: &LatLon, zoom: u32) -> Option<OsString> {
            let t = TileCoord::tile_from_latlon(geo, zoom);
            let s = std::ffi::OsStr::new("{x?t.x}{x?t.y}{x?t.z}");
            None
        }
    }
}
