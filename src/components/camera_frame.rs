use yew::{html, Component, Context, Html};
use web_sys::*;
use js_sys::Boolean;
use gloo_timers::callback::Interval;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};

const API_URL: &str = "http://127.0.0.1:8000";

#[derive(Debug, Serialize, Deserialize)]
struct Prediction {
    x_coordinate: f64,
    y_coordinate: f64,
    x2_coordinate: f64,
    y2_coordinate: f64,
    detected_object: String,
    probs: f64,
}

pub enum Msg {
    StartPrediction,
    StopPrediction,
    MakePrediction,
}

pub struct CameraFrame {
    interval: Option<Interval>,
    client: Client
}

impl Component for CameraFrame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            interval: None,
            client: Client::new()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartPrediction => {
                let window = window().expect("Window should be an object");
                let navigator = window.navigator();
                let document = window
                    .document()
                    .expect("Window should have a Document object");

                // -> Stream webcam <-

                let media_devices = navigator
                    .media_devices()
                    .expect("Navigator should have a MediaDevices object");

                let camera_output = document
                    .get_element_by_id("video")
                    .expect("Element with the id \"video\" should exist")
                    .dyn_into::<HtmlVideoElement>()
                    .expect("Element should be an HtmlVideoElement");

                let mut constraints = MediaStreamConstraints::new();
                constraints.video(&Boolean::from(true));

                let video_devices_promise = media_devices
                    .get_user_media_with_constraints(&constraints)
                    .expect("result should be a Promise");

                spawn_local(async move {
                    let camera = JsFuture::from(video_devices_promise)
                        .await
                        .expect("could not resolve promise")
                        .dyn_into::<MediaStream>()
                        .expect("no camera available");

                    camera_output.set_src_object(Some(&camera));
                });

                // -> Prediction interval <-

                let interval_handle = {
                    let ctx_link = ctx.link().clone();
                    Interval::new(1000, move || ctx_link.send_message(Msg::MakePrediction))
                };

                self.interval = Some(interval_handle);
                true
            }
            Msg::StopPrediction => {
                self.interval = None;

                // -> Stop webcam stream <-

                let document = window()
                    .expect("Window should be an object")
                    .document()
                    .expect("Window should have a Document object");

                let camera_output = document
                    .get_element_by_id("video")
                    .expect("Element with the id \"video\" should exist")
                    .dyn_into::<HtmlVideoElement>()
                    .expect("Element should be an HtmlVideoElement");

                let video_tracks = camera_output
                    .src_object()
                    .expect("camera_output should contain MediaStream object")
                    .get_video_tracks();

                for track in video_tracks {
                    track
                        .dyn_into::<MediaStreamTrack>()
                        .expect("video_tracks should contain video tracks")
                        .stop();
                }

                camera_output.set_src_object(None);

                // -> Clear canvas <-

                let boxes_canvas = document
                    .get_element_by_id("video_top")
                    .expect("Element with the id \"video_top\" should exist")
                    .dyn_into::<HtmlCanvasElement>()
                    .expect("Element should be HtmlCanvasElement");

                let boxes_canvas_ctx = boxes_canvas
                    .get_context("2d")
                    .expect("HtmlCanvasElement should have 2d context")
                    .expect("2d context should be Object")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Object should be CanvasRenderingContext2d");

                boxes_canvas_ctx.clear_rect(0.0, 0.0, 640.0, 480.0);

                true
            }
            Msg::MakePrediction => {
                // -> Capture frame <-
                let document = window()
                    .expect("Window should be an object")
                    .document()
                    .expect("Window should have a Document object");

                let camera_output = document
                    .get_element_by_id("video")
                    .expect("Element with the id \"video\" should exist")
                    .dyn_into::<HtmlVideoElement>()
                    .expect("Element should be an HtmlVideoElement");

                let frame_canvas = document
                    .create_element("canvas")
                    .expect("Element not recognized")
                    .unchecked_into::<HtmlCanvasElement>();

                let frame_canvas_ctx = frame_canvas
                    .get_context("2d")
                    .expect("HtmlCanvasElement should have 2d context")
                    .expect("2d context should be Object")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Object should be CanvasRenderingContext2d");

                frame_canvas.set_width(camera_output.video_width());
                frame_canvas.set_height(camera_output.video_height());

                frame_canvas_ctx
                    .draw_image_with_html_video_element(&camera_output, 0.0, 0.0)
                    .expect("could not draw Canvas image from camera_output");

                let data_url = frame_canvas
                    .to_data_url_with_type("image/jpeg")
                    .expect("could not convert frame_canvas to dataURL");

                // -> Clear canvas <-

                let boxes_canvas = document
                    .get_element_by_id("video_top")
                    .expect("Element with the id \"video_top\" should exist")
                    .dyn_into::<HtmlCanvasElement>()
                    .expect("Element should be HtmlCanvasElement");

                let boxes_canvas_ctx = boxes_canvas
                    .get_context("2d")
                    .expect("HtmlCanvasElement should have 2d context")
                    .expect("2d context should be Object")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Object should be CanvasRenderingContext2d");

                
                // -> Send frame to API <-
                let client = self.client.clone();
                spawn_local(async move {
                    let url = format!("{API_URL}/predict");
                    let res = client.post(url).body(data_url).send().await;

                    if let Ok(response) = res {
                        if response.status() == StatusCode::OK {
                            let predictions = response
                                .json::<Vec<Prediction>>()
                                .await
                                .expect("Response should be in JSON format");
                            for prediction in predictions {
                                let x_top_left = prediction.x_coordinate;
                                let y_top_left = prediction.y_coordinate;
                                let x_bottom_right = prediction.x2_coordinate;
                                let y_bottom_right = prediction.y2_coordinate;

                                let width = x_bottom_right - x_top_left;
                                let height = y_bottom_right - y_top_left;

                                let object = prediction.detected_object.as_str();
                                
                                // -> Draw detection <-
                                
                                boxes_canvas_ctx.set_line_width(3 as f64);
                                boxes_canvas_ctx.set_stroke_style(&JsValue::from_str("#F00"));
                                boxes_canvas_ctx.stroke_rect(x_top_left, y_top_left, width, height);

                                boxes_canvas_ctx.set_font("20px Noto Sans");
                                boxes_canvas_ctx.set_fill_style(&JsValue::from_str("#F00"));
                                boxes_canvas_ctx
                                .fill_text(object, x_top_left, y_top_left - 8.0)
                                .expect("unable to write detected object text");
                            }
                        }
                    }
                    boxes_canvas_ctx.clear_rect(0.0, 0.0, 640.0, 480.0);
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_predicting = self.interval.is_some();

        html! {
            <>
                <div class="camera">
                    <canvas class="camera__canvas" id="video_top" width="640" height="480"/>
                    <video class="camera__video" id="video" autoplay={true} width="640" height="480"/>
                </div>
                <div class="buttons">
                    <button disabled={is_predicting} class="start" onclick={ctx.link().callback(|_| Msg::StartPrediction)}>
                        { "Comenzar" }
                    </button>
                    <button disabled={is_predicting == false} class="stop" onclick={ctx.link().callback(|_| Msg::StopPrediction)}>
                        { "Detener" }
                    </button>
                </div>
            </>
        }
    }
}