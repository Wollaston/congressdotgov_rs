#![allow(clippy::module_inception)]

mod congressional_record;

pub use self::congressional_record::{
    CongressionalRecord, CongressionalRecordBuilder, CongressionalRecordBuilderError,
};
