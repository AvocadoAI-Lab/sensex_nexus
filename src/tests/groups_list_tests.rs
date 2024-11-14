use crate::tests::core::TestFramework;
use dotenv::dotenv;
use futures::stream::{self, StreamExt};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    r#match::MatchValue, CreateCollection, Distance, FieldCondition, Filter
    , Match, PointStruct, ScrollPoints,
    SearchParams, SearchPoints, Value, VectorParams, VectorsConfig
};
use reqwest;
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::Duration;
use uuid::Uuid;

const MODULE_NAME: &str = "groups_list";
const MAX_CONCURRENT_REQUESTS: usize = 5;
const VECTOR_SIZE: u64 = 1536;  // Updated to match text-embedding-ada-002 dimension
const EMBEDDING_MODEL: &str = "text-embedding-ada-002";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings";

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

async fn init_qdrant_client() -> Result<Arc<QdrantClient>, Box<dyn std::error::Error>> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;
    
    match client.list_collections().await {
        Ok(collections) => {
            println!("Successfully connected to Qdrant. Found {} existing collections", collections.collections.len());
            for collection in collections.collections {
                println!("Found collection: {}", collection.name);
            }
        }
        Err(e) => {
            return Err(format!("Failed to connect to Qdrant: {}", e).into());
        }
    }

    Ok(Arc::new(client))
}

fn sanitize_collection_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect()
}

async fn verify_collection_exists(client: &QdrantClient, collection_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let collections = client.list_collections().await?;
    Ok(collections.collections.iter().any(|c| c.name == collection_name))
}

async fn create_collection_if_not_exists(client: &QdrantClient, collection_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized_name = sanitize_collection_name(collection_name);
    
    if !verify_collection_exists(client, &sanitized_name).await? {
        println!("Creating new collection: {}", sanitized_name);
        client.create_collection(&CreateCollection {
            collection_name: sanitized_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                    VectorParams {
                        size: VECTOR_SIZE,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    }
                ))
            }),
            ..Default::default()
        }).await?;
        println!("Successfully created collection: {}", sanitized_name);
    } else {
        println!("Collection {} already exists", sanitized_name);
    }
    Ok(())
}

async fn search_similar_data(
    client: &QdrantClient,
    collection_name: &str,
    query_text: &str,
    limit: u64
) -> Result<Vec<(f32, serde_json::Value)>, Box<dyn std::error::Error>> {
    let sanitized_collection = sanitize_collection_name(collection_name);
    let query_vector = generate_embedding(query_text).await?;

    let search_response = client.search_points(&SearchPoints {
        collection_name: sanitized_collection,
        vector: query_vector,
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
                .unwrap_or(serde_json::Value::Null);
            (score, data)
        })
        .collect();

    Ok(results)
}

