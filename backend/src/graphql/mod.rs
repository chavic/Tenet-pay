use actix_web::web;

mod context;
mod handler;
mod model;
mod mutation;
mod query;

pub(crate) use model::*;

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.route("/graphql", web::post().to(handler::graphql))
        .route("/graphql/schema", web::get().to(handler::schema))
        .route("/playground", web::get().to(handler::playground));
}
