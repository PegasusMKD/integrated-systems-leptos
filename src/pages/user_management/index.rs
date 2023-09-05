use leptos::*;

use leptos_router::A;

use super::UserRow;
use crate::models::{User, BearerRequestBuilder};

use crate::constants::CONFIG;

// TODO: Add proper error handling with status_code checks and custom errors (probably)
async fn get_users() -> reqwest::Result<Vec<User>> {
    // Make this the official return after getting some data in the database
    let client = reqwest::Client::new();
    let request = client.get(format!("{}/users", CONFIG.api.path))
        .add_token()
        .send()
        .await?;
    
    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(Vec::new()); // TODO: Add proper error return so it can be handled down the line
    }
    
    request
        .json::<Vec<User>>()
        .await
}

#[component]
pub fn ImportButton(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex-row pt-6 pl-4 pb-1 mb-5">
            <A href="import"><button class="mx-4 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 focus:outline-none">Import Users</button></A>
        </div>

    }
}

#[component]
pub fn UsersIndexTable(cx: Scope) -> impl IntoView {
    let refresh_trigger = create_rw_signal(cx, true);
    let resource: leptos::Resource<bool, Vec<User>> = create_local_resource(cx, 
        move || refresh_trigger.get(), 
        move |_| async move {
            match get_users().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<User>::new()
                }
            }
        });
    
    let (data, set_data) = create_signal(cx, Vec::<User>::new());

    let users_data_table = move || {
        let value = resource.read(cx);
        match value {
            None => set_data.set(Vec::new()),
            Some(val) => set_data.set(val)
        };
    };


    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <ErrorBoundary fallback=|cx, errors| view!{ cx, <div>{format!("{:?}", errors.get())}</div>}.into_view(cx)>
            <table class="w-full flex-row text-sm text-center rounded-lg bordertext-gray-500">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50">
                    <tr class="border-y rounded-t-lg border-gray-800">
                        // <th scope="col" class="px-6 py-3">#</th>
                        <th scope="col" class="px-6 py-3">Username</th>
                        <th scope="col" class="px-6 py-3">Email</th>
                        <th scope="col" class="px-6 py-3">Roles</th>
                        <th scope="col" class="px-6 py-3"></th>
                    </tr>
                </thead>
                <tbody>
                    { users_data_table }
                    <For 
                        each = move || data.get()
                        key = |record: &User| record.id.unwrap()
                        view = move |cx, record: User| {
                           view! { cx, <UserRow record refresh_trigger/> } 
                        }
                    />
                </tbody>
            </table>
            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn UsersIndexPage(cx:Scope) -> impl IntoView {
    view! {cx,
        <ImportButton/>
        <UsersIndexTable/>
    }
}


