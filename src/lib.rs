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
        <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Navbar />
            <BrowserRouter>
                <Switch<Route>render={switch} />
            </BrowserRouter>
        </main>
    }
}