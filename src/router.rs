use crate::components::pages::{
    about::About, contact::Contact, experience::Experience, projects::Projects,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/portfolio")]
    About,
    #[at("/experience")]
    Experience,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::About => html! {<About />},
        Route::Experience => html! {<Experience />},
        Route::Projects => html! {<Projects />},
        Route::Contact => html! {<Contact />},
    }
}
