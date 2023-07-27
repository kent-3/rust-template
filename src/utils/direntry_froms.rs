//! Newtype pattern example

use crate::prelude::*;
use std::fs::DirEntry;

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;
    fn try_from(val: W<&DirEntry>) -> Result<Self> {
        val.0
            .path()
            .to_str()
            .map(Self::from)
            .ok_or_else(|| Error::Generic(f!("Invalid path {:?}", val.0)))
    }
}
