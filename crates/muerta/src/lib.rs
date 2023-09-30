#![feature(const_for)]
#![feature(core_intrinsics)]

pub mod bitbuf;
pub mod demofile;
pub mod entities;
pub mod entityclasses;
pub mod fielddecoder;
pub mod fieldmetadata;
pub mod fieldpath;
pub mod fieldvalue;
pub mod flattenedserializers;
pub mod fnv1a;
pub mod instancebaseline;
pub(crate) mod nohash;
pub mod quantizedfloat;
pub mod stringtables;
pub mod varint;
