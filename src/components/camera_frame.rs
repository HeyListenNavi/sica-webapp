use yew::prelude::*;

#[function_component(CameraFrame)]
pub fn camera_frame() -> Html {
    html! {
        <div class="camera">
            <canvas class="camera__canvas" id="video_top" width="640" height="480"/>
            <video class="camera__video" id="video" autoplay={true} width="640" height="480"/>
        </div>
    }
}