use crate::graphql;
use dominator::{clone, html, routing, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use web_sys::console::log_1 as log;

#[derive(Debug, Serialize, Deserialize)]
pub struct App {}

impl App {
  pub fn new() -> Arc<Self> {
    Arc::new(Self {})
  }

  pub fn start(app: Arc<Self>) {
    dominator::append_dom(&dominator::body(), Self::render(app));
  }

  pub fn render(state: Arc<Self>) -> Dom {
    graphql::api_version();

    html!("div", {
      .attr("id","root")
      .text("some text")
    })
  }
}
