use leptos::*;

use crate::models::{Ticket, FilterTickets};

// TODO: Add proper error handling with status_code checks and custom errors (probably)
async fn get_data(from_date: String, to_date: String) -> reqwest::Result<Vec<Ticket>> {
    // Make this the official return after getting some data in the database
    let data = FilterTickets::new(from_date, to_date);
    let client = reqwest::Client::new();
    let request = client.post("https://localhost:44316/api/ticket/filter")
        .json(&data)
        .send()
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
pub fn TicketItem(cx: Scope, idx: RwSignal<usize>, record: Ticket) -> impl IntoView {
    idx.update(|val: &mut usize| *val += 1);
    view! {
        cx,
        <tr class="border-b bg-white border-gray-300 hover:bg-gray-50">
            <th scope="row" class="px-6 py-4">{idx.get_untracked().to_string()}</th>
            <td class="px-6 py-4">{record.seat_number}</td>
            <td class="px-6 py-4">{record.price} $</td>
            <td class="px-6 py-4">{record.view_slot.movie_name} - {record.view_slot.time_slot.to_string()}</td>
            <td class="px-6 py-4">{format!("{}", record.ticket_status)}</td>
        </tr>
    }
}

#[component]
pub fn FromToTicketsFilter(cx: Scope, trigger_filter: RwSignal<bool>, from_date: RwSignal<String>, to_date: RwSignal<String>) -> impl IntoView {
    
    view! {cx,
        <div class="rounded-lg border-2 border-neutral-600 flex-row pt-6 pl-4 pb-1 mb-5 shadow-xl">
            <div class="flex flex-row pb-4">
                <div>
                <label for="from">From Date</label>
                <input type="datetime-local" id="from" on:input=move |event| { from_date.set(event_target_value(&event)); } prop:value=move || { from_date.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block pl-5 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                </div>
                <div class="mx-4">
                <label for="to">To Date</label>
                <input type="datetime-local" id="to" on:input=move |event| { to_date.set(event_target_value(&event)); } prop:value=move || { to_date.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block pl-5 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                </div>
            </div>
            <button on:click = move |_| { trigger_filter.set(!trigger_filter.get()); } class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">Filter</button>
        </div>

    }
}

#[component]
pub fn TicketsTable(cx: Scope, trigger_filter: RwSignal<bool>, from_date: RwSignal<String>, to_date: RwSignal<String>) -> impl IntoView {
    let resource: leptos::Resource<bool, Vec<Ticket>> = create_resource(cx, 
        move || trigger_filter.get(), 
        move |_| async move {
            match get_data(from_date.get_untracked(), to_date.get_untracked()).await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<Ticket>::new()
                }
            }
        });
  
    let idx = create_rw_signal(cx, 0);
    let (data, set_data) = create_signal(cx, Vec::<Ticket>::new());

    let tickets_data_table = move || {
        let value = resource.read(cx);
        match value {
            None => set_data.set(Vec::new()),
            Some(val) => set_data.set(val)
        };
        
        idx.set(0);
    };



    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <table class="w-full flex-row text-sm text-center rounded-lg bordertext-gray-500">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50">
                    <tr class="border-y rounded-t-lg border-gray-800">
                        <th scope="col" class="px-6 py-3">#</th>
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
                            view! { cx, <TicketItem record idx/> } 
                        }
                    />    
                </tbody>
            </table>
        </Transition>
    }
}

// TODO: Maybe use #[component(transparent)] and return routes here instead of a direct view, since
// depending on the action, we'd transfer them over to another page (aka CreateViewSlot,
// EditViewSlot, etc.)
#[component]
pub fn TicketsPage(cx: Scope) -> impl IntoView {
    let from_date = create_rw_signal(cx, "".to_string());
    let to_date = create_rw_signal(cx, "".to_string());
    let trigger_filter = create_rw_signal(cx, true);
    

    view! { cx,
        <div class="h-screen w-full">
            <div class="flex place-content-center">
                // TODO: Add filters
                <div class="w-3/4">
                    <h2 class="text-5xl font-semibold mb-4">Tickets</h2>
                    <FromToTicketsFilter from_date to_date trigger_filter/>
                    <TicketsTable from_date to_date trigger_filter/>
                </div>
            </div>
        </div>
    }
}
