use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::body::Body;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    
    let log_in_trigger = create_trigger(cx);
    
    view! {
        cx,
        <meta charset="utf-8" />
        <title>Integrated Systems Leptos</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Stylesheet id="custom-css" href="/style/site.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Header log_in_trigger/>
            <Body log_in_trigger/>
            <Footer/>
        </Router>
    }
}
