use leptos::*;
use leptos_router::A;

use super::ViewSlotItem;

use crate::models::ViewSlot;

use crate::constants::CONFIG;

// TODO: Add proper error handling with status_code checks and custom errors (probably)
async fn fetch_view_slots() -> reqwest::Result<Vec<ViewSlot>> {
    // Make this the official return after getting some data in the database
    let request = reqwest::get(format!("{}/view-slot", CONFIG.api.path))
        .await?;
    
    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(Vec::new()); // TODO: Add proper error return so it can be handled down the line
    }

    //leptos::log!("{}", request.text().await?);
    request
        .json::<Vec<ViewSlot>>()
        .await

    //Ok(Vec::new())
}

#[component]
pub fn ViewSlotActionBar(cx: Scope) -> impl IntoView {
    
    view! {cx,
        <div class="flex-row pt-6 pl-4 pb-1 mb-5">
            <A href="create"><button class="mx-2 text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 focus:outline-none dark:focus:ring-green-800">Create new view slot</button></A>
        </div>

    }
}

#[component]
pub fn ViewSlotIndexTable(cx: Scope) -> impl IntoView {
    let resource: leptos::Resource<(), Vec<ViewSlot>> = create_resource(cx, || (), 
        move |_| async move {
            match fetch_view_slots().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[View Slots Page] Error doing a request to fetch view slots: {:?}", err);
                    Vec::<ViewSlot>::new()
                }
            }
        });
  
    let tickets_data_table = move || {
        let value = resource.read(cx);
        let val = match value {
            None => Vec::new(),
            Some(val) => val
        };
       
        val.chunks(5).map(|chunk: &[ViewSlot]|
            view! {cx, 
                <div class="mx-4 w-full py-6 flex flex-row text-sm text-center text-gray-500">
                    { chunk.iter().map(|record| view! {cx, <ViewSlotItem record={record.clone()}/> }).collect_view(cx) }
                </div>
            }
        ).collect_view(cx)
    };


    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            { tickets_data_table }
        </Transition>
    }
}

#[component]
pub fn ViewSlotIndexPage(cx:Scope) -> impl IntoView {
    view! {cx,
        <ViewSlotActionBar />
        <ViewSlotIndexTable />
    }
}
