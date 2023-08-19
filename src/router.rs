use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::pages::{about::About, contact::Contact, experience::Experience};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    About,
    #[at("/experience")]
    Experience,
    #[at("/contact")]
    Contact,
}


pub fn switch(route: Route) -> Html {
    match route {
        Route::About => html!{<About />},
        Route::Experience => html!{<Experience />},
        Route::Contact => html!{<Contact />},
    }
}