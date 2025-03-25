pub mod queries;
pub mod mutations;

use axum::response::{self, IntoResponse};
use async_graphql::{Schema, EmptySubscription,http::GraphiQLSource};

use crate::util::constant::CFG;
use crate::dbs::mongo;
use crate::gql::queries::QueryRoot;
use crate::gql::mutations::MutationRoot;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription>
{
    // get mongodb datasource. It can be added to:
    // 1. As global data for async-graphql.
    // 2. As application scope state of Tide
    // 3. Use lazy-static.rs.
    let mongo_ds = mongo::DataSource::init().await;
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .finish()
}

pub async fn giql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build().endpoint(CFG.get("GQL_URI").unwrap()).finish(),
    )
}
