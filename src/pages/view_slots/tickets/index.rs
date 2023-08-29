use leptos::*;

use leptos_router::{use_params_map, A};
use crate::pages::tickets::TicketItem;
use crate::models::{Ticket, ViewSlot};

use time::format_description;

use super::super::get_view_slot;

// TODO: Add proper error handling with status_code checks and custom errors (probably)
async fn filter_tickets_by_view_slot(view_slot: String) -> reqwest::Result<Vec<Ticket>> {
    // Make this the official return after getting some data in the database
    let request = reqwest::get(format!("https://localhost:44316/api/ticket/by-view-slot/{}", view_slot))
        .await?;
    
    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(Vec::new()); // TODO: Add proper error return so it can be handled down the line
    }
    
    request
        .json::<Vec<Ticket>>()
        .await
}

#[component]
pub fn ViewSlotDetails(cx: Scope, id: ReadSignal<String>) -> impl IntoView {
    let (movie_name, set_movie_name) = create_signal(cx, "".to_string());
    let (genre, set_genre) = create_signal(cx, "".to_string());
    let (time_slot, set_time_slot) = create_signal(cx, "".to_string());

    let view_slot_resource: leptos::Resource<String, ViewSlot> = create_resource(cx, 
        move || id.get(), 
        |uid| async move {
            match get_view_slot(uid).await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[View Slot Page] Error doing a request to fetch genres: {:?}", err);
                    ViewSlot::new()
                }
            }
        });
  
    let view_slot_data = move || {
        let value = view_slot_resource.read(cx);
        match value {
            None => {},
            Some(val) => {
                let format = format_description::parse("[year]-[month]-[day]T[hour]:[minute]").unwrap();
                set_movie_name.set(val.movie_name);
                set_genre.set(val.genre.unwrap().name);
                set_time_slot.set(val.time_slot.format(&format).unwrap());
            }
        };

    };

    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <h1 class="font-bold text-3xl mb-2">Details</h1>
            { view_slot_data }
            <div class="border-y-2 border-neutral-600 mb-2">
                <div class="p-4 w-1/4 grid grid-rows-3 text-lg">
                    <div class="grid grid-cols-2">
                        <p class="font-semibold">Movie Name</p>
                        <p>{ move || movie_name.get() }</p>
                    </div>
                    <div class="grid grid-cols-2">
                        <p class="font-semibold">Genre</p>
                        <p>{ move || genre.get() }</p>
                    </div> 
                    <div class="grid grid-cols-2">
                        <p class="font-semibold">Time Slot</p>
                        <p>{ move || time_slot.get() }</p>
                    </div> 
                </div>
            </div>
            <div class="flex flex-row mb-2">
                <A href="create"><button class="text-white mx-4 bg-sky-700 hover:bg-sky-800 focus:ring-4 focus:outline-none focus:ring-sky-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-sky-600 dark:hover:bg-sky-700 dark:focus:ring-sky-800">Create ticket</button></A>
                <A href="create-multiple"><button class="text-white bg-sky-700 hover:bg-sky-800 focus:ring-4 focus:outline-none focus:ring-sky-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-sky-600 dark:hover:bg-sky-700 dark:focus:ring-sky-800">Create multiple tickets</button></A>
            </div>
        </Transition>
    }
}

#[component]
pub fn  ViewSlotTicketsTable(cx: Scope, id: ReadSignal<String>) -> impl IntoView {
    let refresh_trigger = create_rw_signal(cx, false);
    let uid = create_memo(cx, move |_| id.get());
    let resource: leptos::Resource<bool, Vec<Ticket>> = create_resource(cx, 
        move || refresh_trigger.get(), 
        move |_| async move {
            leptos::log!("Also fetching data... UUUUH");
            match filter_tickets_by_view_slot(uid.get_untracked()).await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<Ticket>::new()
                }
            }
        });
  
    let (data, set_data) = create_signal(cx, Vec::<Ticket>::new());

    let tickets_data_table = move || {
        let value = resource.read(cx);
        match value {
            None => set_data.set(Vec::new()),
            Some(val) => set_data.set(val)
        };
    };

    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <table class="w-full flex-row text-sm text-center rounded-lg bordertext-gray-500">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50">
                    <tr class="border-y rounded-t-lg border-gray-800">
                        // <th scope="col" class="px-6 py-3">#</th>
                        <th scope="col" class="px-6 py-3">Seat Number</th>
                        <th scope="col" class="px-6 py-3">Price</th>
                        <th scope="col" class="px-6 py-3">View Slot (Movie Name - Time Slot)</th>
                        <th scope="col" class="px-6 py-3">Availability</th>
                    </tr>
                </thead>
                <tbody>
                    { tickets_data_table }                         
                    <For 
                        each = move || data.get()
                        key = |record: &Ticket| record.id
                        view = move |cx, record: Ticket| {
                            view! { cx, <TicketItem record refresh_trigger={Some(refresh_trigger)} actions=true/> } 
                        }
                    />    
                </tbody>
            </table>
        </Transition>
    }
}

#[component]
pub fn ViewSlotTicketsPage(cx:Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let get_id = move || { params.with_untracked(|params_map| params_map.get("id").cloned().unwrap_or_default()) };
    let (id_sig, _ ) = create_signal(cx, get_id());

    view! {cx,
            <ViewSlotDetails id={ id_sig }/>
            <ViewSlotTicketsTable id={ id_sig }/>
    }
}

