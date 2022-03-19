use crate::utils;
use serde_json::value::Value;
use std::collections::HashMap;
use urlencoding::encode;
use wasm_bindgen_futures::JsFuture;
use web_sys::console::log_1 as log;

pub type Payment = (bool, String, String, String);

pub async fn api_version() {
    let query = r#"{ "query": "{ apiVersion{} }" }"#;
    let response = utils::fetch_graphql(query).await;

    let json_object = JsFuture::from(response.json().expect("Failed to get Json"))
        .await
        .expect("Failed to convert to future");

    let values: HashMap<String, HashMap<String, HashMap<String, String>>> =
        json_object.into_serde().expect("Failed to Create hashMap");

    log(&json_object)
}

pub async fn authorize_instagram(code: Option<String>) {
    if let Some(inst_code) = code {
        let query = format!(
            r#"{{ "query": "mutation($code: String!){{ authorizeInstagram(code: $code) {{ bearer }}  }}", "variables": {{ "code": "{}"  }} }}"#,
            inst_code
        );

        let response = utils::fetch_graphql(&query).await;

        let json_object = JsFuture::from(response.json().expect("Failed to get Json"))
            .await
            .expect("Failed to convert to future");

        let data: HashMap<String, Value> =
            json_object.into_serde().expect("Failed to Create hashMap");

        let value = data.get("data").unwrap();
        let token = match value {
            Value::Object(_) => match &value["authorizeInstagram"]["bearer"] {
                Value::String(token) => Some(token),
                _ => None,
            },
            _ => None,
        };
    }
}

pub async fn stripe_authorization(user_url: &str) {
    let query = format!(
        r#"{{  "query": "{{ stripeAuthorization(userUrl: \"{}\") {{authorizeUrl}} }}" }}"#,
        user_url
    );

    let response = utils::fetch_graphql(&query).await;

    let json_object = JsFuture::from(response.json().expect("Failed to get Json"))
        .await
        .expect("Failed to convert to future");

    let values: HashMap<String, HashMap<String, HashMap<String, String>>> =
        json_object.into_serde().expect("Failed to Create hashMap");

    log(&json_object)
}
