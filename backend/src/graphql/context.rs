use crate::jwt::model::DecodedToken;
use crate::settings::Settings;
use juniper::Context as JuniperContext;
use mysql::*;
use std::sync::Arc;

pub(crate) struct Context {
    pub settings: Settings,
    pub session: Arc<Pool>,
    pub token: DecodedToken,
    pub ip_address: Option<String>,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(
        settings: Settings,
        token: DecodedToken,
        ip_address: Option<String>,
        session: Arc<Pool>,
    ) -> Self {
        Self {
            settings,
            token,
            ip_address,
            session,
        }
    }
}
