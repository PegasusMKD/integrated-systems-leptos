use leptos::*;
use leptos_router::{use_navigate, NavigateOptions, A};

use crate::models::{LoginDetails, UserDetails};

use crate::constants::CONFIG;

async fn login(email: String, password: String) -> reqwest::Result<UserDetails> {
    let login_details = LoginDetails::new(email, password);

    let client = reqwest::Client::new();
    let request = client.post(format!("{}/auth/login", CONFIG.api.path))
                    .json(&login_details)
                    .send()
                    .await?;
   
    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
    }

    request.json::<UserDetails>().await
}

#[component]
pub fn LoginPage(cx: Scope, trigger: RwSignal<bool>) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (password, set_password) = create_signal(cx, "".to_string());
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate(cx);
        spawn_local(async move {
            let response = login(email.get_untracked(), password.get_untracked()).await;
            leptos::log!("{:?}", response);
            match response {
                Ok(data) => {
                    data.save();
                    trigger.set(!trigger.get());
                    navigate("/home", NavigateOptions::default()).unwrap();
                },
                Err(err) => leptos::log!("Error during login: {:?}", err)
            };
        });
    };

    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <div class="flex h-full min-h-screen w-full items-center justify-center">
                <div class="w-1/3">
                    <h1 class="font-bold text-3xl">Login</h1>
                    <form class="py-8 px-8" on:submit=on_submit>
                      <div class="mb-6">
                        <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Email</label>
                        <input type="email" id="email" on:input=move |event| { set_email.set(event_target_value(&event)); } prop:value=move || { email.get() } class="w-full bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="test@gmail.com" required/>
                      </div>
                      <div class="mb-6">
                        <label for="password">Password</label>
                        <input type="password" id="password" on:input=move |event| { set_password.set(event_target_value(&event)); } prop:value=move || { password.get() } class="w-full bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block px-5 py-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
                      </div>
                      <div class="px-2 flex w-full justify-start items-center">
                        <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Submit</button>
                        <p class="px-6">"Don't" have an account? <A href="/register">Register now!</A></p>
                      </div>
                    </form>
                </div>
            </div>
        </Transition>
    }
}
