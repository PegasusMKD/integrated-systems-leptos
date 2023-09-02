use leptos::*;
use leptos_router::*;

use crate::pages::home::HomePage;
use crate::pages::view_slots::ViewSlotPage;
use crate::pages::tickets::TicketsPage;

use crate::pages::login::LoginPage;
use crate::pages::shopping_cart::ShoppingCartPage;
// use crate::pages::register::RegisterPage;

#[component]
pub fn Body(cx: Scope, log_in_trigger: Trigger) -> impl IntoView {
    view! {
        cx,
        <div class="w-full h-full">
            <main role="main">
                <div class="min-h-screen mx-auto mt-6">
                <Routes>
                    <Route path="" view=move |cx| view! {
                        cx,
                        <div>Hellooooo</div>
                    }/>
                   <Route path="/login" view=move |cx| { view! {cx, <LoginPage trigger={log_in_trigger}/> } }/>
   //                <Route path="/register" view=RegisterPage/>
                   <Route path="/home" view=move |cx| view! {
                        cx,
                        <HomePage/>
                    }/>
                    <Route path="/shopping-cart" view=ShoppingCartPage/>
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
