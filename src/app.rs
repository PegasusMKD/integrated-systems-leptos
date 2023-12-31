use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::body::Body;
use crate::components::login_partial::LoginPartial;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    
    let log_in_trigger = create_rw_signal(cx, false);
    
    view! {
        cx,
        <meta charset="utf-8" />
        <title>Integrated Systems Leptos</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Stylesheet id="custom-css" href="/style/site.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <LoginPartial trigger={log_in_trigger}/>
            <Header log_in_trigger/>
            <Body log_in_trigger/>
            <Footer/>
        </Router>
    }
}
