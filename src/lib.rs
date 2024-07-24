#![deny(clippy::all)]

use napi::*;
use std::net::TcpStream;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct CreateWebSocketConnectionResult {
  socket: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
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
    self.socket.lock().unwrap().send(Message::Text(payload)).unwrap();
  }
}

#[napi]
pub fn create_connection(options: CreateConnectionCallbacks) -> CreateWebSocketConnectionResult {
  let socket = Arc::new(Mutex::new(connect(options.url).unwrap().0));
  let socket_thread = Arc::clone(&socket);

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

  std::thread::spawn(move || loop {
    let mut socket_thread = socket_thread.lock().unwrap();
    
    match socket_thread.read() {
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
          }
          _ => {}
        }
      }
      Err(e) => {
        on_error_tsfn.call(Ok(e.to_string()), ThreadsafeFunctionCallMode::NonBlocking);
        println!("Error: {}", e);
      }
    }
  });
  
  CreateWebSocketConnectionResult {
    socket,
  }
}
