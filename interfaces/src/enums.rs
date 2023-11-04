use serde::{Deserialize, Serialize};

/// Represents a series's current status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeriesStatus {
    Ongoing,
    Completed,
    Cancelled,
}