async fn verify_data_storage(
    client: &QdrantClient,
    collection_name: &str,
    group_id: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized_collection = sanitize_collection_name(collection_name);
    
    let field_condition = FieldCondition {
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
    };

    let scroll_response = client.scroll(&ScrollPoints {
        collection_name: sanitized_collection.clone(),
        filter: Some(Filter {
            should: vec![],
            must: vec![field_condition.into()],
            must_not: vec![],
            min_should: None,
        }),
        limit: Some(10),
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;

    println!("\nVerifying data in collection {}:", sanitized_collection);
    println!("Found {} points for group {}", scroll_response.result.len(), group_id);
    
    if let Some(first_point) = scroll_response.result.first() {
        println!("Example point payload:");
        if let Some(data) = first_point.payload.get("data") {
            println!("Data: {}", data);
        }
    }

    Ok(())
}

async fn store_agent_data(
    client: &QdrantClient, 
    collection_name: &str,
    group_id: &str,
    agent_id: &str,
    data: JsonValue
) -> Result<(), Box<dyn std::error::Error>> {
    let sanitized_collection = sanitize_collection_name(collection_name);
    
    println!("\nStoring data in collection {} for group {} agent {}", 
        sanitized_collection, group_id, agent_id);

    let mut payload = std::collections::HashMap::new();
    payload.insert("group_id".to_string(), Value::from(group_id.to_string()));
    payload.insert("agent_id".to_string(), Value::from(agent_id.to_string()));
    
    // Convert data to string for embedding generation
    let data_str = serde_json::to_string(&data)?;
    payload.insert("data".to_string(), Value::from(data_str.clone()));

    // Generate embedding for the data
    let vector = generate_embedding(&data_str).await?;
    let point_id = Uuid::new_v4().to_string();

    let point = PointStruct {
        id: Some(point_id.clone().into()),
        payload,
        vectors: Some(vector.into()),
    };

    client.upsert_points(
        &sanitized_collection,
        None,
        vec![point],
        None
    ).await?;

    println!("Successfully stored point {} in collection {}", point_id, sanitized_collection);
    
    // Verify the data was stored and show its contents
    verify_data_storage(client, collection_name, group_id).await?;
    
    Ok(())
}

async fn test_agent_endpoints(
    framework: &TestFramework, 
    agent_id: &str, 
    group_id: &str,
    client: &Arc<QdrantClient>,
    semaphore: Arc<Semaphore>
) -> Result<(), Box<dyn std::error::Error>> {
    let endpoints = vec![
        ("syscollector_hardware", "/syscollector/{agent_id}/hardware"),
        ("syscollector_hotfixes", "/syscollector/{agent_id}/hotfixes"),
        ("syscollector_netaddr", "/syscollector/{agent_id}/netaddr"),
        ("syscollector_netiface", "/syscollector/{agent_id}/netiface"),
        ("syscollector_netproto", "/syscollector/{agent_id}/netproto"),
        ("syscollector_os", "/syscollector/{agent_id}/os"),
        ("syscollector_packages", "/syscollector/{agent_id}/packages"),
        ("syscollector_ports", "/syscollector/{agent_id}/ports"),
        ("syscollector_processes", "/syscollector/{agent_id}/processes"),
        ("syscheck", "/syscheck/{agent_id}"),
        ("syscheck_last_scan", "/syscheck/{agent_id}/last_scan"),
        ("sca", "/sca/{agent_id}"),
        ("rootcheck", "/rootcheck/{agent_id}"),
        ("rootcheck_last_scan", "/rootcheck/{agent_id}/last_scan"),
        ("ciscat_results", "/ciscat/{agent_id}/results")
    ];

    for (collection_name, _) in &endpoints {
        create_collection_if_not_exists(client, collection_name).await?;
    }

    let futures = stream::iter(endpoints)
        .map(|(collection_name, endpoint_template)| {
            let framework = framework.clone();
            let agent_id = agent_id.to_string();
            let group_id = group_id.to_string();
            let client = Arc::clone(client);
            let semaphore = Arc::clone(&semaphore);
            
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                println!("\nTesting GET {} for agent {}", endpoint_template, agent_id);
                let endpoint = framework.create_agent_endpoint(endpoint_template, &agent_id);
                
                match framework.test_endpoint(endpoint).await {
                    Ok(response) => {
                        println!("Successfully tested endpoint");
                        match store_agent_data(
                            &client,
                            collection_name,
                            &group_id,
                            &agent_id,
                            response
                        ).await {
                            Ok(_) => println!("Successfully stored data in Qdrant"),
                            Err(e) => println!("Failed to store data in Qdrant: {}", e)
                        }
                    },
                    Err(e) => println!("Endpoint returned error (this may be normal if agent has no data): {}", e)
                }
                
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    futures.collect::<Vec<_>>().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_groups_list() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    let framework = TestFramework::new(MODULE_NAME).await?;
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));
    
    let client = init_qdrant_client().await?;
    println!("Successfully initialized Qdrant client");

    let groups_endpoint = framework.create_endpoint("/groups");
    let groups_response = framework.test_endpoint(groups_endpoint).await?;

    let mut agent_ids = HashSet::new();
    let empty_vec = Vec::new();
    let affected_items = groups_response["data"]["affected_items"]
        .as_array()
        .unwrap_or(&empty_vec);

    println!("Found {} groups to process", affected_items.len());

    let group_futures = stream::iter(
        affected_items
            .iter()
            .filter_map(|group| {
                group["name"].as_str().map(|name| (name.to_string(), group.clone()))
            })
    )
    .map(|(group_name, group_data)| {
        let framework = framework.clone();
        let client = Arc::clone(&client);
        let semaphore = Arc::clone(&semaphore);
        
        async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("\nProcessing group: {}", group_name);
            
            create_collection_if_not_exists(&client, "groups").await.unwrap_or_else(|e| {
                println!("Failed to create groups collection: {}", e);
            });
            
            if let Err(e) = store_agent_data(&client, "groups", &group_name, "group_info", group_data).await {
                println!("Failed to store group data: {}", e);
            }

            let agents_endpoint = framework.create_param_endpoint(
                "/groups/{group_id}/agents",
                "group_id",
                &group_name
            );
            
            match framework.test_endpoint(agents_endpoint).await {
                Ok(agents_response) => {
                    if let Some(affected_items) = agents_response["data"]["affected_items"].as_array() {
                        println!("Found {} agents in group {}", affected_items.len(), group_name);
                        affected_items
                            .iter()
                            .filter_map(|agent| {
                                agent["id"].as_str().map(|id| (id.to_string(), group_name.clone()))
                            })
                            .collect::<Vec<_>>()
                    } else {
                        println!("No agents found in group {}", group_name);
                        Vec::new()
                    }
                }
                Err(e) => {
                    println!("Error getting agents for group {}: {}", group_name, e);
                    Vec::new()
                }
            }
        }
    })
    .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    let agent_group_pairs: Vec<Vec<(String, String)>> = group_futures.collect().await;
    
    for pairs in &agent_group_pairs {
        for (agent_id, _) in pairs {
            agent_ids.insert(agent_id.clone());
        }
    }

    let agent_group_map: Vec<(String, String)> = agent_group_pairs
        .into_iter()
        .flatten()
        .collect();

    println!("\nProcessing {} unique agents", agent_ids.len());
    
    let agent_futures = stream::iter(agent_group_map)
        .map(|(agent_id, group_id)| {
            let framework = framework.clone();
            let client = Arc::clone(&client);
            let semaphore = Arc::clone(&semaphore);
            
            async move {
                println!("\n=== Processing agent {} in group {} ===", agent_id, group_id);
                if let Err(e) = test_agent_endpoints(&framework, &agent_id, &group_id, &client, semaphore).await {
                    println!("Error processing agent {}: {}", agent_id, e);
                }
            }
        })
        .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    agent_futures.collect::<Vec<_>>().await;

    // Example of searching similar data
    println!("\nSearching for similar data examples:");
    
    // Search for hardware info
    let hardware_results = search_similar_data(&client, "syscollector_hardware", "CPU information", 3).await?;
    println!("\nSimilar hardware data:");
    for (score, data) in hardware_results {
        println!("Score: {}, Data: {}", score, data);
    }

    // Search for OS info
    let os_results = search_similar_data(&client, "syscollector_os", "Windows operating system", 3).await?;
    println!("\nSimilar OS data:");
    for (score, data) in os_results {
        println!("Score: {}, Data: {}", score, data);
    }

    Ok(())
}
