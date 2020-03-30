extern crate chrono;
#[macro_use]
extern crate failure;

pub mod error;
pub mod week;

pub use week::CustomWeek;
pub use week::WeekSpecification;