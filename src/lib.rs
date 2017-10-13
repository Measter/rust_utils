#![feature(collections_range)]
#![feature(slice_get_slice)]

#[cfg(feature = "sem_string")]
extern crate itertools;

pub mod iter;
pub mod text;
pub mod slice;
pub mod time;