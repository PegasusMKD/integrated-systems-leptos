use leptos::*;
use leptos_router::{A, use_location};

use crate::models::UserDetails;


#[component]
fn LinkItem(cx: Scope, url: String, txt: String) -> impl IntoView {
    let copied_url = url.clone();
    let is_current = move || use_location(cx).pathname.get().starts_with(&copied_url);
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
pub fn Header(cx: Scope, log_in_trigger: Trigger) -> impl IntoView {
    let resource = create_local_resource(cx,
        move || log_in_trigger.track(), 
        move |_| async move { UserDetails::user_logged_in() }
    );
    
    let generate_menu = move || {
        let logged_in_resource = resource.read(cx);
        let logged_in = match logged_in_resource {
            None => false,
            Some(val) => val
        };

        if logged_in {
            view! {cx, 
                <LinkItem url="/shopping-cart".to_string() txt="Shopping Cart".to_string()/>
                <LinkItem url="/view-slots".to_string() txt="All View Slots".to_string()/>
                <LinkItem url="/tickets".to_string() txt="All Tickets".to_string()/>
                <LinkItem url="/orders".to_string() txt="Your Orders".to_string()/>
                <LinkItem url="/user-management".to_string() txt="User Management".to_string()/>
                <p class="px-4 font-semibold text-base">Hello, { UserDetails::read_detail("username".to_string()) }!</p>
            }
        } else {
            view! {cx, 
                <LinkItem url="/login".to_string() txt="Login".to_string()/>
                <LinkItem url="/register".to_string() txt="Register".to_string()/>
            }
        }
    };

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
                            { generate_menu }
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
