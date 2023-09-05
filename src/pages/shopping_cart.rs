use leptos::*;

use leptos_router::{use_navigate, NavigateOptions};

use leptos::ev::SubmitEvent;

use crate::models::{Ticket, Cart, BearerRequestBuilder, CreateOrder};

use crate::constants::CONFIG;

async fn get_shopping_cart_tickets() -> reqwest::Result<Cart> {
    let client = reqwest::Client::new();
    let request = client.get(format!("{}/cart/by-user", CONFIG.api.path))
        .add_token()
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(Cart::new()); // TODO: Add proper error return so it can be handled down the line
    }

    request.json::<Cart>().await
}

async fn remove_ticket_from_shopping_cart(ticket: String) -> reqwest::Result<()> {
    let client = reqwest::Client::new();
    let request = client.post(format!("{}/cart/remove-ticket/{}", CONFIG.api.path, ticket))
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


async fn create_order(name: String, date: String, cvc: String, card_number: String) -> reqwest::Result<()> {
    let payload = CreateOrder::new(name, date, cvc, card_number);
    let client = reqwest::Client::new();
    let request = client.post(format!("{}/order", CONFIG.api.path))
        .json(&payload)
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
pub fn ShoppingCartTicketItem(cx: Scope, record: Ticket, refresh_trigger: RwSignal<bool>) -> impl IntoView {
    let remove_from_shopping_cart_click = move |_| {
        spawn_local(async move {
            let _ = remove_ticket_from_shopping_cart(record.id.to_string()).await;
            refresh_trigger.set(!refresh_trigger.get_untracked());
        });
    };

    view! {
        cx,
        <tr class="border-b bg-white border-gray-300 hover:bg-gray-50">
            // <th scope="row" class="px-6 py-4">{idx.get()}</th>
            <td class="px-6 py-4">{record.view_slot.movie_name}</td>
            <td class="px-6 py-4">{record.view_slot.time_slot.to_string()}</td>
            <td class="px-6 py-4">{record.price} $</td>
            <td class="px-6 py-4">{record.seat_number}</td>
            <td class="px-6 py-4">
                <div>
                    <button on:click=remove_from_shopping_cart_click class="text-white mr-2 bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-800">Remove</button>
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn ShoppingCartTable(cx: Scope) -> impl IntoView {
    let refresh_trigger = create_rw_signal(cx, true);
    let total = create_rw_signal(cx, 0.0);

    let resource: leptos::Resource<bool, Vec<Ticket>> = create_local_resource(cx, 
        move || refresh_trigger.get(), 
        move |_| async move {
            match get_shopping_cart_tickets().await {
                Ok(data) => data.tickets,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<Ticket>::new()
                }
            }
        });
    
    let (data, set_data) = create_signal(cx, Vec::<Ticket>::new());

    let tickets_data_table = move || {
        let value = resource.read(cx);
        match value {
            None => {
                total.set(0.0);
                set_data.set(Vec::new())
            },
            Some(val) => {
                total.set(crate::utils::simd_sum(&val.iter().map(|ticket| ticket.price).collect::<Vec<_>>()));
                set_data.set(val)
            }
        };
    };

    view! { cx,
        <div class="relative px-3 mb-0 mx-3">
            <div class="mb-2 mx-6 grid">
                <h2 class="mb-6 font-semibold text-primary text-2xl my-4">Shopping Cart</h2>
                <Transition fallback=move || { view! {cx, <div></div> } }>
                    <ErrorBoundary fallback=|cx, errors| view!{ cx, <div>{format!("{:?}", errors.get())}</div>}.into_view(cx)>
                        <table class="w-full flex-row text-sm text-center rounded-lg bordertext-gray-500">
                            <thead class="text-xs text-gray-200 uppercase bg-slate-700">
                                <tr class="border-y rounded-t-lg border-gray-800">
                                    // <th scope="col" class="px-6 py-3">#</th>
                                    <th scope="col" class="px-6 py-3">Movie Name</th>
                                    <th scope="col" class="px-6 py-3">Time Slot</th>
                                    <th scope="col" class="px-6 py-3">Price</th>
                                    <th scope="col" class="px-6 py-3">Seat Number</th>
                                    <th scope="col"></th>
                                </tr>
                            </thead>
                            <tbody>
                                { tickets_data_table }
                                <For 
                                    each = move || data.get()
                                    key = |record: &Ticket| record.id
                                    view = move |cx, record: Ticket| {
                                       view! { cx, <ShoppingCartTicketItem record refresh_trigger/> } 
                                    }
                                />
                                <tr class="border-b bg-slate-900 text-white border-gray-300">
                                    <td class="px-6 py-4"></td>
                                    <td class="px-6 py-4"></td>
                                    <td class="px-6 py-4"></td>
                                    <td class="px-6 py-4"></td>
                                    <td class="font-semibold text-lg px-6 py-4">Total: {move || total.get()} $</td>
                                </tr>
                            </tbody>
                        </table>
                    </ErrorBoundary>
                </Transition>
            </div>
            <div class="absolute right-0 top-0 h-full min-h-[1em] w-px self-stretch border-t-0 bg-gradient-to-tr from-transparent via-neutral-500 to-transparent opacity-25 block"/>
      </div>
    }
}

#[component]
pub fn ShoppingCartPaymentForm(cx: Scope) -> impl IntoView {
    let full_name = create_rw_signal(cx, String::new());
    let cvc = create_rw_signal(cx, String::new());
    let card_number = create_rw_signal(cx, String::new());
    let date = create_rw_signal(cx, String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate(cx);
        spawn_local(async move {
            let res = create_order(full_name.get_untracked(), date.get_untracked(), cvc.get_untracked(), card_number.get_untracked()).await; 
            match res {
                Ok(_) => navigate("/home", NavigateOptions::default()).unwrap(),
                Err(_) => {}
            }
        });
    };

    view!{ cx, 
        <div class="relative px-3 mb-0">
            <div class="mb-2 px-4 grid">
                <h2 class="mb-6 font-semibold text-primary text-2xl">Payment Form</h2>
                <Transition fallback=move || { view! {cx, <div></div> } }>
                    <form class="py-8 px-8 text-left" on:submit=on_submit>
                      <div class="mb-6">
                        <label for="full-name" class="font-semibold">Full Name</label>
                        <input type="text" id="full-name" on:input=move |event| { full_name.set(event_target_value(&event)); } prop:value=move || { full_name.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 pl-4 pr-72 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Filip Jovanov" required/>
                      </div>
                      <div class="mb-6">
                        <label class="font-semibold" for="card-number">Card Number</label>
                        <input type="text" id="card-number" on:input=move |event| { card_number.set(event_target_value(&event)); } prop:value=move || { card_number.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 pl-4 pr-72 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="**** **** **** ****" pattern=r"\d*" maxlength="16" title="Only 16 numbers allowed" required/>
                      </div>
                      <div class="mb-6 flex">
                        <div>
                            <label class="font-semibold" for="cvc">CVC</label>
                            <input type="text" id="cvc" on:input=move |event| { cvc.set(event_target_value(&event)); } prop:value=move || { cvc.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="***" pattern=r"\d*" maxlength=3 title="Only 3 numbers allowed" required/>
                        </div>
                        <div class="mx-12">
                            <label class="font-semibold" for="expiration-date">Expiration Date</label>
                            <input type="text" id="expiration-date" on:input=move |event| { date.set(event_target_value(&event)); } prop:value=move || { date.get() } class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block py-2.5 px-6 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="MM/YY" pattern=r"\d{2}\/\d{2}" title="Has to be formatted as MM/YY" maxlength=5 required/>
                        </div>
                      </div>
                      <button type="submit" class="text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 font-medium text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">Submit</button>
                    </form>
                </Transition>
     
            </div>
        </div>
    }
}


#[component]
pub fn ShoppingCartPage(cx:Scope) -> impl IntoView {
    view! {cx,
        <div class="h-screen w-full">
            <div class="flex py-12 place-content-center">
                <div class="w-3/4">
                    <div class="grid grid-cols-2">
                        <ShoppingCartTable/>
                        <ShoppingCartPaymentForm/>
                    </div>
                </div>
            </div>
        </div>

    }
}
