use chrono::{Duration, Local};

#[derive(Clone, Debug)]
pub struct DecodedToken {
    pub jwt: Option<(String, Claims)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
}

// struct to get converted to token and back
impl Claims {
    pub(crate) fn new(issuer: String, phone_number: String, duration_in_minutes: u16) -> Self {
        let iat = Local::now();
        let exp = iat + Duration::minutes(i64::from(duration_in_minutes));

        Claims {
            iss: issuer,
            sub: phone_number.to_string(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }

    pub fn user_id(&self) -> String {
        self.sub.clone()
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Token {
    pub bearer: String,
}
