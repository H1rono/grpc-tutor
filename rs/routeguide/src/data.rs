use std::{fs, hash, io, path};

use serde::{de, ser, Deserialize, Serialize};

use crate::{Feature, Point};

// MARK: Point

impl hash::Hash for Point {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct SerdePoint {
    latitude: i32,
    longitude: i32,
}

impl From<Point> for SerdePoint {
    fn from(value: Point) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}

impl From<SerdePoint> for Point {
    fn from(value: SerdePoint) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}

impl ser::Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        SerdePoint::from(*self).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        SerdePoint::deserialize(deserializer).map(Point::from)
    }
}

// MARK: Feature

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
struct SerFeature<'a> {
    name: &'a str,
    location: &'a Point,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
struct DeFeature {
    name: String,
    location: Point,
}

impl From<DeFeature> for Feature {
    fn from(value: DeFeature) -> Self {
        Self {
            name: value.name,
            location: Some(value.location),
        }
    }
}

impl<'a> From<&'a Feature> for SerFeature<'a> {
    fn from(value: &'a Feature) -> Self {
        Self {
            name: &value.name,
            location: value.location.as_ref().unwrap(), // FIXME
        }
    }
}

impl ser::Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        SerFeature::from(self).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Feature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        DeFeature::deserialize(deserializer).map(Feature::from)
    }
}

// MARK: load db

pub struct DbLoader<R = ()> {
    reader: R,
}

impl DbLoader {
    fn new() -> Self {
        Self { reader: () }
    }

    pub fn open(self, path: impl AsRef<path::Path>) -> io::Result<DbLoader<fs::File>> {
        fs::File::open(path).map(|r| self.with_reader(r))
    }

    pub fn with_reader<R>(self, reader: R) -> DbLoader<R>
    where
        R: io::Read,
    {
        DbLoader { reader }
    }
}

impl<R> DbLoader<R>
where
    R: io::Read,
{
    pub fn load(self) -> serde_json::Result<Vec<Feature>> {
        serde_json::from_reader(self.reader)
    }
}

impl Feature {
    pub fn db_loader() -> DbLoader {
        DbLoader::new()
    }
}
