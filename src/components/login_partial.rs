use leptos::*;
use leptos_router::*;

use crate::constants::CONFIG;
use crate::models::{BearerRequestBuilder, UserDetails};

#[derive(Clone, PartialEq)]
enum TokenState {
    Valid,
    Invalid,
    NotAvailable
}

async fn token_valid() -> TokenState {
    let client = reqwest::Client::new();
    let request = client.post(format!("{}/auth/is-valid-token", CONFIG.api.path))
        .add_token()
        .send()
        .await;

    if request.is_ok() && request.unwrap().status() == 200 { TokenState::Valid } else { TokenState::Invalid }
}


#[component]
pub fn LoginPartial(cx:Scope, trigger: RwSignal<bool>) -> impl IntoView {
    let resource = create_local_resource(cx, || (), move |_| async move {
        if !UserDetails::user_logged_in() {
            return TokenState::NotAvailable;
        }

        token_valid().await 
    });

    let check_logged_in = move || {
        let validity = resource.read(cx);
        let validity = match validity {
            Some(val) => val,
            None => TokenState::NotAvailable
        };

        leptos::log!("Print here");
        let navigate = use_navigate(cx);
        if validity == TokenState::Invalid {
            leptos::log!("And here...");
            UserDetails::delete();
            trigger.set(!trigger.get());
            navigate("/home", NavigateOptions::default()).unwrap();
        }

    };

    view! {cx,
        <>
            { check_logged_in }
        </>
    }
}
