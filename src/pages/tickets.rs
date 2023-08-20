use leptos::*;
use leptos_router::*;

mod index;
mod export;

use crate::models::Ticket;

use crate::pages::tickets::{index::TicketsIndexPage, export::TicketsExportPage};

#[component]
pub fn TicketItem(cx: Scope, record: Ticket) -> impl IntoView {
    // idx.update(|val: &mut usize| *val += 1);
    view! {
        cx,
        <tr class="border-b bg-white border-gray-300 hover:bg-gray-50">
            // <th scope="row" class="px-6 py-4">{idx.get()}</th>
            <td class="px-6 py-4">{record.seat_number}</td>
            <td class="px-6 py-4">{record.price} $</td>
            <td class="px-6 py-4">{record.view_slot.movie_name} - {record.view_slot.time_slot.to_string()}</td>
            <td class="px-6 py-4">{format!("{}", record.ticket_status)}</td>
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
