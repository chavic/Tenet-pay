use super::index;
use actix_web::web;

pub fn frontend_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/", actix_web::web::get().to(index))
    .route("/video-call", actix_web::web::get().to(index))
    .route("/vip-here", actix_web::web::get().to(index))
    .route("/terms-of-use", actix_web::web::get().to(index))
    .route(
      "/video-call-with-vip-terms-and-conditions",
      actix_web::web::get().to(index),
    )
    .route("/vip-terms-and-conditions", actix_web::web::get().to(index))
    .route("/privacy-policy", actix_web::web::get().to(index))
    .route("/contact-us", actix_web::web::get().to(index))
    .route("/report-bug", actix_web::web::get().to(index))
    .route("/payment-info", actix_web::web::get().to(index))
    .route("/vip/confirm", actix_web::web::get().to(index))
    .route("/vip/terms-confirm", actix_web::web::get().to(index))
    .route("/vip/stripe-confirm", actix_web::web::get().to(index))
    .route("/vip/data", actix_web::web::get().to(index))
    .route("/vip/unqualified", actix_web::web::get().to(index))
    .route("/vip/no-vip-access", actix_web::web::get().to(index))
    .route("/vip/unqualified_beta", actix_web::web::get().to(index))
    .route("/b823a745/e4a6c13b", actix_web::web::get().to(index))
    .route("/u/{username}", actix_web::web::get().to(index));
}
