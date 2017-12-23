#![feature(collections_range)]
#![feature(slice_get_slice)]
#![feature(inclusive_range)]
#![feature(try_from)]
#![feature(inclusive_range_syntax)]

#[cfg(feature = "sem_string")]
extern crate itertools;

pub mod iter;
pub mod text;
pub mod slice;
pub mod time;