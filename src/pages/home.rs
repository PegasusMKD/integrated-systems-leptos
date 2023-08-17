use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    return view! {cx,
            <div class="text-center">
                <h1 class="text-[80px] font-thin">Welcome</h1>
                <p>Learn about <a href="https://docs.microsoft.com/aspnet/core">building Web apps with ASP.NET Core</a>.</p>
            </div>
    };
}
