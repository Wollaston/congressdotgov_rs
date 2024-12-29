use std::{borrow::Cow, fmt::Display};

pub mod amendments;
pub mod bill;
pub mod bound_congressional_record;
pub mod committee;
pub mod committee_meeting;
pub mod committee_print;
pub mod committee_report;
pub mod congressional_record;
pub mod daily_congressional_record;
pub mod error;
pub mod hearing;
pub mod house_communication;
pub mod house_requirement;
pub mod law;
pub mod member;
pub mod summaries;

#[derive(Debug, Clone, Copy)]
pub enum CommitteeChamber {
    House,
    Senate,
    NoChamber,
}

impl CommitteeChamber {
    fn as_str(self) -> &'static str {
        match self {
            CommitteeChamber::House => "house",
            CommitteeChamber::Senate => "senate",
            CommitteeChamber::NoChamber => "nochamber",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Chamber {
    House,
    Senate,
    Joint,
}

impl Chamber {
    fn as_str(self) -> &'static str {
        match self {
            Chamber::House => "house",
            Chamber::Senate => "senate",
            Chamber::Joint => "joint",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ReportType {
    Hrpt,
    Srpt,
    Erpt,
}

impl ReportType {
    fn as_str(self) -> &'static str {
        match self {
            ReportType::Hrpt => "hrpt",
            ReportType::Srpt => "srpt",
            ReportType::Erpt => "erpt",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LawType {
    Public,
    Private,
}

impl LawType {
    fn as_str(self) -> &'static str {
        match self {
            LawType::Public => "pub",
            LawType::Private => "priv",
        }
    }
}

impl Default for LawType {
    fn default() -> Self {
        Self::Public
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BillType {
    Hr,
    S,
    Hjres,
    Sjres,
    Hconres,
    Sconres,
    Hres,
    Sres,
}

impl BillType {
    fn as_str(self) -> &'static str {
        match self {
            BillType::Hr => "hr",
            BillType::S => "s",
            BillType::Hjres => "hjres",
            BillType::Sjres => "sjres",
            BillType::Hconres => "hconres",
            BillType::Sconres => "sconres",
            BillType::Hres => "hres",
            BillType::Sres => "sres",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Xml,
}

impl Default for Format {
    fn default() -> Self {
        Self::Json
    }
}

impl From<Format> for Cow<'static, str> {
    fn from(format: Format) -> Self {
        format.to_string().into()
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Xml => write!(f, "xml"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Asc,
    Desc,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Asc
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}
