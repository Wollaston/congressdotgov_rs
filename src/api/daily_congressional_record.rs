#![allow(clippy::module_inception)]

//! Daily-congressional-record API endpoints.

mod articles;
mod daily_congressional_record;
mod issue_number;
mod volume_number;

pub use self::articles::{Articles, ArticlesBuilder, ArticlesBuilderError};
pub use self::daily_congressional_record::{
    DailyCongressionalRecord, DailyCongressionalRecordBuilder, DailyCongressionalRecordBuilderError,
};
pub use self::issue_number::{IssueNumber, IssueNumberBuilder, IssueNumberBuilderError};
pub use self::volume_number::{VolumeNumber, VolumeNumberBuilder, VolumeNumberBuilderError};
