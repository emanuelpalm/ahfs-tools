//! This module holds functionality related to maintaining and analyzing AHF
//! specification data.

mod predicate;
mod triple;
mod model;

pub use self::predicate::Predicate;
pub use self::triple::Triple;
pub use self::model::Model;