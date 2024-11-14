use dotenv::dotenv;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::r#match::MatchValue;
use qdrant_client::qdrant::{
    FieldCondition, Filter, Match, SearchParams, SearchPoints
};
use reqwest;
use serde_json::Value as JsonValue;
use std::sync::Arc;

const VECTOR_SIZE: u64 = 1536;
const EMBEDDING_MODEL: &str = "text-embedding-ada-002";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings";

async fn init_qdrant_client() -> Result<Arc<QdrantClient>, Box<dyn std::error::Error>> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;
    Ok(Arc::new(client))
}

fn sanitize_collection_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect()
}

async fn generate_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

    let client = reqwest::Client::new();
    let response = client
        .post(OPENAI_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "input": text,
            "model": EMBEDDING_MODEL
        }))
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    let embedding = response_json["data"][0]["embedding"]
        .as_array()
        .ok_or("Invalid embedding response")?
        .iter()
        .map(|v| v.as_f64().unwrap_or_default() as f32)
        .collect();

    Ok(embedding)
}

async fn search_collection(
    client: &QdrantClient,
    collection_name: &str,
    query: &str,
    group_id: Option<&str>,
    agent_id: Option<&str>,
    limit: u64,
) -> Result<Vec<(f32, JsonValue)>, Box<dyn std::error::Error>> {
    let sanitized_collection = sanitize_collection_name(collection_name);
    let query_vector = generate_embedding(query).await?;

    // Build filter based on provided group_id and agent_id
    let mut must_conditions = Vec::new();
    
    if let Some(group_id) = group_id {
        must_conditions.push(FieldCondition {
            key: "group_id".to_string(),
            r#match: Some(Match {
                match_value: Some(MatchValue::Keyword(group_id.to_string())),
            }),
            range: None,
            geo_bounding_box: None,
            geo_radius: None,
            values_count: None,
            geo_polygon: None,
            datetime_range: None,
        }.into());
    }

    if let Some(agent_id) = agent_id {
        must_conditions.push(FieldCondition {
            key: "agent_id".to_string(),
            r#match: Some(Match {
                match_value: Some(MatchValue::Keyword(agent_id.to_string())),
            }),
            range: None,
            geo_bounding_box: None,
            geo_radius: None,
            values_count: None,
            geo_polygon: None,
            datetime_range: None,
        }.into());
    }

    let filter = if !must_conditions.is_empty() {
        Some(Filter {
            should: vec![],
            must: must_conditions,
            must_not: vec![],
            min_should: None,
        })
    } else {
        None
    };

    let search_response = client.search_points(&SearchPoints {
        collection_name: sanitized_collection,
        vector: query_vector,
        filter,
        limit,
        with_payload: Some(true.into()),
        params: Some(SearchParams {
            hnsw_ef: Some(128),
            exact: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    }).await?;

    let results = search_response.result
        .into_iter()
        .map(|point| {
            let score = point.score;
            let data = point.payload.get("data")
                .and_then(|v| v.as_str())
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(JsonValue::Null);
            (score, data)
        })
        .collect();

    Ok(results)
}

#[tokio::test]
#[ignore]
async fn test_rag_queries() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    let client = init_qdrant_client().await?;
    println!("Successfully connected to Qdrant");

    // Example queries
    println!("\n=== Querying Hardware Information ===");
    let hardware_queries = vec![
        "Find systems with high CPU usage",
        "Show me machines with multiple CPU cores",
        "List computers with more than 8GB RAM",
    ];

    for query in hardware_queries {
        println!("\nQuery: {}", query);
        let results = search_collection(
            &client,
            "syscollector_hardware",
            query,
            None,
            None,
            3
        ).await?;

        for (score, data) in results {
            println!("Score: {:.4}", score);
            println!("Data: {}", serde_json::to_string_pretty(&data)?);
            println!("---");
        }
    }

    println!("\n=== Querying OS Information ===");
    let os_queries = vec![
        "Find Windows systems",
        "Show me Linux servers",
        "List systems with recent OS updates",
    ];

    for query in os_queries {
        println!("\nQuery: {}", query);
        let results = search_collection(
            &client,
            "syscollector_os",
            query,
            None,
            None,
            3
        ).await?;

        for (score, data) in results {
            println!("Score: {:.4}", score);
            println!("Data: {}", serde_json::to_string_pretty(&data)?);
            println!("---");
        }
    }

    println!("\n=== Querying Network Information ===");
    let network_queries = vec![
        "Find systems with open ports",
        "Show me network interfaces in promiscuous mode",
        "List active network connections",
    ];

    for query in network_queries {
        println!("\nQuery: {}", query);
        let results = search_collection(
            &client,
            "syscollector_netproto",
            query,
            None,
            None,
            3
        ).await?;

        for (score, data) in results {
            println!("Score: {:.4}", score);
            println!("Data: {}", serde_json::to_string_pretty(&data)?);
            println!("---");
        }
    }

    // Example of filtering by group and agent
    println!("\n=== Querying with Filters ===");
    let group_id = "default";  // Replace with actual group ID
    let agent_id = "005";      // Replace with actual agent ID

    println!("\nQuerying hardware info for specific agent in group");
    let results = search_collection(
        &client,
        "syscollector_hardware",
        "Show system specifications",
        Some(group_id),
        Some(agent_id),
        3
    ).await?;

    for (score, data) in results {
        println!("Score: {:.4}", score);
        println!("Data: {}", serde_json::to_string_pretty(&data)?);
        println!("---");
    }

    Ok(())
}
