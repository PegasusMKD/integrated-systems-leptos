pub mod index;
pub mod edit;
pub mod import;

use leptos::*;
use leptos_router::*;

use crate::models::{User, BearerRequestBuilder};

use crate::pages::user_management::{index::UsersIndexPage, import::UsersImportPage, edit::UsersEditPage};


async fn delete_user(ticket: String) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let request = client.delete(format!("https://localhost:44316/api/users/{}", ticket))
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

#[component]
pub fn UserRow(cx: Scope, record: User, refresh_trigger: RwSignal<bool>) -> impl IntoView {
    let delete_user_click = move |_| {
        spawn_local(async move {
            let _ = delete_user(record.id.unwrap().to_string()).await;
            refresh_trigger.set(!refresh_trigger.get_untracked());
        });
    };

    view! {
        cx,
        <tr class="border-b bg-white border-gray-300 hover:bg-gray-50">
            // <th scope="row" class="px-6 py-4">{idx.get()}</th>
            <td class="px-6 py-4">{record.username}</td>
            <td class="px-6 py-4">{record.email}</td>
            <td class="px-6 py-4">{record.roles}</td>
            <td class="px-6 py-4">
                <div>
                    <A href={format!("edit/{}", record.id.unwrap().to_string())}><button class="text-white mr-2 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Edit</button></A>
                    <button on:click=delete_user_click class="text-white mr-2 bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800">Delete</button>
                </div>
            </td>
        </tr>
    }
}


#[component]
pub fn UsersBaseTemplate(cx:Scope) -> impl IntoView {
    view! {cx,
        <div class="h-screen w-full">
            <div class="flex place-content-center">
                <div class="w-3/4">
                    <h2 class="text-5xl font-semibold mb-4">User Management</h2>
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
pub fn UsersPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="/" view=UsersBaseTemplate>
            <Route path="/" view=UsersIndexPage/>
            <Route path="import" view=UsersImportPage/>
            <Route path="edit/:id" view=UsersEditPage/>
        </Route>
    }
}
