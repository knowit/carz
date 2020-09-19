extern crate logic;
extern crate futures;
extern crate tokio;
extern crate tokio_tungstenite;

use futures::{future, pin_mut,
    stream::TryStreamExt,
    StreamExt, channel::mpsc::{unbounded, UnboundedSender}
};

use std::{
    env,
    io::Error as IoError,
    net::SocketAddr
};

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), IoError> {

    // let game = logic::init_game();

    // println!("{}", game.get_all_actions()[0]);

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6565".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}


async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (outgoing, incoming) = ws_stream.split();

    let get_incoming = incoming.try_for_each(|msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            msg.to_text().unwrap()
        );
        future::ok(())
    });

    get_incoming.await;

    /*
    for episode in 0..20 {
        observation = game.reset();
        for t in 0..100 {
            // render game
            action = game.get_random_action();
            game.step();
            observation, status = game.step();
            if status = "done" {
                break;
            }
        }
    }
    */

}

fn get_user_action() {

}

