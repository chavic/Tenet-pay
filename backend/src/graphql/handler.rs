use crate::graphql::{context::Context, model::Schema};
use crate::jwt::model::DecodedToken;
use crate::settings::Settings;
use actix_web::FromRequest;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use juniper::http::GraphQLRequest;
use juniper_actix::playground_handler;
use mysql::Pool;

pub(super) async fn graphql(
    req: HttpRequest,
    st: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    settings: web::Data<Settings>,
    session: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    // handle jwt auth
    let token: DecodedToken = DecodedToken::extract(&req)
        .into_inner()
        .map_err(error::ErrorInternalServerError)?;

    let settings = settings.into_inner().as_ref().clone();
    let session = session.into_inner().to_owned();

    // client ip address
    let ip_addr = match req.connection_info().peer_addr() {
        Some(ip) => Some(ip.to_string()),
        None => None,
    };
    let ctx = Context::new(settings, token, ip_addr, session);

    let json = web::block(move || async move {
        let res = data.execute(&st, &ctx).await;
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json.await.unwrap()))
}

pub(super) async fn schema(st: web::Data<Schema>) -> Result<HttpResponse, Error> {
    let result = st.as_schema_language();

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}

pub(super) async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql/play", None).await
}
