#![allow(clippy::module_inception)]

//! Congressional-record API endpoint.

mod congressional_record;

pub use self::congressional_record::{
    CongressionalRecord, CongressionalRecordBuilder, CongressionalRecordBuilderError,
};
