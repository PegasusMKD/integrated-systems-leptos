use leptos::*;

use crate::services::fetch_genres;
use crate::pages::tickets::TicketItem;
use crate::models::{Ticket, Genre};

async fn filter_tickets_by_genre(genre: String) -> reqwest::Result<Vec<Ticket>> {
    // Make this the official return after getting some data in the database
    //let data = FilterTickets::new(from_date, to_date);
    let client = reqwest::Client::new();
    leptos::log!("Data is {:?}", genre);
    let mut request = client.get("https://localhost:44316/api/ticket/by-genre");
    if !genre.is_empty() {
        request = request
        .query(&[("genre", genre)]);
    }
    let response = request.send().await?;
    
    leptos::log!("Getting the requested data...");
    if !response.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", response.status());
        return Ok(Vec::new()); // TODO: Add proper error return so it can be handled down the line
    }
    
    response
        .json::<Vec<Ticket>>()
        .await
}

#[component]
pub fn GenreFilter(cx: Scope, trigger_filter: RwSignal<bool>, genre: RwSignal<String>) -> impl IntoView {
    let genres = create_rw_signal(cx, Vec::new());
    let resource: leptos::Resource<(), Vec<Genre>> = create_resource(cx, 
        || (), 
        |_| async move {
            match fetch_genres().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<Genre>::new()
                }
            }
        });
  

    let genres_data = move || {
        let value = resource.read(cx);
        match value {
            None => genres.set(Vec::new()),
            Some(val) => genres.set(val)
        };

    };


    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            { genres_data }
            <div class="rounded-lg border-2 border-neutral-600 flex-row pt-6 pl-4 pb-1 mb-5 shadow-xl">
               <div class="flex flex-row pb-4">
                    <div class="mx-4">
                        <label for="genre">Genre</label>
                        <select id="genre" on:input=move |event| { genre.set(event_target_value(&event)); } prop:value=move || { genre.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                            <option value="">Select genre</option>
                            <For each = move || genres.get()
                                 key = move |gen: &Genre| gen.id
                                 view = move |cx, gen: Genre| view! {cx, <option value={gen.name.clone()}>{gen.name}</option> }
                            />
                        </select>
                    </div>
                </div>
                <button on:click = move |_| { trigger_filter.set(!trigger_filter.get()); } class="mx-2 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Filter</button>
                <a href=move || {format!("https://localhost:44316/api/ticket/excel{}",if genre.get().is_empty() { "".to_string() } else { format!("?genre={}", genre.get()) })} download=""><button class="mx-4 text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 focus:outline-none">Export XLSX</button></a>
            </div>
        </Transition>
    }
}


#[component]
pub fn TicketsExportTable(cx: Scope, trigger_filter: RwSignal<bool>, genre: RwSignal<String>) -> impl IntoView {
    // let idx = create_rw_signal(cx, 0);
    let resource: leptos::Resource<bool, Vec<Ticket>> = create_resource(cx, 
        move || trigger_filter.get(), 
        move |_| async move {
            match filter_tickets_by_genre(genre.get_untracked()).await {
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
        
        // idx.set(0);
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
                            view! { cx, <TicketItem record actions=false refresh_trigger=None/> } 
                        }
                    />    
                </tbody>
            </table>
        </Transition>
    }
}


#[component]
pub fn TicketsExportPage(cx: Scope) -> impl IntoView {
    let trigger_filter = create_rw_signal(cx, true);
    let genre = create_rw_signal(cx, "".to_string());

    view! {cx,
        <GenreFilter trigger_filter genre/>
        <TicketsExportTable trigger_filter genre/>
    }
}

