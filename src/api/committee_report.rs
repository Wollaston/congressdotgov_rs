#![allow(clippy::module_inception)]

mod committee_report;
mod congress;
mod report_number;
mod report_type;
mod text;

pub use self::committee_report::{
    CommitteeReport, CommitteeReportBuilder, CommitteeReportBuilderError,
};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::report_number::{ReportNumber, ReportNumberBuilder, ReportNumberBuilderError};
pub use self::report_type::{ReportType, ReportTypeBuilder, ReportTypeBuilderError};
pub use self::text::{Text, TextBuilder, TextBuilderError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitteeReportType {
    Hrpt,
    Srpt,
    Erpt,
}

impl CommitteeReportType {
    fn as_str(self) -> &'static str {
        match self {
            CommitteeReportType::Hrpt => "hrpt",
            CommitteeReportType::Srpt => "srpt",
            CommitteeReportType::Erpt => "erpt",
        }
    }
}
