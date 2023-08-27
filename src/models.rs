use reqwest::RequestBuilder;
use uuid::Uuid;
use serde::*;
use serde_repr::*;
use time::{PrimitiveDateTime, macros::format_description};
use std::{fmt, str::FromStr};
use crate::constants::*;

use gloo_storage::{LocalStorage, Storage};

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
    #[serde(rename(serialize = "guid", deserialize = "guid"))]
    pub id: Option<Uuid>,

    #[serde(rename(serialize = "movieName", deserialize = "movieName"))]
    pub movie_name: String,
    
    #[serde(rename(serialize = "timeSlot", deserialize = "timeSlot"), with="standard_format")]
    pub time_slot: PrimitiveDateTime,

    pub genre: Option<Genre>,

    #[serde(rename(serialize = "genreId", deserialize = "genreId"))]
    pub genre_id: Option<usize>
}


impl ViewSlot {
    
    pub fn new() -> ViewSlot {
        ViewSlot::from("".to_string(), Genre::new(), "".to_string())
    }

    
    pub fn from(movie_name: String, genre: Genre, time_slot: String) -> ViewSlot {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]");
        leptos::log!("{:?}", time_slot);
        let slot = format!("{}:00.01", &time_slot).to_string();
        let selected = PrimitiveDateTime::parse(&slot, &format).unwrap();
        let genre_id = genre.id;
        ViewSlot {id: None, movie_name, time_slot: selected, genre: Some(genre), genre_id: Some(genre_id) }
    }


    pub fn from_full(id: String, movie_name: String, genre: Genre, time_slot: String) -> ViewSlot {
        let mut slot = ViewSlot::from(movie_name, genre, time_slot);
        if !id.is_empty() {
            slot.id = Some(uuid::Uuid::from_str(id.as_str()).unwrap());
        }

        slot
    }
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

impl Genre {

    pub fn new() -> Genre {
        Genre {
            id: 1,
            name: "Test".to_string()
        }
    }

    pub fn from(genre: String, genres: Vec<Genre>) -> Genre {
        genres.iter()
            .filter(|gen| gen.name == genre).nth(0)
            .unwrap().clone()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginDetails {
    pub email: String,
    pub password: String
}

impl LoginDetails {
    
    pub fn new(email: String, password: String) -> LoginDetails {
        LoginDetails { email, password }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDetails {
    pub token: String,
    pub expiration: String,
    pub roles: Vec<String>,

    #[serde(alias = "userName")]
    pub username: String
}

impl UserDetails {
    
    pub fn save(self) {
        LocalStorage::set("token", self.token).unwrap();
        LocalStorage::set("username", self.username).unwrap();
        LocalStorage::set("roles", self.roles).unwrap();
    }

    pub fn read_detail(key: String) -> String {
        LocalStorage::get(key).unwrap()
    }

    pub fn delete() {
        LocalStorage::delete("token");
        LocalStorage::delete("username");
        LocalStorage::delete("roles");
    }

    pub fn user_logged_in() -> bool {
        let token: gloo_storage::Result<String> = LocalStorage::get("token");
        let username: gloo_storage::Result<String> = LocalStorage::get("username");
        let roles: gloo_storage::Result<Vec<String>> = LocalStorage::get("roles");
        
        return !(token.is_err() || username.is_err() || roles.is_err());
    }
}

pub trait BearerRequestBuilder {
    fn add_token(self) -> Self;
}

impl BearerRequestBuilder for RequestBuilder {
    fn add_token(self) -> Self {
        self.bearer_auth(UserDetails::read_detail("token".to_string()))
    }
}
