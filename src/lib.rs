#[cfg(feature = "chrono")]
pub mod chrono;
mod duration_identifier;
mod parser;
#[cfg(feature = "time")]
pub mod time;
use chumsky::{error::Rich, span::SimpleSpan};
pub use duration_identifier::*;
pub use parser::*;
#[doc(hidden)]
pub type Span = SimpleSpan<usize>;
#[doc(hidden)]
pub type ErrType<'src> = chumsky::extra::Err<chumsky::error::Cheap>;

pub trait DurationType {
    fn from_sets_of_time(sets_of_time: Vec<SetOfTime>) -> Self;
    fn from_set_of_time(set_of_time: SetOfTime) -> Self;

    fn to_units(&self) -> Vec<SetOfTime>;
    fn to_duration_string(&self) -> String {
        self.to_units()
            .iter()
            .map(|unit| unit.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}
