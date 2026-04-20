//! Message system for leaving warnings across time loops.
//!
//! Players can leave warning signs for their future selves,
//! persisting knowledge across loop resets.

mod warning_signs;

pub use warning_signs::{MessageManager, WarningSign};
