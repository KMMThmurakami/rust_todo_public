#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

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
    let todo_data = use_server_future(get_todo_data)?.value(); // SSR
    let label_data = use_server_future(get_label_data)?.value(); // SSR

    rsx! {
        // Link {
        //     to: Route::Blog {
        //         id: count()
        //     },
        //     "Go to blog"
        // }
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
            p { "Todo data server: {todo_data:?}"}
            p { "Label data server: {label_data:?}"}
            // p { "Todo data client: {todo_data_resource:?}"}
        }
    }
}

// #[server(PostServerData)]
// async fn post_server_data(data: String) -> Result<(), ServerFnError> {
//     tracing::info!("Server received: {}", data);
//     Ok(())
// }

// #[server(GetServerData)]
// async fn get_server_data() -> Result<String, ServerFnError> {
//     Ok("Hello from the server!".to_string())
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoEntity {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub labels: Vec<Label>,
}

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
