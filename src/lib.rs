use napi_derive::napi;
use tokio::time;

const INITIAL_VALUE: u32 = 0;

#[napi]
pub struct App {
  value: u32,
}

#[napi]
impl App {
  #[napi(constructor)]
  pub fn new() -> napi::Result<Self> {
    Ok(App { value: INITIAL_VALUE })
  }

  #[napi]
  pub async fn start(&self) {
    loop {
      println!("App value: {}", self.value);

      // We never change the value so this should never fail
      assert_eq!(self.value, INITIAL_VALUE, "value({}) is never being set in ths example so it should stay at INITIAL_VALUE({})", self.value, INITIAL_VALUE);


      time::sleep(time::Duration::from_secs(1)).await;
    }
  }
}

impl Drop for App {
  fn drop(&mut self) {
    println!("App dropped");
  }
}
