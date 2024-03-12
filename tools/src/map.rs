pub mod map {

    use crate::geometry::geometry_core::LatLon;
    use crate::geometry::geometry_core::TileCoord;
    use crate::geometry::geometry_core::Geometry;

    struct Map {
        center: LatLon,
        zoom_min: u64,
        zoom_max: u64,
        zoom: u32,
        sub_scale: f64,
        window_size: (u64, u64),    // in pixel
    }

    impl Map {

    }

}