use leptos::*;
use leptos_router::*;

use crate::pages::home::HomePage;
use crate::pagination::PaginationBar;
use crate::pages::view_slots::ViewSlotPage;
use crate::pages::tickets::TicketsPage;

#[component]
pub fn Body(cx: Scope, pagination: RwSignal<usize>) -> impl IntoView {
    view! {
        cx,
        <div class="w-full h-full">
            <main role="main">
                <div class="min-h-screen mx-auto mt-6">
                <Routes>
                    <Route path="" view=move |cx| view! {
                        cx,
                        <PaginationBar count={14} signal={pagination}/>
                        {move || {
                            pagination.get()
                        }}
                    }/>
                   <Route path="/home" view=move |cx| view! {
                        cx,
                        <HomePage/>
                    }/>
                    <Route path="/shopping-cart" view=move |cx| view! {
                        cx,
                        <p>Shopping Cart</p>
                    }/>
                    <Route path="/view-slots" view=Outlet>
                        <ViewSlotPage/>
                    </Route>
                    <Route path="/tickets" view=Outlet>
                        <TicketsPage/>   
                    </Route>
                   <Route path="/orders" view=move |cx| view! {
                        cx,
                        <p>Orders</p>
                    }/>
                    <Route path="/user-management" view=move |cx| view! {
                        cx,
                        <p>User Management</p>
                    }/> 
              </Routes>
            </div>
        </main>
        </div>
    }
}
