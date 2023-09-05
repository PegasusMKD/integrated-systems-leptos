use leptos::*;

use super::TicketUpdateEvent;

use crate::models::Ticket;

use leptos_router::{use_params_map, NavigateOptions, use_navigate};

use crate::constants::CONFIG;

async fn update_ticket_for_view_slot(id: String, view_slot: String, price: String) -> reqwest::Result<()> {
    let payload = TicketUpdateEvent::update(id, view_slot, price.parse::<i32>().unwrap());

    let client = reqwest::Client::new();
    let request = client.put(format!("{}/ticket", CONFIG.api.path))
                    .json(&payload)
                    .send()
                    .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(()); // TODO: Add proper error return so it can be handled down the line
    } 

    Ok(())
}


pub async fn get_ticket(id: String) -> reqwest::Result<Ticket> {
   let client = reqwest::Client::new();
    let request = client.get(format!("{}/ticket/{id}", CONFIG.api.path))
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
    }
    
    request
        .json::<Ticket>()
        .await 
}


#[component]
pub fn ViewSlotTicketsEditPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let get_view_slot_id = move || { params.with_untracked(|params_map| params_map.get("id").cloned().unwrap_or_default()) };
    let (view_slot_id, _ ) = create_signal(cx, get_view_slot_id());
    
    let get_ticket_id = move || { params.with_untracked(|params_map| params_map.get("ticket_id").cloned().unwrap_or_default()) };
    let (ticket_id, _ ) = create_signal(cx, get_ticket_id());

    let price = create_rw_signal(cx, "0".to_string());

    let resource = create_resource(cx, move || (),
        move |_| async move {
            match get_ticket(ticket_id.get_untracked()).await {
                Ok(data) => data,
                Err(_) => Ticket::new()
            }
        });

    let ticket_data = move || {
        let value = resource.read(cx);
        match value {
            Some(val) => price.set(val.price.to_string()),
            None => ()
        };
    };

    let update_ticket = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate(cx);
        spawn_local(async move {
            let _ = update_ticket_for_view_slot(ticket_id.get_untracked(), view_slot_id.get_untracked(), price.get_untracked()).await;
            navigate(format!("/view-slots/tickets/{}", view_slot_id.get_untracked()).as_str(), NavigateOptions::default()).unwrap();
        });
    };

    view! {cx,
        <Suspense fallback=move || view! { cx, <></> }.into_view(cx)>
            { ticket_data }
        </Suspense>
       <form class="py-4 px-8" on:submit=update_ticket> 
          <div class="mb-6">
            <label for="price" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Price ($)</label>
            <input type="number" id="price" on:input=move |event| { price.set(event_target_value(&event)); } prop:value=move || { price.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="10" required/>
          </div>
          <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Submit</button>
        </form>
    }
}
