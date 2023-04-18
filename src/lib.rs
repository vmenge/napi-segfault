use chrono::{DateTime, Utc};
use js_sender::JsSender;
use napi::JsFunction;
use serde::Serialize;

#[macro_use]
extern crate napi_derive;

mod js_sender;

#[napi(object)]
pub struct AppArgs {
  pub payload_handler: JsFunction,
}

#[derive(Debug)]
enum Case {
  One,
  Two,
  Three,
}

impl Case {
  pub fn as_str(&self) -> &'static str {
    match self {
      Case::One => "one",
      Case::Two => "two",
      Case::Three => "three",
    }
  }
}

#[derive(Debug)]
struct Payload {
  name: String,
  age: Option<u32>,
  datetime: Option<DateTime<Utc>>,
  case: Case,
}

#[napi]
pub struct App {
  sender: JsSender<Payload>,
}

#[napi]
impl App {
  #[napi(constructor)]
  pub fn new(args: AppArgs) -> napi::Result<Self> {
    let sender = JsSender::new(args.payload_handler, |ctx| {
      let payload: Payload = ctx.value;
      let mut obj = ctx.env.create_object().unwrap();
      obj.set_named_property("name", payload.name).unwrap();
      obj.set_named_property("age", payload.age).unwrap();
      obj
        .set_named_property("case", payload.case.as_str())
        .unwrap();
      obj
        .set_named_property(
          "datetime",
          payload
            .datetime
            .map(|x| x.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
        )
        .unwrap();

      Ok(obj)
    })?;

    Ok(App { sender })
  }

  #[napi]
  pub async fn start(&self, wait_ms: u32) {
    self
      .sender
      .stream_to_js(24, |tx| {
        tokio::spawn(async move {
          println!("Started");

          loop {
            let payload = Payload {
              name: "foo".to_owned(),
              age: Some(999),
              case: Case::One,
              datetime: Some(Utc::now()),
            };

            tx.send(payload).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(wait_ms as u64)).await;
          }
        });
      })
      .await;
  }
}
