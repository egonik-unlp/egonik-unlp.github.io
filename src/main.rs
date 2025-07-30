use leptos::ev::{DragEvent, SubmitEvent};
use leptos::html;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::{either::Either, ev::Event};
use serde::{Deserialize, Serialize};

use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use wasm_bindgen::{JsCast, JsValue};

use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Serialize, Clone, Deserialize, Default)]
struct Ingredients {
    id: i32,
    doc: String,
    score_db: f32,
    info: String,
}
#[derive(Debug, Serialize, Clone, Deserialize, Default)]
struct Response {
    duration_seconds: f32,
    ingredientes: Vec<Ingredients>,
    len: u8,
    len90: u8,
}
#[derive(Debug, Clone)]
struct CloneableError(String);

impl std::fmt::Display for CloneableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CloneableError {}

pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("could not initialize logger");
    leptos::mount::mount_to_body(App);
}
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="header-app">

            <div class="flex justify-center bg-purple-300 h-30 justify-start  ">
                <img class="object-contain " src="../assets/images/logo.png" alt="EE" />
                <h2 class="title self-center justify-self-center ">Test api gv 2.0</h2>
            </div>
        </div>
        <Router>
            <nav class="flex border-t p-2 m-4 justify-around">
                <A href="/">"Single"</A>
                <A href="/triple">"Triple"</A>
            </nav>
            <main class="border-b">
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=UrlHandlerSingle />
                    <Route path=path!("/triple") view=UrlHandler />
                </Routes>
            </main>
        </Router>
        <script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/js/bootstrap.bundle.min.js"
            integrity="sha384-ndDqU0Gzau9qJ1lfW4pNLlhNTkCfHzAVBReH9diLvGRem5+R9g2FzA8ZGN954O5Q"
            crossorigin="anonymous"
        ></script>
    }
}
#[component]
fn UrlHandler() -> impl IntoView {
    let (url, set_url) = signal("".to_owned());
    let input_element: NodeRef<html::Input> = NodeRef::new();
    let (url2, set_url2) = signal("".to_owned());
    let input_element2: NodeRef<html::Input> = NodeRef::new();
    let (url3, set_url3) = signal("".to_owned());
    let input_element3: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_url.set(value);
        let value = input_element2
            .get()
            .expect("<input> should be mounted")
            .value();
        set_url2.set(value);
        let value = input_element3
            .get()
            .expect("<input> should be mounted")
            .value();
        set_url3.set(value);
    };
    view! {
        <div class="submit-box">
            <p>Ingrese url de imagen de ingredientes</p>
            <div class="search-elements flex justify-center">
                <form class="flex flex-column w-40 items-center justify-center" on:submit=on_submit>
                    <input class="border" type="text" value=url node_ref=input_element />
                    <input class="border" type="text" value=url2 node_ref=input_element2 />
                    <input class="border" type="text" value=url3 node_ref=input_element3 />
                    <input type="submit" value="Submit" />
                </form>
            </div>
            <p>
                {move || {
                    if !url.get().is_empty() || !url2.get().is_empty() || !url3.get().is_empty() {
                        Either::Left(
                            view! {
                                <FetchHandler url=url />
                                <FetchHandler url=url2 />
                                <FetchHandler url=url3 />
                            },
                        )
                    } else {
                        Either::Right("Esperando a que ingrese todas las URLs")
                    }
                }}
            </p>
        </div>
    }
}

#[component]
fn UrlHandlerSingle() -> impl IntoView {
    let (url, set_url) = signal("".to_owned());
    let input_element: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_url.set(value);
    };
    view! {
        <div class="submit-box">
            <p>Ingrese url de imagen de ingredientes</p>
            <div>
                <form class="flex justify-center" on:submit=on_submit>
                    <input class="border mx-4" type="text" value=url node_ref=input_element />
                    <input type="submit" value="Submit" />
                </form>
            </div>
            <p>
                {move || {
                    if !url.get().is_empty() {
                        Either::Left(view! { <FetchHandler url=url /> })
                    } else {
                        Either::Right("Esperando a que ingrese la URL")
                    }
                }}
            </p>
        </div>
    }
}
async fn fetch_response_data(url: String) -> Result<Response, CloneableError> {
    use gloo_net::http::Request;
    let request_string = format!("http://35.208.164.235:8010?image_url={}", url);
    log::debug!("Request string = {}", url.clone());
    let string_response = Request::get(&request_string)
        .send()
        .await
        .map_err(|err| CloneableError(err.to_string()))?
        .text()
        .await
        .map_err(|err| CloneableError(err.to_string()))?;

    leptos::logging::log!("Response {:?}", string_response);
    let response: Response = serde_json::from_str(string_response.as_str())
        .map_err(|err| CloneableError("No se pudo procesar la imagen".to_owned()))?;
    log::debug!("Yo serialize todo bien ");
    return Ok(response);
}

