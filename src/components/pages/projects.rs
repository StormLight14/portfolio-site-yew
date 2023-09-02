use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <main>
            <h1>{"> Projects"}</h1>
            <h2>{"Completed"}</h2>
            <ul>
                <li><a href="https://github.com/StormLight14/arduino-chess-timer" target="_blank">{"Arduino Chess Timer "}</a>{"- Play chess, but with a timer! (Requires LCD Screen)"}</li> 
                <li><a href="https://stormlight-base64.netlify.app/" target="_blank">{"Base64 Converter "}</a>{"- Encodes and Decodes Base64 Text"}</li>
            </ul>
            <h2>{"In Progress"}</h2>
            <ul>
                <li><a href="https://github.com/StormLight14/void-ascender" target="_blank">{"Void Ascender "}</a>{"- Fast-paced platformer, being made with Godot 4."}</li>
                <li><a href="https://github.com/StormLight14/rust-chess" target="_blank">{"Chess "}</a>{"- CLI interface to play Chess, created with Rust."}</li>
                <li><a href="https://github.com/StormLight14/plant-waterer" target="_blank">{"Arduino Plant Waterer "}</a>{"- Water your plants! (Requires 5v water pump)"}</li>
            </ul>
        </main>
    }
}
