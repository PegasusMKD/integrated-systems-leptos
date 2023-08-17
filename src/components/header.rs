use leptos::*;
use leptos_router::{A, use_location};

// TODO: Implement later with proper toggle and checks, at the moment it doesn't work as it was
// copied over from FlowBite
#[component]
fn MobileMenu(cx: Scope) -> impl IntoView {
    view! {
            cx,
            // Extra menu
            <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false">
                <span class="sr-only">Open main menu</span>
                <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
                </svg>
            </button>
        }
    }

#[component]
fn LinkItem(cx: Scope, url: String, txt: String) -> impl IntoView {
    let copied_url = url.clone();
    let is_current = move || use_location(cx).pathname.get() == copied_url;
    let non_current_class = "block py-2 pl-3 pr-4 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent";
    let current = "block py-2 pl-3 pr-4 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500";
    view! {
        cx,
        <li>
            <A href=url class=move || if is_current() { current } else { non_current_class } >{txt}</A>
        </li>
    }

}


#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <header>
            <nav class="bg-white border-gray-200 dark:bg-gray-900 shadow-xl">
                <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                    <A href="/home" class="flex items-center">
                        <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">ISH</span>
                    </A>
                   
                    <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                        <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                                <LinkItem url="/shopping-cart".to_string() txt="Shopping Cart".to_string()/>
                                <LinkItem url="/view-slots".to_string() txt="All View Slots".to_string()/>
                                <LinkItem url="/tickets".to_string() txt="All Tickets".to_string()/>
                                <LinkItem url="/orders".to_string() txt="Your Orders".to_string()/>
                                <LinkItem url="/user-management".to_string() txt="User Management".to_string()/>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
