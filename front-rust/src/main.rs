#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

#[component]
fn Home() -> Element {
    // let mut count = use_signal(|| 0);
    // let mut text = use_signal(|| String::from("..."));
    // let todo_data_resource = use_resource(get_todo_data).value();  // CSR
    let todo_data = use_server_future(get_todo_data)?.value().unwrap(); // SSR
    let label_data = use_server_future(get_label_data)?.value().unwrap(); // SSR

    rsx! {
        // Link {
        //     to: Route::Blog {
        //         id: count()
        //     },
        //     "Go to blog"
        // }
        h2 { "Label Post" }
        form { onsubmit: move |event| {
                tracing::info!("Submitted! {event:?}");
                let input_name = event.values().get("name").unwrap().as_value();
                wasm_bindgen_futures::spawn_local(async move { // Use `spawn_local` for async tasks in WASM
                    if let Err(err) = post_label_data(input_name.clone()).await {
                        tracing::error!("Failed to post data: {:?}", err);
                    }
                });
            },
            input { name: "name" }
            input { r#type: "submit", value: "ADD LABEL" }
        }
        h2 { "Todo Post" }
        form { onsubmit: move |event| {
                tracing::info!("Submitted! {event:?}");
                let input_text = event.values().get("text").unwrap().as_value();
                wasm_bindgen_futures::spawn_local(async move { // Use `spawn_local` for async tasks in WASM
                    if let Err(err) = post_todo_data(input_text.clone()).await {
                        tracing::error!("Failed to post data: {:?}", err);
                    }
                });
            },
            input { name: "text" }
            input { r#type: "submit", value: "ADD TODO" }
        }
        div {
            // h1 { "High-Five counter: {count}" }
            // button { onclick: move |_| count += 1, "Up high!" }
            // button { onclick: move |_| count -= 1, "Down low!" }
            // button {
            //     onclick: move |_| async move {
            //         if let Ok(data) = get_server_data().await {
            //             tracing::info!("Client received: {}", data);
            //             text.set(data.clone());
            //             post_server_data(data).await.unwrap();
            //         }
            //     },
            //     "Get Server Data"
            // }
            // p { "Server data : {text}"}
            div {
                h2 { "Label" }
                if let Ok(data) = label_data {
                    for (i, label) in data.iter().enumerate() {
                        div { "{i+1}" }
                        div {
                            p { "Label ID: {label.id}" }
                            p { "Label Name: {label.name}" }
                        }
                    }
                } else {
                    div { "Labels Data get error" }
                }
            }
            div {
                h2 { "Todo" }
                if let Ok(data) = todo_data {
                    for (i, todo) in data.iter().enumerate() {
                        div { "{i+1}" }
                        div {
                            p { "Todo ID: {todo.id}" }
                            p { "Todo Text: {todo.text}" }
                            p { "Todo Completed: {todo.completed}" }
                            for todo_label in todo.labels.iter() {
                                p { "Todo Labels ID: {todo_label.id}" }
                                p { "Todo Labels Name: {todo_label.name}" }
                            }
                        }
                    }
                } else {
                    div { "Todo Data get error" }
                }
            }
            // p { "Label data server: {label_data:?}"}
            // p { "Todo data client: {todo_data_resource:?}"}
        }
    }
}

// --------------
// struct
// --------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
}

// --------------
// todo function
// --------------
#[server(GetTodoData)]
async fn get_todo_data() -> Result<Vec<TodoEntity>, ServerFnError> {
    let todo = reqwest::get("データベースURL/todos")
        .await
        .unwrap()
        .json::<Vec<TodoEntity>>()
        .await?;
    tracing::info!("todo: {:?}", todo);

    Ok(todo)
}

async fn post_todo_data(text: String) -> Result<(), ServerFnError> {
    tracing::info!("post: {:?}", text);

    let body = json!({
        "text": text,
        "labels": [] // Use an empty array for labels
    });

    let client = reqwest::Client::new();
    let res = client
        .post("データベースURL/todos")
        .json(&body)
        .send()
        .await;

    match res {
        Ok(response) => tracing::info!("POST successful: {:?}", response),
        Err(err) => tracing::error!("POST failed: {:?}", err),
    }

    Ok(())
}

// --------------
// label function
// --------------
#[server(GetLabelData)]
async fn get_label_data() -> Result<Vec<Label>, ServerFnError> {
    let label = reqwest::get("データベースURL/labels")
        .await
        .unwrap()
        .json::<Vec<Label>>()
        .await?;
    tracing::info!("label: {:?}", label);

    Ok(label)
}

async fn post_label_data(name: String) -> Result<(), ServerFnError> {
    tracing::info!("post: {:?}", name);

    let body = json!({
        "name": name,
    });

    let client = reqwest::Client::new();
    let res = client
        .post("データベースURL/labels")
        .json(&body)
        .send()
        .await;

    match res {
        Ok(response) => tracing::info!("POST successful: {:?}", response),
        Err(err) => tracing::error!("POST failed: {:?}", err),
    }

    Ok(())
}