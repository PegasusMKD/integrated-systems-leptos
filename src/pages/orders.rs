use leptos::*;

use crate::models::{BearerRequestBuilder, Order};

async fn get_orders_by_user() -> reqwest::Result<Vec<Order>> {
    let client = reqwest::Client::new();
    let request = client.get("https://localhost:44316/api/order/by-user")
        .add_token()
        .send()
        .await?;

    leptos::log!("Getting the requested data...");
    if !request.status().is_success() {
        leptos::log!("Passed the get...");
        leptos::log!("Status: {:?}", request.status());
        return Ok(Vec::new()); // TODO: Add proper error return so it can be handled down the line
    }

    request.json::<Vec<Order>>().await
}

#[component]
pub fn OrderRow(cx: Scope, record: Order) -> impl IntoView {
    let pdf_url = format!("https://localhost:44316/api/order/invoice/{}", record.id);
    view! {
        cx,
        <tr class="border-b bg-white border-gray-300 hover:bg-gray-50">
            // <th scope="row" class="px-6 py-4">{idx.get()}</th>
            <td class="px-6 py-4">{"Order#"}{record.order_number}</td>
            <td class="px-6 py-4">{record.total_price} $</td>
            <td class="px-6 py-4">
                <a href=pdf_url download=""><button class="text-white mr-2 bg-green-700 hover:bg-green-800 focus:ring-4 focus:outline-none focus:ring-green-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">Generate PDF Invoice</button></a>
            </td>
        </tr>
    }
}


#[component]
pub fn OrdersPage(cx:Scope) -> impl IntoView {
    let resource: leptos::Resource<(), Vec<Order>> = create_local_resource(cx, 
        move || (), 
        move |_| async move {
            match get_orders_by_user().await {
                Ok(data) => data,
                Err(err) => {
                    leptos::log!("[Tickets Page] Error doing a request to fetch tickets: {:?}", err);
                    Vec::<Order>::new()
                }
            }
        });
    
    let (data, set_data) = create_signal(cx, Vec::<Order>::new());

    let orders_data_table = move || {
        let value = resource.read(cx);
        match value {
            None => set_data.set(Vec::new()),
            Some(val) => set_data.set(val)
        };
    };

 

    view! {cx,
        <div class="h-screen w-full">
            <div class="flex place-content-center">
                <div class="w-3/4">
                   <h2 class="text-5xl font-semibold mb-4">Orders</h2>
                    <Transition fallback=move || { view! {cx, <div></div> } }>
                        <ErrorBoundary fallback=|cx, errors| view!{ cx, <div>{format!("{:?}", errors.get())}</div>}.into_view(cx)>
                            <table class="w-full flex-row text-sm text-center rounded-lg bordertext-gray-500">
                                <thead class="text-xs text-gray-700 uppercase bg-gray-50">
                                    <tr class="border-y rounded-t-lg border-gray-800">
                                        // <th scope="col" class="px-6 py-3">#</th>
                                        <th scope="col" class="px-6 py-3">Order Number</th>
                                        <th scope="col" class="px-6 py-3">Total Price</th>
                                        <th scope="col" class="px-6 py-3"></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { orders_data_table }
                                    <For 
                                        each = move || data.get()
                                        key = |record: &Order| record.id
                                        view = move |cx, record: Order| {
                                           view! { cx, <OrderRow record/> } 
                                        }
                                    />
                                </tbody>
                            </table>
                        </ErrorBoundary>
                    </Transition>
                </div>
            </div>
        </div>

    }
}
