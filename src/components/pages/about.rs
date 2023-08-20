use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <h1>{"> About Me"}</h1>
            <p>{"Some of my interests include software development, cybersecurity, Linux, and playing the guitar!"}</p>
            //<footer>{"(I use arch btw)"}</footer>
        </>
    }
}