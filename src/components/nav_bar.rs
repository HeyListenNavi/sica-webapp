use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let navigator =use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button class="button--home" {onclick}>
                <span class="material-symbols-rounded">{ "home" }</span>
                <span class="text">{ "Inicio" }</span>
            </button>
        }
    };

    let go_capturing_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Capturing));
        html! {
            <button class="button--capturing" {onclick}>
                <span class="material-symbols-rounded">{ "photo_library" }</span>
                <span class="text">{ "Capturas" }</span>
            </button>
        }
    };

    let go_camera_management_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::CameraManagement));
        html! {
            <button class="button--cameras" {onclick}>
                <span class="material-symbols-rounded">{ "photo_camera" }</span>
                <span class="text">{ "Cámaras" }</span>
            </button>
        }
    };

    let go_configuration_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Configuration));
        html! {
            <button class="button--configuration" {onclick}>
                <span class="material-symbols-rounded">{ "settings" }</span>
                <span class="text">{ "Configuración" }</span>
            </button>
        }
    };
    html! {
        <nav class="navbar">
            <ul>
                <li>
                    {go_home_button}
                </li>
                <li>
                    {go_capturing_button}
                </li>
                <li>
                    {go_camera_management_button}
                </li>
                <li>
                    {go_configuration_button}
                </li>
            </ul>
        </nav>
    }
}