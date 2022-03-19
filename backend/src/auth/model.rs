#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct MobilePayment {
    phone_number: Option<String>
}
