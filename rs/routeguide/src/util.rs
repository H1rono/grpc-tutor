use std::hash;

use crate::{Point, Rectangle};

impl hash::Hash for Point {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}

impl Point {
    pub fn distance_between(&self, other: &Point) -> f64 {
        let diff_latitude = self.latitude as f64 - other.latitude as f64;
        let diff_longitude = self.longitude as f64 - other.longitude as f64;
        f64::sqrt(diff_latitude * diff_latitude + diff_longitude * diff_longitude)
    }
}

impl Rectangle {
    pub fn contains(&self, point: &Point) -> bool {
        let Some((lo, hi)) = self.lo.as_ref().zip(self.hi.as_ref()) else {
            return false;
        };
        let lat_min = i32::min(lo.latitude, hi.latitude);
        let lat_max = i32::max(lo.latitude, hi.latitude);
        let contain_lat = lat_min <= point.latitude && point.latitude <= lat_max;
        let long_min = i32::min(lo.longitude, hi.longitude);
        let long_max = i32::max(lo.longitude, hi.longitude);
        let contain_long = long_min <= point.longitude && point.longitude <= long_max;
        contain_lat && contain_long
    }
}
