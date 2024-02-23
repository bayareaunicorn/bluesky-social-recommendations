use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::{vectors_config::Config, VectorParams, VectorsConfig};



async fn make_client() -> Result<QdrantClient> {
    let client = QdrantClient::from_url("https://fc746fb1-5e07-41a1-ae31-5cb929d22810.us-east4-0.gcp.cloud.qdrant.io:6333")
        // using an env variable for the API KEY for example
        .with_api_key(std::env::var("QDRANT_CLIENT"))
        .build()?;
    Ok(client)
}


async fn spam_c() -> Result<QdrantClient> {
client
    .create_collection(&CreateCollection {
        collection_name: "spam_collection".to_string(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 4,
                distance: Distance::Dot.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    })
    .await?;
}