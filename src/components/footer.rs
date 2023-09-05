use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let sign = "\u{00A9}";

    view! {
        cx,
        <footer class="border-top footer text-muted">
            <div class="container">
            {sign} 2023 - ISH - <a>Privacy</a>
            </div>
        </footer>
    }
}
