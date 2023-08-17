use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TicketStatus {
    Bought
}

impl fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TicketStatus::Bought => write!(f, "Bought"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewSlot {
    pub movie_name: String,
    pub time_slot: DateTime<Utc> 
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub seat_number: String,
    pub price: f32,
    pub view_slot: ViewSlot,
    pub ticket_status: TicketStatus
}


