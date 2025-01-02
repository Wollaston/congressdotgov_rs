#![allow(clippy::module_inception)]

//! Bill API endpoints and types.

mod actions;
mod amendments;
mod bill;
mod bill_number;
mod bill_type;
mod committees;
mod congress;
mod cosponsors;
mod related_bills;
mod subjects;
mod summaries;
mod text;
mod titles;

pub use self::actions::{Actions, ActionsBuilder, ActionsBuilderError};
pub use self::amendments::{Amendments, AmendmentsBuilder, AmendmentsBuilderError};
pub use self::bill::{Bill, BillBuilder, BillBuilderError};
pub use self::bill_number::{BillNumber, BillNumberBuilder, BillNumberBuilderError};
pub use self::bill_type::{BillType, BillTypeBuilder, BillTypeBuilderError};
pub use self::committees::{Committees, CommitteesBuilder, CommitteesBuilderError};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::cosponsors::{Cosponsors, CosponsorsBuilder, CosponsorsBuilderError};
pub use self::related_bills::{RelatedBills, RelatedBillsBuilder, RelatedBillsBuilderError};
pub use self::subjects::{Subjects, SubjectsBuilder, SubjectsBuilderError};
pub use self::summaries::{Summaries, SummariesBuilder, SummariesBuilderError};
pub use self::text::{Text, TextBuilder, TextBuilderError};
pub use self::titles::{Titles, TitlesBuilder, TitlesBuilderError};
