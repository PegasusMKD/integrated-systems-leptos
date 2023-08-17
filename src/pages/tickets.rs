use leptos::*;

use chrono::prelude::*;

use uuid::Uuid;
use crate::models::{Ticket, TicketStatus, ViewSlot};

#[component]
pub fn TicketItem(cx: Scope, record: Ticket) -> impl IntoView {
    view! {
        cx,
        <tr>
            <th scope="row">1</th>
            <td>{record.seat_number}</td>
            <td>{record.price} $</td>
            <td>{record.view_slot.movie_name} - {record.view_slot.time_slot.format("%Y-%m-%d %H:%M:%S").to_string()}</td>
            <td>{format!("{}", record.ticket_status)}</td>
        </tr>
    }
}

async fn get_data() -> Vec<Ticket> {
    // Make this the official return after getting some data in the database
    let _data = reqwest::get("https://localhost:44316/api/ticket")
        .await.unwrap()
        .json::<Vec<Ticket>>()
        .await.unwrap();
    
    vec![
        Ticket {
            id: Uuid::new_v4(),
            seat_number: "A11".to_string(),
            price: 4.32,
            view_slot: ViewSlot {
                movie_name: "Test name".to_string(),
                time_slot: Utc::now(),
            },
            ticket_status: TicketStatus::Bought,
        }
    ]
}

// TODO: Maybe use #[component(transparent)] and return routes here instead of a direct view, since
// depending on the action, we'd transfer them over to another page (aka CreateViewSlot,
// EditViewSlot, etc.)
#[component]
pub fn TicketsPage(cx: Scope) -> impl IntoView {
    // TODO: Add button to swap to "create" page
    // TODO: Add table of ViewSlots, each with their own associated action
    let resource: leptos::Resource<(), Vec<Ticket>> = create_resource(cx, || (), |_| async move { get_data().await });
   
    // So signals are essentially like useState in React
    let (data, set_data) = create_signal(cx, Vec::<Ticket>::new());

    let tickets_data_table = move || {
        let value = resource.read(cx);
        match value {
            None => set_data.set(Vec::new()),
            Some(val) => set_data.set(val)
        };

        view! {cx,
            <For 
                each = move || data.get()
                key = |record: &Ticket| record.id
                view = move |cx, record: Ticket| { view! { cx, <TicketItem record/> } }
            />    
        }
    };

    view! {
        cx,
        <div class="h-screen justify-items-center">
            <Transition fallback=move || { view! {cx, <div></div> } }>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">#</th>
                            <th scope="col">Seat Number</th>
                            <th scope="col">Price</th>
                            <th scope="col">View Slot</th>
                            <th scope="col">Availability</th>
                        </tr>
                    </thead>
                    <tbody>
                        { tickets_data_table }                         
                    </tbody>
                </table>
            </Transition>
        </div>

    }
}
