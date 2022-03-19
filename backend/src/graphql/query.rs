use crate::graphql::context::Context;
use crate::jwt::manager::create_token;

use juniper::FieldResult;
pub(crate) struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    fn api_version() -> &'static str {
        "1.0"
    }

    async fn generate_code(issuer: String, phone_number: String) -> FieldResult<String> {
        let token = create_token(issuer, phone_number, 10).unwrap();

        Ok(token)
    }
}
