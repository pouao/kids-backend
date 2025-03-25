#[tokio::test]
async fn sdl() {
    use kids_backend::gql::build_schema;
    let schema = build_schema().await;
    std::fs::write("./schema.graphql", schema.sdl()).unwrap();
}
