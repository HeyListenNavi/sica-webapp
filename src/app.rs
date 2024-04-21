use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::capturing::Capturing;
use crate::pages::camera_management::CameraManagement;
use crate::pages::configuration::Configuration;

use crate::components::nav_bar::NavBar;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/capturing")]
    Capturing,
    #[at("/camera-management")]
    CameraManagement,
    #[at("/configuration")]
    Configuration,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Capturing => html! { <Capturing/> },
        Route::CameraManagement => html! { <CameraManagement/> },
        Route::Configuration => html! { <Configuration/> }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <NavBar/>
                <div class="main">
                    <Switch<Route> render={switch}/>
                </div>
            </BrowserRouter>
        </>
    }
}