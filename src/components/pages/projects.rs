use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <main>
            <h1>{"> Projects"}</h1>
            <h2>{"Completed"}</h2>
            <ul>
                <li><a href="https://github.com/StormLight14/plant-waterer" target="_blank">{"Arduino Plant Waterer "}</a>{"- Automatically water a plant!"}</li>
                <li><a href="https://github.com/StormLight14/arduino-chess-timer" target="_blank">{"Arduino Chess Timer "}</a>{"- Chess timer using LCD screen."}</li>
                //<li><a href="https://stormlight-base64.netlify.app/" target="_blank">{"Base64 Converter "}</a>{"- Encodes and Decodes Base64 Text"}</li>
            </ul>
            <h2>{"In Progress"}</h2>
            <ul>
                <li><a href="https://github.com/StormLight14/pew-pew-io" target="_blank">{"Pew Pew "}</a>{"- Multiplayer 2D clone of Counter-Strike."}</li>
                <li><a href="https://github.com/StormLight14/void-ascender" target="_blank">{"Void Ascender "}</a>{"- Fast-paced platformer."}</li>
                <li><a href="https://github.com/StormLight14/rust-chess" target="_blank">{"Chess "}</a>{"- CLI interface for playing Chess."}</li>
            </ul>
        </main>
    }
}
