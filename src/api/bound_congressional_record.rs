#![allow(clippy::module_inception)]

//! Bound-congressional-record API endpoints.

mod bound_congressional_record;
mod day;
mod month;
mod year;

pub use self::bound_congressional_record::{
    BoundCongressionalRecord, BoundCongressionalRecordBuilder, BoundCongressionalRecordBuilderError,
};
pub use self::day::{Day, DayBuilder, DayBuilderError};
pub use self::month::{Month, MonthBuilder, MonthBuilderError};
pub use self::year::{Year, YearBuilder, YearBuilderError};
