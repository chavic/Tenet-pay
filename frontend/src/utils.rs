use dominator::routing;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Url;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub fn server_url() -> String {
  let url_str = routing::url().get_cloned();
  let url = Url::new(&url_str).unwrap_throw();
  //format!("http://{}", url.host())
  format!("http://localhost:8080")
}

pub async fn fetch_graphql(query_json: &str) -> Response {
  let url_endpoint = format!("{}/graphql", server_url());
  let mut options = RequestInit::new();
  options.method("POST");
  options.mode(RequestMode::Cors);
  options.body(Some(&JsValue::from_str(query_json)));
  let request = Request::new_with_str_and_init(&url_endpoint, &options).unwrap();

  request
    .headers()
    .set("Content-Type", "application/json")
    .unwrap();

  let window = web_sys::window().unwrap();
  let resp_value = JsFuture::from(window.fetch_with_request(&request))
    .await
    .unwrap();

  assert!(resp_value.is_instance_of::<Response>());
  let resp: Response = resp_value.dyn_into().unwrap();
  resp
}
