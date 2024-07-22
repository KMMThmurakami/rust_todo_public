use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;

use mini_redis::{Connection, Frame};
use tokio::net::TcpStream;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // ハッシュマップへのハンドルを複製する
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // `mini-redis` が提供するコネクションによって、ソケットから来るフレームをパースする
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("set");
                let mut db = db.lock().unwrap();
                println!("{:?}", db);
                db.insert(cmd.key().to_string(), cmd.value().clone());
                println!("{:?}", db);
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("get");
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // クライアントへのレスポンスを書き込む
        println!("write");
        connection.write_frame(&response).await.unwrap();
    }
}
