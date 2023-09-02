use yew::prelude::*;

#[function_component(Experience)]
pub fn about() -> Html {
    html! {
        <main>
            <h1>{"> Experience"}</h1>
            <h2>{"Programming"}</h2>
            <p>{"Proficient in Python for approximately 2 years, and started learning Rust (the language this site was constructed with!) about 1 year ago."}</p>
            <h2>{"Linux"}</h2>
            <p>{"Used Linux for both desktop and server usage over 2 years; understand the fundamentals."}</p>
        </main>
    }
}
