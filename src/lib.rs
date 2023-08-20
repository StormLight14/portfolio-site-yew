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
                <div class="link">
                    <li><a href="https://github.com/StormLight14" target="_blank"><img src="img/github-white.png"/></a></li>
                </div>
            // in reverse order
                <div class="nav_item">
                    <li><Link<Route> to={Route::Contact}>{ "contact.txt" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Projects}>{ "projects.txt" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Experience}>{ "experience.txt" }</Link<Route>></li>
                    <li><Link<Route> to={Route::About}>{ "about.txt" }</Link<Route>></li>
                    <li><code>{"[portfolio@storm ~]$"}</code></li>
                </div>
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