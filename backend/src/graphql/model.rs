use crate::graphql::{context::Context, mutation::Mutation, query::QueryRoot};
use juniper::{EmptySubscription, RootNode};

pub(crate) type Schema = RootNode<'static, QueryRoot, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {}, EmptySubscription::new())
}