#[component]
fn FetchHandler(url: ReadSignal<String>) -> impl IntoView {
    let data = LocalResource::new(move || {
        let reactive_url = url.get();
        fetch_response_data(reactive_url)
        // mock_response()
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="spinner-border m4" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
            }
        }>
            <h2>"Api Response"</h2>
            {move || {
                data.get()
                    .map(|inner| {
                        match inner {
                            Ok(value) => view! { <IngredientsCard ingredients=value /> }.into_any(),
                            Err(err) => {
                                view! {
                                    <div class="error">
                                        <p>"Error haciendo solicitud "{err.0}</p>
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
fn IngredientsCard(ingredients: Response) -> impl IntoView {
    //  --TDOO use read signal
    let (ing_value, _ing_set_value) = signal(ingredients);
    let has_field_populated = move || ing_value.get().ingredientes.len() != 0;
    view! {
        <p class="verdaderamente-es-tan-grande?">Ingredientes encontrados:</p>
        {move || {
            if has_field_populated() {
                view! {
                    <h4>Se detectaron {ing_value.get().ingredientes.len()}ingredientes</h4>
                    <Accordion response=ing_value.get() />
                }
                    .into_any()
            } else {
                view! { <p>Si no me das nada no te doy nada</p> }.into_any()
            }
        }}
    }
}

#[component]
fn Accordion(response: Response) -> impl IntoView {
    view! {
        <div class="container-sm">
            <div class="accordion accordion-flush" id="accordionFlushExample">
                {response
                    .ingredientes
                    .into_iter()
                    .enumerate()
                    .map(|(n, ingredient)| {
                        view! {
                            <AccordionNode name_of_child=ingredient.doc position=n>
                                <div>
                                    <p>"Informacion sobre el ingrediente :  "{ingredient.info}</p>
                                    <p style=match ingredient.score_db as u32 {
                                        0 => "color: purple;",
                                        1 => "color: orange;",
                                        3 => "color: green;",
                                        _ => "color: red;",
                                    }>"Puntaje en nuestra db:  "{ingredient.score_db}</p>
                                </div>
                            </AccordionNode>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
fn AccordionNode(
    children: ChildrenFragment,
    name_of_child: String,
    position: usize,
) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| {
            let unique_id = format!("flush-collapseOne-{}", position);
            log::debug!("unique_id = {}", unique_id);
            view! {
                <div class="accordion-item">
                    <h2 class="accordion-header">
                        <button
                            class="accordion-button collapsed"
                            type="button"
                            data-bs-toggle="collapse"
                            data-bs-target=format!("#{}", unique_id)

                            aria-expanded="false"
                            aria-controls=unique_id.clone()
                        >
                            <p>{name_of_child.clone()}</p>
                        </button>
                    </h2>
                    <div
                        id=unique_id.clone()
                        class="accordion-collapse collapse"
                        data-bs-parent="#accordionFlushExample"
                    >
                        <div class="accordion-body">{child}</div>
                    </div>
                </div>
            }
        })
        .collect_view();
    return children;
}

async fn mock_response() -> Result<Response, CloneableError> {
    let string = include_str!("../sample_response.json");
    let response: Response =
        serde_json::from_str(string).map_err(|err| CloneableError(err.to_string()))?;
    return Ok(response);
}
// async fn process_and_upload(file: web_sys::File) -> Ingredients {
//     let (uploading, set_uploading) = signal(false);
//     set_uploading.set(true);
//     let promise = window()
//         .create_image_bitmap_with_blob(&file)
//         .expect("problema creando promesa");
//     let jsimage = JsFuture::from(promise).await.unwrap();
//     let image_bitmap: web_sys::ImageBitmap = jsimage.dyn_into().unwrap();
//     let resp = Request::post("http://proto-api.work.gd:8081/post")
//         .header("Content-Type", "image/jpeg")
//         .body(JsValue::from(image_bitmap))
//         .unwrap()
//         .send()
//         .await;
//     if let Ok(response) = resp {
//         log::debug!("Upload succesful");
//         let ingredientes: Ingredients =
//             serde_json::from_str(&response.text().await.unwrap()).unwrap();
//         set_uploading.set(false);
//         return ingredientes;
//     } else {
//         log::debug!("Upload unsuccesful");
//         set_uploading.set(false);
//         return Ingredients::default();
//     }
// pub async fn process_and_upload(file: web_sys::File) -> Ingredients {
//     // Decode image into ImageBitmap
//     let promise = window().create_image_bitmap_with_blob(&file).unwrap();
//     let js_img = JsFuture::from(promise).await.unwrap();
//     let image_bitmap: web_sys::ImageBitmap = js_img.dyn_into().unwrap();
//
//     // Create canvas and draw the image
//     let document = window().document().unwrap();
//     let canvas: web_sys::HtmlCanvasElement = document
//         .create_element("canvas")
//         .unwrap()
//         .dyn_into()
//         .unwrap();
//     let (w, h) = (image_bitmap.width(), image_bitmap.height());
//     canvas.set_width(w);
//     canvas.set_height(h);
//     let ctx: web_sys::CanvasRenderingContext2d = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into()
//         .unwrap();
//     ctx.draw_image_with_image_bitmap(&image_bitmap, 0.0, 0.0)
//         .unwrap();
//
//     // Convert canvas to Blob (JPEG)
//     let (tx, rx) = futures::channel::oneshot::channel();
//     let cb = wasm_bindgen::closure::Closure::once(move |b: JsValue| {
//         tx.send(b.dyn_into::<web_sys::Blob>().unwrap()).ok();
//     });
//     canvas
//         .to_blob_with_callback_and_type(cb.as_ref().unchecked_ref(), "image/jpeg")
//         .unwrap();
//     cb.forget();
//     let jpeg_blob = rx.await.unwrap();
//
//     // Convert Blob to Vec<u8>
//     let abuf = JsFuture::from(jpeg_blob.array_buffer()).await.unwrap();
//     let u8arr = Uint8Array::new(&abuf);
//     let mut bytes = vec![0; u8arr.length() as usize];
//     u8arr.copy_to(&mut bytes[..]);
//
// // Send POST request
//     let resp = Request::post("http://proto-api.work.gd:8081/post")
//         .header("Content-Type", "image/jpeg")
//         .body(bytes)
//         .unwrap()
//         .send()
//         .await;
//
//     match resp {
//         Ok(r) => {
//             let text = r.text().await.unwrap_or_default();
//             serde_json::from_str(&text).unwrap_or_default()
//         }
//         Err(_) => Ingredients::default(),
//     }
// }

#[component]
fn ImageUploader() -> impl IntoView {
    let file_input_ref: NodeRef<Input> = NodeRef::new();
    let on_file_change = move |ev: Event| {
        if let Some(file_list) = file_input_ref.get() {
            let files = event_target_value(&ev);
        }
    };
    let on_drop = move |ev: DragEvent| {
        if let Some(file_list) = file_input_ref.get() {
            let files = file_list.files();
        }
    };

    let on_dragover = move |ev: DragEvent| {
        ev.prevent_default(); // Allow drop
    };
    view! {
        <div class="upload-container">
            <div

                class="drop-zone"
                on:click=move |_| {
                    file_input_ref.get().unwrap().click();
                }
                on:dragover=on_dragover
                on:drop=on_drop
            >
                <p>ACA van archis</p>
                <input
                    type="file"
                    accept="image/*"
                    node_ref=file_input_ref
                    on:change=on_file_change
                    style="display: none;"
                />
            </div>
        </div>
    }
}
