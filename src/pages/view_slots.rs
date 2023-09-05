use leptos::*;
use leptos_router::*;
use time::macros::format_description;

mod index;
mod create;
mod edit;
mod tickets;

use crate::models::{ViewSlot, Genre};

use crate::pages::view_slots::{
    index::ViewSlotIndexPage, create::ViewSlotCreatePage, edit::ViewSlotEditPage, tickets::{
        index::ViewSlotTicketsPage, edit::ViewSlotTicketsEditPage, create::ViewSlotTicketsCreatePage, create_multiple::ViewSlotTicketsCreateMultiplePage 
    } 
};

use crate::constants::CONFIG;

pub async fn get_view_slot(id: String) -> reqwest::Result<ViewSlot> {
   let client = reqwest::Client::new();
    let request = client.get(format!("{}/view-slot/{id}", CONFIG.api.path))
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
    }
    
    request
        .json::<ViewSlot>()
        .await 
}

#[component]
pub fn ViewSlotItem(cx: Scope, record: ViewSlot) -> impl IntoView {
    // idx.update(|val: &mut usize| *val += 1);
    let display_format = format_description!("[year]-[month]-[day] [hour]:[minute]");
    view! {
        cx,
        <div class="max-w-sm border-2 rounded border-neutral-100 overflow-hidden shadow-lg w-1/5 mx-12">
          <img class="w-full" src="/favicon.ico" alt="Sunset in the mountains"/>
          <div class="px-6 py-4">
            <div class="font-bold text-xl mb-2">{record.movie_name}</div>
            
            <p class="text-gray-700 text-base">
              Time: {record.time_slot.format(&display_format).unwrap().to_string()}
            </p>
          </div>
          <div class="px-6 pt-4 pb-2 bg-gray-200">
            <A href={ format!("tickets/{}", record.id.map(|uid| uid.to_string()).unwrap_or("".to_string())) }><button class="mx-2 text-white bg-sky-700 hover:bg-sky-800 focus:ring-4 focus:ring-sky-300 font-medium rounded-lg text-sm px-3 py-2.5 mr-2 mb-2 dark:bg-sky-600 dark:hover:bg-sky-700 focus:outline-none dark:focus:ring-sky-800">View tickets</button></A>
            <A href={ format!("edit/{}", record.id.map(|uid| uid.to_string()).unwrap_or("".to_string())) }><button class="mx-2 text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-3 py-2.5 mr-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 focus:outline-none dark:focus:ring-red-800">Edit view slot</button></A>
            <span class="inline-block bg-gray-300 rounded-full px-3 py-1 text-sm font-semibold text-gray-700 mr-2 mb-2">#{record.genre.unwrap_or(Genre::new()).name}</span>
          </div>
        </div>
    }
}


#[component]
pub fn ViewSlotBaseTemplate(cx:Scope) -> impl IntoView {

    view! {cx,
        <div class="h-full min-h-screen w-full">
            <div class="flex place-content-center">
                <div class="w-3/4">
                    <h2 class="text-5xl font-semibold mb-4">View Slots</h2>
                    <Outlet/>
            </div>
        </div>
    </div>

    }
}


// TODO: Maybe use #[component(transparent)] and return routes here instead of a direct view, since
// depending on the action, we'd transfer them over to another page (aka CreateViewSlot,
// EditViewSlot, etc.)
#[component(transparent)]
pub fn ViewSlotPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="/" view=ViewSlotBaseTemplate>
            <Route path="/" view=ViewSlotIndexPage/>
            <Route path="/create" view=ViewSlotCreatePage/>
            <Route path="/edit/:id" view=ViewSlotEditPage/>
            <Route path="/tickets/:id" view=Outlet>
                <Route path="" view=ViewSlotTicketsPage/>
                <Route path="/edit/:ticket_id" view=ViewSlotTicketsEditPage/>
                <Route path="/create" view=ViewSlotTicketsCreatePage/>
                <Route path="/create-multiple" view=ViewSlotTicketsCreateMultiplePage/>
            </Route>
        </Route>
    }
}
