use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use time::{PrimitiveDateTime, OffsetDateTime};

use crate::models::{ViewSlot, Genre, BearerRequestBuilder};

use crate::services::fetch_genres;
use time::format_description;

async fn create_view_slot(view_slot: ViewSlot) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let request = client.post("https://localhost:44316/api/view-slot")
        .json(&view_slot)
        .add_token()
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
    }
    
    return Ok(());
}


#[component]
pub fn ViewSlotCreatePage(cx: Scope) -> impl IntoView {
    let current = OffsetDateTime::now_utc();
    let format = format_description::parse("[year]-[month]-[day]T[hour]:[minute]").unwrap();
    let primitive = PrimitiveDateTime::new(current.date(), current.time()).format(&format).unwrap(); 

    let (movie_name, set_movie_name) = create_signal(cx, "".to_string());
    let (genre, set_genre) = create_signal(cx, "".to_string());
    let (time_slot, set_time_slot) = create_signal(cx, primitive);
    
    let genres = create_rw_signal(cx, Vec::new());
    let resource: leptos::Resource<(), Vec<Genre>> = create_resource(cx, 
        || (), 
        |_| async move {
            match fetch_genres().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[View Slot Page] Error doing a request to fetch genres: {:?}", err);
                    Vec::<Genre>::new()
                }
            }
        });
  

    let genres_data = move || {
        let value = resource.read(cx);
        match value {
            None => genres.set(Vec::new()),
            Some(val) => genres.set(val)
        };

    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate(cx);
        spawn_local(async move {
            let _ = create_view_slot(ViewSlot::from(movie_name.get_untracked(), Genre::from(genre.get_untracked(), genres.get_untracked()), time_slot.get_untracked())).await;
            navigate("/view-slots", NavigateOptions::default()).unwrap();
        });
    };

    view! {cx,
        <Transition fallback=move || { view! {cx, <div></div> } }>
            <form class="py-4 px-8" on:submit=on_submit>
              { genres_data }
              <div class="mb-6">
                <label for="movie-name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Movie Name</label>
                <input type="text" id="movie-name" on:input=move |event| { set_movie_name.set(event_target_value(&event)); } prop:value=move || { movie_name.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Fast X" required/>
              </div>
              <div class="mb-6">
                <label for="genre">Genre</label>
                <select id="genre" on:input=move |event| { set_genre.set(event_target_value(&event)); } prop:value=move || { genre.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-10 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                    <option value="">Select genre</option>
                    <For each = move || genres.get()
                         key = move |gen: &Genre| gen.id
                         view = move |cx, gen: Genre| view! {cx, <option value={gen.name.clone()}>{gen.name}</option> }
                    />
                </select>
              </div>
              <div class="mb-6">
                <label for="time-slot">Time Slot</label>
                <input type="datetime-local" id="time-slot" on:input=move |event| { set_time_slot.set(event_target_value(&event)); } prop:value=move || { time_slot.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block px-5 py-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
              </div>
              <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Submit</button>
            </form>
        </Transition>
    }
}
