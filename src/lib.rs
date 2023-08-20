use yew::prelude::*;
use yew_router::prelude::*;
use router::{Route, switch};

mod router;
mod components;

pub fn convert_list(list: &Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
}

pub fn convert_list_uppercase(list: &Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item.to_uppercase()}</li>}).collect()
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class="navbar">
            <ul>
            // in reverse order
                <li><Link<Route> to={Route::Contact}>{ "03. contact" }</Link<Route>></li>
                <li><Link<Route> to={Route::Experience}>{ "02. experience" }</Link<Route>></li>
                <li><Link<Route> to={Route::About}>{ "01. about" }</Link<Route>></li>
                <li><code>{"[portfolio@storm ~]$"}</code></li>
            </ul>
        </div>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Navbar />
                <Switch<Route>render={switch} />
            </BrowserRouter>
        </main>
    }
}