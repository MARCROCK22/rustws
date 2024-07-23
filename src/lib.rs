#![deny(clippy::all)]

use std::{thread, vec};

use napi::*;
use std::net::TcpStream;
use threadsafe_function::{
  ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct CreateWebSocketConnectionResult {
  socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

#[napi(object)]
pub struct CreateConnectionCallbacks {
  pub on_open: JsFunction,
  pub on_message: JsFunction,
  pub on_error: JsFunction,
  pub on_close: JsFunction,
  pub url: String,
}

#[napi]
impl CreateWebSocketConnectionResult {
  #[napi]
  pub fn send(&mut self, payload: String) {
    self.socket.send(Message::Text(payload)).unwrap();
  }
}

#[napi]
pub fn create_connection(options: CreateConnectionCallbacks) -> CreateWebSocketConnectionResult {
  let (mut socket, _) = connect("ws://localhost:3012").expect("Can't connect");

  let on_message_tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = options
    .on_message
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
      ctx.env.create_string(ctx.value.as_str()).map(|v| vec![v])
    })
    .unwrap();

  let on_close_tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = options
    .on_close
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
      ctx.env.create_string(ctx.value.as_str()).map(|v| vec![v])
    })
    .unwrap();

  let on_error_tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = options
    .on_error
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
      ctx.env.create_string(ctx.value.as_str()).map(|v| vec![v])
    })
    .unwrap();

  let on_open_tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = options
    .on_open
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
      ctx.env.create_string(ctx.value.as_str()).map(|v| vec![v])
    })
    .unwrap();

  // Call the on_open callback to indicate that the connection has been opened.
  on_open_tsfn.call(
    Ok("Connection opened".to_string()),
    ThreadsafeFunctionCallMode::NonBlocking,
  );

  socket
    .send(Message::Text("Hello WebSocket".into()))
    .unwrap();

  on_message_tsfn.call(
    Ok("XD".to_string()),
    ThreadsafeFunctionCallMode::NonBlocking,
  );

  thread::spawn(move || loop {
    match socket.read() {
      Ok(msg) => {
        match msg {
          Message::Text(text) => {
            // Err(napi::Error::new(napi::Status::Cancelled, ":v")),
            on_message_tsfn.call(Ok(text), ThreadsafeFunctionCallMode::NonBlocking);
          }
          Message::Binary(bin) => {
            println!("Binary received: {:?}", bin);
          }
          Message::Close(_) => {
            on_close_tsfn.call(
              Ok("Connection closed".to_string()),
              ThreadsafeFunctionCallMode::NonBlocking,
            );
            return;
          }
          _ => {}
        }
      }
      Err(e) => {
        on_error_tsfn.call(Ok(e.to_string()), ThreadsafeFunctionCallMode::NonBlocking);
        println!("Error: {}", e);
        return;
      }
    }
  });

  CreateWebSocketConnectionResult { socket }
}
