use yew::prelude::*;
use web_sys::*;
use js_sys::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::{JsCast, prelude::Closure, JsValue};

#[function_component(App)]
pub fn app() -> Html {
    // Use wasm_bindgen_futures to be able to process javascript promises
    wasm_bindgen_futures::spawn_local(async move {
        // The Window.navigator read-only property returns a reference to the Navigator object, which has methods and properties about the application running the script
        let navigator = window().unwrap().navigator();

        // List of media devices obtained from the navigator
        let media_devices = navigator.media_devices().unwrap();

        // Get the video DOM element
        let video_element = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("video")
            .unwrap()
            .unchecked_into::<HtmlVideoElement>();

        // These constraints specify the desired properties of the media stream
        let mut constraints = MediaStreamConstraints::new();

        // Create a media stream from a camera
        constraints.video(&Boolean::from(true));
        let video_devices_promise = media_devices.get_user_media_with_constraints(&constraints).unwrap();
        let camera = JsFuture::from(video_devices_promise).await.unwrap().unchecked_into::<MediaStream>();
    
        // Sets the source object of the video element in the dom to the camera output
        video_element.set_src_object(Some(&camera));
    });

    let snap_frame = Callback::from(move |_| {
        // Get elements
        let video_element = window().unwrap()
            .document().unwrap()
            .get_element_by_id("video").unwrap()
            .unchecked_into::<HtmlVideoElement>();

        let canvas_element = window().unwrap()
            .document().unwrap()
            .get_element_by_id("frame_canvas").unwrap()
            .unchecked_into::<HtmlCanvasElement>();

        let ctx = canvas_element
            .get_context("2d").unwrap().unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();
        // Set the canvas element size to the same size as the video element
        canvas_element.set_width(video_element.video_width());
        canvas_element.set_height(video_element.video_height());
        
        // Draw an image from a video element
        ctx.draw_image_with_html_video_element(&video_element, 0.0, 0.0).unwrap();
        // Export canvas element into image
        let data_url = canvas_element.to_data_url().unwrap();
        console::log_1(&data_url.into()); 
    });

    let draw_rectangle = Callback::from(move |_| {
        // Get canvas
        let canvas_element = window().unwrap()
            .document().unwrap()
            .get_element_by_id("video_top").unwrap()
            .unchecked_into::<HtmlCanvasElement>();
        let ctx = canvas_element
            .get_context("2d").unwrap().unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();

        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#FF0000"));
        ctx.stroke_rect(20.0,20.0,400.0,450.0);
        ctx.fill();
    });

    html! {
        <>
            <header>
                <h1>{"Detección de armas"}</h1>
            </header>
            <div>
            <div class="sidebar">
                <ul>
                    <li>{ "Inicio" }</li>
                    <li>{ "Capturas" }</li>
                    <li>{ "Camaras" }</li>
                    <li>{ "Agregar camaras" }</li>
                    <li>{ "Configuración" }</li>
                </ul>
            </div>
            <div class="main">
                <div class="camera">
                    <canvas class="camera__canvas" id="video_top" width="640" height="480"/>
                    <video class="camera__video" id="video" autoplay={true} width="640" height="480"/>
                </div>
                </div>
                <canvas id="frame_canvas"></canvas>
                <div class="output">
                    <img id="photo" alt="The screen capture will appear in this box."/>
                </div>
                <button onclick={snap_frame}>{ "Capturar frame" }</button>
                <button onclick={draw_rectangle}>{ "Dibujar rectángulo" }</button>
            </div>
        </>
    }
}