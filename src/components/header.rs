use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <h1>{ "SICA" }</h1>
        </header>
    }
}