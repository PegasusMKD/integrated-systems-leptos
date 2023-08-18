use std::fmt;
use serde::*;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TicketStatus {
    Bought = 0,
    Available = 1
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TicketStatus::Bought => write!(f, "Bought"),
            TicketStatus::Available => write!(f, "Available"),
        }
    }
}

