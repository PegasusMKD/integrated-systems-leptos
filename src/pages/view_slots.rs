use leptos::*;

use crate::models::ViewSlot;

#[component]
pub fn ViewSlotsPage(cx: Scope) -> impl IntoView {

    spawn_local(async {
       let result = reqwest::get("https://localhost:44316/api/view-slot").await.unwrap();
        println!("{:?}", result.status());
        println!("{:?}", result.json::<Vec<ViewSlot>>().await); 
    });

    view! {
        cx,
        <p>View Slots</p>
    }
}
