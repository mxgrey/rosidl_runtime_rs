#![warn(missing_docs)]
//! Bindings to `rosidl_runtime_c` and related functionality for messages.

#[macro_use]
mod sequence;
pub use sequence::{BoundedSequence, Sequence, SequenceExceedsBoundsError};

mod string;
pub use string::{BoundedString, BoundedWString, String, StringExceedsBoundsError, WString};

mod traits;
pub use traits::*;

#[cfg(feature = "schemars")]
mod big_array_schema;
#[cfg(feature = "schemars")]
pub use big_array_schema::*;
