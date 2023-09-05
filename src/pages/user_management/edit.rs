use leptos::*;
use leptos_router::*;

use crate::models::{BearerRequestBuilder, User, UpdateUserDto};

use crate::constants::CONFIG;

async fn fetch_roles() -> reqwest::Result<Vec<String>> {
    let client = reqwest::Client::new();
    let request = client.get(format!("{}/users/roles", CONFIG.api.path))
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(Vec::new());
    }
    
    return request.json::<Vec<String>>().await;
 
}

async fn update_user(payload: UpdateUserDto) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let request = client.put(format!("{}/users/{}", CONFIG.api.path, payload.id.unwrap()))
        .json(&payload)
        .add_token()
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(());
    }
    
    Ok(())
}

async fn get_user(id: &str) -> reqwest::Result<User> {
    let client = reqwest::Client::new();
    let request = client.get(format!("{}/users/{}", CONFIG.api.path, id))
        .add_token()
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(User::new());
    }
    
    return request.json::<User>().await;
}


#[component]
pub fn UsersEditPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let get_id = move || { params.with_untracked(|params_map| params_map.get("id").cloned().unwrap_or_default()) };
    let (id_sig, _ ) = create_signal(cx, get_id());

    let user = create_rw_signal(cx, User::new());
    let username = create_rw_signal(cx, String::new());
    let current_password = create_rw_signal(cx, String::new());
    let new_password = create_rw_signal(cx, String::new());
    let selected_role = create_rw_signal(cx, String::new());

    let roles = create_rw_signal(cx, Vec::new());
    let roles_resource: leptos::Resource<(), Vec<String>> = create_resource(cx, 
        || (), 
        |_| async move {
            match fetch_roles().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[View Slot Page] Error doing a request to fetch genres: {:?}", err);
                    Vec::new()
                }
            }
        });
    
    let user_resource: leptos::Resource<String, User> = create_local_resource(cx, 
        move || id_sig.get(),
        |uid| async move {
            match get_user(&uid).await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[View Slot Page] Error doing a request to fetch genres: {:?}", err);
                    User::new()
                }
            }
        });
  

    let roles_data = move || {
        let value = roles_resource.read(cx);
        match value {
            None => roles.set(Vec::new()),
            Some(val) => roles.set(val)
        };

    };

    let user_data = move || {
        let value = user_resource.read(cx);
        match value {
            None => {},
            Some(val) => {
                let available_roles = val.roles.clone();
                username.set(val.username.clone());
                user.set(val);
                let default_role = String::new();
                let possible_role = available_roles.get(0).unwrap_or(&default_role);
                selected_role.set(possible_role.to_string());
            }
        };

    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate(cx);
        spawn_local(async move {
            let _ = update_user(UpdateUserDto::create_from_user(
                user.get_untracked(),
                username.get_untracked(),
                current_password.get_untracked(),
                new_password.get_untracked(),
                selected_role.get_untracked()
            )).await;
            navigate("/user-management", NavigateOptions::default()).unwrap();
        });
    };

    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <form class="py-4 px-8" on:submit=on_submit>
              { roles_data }
              { user_data }
              <div class="mb-6">
                <label for="movie-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Username</label>
                <input type="text" id="movie-name" on:input=move |event| { username.set(event_target_value(&event)); } prop:value=move || { username.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Username" required/>
              </div>
              <div class="mb-6">
                <label for="movie-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Current Password</label>
                <input type="text" id="movie-name" on:input=move |event| { current_password.set(event_target_value(&event)); } prop:value=move || { current_password.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Current Password"/>
              </div>
              <div class="mb-6">
                <label for="movie-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">New Password</label>
                <input type="text" id="movie-name" on:input=move |event| { new_password.set(event_target_value(&event)); } prop:value=move || { new_password.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="New Password"/>
              </div>
              <div class="mb-6">
                <label for="genre">Role</label>
                <select id="genre" on:input=move |event| { selected_role.set(event_target_value(&event)); } prop:value=move || { selected_role.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-12 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                    <option value="">Select role</option>
                    <For each = move || roles.get()
                         key = move |gen: &String| gen.clone()
                         view = move |cx, gen: String| view! {cx, <option value={gen.clone()}>{gen.clone()}</option> }
                    />
                </select>
              </div>
              
              <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Submit</button>
              <A href="/user-management"><button class="mx-2 text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800">Back</button></A>
            </form>
        </Transition>
    }
}

