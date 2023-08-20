use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <main>
            <h1>{"> Projects"}</h1>
            <h2>{"Completed"}</h2>
            <ul>
                <li>{"Nothing.. 3:"}</li>
            </ul>
            <h2>{"In Progress"}</h2>
            <ul>
                <li><a href="https://github.com/StormLight14/void-ascender" target="_blank">{"Void Ascender "}</a>{"- Fast-paced platformer, being made with Godot 4."}</li>
                <li><a href="https://github.com/StormLight14/rust-chess" target="_blank">{"Chess "}</a>{"- CLI interface to play Chess, created with Rust."}</li>
            </ul>
        </main>
    }
}