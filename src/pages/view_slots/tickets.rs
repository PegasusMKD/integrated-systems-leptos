pub mod index;
pub mod create;
pub mod edit;
pub mod create_multiple;

use serde::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketUpdateEvent {
    #[serde(rename(serialize = "guid", deserialize = "guid"))]
    pub id: Option<String>,
    
    #[serde(rename(serialize = "xTickets", deserialize = "xTickets"))]
    pub x_tickets: Option<i32>,

    #[serde(rename(serialize = "viewSlotId", deserialize = "viewSlotId"))]
    pub view_slot: String,
    pub price: i32
}

impl TicketUpdateEvent {

    pub fn multiple(view_slot: String, price: i32, x_tickets: i32) -> TicketUpdateEvent {
        let mut event = TicketUpdateEvent::create(view_slot, price);
        event.x_tickets = Some(x_tickets);
        return event;
    }

    pub fn update(id: String, view_slot: String, price: i32) -> TicketUpdateEvent {
        let mut event = TicketUpdateEvent::create(view_slot, price);
        event.id = Some(id);
        return event;
    }

    pub fn create(view_slot: String, price: i32) -> TicketUpdateEvent {
        TicketUpdateEvent { id: None, x_tickets: None, view_slot, price }
    }

}
