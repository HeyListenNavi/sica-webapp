use js_sys::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::*;
use yew::prelude::*;

use crate::components::camera_frame::CameraFrame;
use crate::components::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    let snap_frame = Callback::from(|_| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        // Get elements
        let video_element = document.get_element_by_id("video")
            .unwrap()
            .unchecked_into::<HtmlVideoElement>();

        let canvas_element = document.create_element("canvas")
            .unwrap()
            .unchecked_into::<HtmlCanvasElement>();

        let ctx = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();
        
        // Set the canvas element size to the same size as the video element
        canvas_element.set_width(video_element.video_width());
        canvas_element.set_height(video_element.video_height());

        // Draw an image from a video element
        ctx.draw_image_with_html_video_element(&video_element, 0.0, 0.0)
            .unwrap();
        // Export canvas element into image
        let data_url = canvas_element.to_data_url().unwrap();
        console::log_1(&data_url.into());
    });

    let draw_rectangle = Callback::from(|_| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        
        // Get canvas
        let canvas_element = document.get_element_by_id("video_top")
            .unwrap()
            .unchecked_into::<HtmlCanvasElement>();
        let ctx = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<CanvasRenderingContext2d>();

        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#FF0000"));
        ctx.stroke_rect(20.0, 20.0, 400.0, 450.0);
        ctx.fill();
    });

    let capture_camera = Callback::from(|_| {
        wasm_bindgen_futures::spawn_local(async move {
            let navigator = window().unwrap().navigator();
            let window = window().unwrap();
            let document = window.document().unwrap();

            // List of media devices obtained from the navigator
            let media_devices = navigator.media_devices().unwrap();

            // Get the video DOM element
            let video_element = document.get_element_by_id("video")
                .unwrap()
                .unchecked_into::<HtmlVideoElement>();

            // These constraints specify the desired properties of the media stream
            let mut constraints = MediaStreamConstraints::new();

            // Create a media stream from a camera
            constraints.video(&Boolean::from(true));
            let video_devices_promise = media_devices
                .get_user_media_with_constraints(&constraints)
                .unwrap();

            let camera = JsFuture::from(video_devices_promise)
                .await
                .unwrap()
                .unchecked_into::<MediaStream>();

            // Sets the source object of the video element in the dom to the camera output
            video_element.set_src_object(Some(&camera));
        })
    });

       let stop_capture = Callback::from(|_| {
            let window = window().unwrap();
            let document = window.document().unwrap();

            // Get the video DOM element
            let video_element = document.get_element_by_id("video")
                .unwrap()
                .unchecked_into::<HtmlVideoElement>();

            // Sets the source object of the video element in the dom to the camera output
            video_element.set_src_object(None);
        });

    html! {
        <>
            <Header/>
            <CameraFrame/>
            <div class="buttons">
                <button onclick={snap_frame}>{ "Capturar frame" }</button>
                <button onclick={draw_rectangle}>{ "Dibujar rectángulo" }</button>
                <button onclick={capture_camera}>{ "Comenzar" }</button>
                <button onclick={stop_capture}>{ "Detener" }</button>
            </div>
        </>
    }
}
