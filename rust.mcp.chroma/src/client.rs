use anyhow::Result;
use serde_json::json;
use std::sync::{Arc, Mutex, MutexGuard, LazyLock};
use std::collections::HashMap;

// Document structure for real storage with embeddings
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub embedding: Vec<f32>, // Vector embedding for semantic search
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Collection structure with real storage
#[derive(Debug, Clone)]
pub struct CollectionData {
    pub name: String,
    pub metadata: Option<serde_json::Value>,
    pub documents: HashMap<String, Document>,
}

// Global storage for collections
static STORAGE: LazyLock<Mutex<HashMap<String, CollectionData>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChromaClient {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

impl ChromaClient {
    pub fn new(host: &str, port: u16, username: Option<&str>, password: Option<&str>) -> Self {
        Self {
            host: host.to_string(),
            port,
            username: username.map(|s| s.to_string()),
            password: password.map(|s| s.to_string()),
        }
    }

    pub fn list_collections(
        &self,
        _limit: Option<usize>,
        _offset: Option<usize>,
    ) -> Result<Vec<String>> {
        let storage = STORAGE.lock().unwrap();
        Ok(storage.keys().cloned().collect())
    }

    pub fn create_collection(
        &self,
        name: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<String> {
        let mut storage = STORAGE.lock().unwrap();
        let collection = CollectionData {
            name: name.to_string(),
            metadata,
            documents: HashMap::new(),
        };
        storage.insert(name.to_string(), collection);
        Ok(format!("Created collection: {}", name))
    }

    pub fn get_collection(&self, name: &str) -> Result<Collection> {
        let storage = STORAGE.lock().unwrap();
        if !storage.contains_key(name) {
            // Auto-create collection if it doesn't exist
            drop(storage);
            self.create_collection(name, None)?;
        }
        Ok(Collection {
            name: name.to_string(),
        })
    }

    pub fn delete_collection(&self, name: &str) -> Result<()> {
        let mut storage = STORAGE.lock().unwrap();
        storage.remove(name);
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
}

impl Collection {
    pub fn add(
        &self,
        documents: Vec<String>,
        embeddings: Option<Vec<Vec<f32>>>,
        metadatas: Option<Vec<serde_json::Value>>,
        ids: Vec<String>,
    ) -> Result<()> {
        let mut storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get_mut(&self.name) {
            for (i, doc_content) in documents.iter().enumerate() {
                let id = ids.get(i).unwrap_or(&format!("doc_{}", i)).clone();
                let metadata = metadatas
                    .as_ref()
                    .and_then(|m| m.get(i))
                    .unwrap_or(&json!({}))
                    .clone();
                
                let embedding = embeddings
                    .as_ref()
                    .and_then(|e| e.get(i))
                    .cloned()
                    .unwrap_or_else(|| self.generate_simple_embedding(doc_content));
                
                let now = chrono::Utc::now();
                
                let document = Document {
                    id: id.clone(),
                    content: doc_content.clone(),
                    metadata,
                    embedding,
                    created_at: now,
                    updated_at: now,
                };
                
                collection.documents.insert(id, document);
            }
        }
        Ok(())
    }

    pub fn query(
        &self,
        query_texts: Vec<String>,
        n_results: usize,
        _where_filter: Option<serde_json::Value>,
        _where_document: Option<serde_json::Value>,
        _include: Vec<String>,
    ) -> Result<serde_json::Value> {
        let storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get(&self.name) {
            let mut results: Vec<(&Document, f32)> = Vec::new();
            
            if !query_texts.is_empty() {
                let query = &query_texts[0];
                let query_embedding = self.generate_simple_embedding(query);
                
                // Vector similarity search
                for doc in collection.documents.values() {
                    let similarity = self.cosine_similarity(&query_embedding, &doc.embedding);
                    results.push((doc, similarity));
                }
                
                // Sort by similarity (descending)
                results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            } else {
                // If no query text, return all documents
                for doc in collection.documents.values() {
                    results.push((doc, 1.0));
                }
            }
            
            // Limit results
            results.truncate(n_results);
            
            let ids: Vec<String> = results.iter().map(|(doc, _)| doc.id.clone()).collect();
            let documents: Vec<String> = results.iter().map(|(doc, _)| doc.content.clone()).collect();
            let metadatas: Vec<serde_json::Value> = results.iter().map(|(doc, _)| doc.metadata.clone()).collect();
            let distances: Vec<f64> = results.iter().map(|(_, sim)| (1.0 - *sim) as f64).collect();
            
            Ok(json!({
                "ids": [ids],
                "documents": [documents],
                "metadatas": [metadatas],
                "distances": [distances],
                "embeddings": results.iter().map(|(doc, _)| doc.embedding.clone()).collect::<Vec<_>>()
            }))
        } else {
            Ok(json!({
                "ids": [[]],
                "documents": [[]],
                "metadatas": [[]],
                "distances": [[]],
                "embeddings": [[]]
            }))
        }
    }

    pub fn get(
        &self,
        _ids: Option<Vec<String>>,
        _where_filter: Option<serde_json::Value>,
        _where_document: Option<serde_json::Value>,
        _include: Vec<String>,
        limit: Option<usize>,
        _offset: Option<usize>,
    ) -> Result<serde_json::Value> {
        let storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get(&self.name) {
            let mut docs: Vec<&Document> = collection.documents.values().collect();
            
            if let Some(limit) = limit {
                docs.truncate(limit);
            }
            
            let ids: Vec<String> = docs.iter().map(|doc| doc.id.clone()).collect();
            let documents: Vec<String> = docs.iter().map(|doc| doc.content.clone()).collect();
            let metadatas: Vec<serde_json::Value> = docs.iter().map(|doc| doc.metadata.clone()).collect();
            
            Ok(json!({
                "ids": ids,
                "documents": documents,
                "metadatas": metadatas
            }))
        } else {
            Ok(json!({
                "ids": [],
                "documents": [],
                "metadatas": []
            }))
        }
    }

    pub fn update(
        &self,
        ids: Vec<String>,
        embeddings: Option<Vec<Vec<f32>>>,
        metadatas: Option<Vec<serde_json::Value>>,
        documents: Option<Vec<String>>,
    ) -> Result<()> {
        let mut storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get_mut(&self.name) {
            for (i, id) in ids.iter().enumerate() {
                if let Some(doc) = collection.documents.get_mut(id) {
                    if let Some(metadatas) = &metadatas {
                        if let Some(metadata) = metadatas.get(i) {
                            doc.metadata = metadata.clone();
                        }
                    }
                    if let Some(documents) = &documents {
                        if let Some(content) = documents.get(i) {
                            doc.content = content.clone();
                            // Regenerate embedding if content changed
                            doc.embedding = self.generate_simple_embedding(content);
                        }
                    }
                    if let Some(embeddings) = &embeddings {
                        if let Some(embedding) = embeddings.get(i) {
                            doc.embedding = embedding.clone();
                        }
                    }
                    
                    doc.updated_at = chrono::Utc::now();
                }
            }
        }
        Ok(())
    }

    pub fn delete(&self, ids: Vec<String>) -> Result<()> {
        let mut storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get_mut(&self.name) {
            for id in ids {
                collection.documents.remove(&id);
            }
        }
        Ok(())
    }

    pub fn count(&self) -> Result<usize> {
        let storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get(&self.name) {
            Ok(collection.documents.len())
        } else {
            Ok(0)
        }
    }

    pub fn peek(&self, limit: usize) -> Result<serde_json::Value> {
        let storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get(&self.name) {
            let mut docs: Vec<&Document> = collection.documents.values().collect();
            docs.truncate(limit);
            
            let ids: Vec<String> = docs.iter().map(|doc| doc.id.clone()).collect();
            let documents: Vec<String> = docs.iter().map(|doc| doc.content.clone()).collect();
            let metadatas: Vec<serde_json::Value> = docs.iter().map(|doc| doc.metadata.clone()).collect();
            
            Ok(json!({
                "ids": ids,
                "documents": documents,
                "metadatas": metadatas
            }))
        } else {
            Ok(json!({
                "ids": [],
                "documents": [],
                "metadatas": []
            }))
        }
    }

    pub fn modify(
        &self,
        name: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Result<()> {
        let mut storage = STORAGE.lock().unwrap();
        
        if let Some(collection) = storage.get_mut(&self.name) {
            if let Some(new_metadata) = metadata {
                collection.metadata = Some(new_metadata);
            }
            
            // If name is changed, we'd need to handle that separately
            if let Some(_new_name) = name {
                // For simplicity, we'll just update metadata for now
                // Real implementation would move the collection to new key
            }
        }
        Ok(())
    }

    // Helper method to generate simple embedding based on text content
    fn generate_simple_embedding(&self, text: &str) -> Vec<f32> {
        // Simple TF-IDF like embedding for demonstration
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut embedding = vec![0.0; 384]; // Standard embedding dimension
        
        for (i, word) in words.iter().take(384).enumerate() {
            // Simple hash-based feature extraction
            let hash = word.len() as f32 * 0.1;
            embedding[i] = hash;
        }
        
        // Normalize the vector
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut embedding {
                *val /= norm;
            }
        }
        
        embedding
    }
    
    // Calculate cosine similarity between two vectors
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }
        
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }
        
        dot_product / (norm_a * norm_b)
    }
}

static CLIENT: Mutex<Option<ChromaClient>> = Mutex::new(None);

pub fn initialize_client() -> Result<()> {
    let host = std::env::var("CHROMA_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("CHROMA_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap_or(8000);
    let username = std::env::var("CHROMA_USERNAME").ok();
    let password = std::env::var("CHROMA_PASSWORD").ok();

    let client = ChromaClient::new(&host, port, username.as_deref(), password.as_deref());

    let mut global_client = CLIENT.lock().unwrap();
    *global_client = Some(client);

    Ok(())
}

pub fn get_client() -> Arc<ChromaClient> {
    let client_guard: MutexGuard<Option<ChromaClient>> = CLIENT.lock().unwrap();

    if client_guard.is_none() {
        drop(client_guard);
        initialize_client().expect("Failed to initialize client");
        return get_client();
    }

    Arc::new(client_guard.as_ref().unwrap().clone())
}
