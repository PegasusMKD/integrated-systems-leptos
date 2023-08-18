use leptos::*;

use uuid::Uuid;
use crate::models::{Ticket, TicketStatus, ViewSlot};


// TODO: Add proper error handling with status_code checks and custom errors (probably)
async fn get_data() -> reqwest::Result<Vec<Ticket>> {
    // Make this the official return after getting some data in the database
    let _data = reqwest::get("https://localhost:44316/api/ticket")
        .await?;
    
    // if !_data.status().is_success() {
    //    leptos::log!("Passed the get...");
    //    leptos::log!("Status: {:?}", _data.status());
    //    return Ok(Vec::new()); // TODO: Add proper error return so it can be handled down the line
    // }

    _data
        .json::<Vec<Ticket>>()
        .await?;

    Ok(    vec![
        Ticket {
            id: Uuid::new_v4(),
            seat_number: "A11".to_string(),
            price: 4.32,
            view_slot: ViewSlot {
                movie_name: "Test name".to_string(),
                time_slot: time::macros::datetime!(2019-01-01 0:00),
            },
            ticket_status: TicketStatus::Bought,
        }
    ])
}

#[component]
pub fn TicketItem(cx: Scope, record: Ticket) -> impl IntoView {
    view! {
        cx,
        <tr>
            <th scope="row">1</th>
            <td>{record.seat_number}</td>
            <td>{record.price} $</td>
            <td>{record.view_slot.movie_name} - {record.view_slot.time_slot.to_string()}</td>
            <td>{format!("{}", record.ticket_status)}</td>
        </tr>
    }
}

// TODO: Maybe use #[component(transparent)] and return routes here instead of a direct view, since
// depending on the action, we'd transfer them over to another page (aka CreateViewSlot,
// EditViewSlot, etc.)
#[component]
pub fn TicketsPage(cx: Scope) -> impl IntoView {
    // TODO: Add button to swap to "create" page
    // TODO: Add table of ViewSlots, each with their own associated action
    let resource: leptos::Resource<(), Vec<Ticket>> = create_resource(cx, || (), 
        |_| async move {
            match get_data().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<Ticket>::new()
                }
            }
        });
   
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
