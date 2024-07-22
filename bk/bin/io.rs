// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let mut buf = vec![0; 1024];
//             println!("{:?}", buf);

//             loop {
//                 match socket.read(&mut buf).await {
//                     // `Ok(0)` が返ってきたらリモート側が閉じられたことを意味する
//                     Ok(0) => return,
//                     Ok(n) => {
//                         // データをソケットへとコピーする
//                         if socket.write_all(&buf[..n]).await.is_err() {
//                             // 予期しないソケットエラーが発生した場合。
//                             // ここで何かできることはさほどないので、処理を停止する
//                             return;
//                         }
//                         println!("{:?}", buf);
//                     }
//                     Err(_) => {
//                         // 予期しないソケットエラーが発生した場合。
//                         // ここで何かできることはさほどないので、処理を停止する
//                         println!("error");
//                         return;
//                     }
//                 }
//             }
//         });
//     }
// }

use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
            println!("{:?}", rd);
            println!("{:?}", wr);
        });
    }
}
