use uuid::Uuid;
use serde::*;
use serde_repr::*;
use time::PrimitiveDateTime;
use std::fmt;
use crate::constants::DATE_TIME_FORMAT;

time::serde::format_description!(standard_format, PrimitiveDateTime, DATE_TIME_FORMAT);


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize_repr)]
#[repr(u8)]
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


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewSlot {
    #[serde(alias = "movieName")]
    pub movie_name: String,
    
    #[serde(alias = "timeSlot", with="standard_format")]
    pub time_slot: PrimitiveDateTime
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(alias = "guid")]
    pub id: Uuid,
    
    #[serde(alias = "seatNumber")]
    pub seat_number: String,
    
    pub price: f32,
    
    #[serde(alias = "viewSlot")]
    pub view_slot: ViewSlot,

    #[serde(alias = "ticketStatus")]
    pub ticket_status: TicketStatus
}


