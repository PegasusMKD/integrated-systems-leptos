use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    return view! {cx,
            <div class="text-center">
                <h1 class="text-[70px] font-thin">Welcome</h1>
                <p>Learn about <a href="https://leptos.dev/">building Web apps with Rust & Leptos</a>.</p>
            </div>
    };
}
