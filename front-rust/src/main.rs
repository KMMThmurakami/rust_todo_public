#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

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
    tracing::info!("label_data: {:?}", type_of(&label_data));

    let mut selected_labels = use_signal(Vec::<i32>::new);

    rsx! {
        // Link {
        //     to: Route::Blog {
        //         id: count()
        //     },
        //     "Go to blog"
        // }
        h2 { "Label Set" }
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
        // h2 { "Label Delete" }
        // p { "↓input label id" }
        // form { onsubmit: move |event| {
        //         tracing::info!("Submitted! {event:?}");
        //         let delete_id = event.values().get("id").unwrap().as_value();
        //         wasm_bindgen_futures::spawn_local(async move { // Use `spawn_local` for async tasks in WASM
        //             if let Err(err) = delete_data("labels".to_string(), delete_id.parse().unwrap()).await {
        //                 tracing::error!("Failed to post data: {:?}", err);
        //             }
        //         });
        //     },
        //     input { name: "id" }
        //     input { r#type: "submit", value: "DELETE LABEL" }
        // }
        h2 { "Todo Set" }
        form { onsubmit: move |event| {
                tracing::info!("Submitted! {event:?}");
                let input_text = event.values().get("text").unwrap().as_value();

                // Labelの初期選択状態を反映する
                // 画面上のインデックスをVec型に変換
                let selected_ids: Vec<i32> = selected_labels.read().clone();
                // DBから取得したデータのインデックスと画面上のインデックスを対応させる
                let selected_labels_data: Vec<Label> = selected_ids.iter()
                    .filter_map(|&index| label_data.as_ref().ok()?.get(index as usize).cloned())
                    .collect();
                tracing::info!("selected_labels_data: {:?}", selected_labels_data);

                wasm_bindgen_futures::spawn_local(async move { // Use `spawn_local` for async tasks in WASM
                    if let Err(err) = post_todo_data(input_text.clone(), selected_labels_data).await {
                        tracing::error!("Failed to post data: {:?}", err);
                    }
                });
            },
            input { name: "text" }
            // Generate checkboxes for labels
            if let Ok(ref data) = label_data {
                for (i, label) in data.iter().enumerate() {
                    div {
                        input {
                            r#type: "checkbox",
                            value: "{label.id}",
                            onchange: move |event| {
                                let bool = bool::from_str(&event.value()).unwrap();
                                if bool {
                                    // trueの場合、selected_labelsにlabel.idを追加
                                    selected_labels.write().push(i as i32); // 本当は画面表示上のインデックスではなくてlabel.idを入れたい
                                } else {
                                    // falseの場合、selected_labelsからlabel.idを削除
                                    selected_labels.write().retain(|&id| id != i as i32); // 本当は画面表示上のインデックスではなくてlabel.idを入れたい
                                }
                                tracing::info!("checked: {:?}", event.value());
                                tracing::info!("selected_labels: {:?}", selected_labels);
                            }
                        }
                        label { "{label.name}" }  // Display label name
                    }
                }
            }
            input { r#type: "submit", value: "ADD TODO" }
        }
        h2 { "Todo Delete" }
        p { "↓input todo id" }
        form { onsubmit: move |event| {
                tracing::info!("Submitted! {event:?}");
                let delete_id = event.values().get("id").unwrap().as_value();
                wasm_bindgen_futures::spawn_local(async move { // Use `spawn_local` for async tasks in WASM
                    if let Err(err) = delete_data("todos".to_string(), delete_id.parse().unwrap()).await {
                        tracing::error!("Failed to post data: {:?}", err);
                    }
                });
            },
            input { name: "id" }
            input { r#type: "submit", value: "DELETE TODO" }
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
                if let Ok(ref data) = label_data {
                    for label in data.iter() {
                        div { "===============================" }
                        div {
                            p { "Label ID: {label.id}" }
                            p { "Label Name: {label.name}" }
                        }
                    }
                    div { "===============================" }
                } else {
                    div { "Labels Data get error" }
                }
            }
            div {
                h2 { "Todo" }
                if let Ok(ref data) = todo_data {
                    for todo in data.iter() {
                        div { "===============================" }
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
                    div { "===============================" }
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

async fn post_todo_data(text: String, labels: Vec<Label>) -> Result<(), ServerFnError> {
    tracing::info!("post: {:?}, labels: {:?}", text, labels);

    let body = json!({
        "text": text,
        "labels": labels.iter().map(|label| {
            json!(label.id)
        }).collect::<Vec<_>>(), // 各ラベルをJSON形式に変換してPOST
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

async fn delete_data(kind: String, id: i32) -> Result<(), ServerFnError> {
    tracing::info!("delete_todo_data: {:?}", id);

    let client = reqwest::Client::new();
    let res = client
        .delete(format!("データベースURL/{}/{}", kind, id))
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

// --------------
// debug function
// --------------
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
