use leptos::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::FormData;
use gloo_net::http::Request;
use leptos_router::{use_navigate, NavigateOptions};

use crate::models::UserDetails;

use crate::constants::CONFIG;

pub async fn upload_file(event: ev::SubmitEvent) {
    event.prevent_default();
    let event_data = event
        .unchecked_ref::<web_sys::Event>()
        .target()
        .unwrap_throw()
        .unchecked_into::<web_sys::HtmlFormElement>();
    leptos::log!("test");
    let form_data = FormData::new_with_form(&event_data).unwrap();
    leptos::log!("Data: {:?}", form_data.get("file"));
    let token_data = format!("Bearer {}", UserDetails::read_detail("token".to_string()));

    let _ = Request::post(format!("{}/users/import/leptos", CONFIG.api.path).as_str())
        .body(form_data)
        .header("Authorization", &token_data)
        .send()
        .await;
}




#[component]
pub fn UsersImportPage(cx: Scope) -> impl IntoView {
    let uploads = move |ev| {
        let navigate = use_navigate(cx);
        spawn_local(async move {
            upload_file(ev).await;
            navigate("/user-management", NavigateOptions::default()).unwrap();
        });
    };

    view! {cx, 
        <div>
            <h3 class="font-semibold text-lg">Import Users</h3>
            <form on:submit=uploads enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input class="mx-4 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 focus:outline-none" type="submit"/>
            </form>
        </div>
    }
}

