use leptos::*;

use super::TicketUpdateEvent;

use leptos_router::{use_params_map, NavigateOptions, use_navigate};

async fn create_ticket_for_view_slot(view_slot: String, price: String) -> reqwest::Result<()> {
    let payload = TicketUpdateEvent::create(view_slot, price.parse::<i32>().unwrap());

    let client = reqwest::Client::new();
    let request = client.post("https://localhost:44316/api/ticket")
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


#[component]
pub fn ViewSlotTicketsCreatePage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let get_view_slot_id = move || { params.with_untracked(|params_map| params_map.get("id").cloned().unwrap_or_default()) };
    let (view_slot_id, _ ) = create_signal(cx, get_view_slot_id());

    let price = create_rw_signal(cx, "0".to_string());

    let create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate(cx);
        spawn_local(async move {
            let _ = create_ticket_for_view_slot(view_slot_id.get_untracked(), price.get_untracked()).await;
            navigate(format!("/view-slots/tickets/{}", view_slot_id.get_untracked()).as_str(), NavigateOptions::default()).unwrap();
        });
    };

    view! {cx,
       <form class="py-4 px-8" on:submit=create>
          <div class="mb-6">
            <label for="price" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Price ($)</label>
            <input type="number" id="price" on:input=move |event| { price.set(event_target_value(&event)); } prop:value=move || { price.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="10" required/>
          </div>
          <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Submit</button>
        </form>
    }
}
