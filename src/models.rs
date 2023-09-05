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

impl Ticket {
   
    // Serves as simple new empty ticket for now
    pub fn new() -> Ticket {
        Ticket { id: Uuid::new_v4(), seat_number: "".to_string(), price: 0.0, view_slot: ViewSlot::new(), ticket_status: TicketStatus::Available }
    }
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cart {
    pub tickets: Vec<Ticket>
}

impl Cart {
    pub fn new() -> Cart {
        Cart { tickets: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrder {
    pub name: String,

    #[serde(rename(serialize = "expirationMonth", deserialize = "expirationMonth"))]
    pub expiration_month: u8,

    #[serde(rename(serialize = "expirationYear", deserialize = "expirationYear"))]
    pub expiration_year: u16,

    #[serde(rename(serialize = "cardNumber", deserialize = "cardNumber"))]
    pub card_number: u64,
    
    pub cvc: String
}

impl CreateOrder {

    pub fn new(name: String, date: String, cvc: String, card_number: String) -> CreateOrder {
        leptos::log!("Card Number: {}", card_number);
        let c_number = card_number.trim().parse::<u64>().unwrap();
        leptos::log!("Date: {}", date);
        let mut date_values = date.split("/").map(|val| val.trim().parse::<u8>().unwrap());
        let expiration_month = date_values.next().unwrap();
        let expiration_year = date_values.next().unwrap() as u16 + 2000;
        CreateOrder { name, expiration_month, expiration_year, card_number: c_number, cvc }
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(alias = "guid")]
    pub id: Uuid,

    #[serde(alias = "orderNumber")]
    pub order_number: u32,

    #[serde(alias = "totalPrice")]
    pub total_price: f32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    
    #[serde(alias = "userName")]
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    #[serde(alias = "emailConfirmed")]
    pub email_confirmed: bool
}

impl User {
    
    pub fn new() -> User {
        User { id: None, username: String::new(), email: String::new(), roles: Vec::new(), email_confirmed: false }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub id: Option<Uuid>,
    
    #[serde(rename(serialize = "userName"))]
    pub username: String,
    pub email: String,
    
    // Only update related fields
    #[serde(rename(serialize = "currentPassword"))]
    pub current_password: Option<String>,

    #[serde(rename(serialize = "newPassword"))]
    pub new_password: Option<String>,

    #[serde(rename(serialize = "emailConfirmed"))]
    pub email_confirmed: bool,
    pub role: String,
}

impl UpdateUserDto {
    
    pub fn configure_from_user(&mut self, user: User) {
        self.id = user.id;
        self.email_confirmed = user.email_confirmed;
        self.email = user.email;
    }

    pub fn create_from_user(user: User, username: String, current_password: String, new_password: String, role: String) -> UpdateUserDto {
        let mut data = UpdateUserDto::default();
        data.configure_from_user(user);
        data.username = username;
        data.role = role;
        if current_password.is_empty() || new_password.is_empty() {
            return data;
        }
        data.current_password = Some(current_password);
        data.new_password = Some(new_password);
        data
    }
}

impl Default for UpdateUserDto {

    fn default() -> UpdateUserDto {
        UpdateUserDto { 
            id: None,
            username: String::new(),
            email: String::new(),
            current_password: None,
            new_password: None,
            email_confirmed: false, 
            role: String::new() 
        }
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
