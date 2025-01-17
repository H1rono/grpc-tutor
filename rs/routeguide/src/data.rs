use std::{fs, io, path};

use crate::Feature;

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
