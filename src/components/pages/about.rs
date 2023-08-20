use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <h1>{"About Me"}</h1>
            <p>{"Interested in software development, game development, cybersecurity, and Linux!"}</p>
        </>
    }
}