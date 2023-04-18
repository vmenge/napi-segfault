use napi::{
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  JsFunction, JsObject,
};
use tokio::sync::mpsc::{self, Sender};

pub struct JsSender<T: 'static + Send + Sync> {
  receiver: ThreadsafeFunction<T, ErrorStrategy::CalleeHandled>,
}

impl<T: 'static + Send + Sync> JsSender<T> {
  pub fn new<F>(js_fun: JsFunction, into_js_obj: F) -> napi::Result<JsSender<T>>
  where
    F: 'static + Send + Sync + Fn(ThreadSafeCallContext<T>) -> napi::Result<JsObject>,
  {
    let receiver = js_fun.create_threadsafe_function(0, move |ctx| {
      let obj = into_js_obj(ctx)?;
      Ok(vec![obj])
    })?;

    Ok(JsSender { receiver })
  }

  pub async fn stream_to_js(&self, buffer: usize, sender_fn: impl Fn(Sender<T>))
  where
    T: 'static,
  {
    let (tx, mut rx) = mpsc::channel(buffer);
    sender_fn(tx);

    while let Some(payload) = rx.recv().await {
      self
        .receiver
        .call(Ok(payload), ThreadsafeFunctionCallMode::NonBlocking);
    }
  }
}
