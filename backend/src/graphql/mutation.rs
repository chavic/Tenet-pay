use crate::graphql::context::Context;
use crate::jwt::manager::decode_token;
use juniper::FieldResult;
pub(crate) struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn verify(code: String) -> FieldResult<String> {
        let decoded = decode_token(&code).expect("code id Invalid");
        Ok(format!(
            "Issuer: {}, Phone Numer: {}, issued at: {}, expires at: {},",
            decoded.iss, decoded.sub, decoded.iat, decoded.exp
        ))
    }
}
