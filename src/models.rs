use uuid::Uuid;
use serde::*;
use serde_repr::*;
use time::{PrimitiveDateTime, macros::format_description, Date};
use std::fmt;
use crate::constants::*;

time::serde::format_description!(standard_format, PrimitiveDateTime, DATE_TIME_FORMAT);

#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TicketStatus {
    Available = 0,
    Bought = 1,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterTicketsByDates {
    #[serde(rename(serialize = "fromTimeSlot"), with = "standard_format::option")]
    pub from_date: Option<PrimitiveDateTime>,

    #[serde(rename(serialize = "toTimeSlot"), with = "standard_format::option")]
    pub to_date: Option<PrimitiveDateTime>
}

impl FilterTicketsByDates {

    pub fn new(from: String, to: String) -> FilterTicketsByDates {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]");
        let mut filter = FilterTicketsByDates {
            from_date: None,
            to_date: None
        };

        if !from.is_empty() {
            if let Ok(date) = PrimitiveDateTime::parse(&from, &format) {
                filter.from_date = Some(date);
            }
        }

        if !to.is_empty() {
            if let Ok(date) = PrimitiveDateTime::parse(&to, &format) {
                filter.to_date = Some(date);
            }
        }

        filter
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub id: usize,
    pub name: String
}
