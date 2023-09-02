use leptos::*;
use leptos_router::*;

mod index;
mod export;

use crate::models::{Ticket, TicketStatus, BearerRequestBuilder};

use crate::pages::tickets::{index::TicketsIndexPage, export::TicketsExportPage};

async fn add_to_cart(ticket: String) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let request = client.post(format!("https://localhost:44316/api/cart/add-ticket/{}", ticket))
        .add_token()
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

async fn delete_ticket(ticket: String) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let request = client.delete(format!("https://localhost:44316/api/ticket/{}", ticket))
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
fn AddToCartButton(cx: Scope, id: String, status: RwSignal<TicketStatus>, refresh_trigger: Option<RwSignal<bool>>) -> impl IntoView {
    let add_to_cart_click = move |_| {
        let trigger = refresh_trigger.unwrap();
        let uid = id.clone();
        spawn_local(async move {
            let _ = add_to_cart(uid).await;
            trigger.set(!trigger.get_untracked());
        });
    };

    if status.get_untracked() == TicketStatus::Available {
        return view! {cx, 
            <button on:click=add_to_cart_click class="text-white mr-2 bg-green-700 hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">Add to cart</button>
        }.into_view(cx);
    }

    view! {cx, <></> }.into_view(cx)
     
}

#[component]
pub fn TicketItem(cx: Scope, record: Ticket, actions: bool, refresh_trigger: Option<RwSignal<bool>>) -> impl IntoView {
    let delete_ticket_click = move |_| {
        let trigger = refresh_trigger.unwrap();
        spawn_local(async move {
            let _ = delete_ticket(record.id.to_string()).await;
            trigger.set(!trigger.get_untracked());
        });
    };

    let status = create_rw_signal(cx, record.ticket_status.clone());

    let actions = move || {
        if actions {
            return view! {cx,
                <td class="px-6 py-4">
                    <div>
                        <AddToCartButton id={record.id.to_string()} status refresh_trigger/>
                        <A href={format!("edit/{}", record.id.to_string())}><button class="text-white mr-2 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Edit</button></A>
                        <button on:click=delete_ticket_click class="text-white mr-2 bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800">Delete</button>
                    </div>
                </td>
            }.into_view(cx);
        }
        
        view! {cx, <></> }.into_view(cx)
    };



    view! {
        cx,
        <tr class="border-b bg-white border-gray-300 hover:bg-gray-50">
            // <th scope="row" class="px-6 py-4">{idx.get()}</th>
            <td class="px-6 py-4">{record.seat_number}</td>
            <td class="px-6 py-4">{record.price} $</td>
            <td class="px-6 py-4">{record.view_slot.movie_name} - {record.view_slot.time_slot.to_string()}</td>
            <td class="px-6 py-4">{format!("{}", record.ticket_status)}</td>
            { actions }
        </tr>
    }
}


#[component]
pub fn TicketsBaseTemplate(cx:Scope) -> impl IntoView {
    view! {cx,
        <div class="h-screen w-full">
            <div class="flex place-content-center">
                <div class="w-3/4">
                    <h2 class="text-5xl font-semibold mb-4">Tickets</h2>
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
pub fn TicketsPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="/" view=TicketsBaseTemplate>
            <Route path="/" view=TicketsIndexPage/>
            <Route path="export" view=TicketsExportPage/>   
        </Route>
    }
}
