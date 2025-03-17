#![allow(clippy::module_inception)]

//! CrsReport API endpoints.

mod crsreport;
mod report_number;

pub use self::crsreport::{CrsReport, CrsReportBuilder, CrsReportBuilderError};
pub use self::report_number::{ReportNumber, ReportNumberBuilder, ReportNumberBuilderError};
