use crate::classifier::{AutoClassifier, EnhancedClassificationResult};
use crate::client::get_client;
use anyhow::{Result, anyhow};
use mcp_spec::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCollectionsRequest {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn chroma_list_collections(request: ListCollectionsRequest) -> Result<Vec<String>> {
    let client = get_client();
    client.list_collections(request.limit, request.offset)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    pub collection_name: String,
    pub embedding_function_name: Option<String>,
    pub metadata: Option<Value>,
    pub space: Option<String>,
    pub ef_construction: Option<i32>,
    pub ef_search: Option<i32>,
    pub max_neighbors: Option<i32>,
    pub num_threads: Option<i32>,
    pub batch_size: Option<i32>,
    pub sync_threshold: Option<i32>,
    pub resize_factor: Option<f32>,
}

pub async fn chroma_create_collection(request: CreateCollectionRequest) -> Result<String> {
    let client = get_client();
    client.create_collection(&request.collection_name, request.metadata)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeekCollectionRequest {
    pub collection_name: String,
    pub limit: usize,
}

pub async fn chroma_peek_collection(request: PeekCollectionRequest) -> Result<Value> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    collection.peek(request.limit)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCollectionInfoRequest {
    pub collection_name: String,
}

pub async fn chroma_get_collection_info(request: GetCollectionInfoRequest) -> Result<Value> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    let count = collection.count()?;
    let sample_documents = collection.peek(3)?;

    Ok(serde_json::json!({
        "name": request.collection_name,
        "count": count,
        "sample_documents": sample_documents
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCollectionCountRequest {
    pub collection_name: String,
}

pub async fn chroma_get_collection_count(request: GetCollectionCountRequest) -> Result<usize> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    collection.count()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyCollectionRequest {
    pub collection_name: String,
    pub new_name: Option<String>,
    pub new_metadata: Option<Value>,
    pub ef_search: Option<i32>,
    pub num_threads: Option<i32>,
    pub batch_size: Option<i32>,
    pub sync_threshold: Option<i32>,
    pub resize_factor: Option<f32>,
}

pub async fn chroma_modify_collection(request: ModifyCollectionRequest) -> Result<String> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    collection.modify(request.new_name.clone(), request.new_metadata.clone())?;

    let mut modified_aspects = Vec::new();
    if request.new_name.is_some() {
        modified_aspects.push("name");
    }
    if request.new_metadata.is_some() {
        modified_aspects.push("metadata");
    }
    if request.ef_search.is_some()
        || request.num_threads.is_some()
        || request.batch_size.is_some()
        || request.sync_threshold.is_some()
        || request.resize_factor.is_some()
    {
        modified_aspects.push("hnsw");
    }

    Ok(format!(
        "Successfully modified collection {}: updated {}",
        request.collection_name,
        modified_aspects.join(" and ")
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCollectionRequest {
    pub collection_name: String,
}

pub async fn chroma_delete_collection(request: DeleteCollectionRequest) -> Result<String> {
    let client = get_client();
    client.delete_collection(&request.collection_name)?;
    Ok(format!(
        "Successfully deleted collection {}",
        request.collection_name
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDocumentsRequest {
    pub collection_name: String,
    pub documents: Vec<String>,
    pub metadatas: Option<Vec<Value>>,
    pub ids: Option<Vec<String>>,
}

pub async fn chroma_add_documents(request: AddDocumentsRequest) -> Result<String> {
    if request.documents.is_empty() {
        return Err(anyhow!("The 'documents' list cannot be empty."));
    }

    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;

    let ids = match request.ids {
        Some(ids) => ids,
        None => (0..request.documents.len())
            .map(|i| i.to_string())
            .collect(),
    };

    let documents_len = request.documents.len();
    collection.add(request.documents.clone(), None, request.metadatas.clone(), ids)?;

    Ok(format!(
        "Successfully added {} documents to collection {}",
        documents_len, request.collection_name
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDocumentsRequest {
    pub collection_name: String,
    pub query_texts: Vec<String>,
    pub n_results: Option<usize>,
    pub where_filter: Option<Value>,
    pub where_document: Option<Value>,
    pub include: Option<Vec<String>>,
}

pub async fn chroma_query_documents(request: QueryDocumentsRequest) -> Result<Value> {
    if request.query_texts.is_empty() {
        return Err(anyhow!("The 'query_texts' list cannot be empty."));
    }

    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;

    let n_results = request.n_results.unwrap_or(5);
    let include = request.include.unwrap_or_else(|| {
        vec![
            "documents".to_string(),
            "metadatas".to_string(),
            "distances".to_string(),
        ]
    });

    collection.query(
        request.query_texts,
        n_results,
        request.where_filter,
        request.where_document,
        include,
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDocumentsWithTranslationRequest {
    pub collection_name: String,
    pub query_texts: Vec<String>,
    pub n_results: Option<usize>,
    pub where_filter: Option<Value>,
    pub where_document: Option<Value>,
    pub include: Option<Vec<String>>,
    pub auto_translate: Option<bool>,
    pub target_language: Option<String>,
}

pub async fn chroma_query_documents_with_translation(
    request: QueryDocumentsWithTranslationRequest,
) -> Result<Value> {
    if request.query_texts.is_empty() {
        return Err(anyhow!("The 'query_texts' list cannot be empty."));
    }

    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;

    let n_results = request.n_results.unwrap_or(5);
    let include = request.include.unwrap_or_else(|| {
        vec![
            "documents".to_string(),
            "metadatas".to_string(),
            "distances".to_string(),
        ]
    });

    // Execute the query first
    let result = collection.query(
        request.query_texts.clone(),
        n_results,
        request.where_filter,
        request.where_document,
        include,
    )?;

    // If auto_translate is enabled, process the results
    if request.auto_translate.unwrap_or(false) {
        let classifier = AutoClassifier::new();

        // Detect query language
        let query_language = if let Some(first_query) = request.query_texts.first() {
            classifier.detect_language(first_query)
        } else {
            "english".to_string()
        };

        // Target language (default to Vietnamese if query is English, English if query is Vietnamese)
        let target_language = request.target_language.unwrap_or_else(|| {
            if query_language == "english" {
                "vietnamese".to_string()
            } else {
                "english".to_string()
            }
        });

        // Extract documents from result
        if let Some(documents_array) = result.get("documents").and_then(|d| d.as_array()) {
            if let Some(first_query_docs) = documents_array.first().and_then(|d| d.as_array()) {
                let documents: Vec<String> = first_query_docs
                    .iter()
                    .filter_map(|doc| doc.as_str().map(|s| s.to_string()))
                    .collect();
                let metadata: Vec<Value> = result
                    .get("metadatas")
                    .and_then(|m| m.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|m| m.as_array())
                    .cloned()
                    .unwrap_or_default();

                let distances: Vec<f32> = result
                    .get("distances")
                    .and_then(|d| d.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|d| d.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64().map(|f| f as f32))
                            .collect()
                    })
                    .unwrap_or_default();

                // Translate results if needed
                let query_result = crate::classifier::QueryResult {
                    documents,
                    translated_documents: None,
                    metadata,
                    distances,
                    query_language: "auto".to_string(),
                    auto_translated: false,
                };
                
                let translated_result = crate::classifier::AutoClassifier::translate_query_results(
                    query_result,
                    &target_language,
                    "auto"
                )?;

                // Return enhanced result with translation info
                return Ok(serde_json::json!({
                    "original_result": result,
                    "translated_documents": translated_result.translated_documents,
                    "query_language": translated_result.query_language,
                    "auto_translated": translated_result.auto_translated,
                    "translation_enabled": true
                }));
            }
        }
    }

    // Return original result if no translation
    Ok(result)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentsRequest {
    pub collection_name: String,
    pub ids: Option<Vec<String>>,
    pub where_filter: Option<Value>,
    pub where_document: Option<Value>,
    pub include: Option<Vec<String>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn chroma_get_documents(request: GetDocumentsRequest) -> Result<Value> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;

    let include = request
        .include
        .unwrap_or_else(|| vec!["documents".to_string(), "metadatas".to_string()]);

    collection.get(
        request.ids,
        request.where_filter,
        request.where_document,
        include,
        request.limit,
        request.offset,
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDocumentsRequest {
    pub collection_name: String,
    pub ids: Vec<String>,
    pub embeddings: Option<Vec<Vec<f32>>>,
    pub metadatas: Option<Vec<Value>>,
    pub documents: Option<Vec<String>>,
}

pub async fn chroma_update_documents(request: UpdateDocumentsRequest) -> Result<String> {
    if request.ids.is_empty() {
        return Err(anyhow!("The 'ids' list cannot be empty."));
    }

    if request.embeddings.is_none() && request.metadatas.is_none() && request.documents.is_none() {
        return Err(anyhow!(
            "At least one of 'embeddings', 'metadatas', or 'documents' must be provided for update."
        ));
    }

    let check_length = |name: &str, len: usize| {
        if len != request.ids.len() {
            return Err(anyhow!(
                "Length of '{}' list must match length of 'ids' list.",
                name
            ));
        }
        Ok(())
    };

    if let Some(ref embeddings) = request.embeddings {
        check_length("embeddings", embeddings.len())?;
    }

    if let Some(ref metadatas) = request.metadatas {
        check_length("metadatas", metadatas.len())?;
    }

    if let Some(ref documents) = request.documents {
        check_length("documents", documents.len())?;
    }

    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;

    collection.update(
        request.ids.clone(),
        request.embeddings,
        request.metadatas,
        request.documents,
    )?;

    Ok(format!(
        "Successfully updated {} documents in collection '{}'",
        request.ids.len(),
        request.collection_name
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDocumentsRequest {
    pub collection_name: String,
    pub ids: Vec<String>,
}

pub async fn chroma_delete_documents(request: DeleteDocumentsRequest) -> Result<String> {
    if request.ids.is_empty() {
        return Err(anyhow!("The 'ids' list cannot be empty."));
    }

    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;

    collection.delete(request.ids.clone())?;

    Ok(format!(
        "Successfully deleted {} documents from collection '{}'",
        request.ids.len(),
        request.collection_name
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtData {
    pub session_id: String,
    pub thought: String,
    pub thought_number: usize,
    pub total_thoughts: usize,
    pub next_thought_needed: bool,
    pub is_revision: Option<bool>,
    pub revises_thought: Option<usize>,
    pub branch_from_thought: Option<usize>,
    pub branch_id: Option<String>,
    pub needs_more_thoughts: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtResponse {
    pub session_id: String,
    pub thought_number: usize,
    pub total_thoughts: usize,
    pub next_thought_needed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

fn validate_thought_data(input_data: &ThoughtData) -> Result<()> {
    if input_data.session_id.is_empty() {
        return Err(anyhow!("Invalid sessionId: must be provided"));
    }
    if input_data.thought.is_empty() {
        return Err(anyhow!("Invalid thought: must be a string"));
    }
    if input_data.thought_number == 0 {
        return Err(anyhow!(
            "Invalid thoughtNumber: must be a number greater than 0"
        ));
    }
    if input_data.total_thoughts == 0 {
        return Err(anyhow!(
            "Invalid totalThoughts: must be a number greater than 0"
        ));
    }

    Ok(())
}

pub async fn process_thought(input_data: ThoughtData) -> Result<ThoughtResponse> {
    match validate_thought_data(&input_data) {
        Ok(_) => {
            let total_thoughts =
                std::cmp::max(input_data.thought_number, input_data.total_thoughts);

            Ok(ThoughtResponse {
                session_id: input_data.session_id,
                thought_number: input_data.thought_number,
                total_thoughts,
                next_thought_needed: input_data.next_thought_needed,
                error: None,
                status: None,
            })
        }
        Err(e) => Ok(ThoughtResponse {
            session_id: input_data.session_id,
            thought_number: input_data.thought_number,
            total_thoughts: input_data.total_thoughts,
            next_thought_needed: input_data.next_thought_needed,
            error: Some(e.to_string()),
            status: Some("failed".to_string()),
        }),
    }
}

pub fn get_tool_definitions() -> Vec<Tool> {
    let mut tools = Vec::new();

    let add_tool = |tools: &mut Vec<Tool>, name: &str, description: &str, schema: Value| {
        tools.push(Tool {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: schema,
        });
    };

    add_tool(
        &mut tools,
        "chroma_list_collections",
        "Lists all collections in the ChromaDB instance",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "properties": {
                "limit": {"type": "integer", "description": "Maximum number of collections to return"},
                "offset": {"type": "integer", "description": "Offset for pagination"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_create_collection",
        "Creates a new collection in ChromaDB",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to create"},
                "metadata": {"type": "object", "description": "Optional metadata for the collection"},
                "embedding_function_name": {"type": "string", "description": "Name of the embedding function to use"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_peek_collection",
        "Shows a sample of documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "limit"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to peek"},
                "limit": {"type": "integer", "description": "Number of documents to return"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_get_collection_info",
        "Gets metadata about a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object",
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"}
            }
        }))
        .unwrap(),
    );

    add_tool(
        &mut tools,
        "chroma_get_collection_count",
        "Counts the number of documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object",
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"}
            }
        }))
        .unwrap(),
    );

    add_tool(
        &mut tools,
        "chroma_modify_collection",
        "Modifies collection properties",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to modify"},
                "new_name": {"type": "string", "description": "New name for the collection"},
                "new_metadata": {"type": "object", "description": "New metadata for the collection"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_delete_collection",
        "Deletes a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to delete"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_add_documents",
        "Adds documents to a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "documents"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "documents": {"type": "array", "items": {"type": "string"}, "description": "List of documents to add"},
                "metadatas": {"type": "array", "items": {"type": "object"}, "description": "List of metadata objects for documents"},
                "ids": {"type": "array", "items": {"type": "string"}, "description": "List of IDs for documents"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_query_documents",
        "Searches for similar documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "query_texts"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "query_texts": {"type": "array", "items": {"type": "string"}, "description": "List of query texts"},
                "n_results": {"type": "integer", "description": "Number of results to return per query"},
                "where_filter": {"type": "object", "description": "Filter by metadata"},
                "where_document": {"type": "object", "description": "Filter by document content"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_query_documents_with_translation",
        "Searches for similar documents in a collection with automatic translation support",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "query_texts"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "query_texts": {"type": "array", "items": {"type": "string"}, "description": "List of query texts"},
                "n_results": {"type": "integer", "description": "Number of results to return per query"},
                "where_filter": {"type": "object", "description": "Filter by metadata"},
                "where_document": {"type": "object", "description": "Filter by document content"},
                "auto_translate": {"type": "boolean", "description": "Enable automatic translation of results"},
                "target_language": {"type": "string", "description": "Target language for translation (e.g., 'vietnamese', 'english')"}
            }
        })).unwrap()
    );

    add_tool(
        &mut tools,
        "chroma_smart_add_documents",
        "Smartly adds documents to collections with optional auto-classification",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["documents"],
            "properties": {
                "documents": {"type": "array", "items": {"type": "string"}, "description": "List of documents to add"},
                "ids": {"type": "array", "items": {"type": "string"}, "description": "List of IDs for documents"},
                "metadatas": {"type": "array", "items": {"type": "object"}, "description": "List of metadata objects for documents"},
                "titles": {"type": "array", "items": {"type": "string"}, "description": "Optional list of titles for the documents"},
                "auto_classify": {"type": "boolean", "description": "Whether to auto-classify documents into collections"},
                "force_collection": {"type": "string", "description": "If provided, documents will be added to this collection directly"}
            }
        })).unwrap()
    );

    tools
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartAddDocumentsRequest {
    pub documents: Vec<String>,
    pub ids: Option<Vec<String>>,
    pub metadatas: Option<Vec<Value>>,
    pub titles: Option<Vec<String>>,
    pub auto_classify: Option<bool>,
    pub force_collection: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartAddDocumentsResponse {
    pub results: Vec<SmartAddDocumentResult>,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartAddDocumentResult {
    pub document_id: String,
    pub collection_name: String,
    pub classification: Option<Value>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionAnalytics {
    pub name: String,
    pub document_count: usize,
    pub avg_embedding_quality: f32,
    pub dominant_topics: Vec<String>,
    pub tech_stack_distribution: HashMap<String, u32>,
    pub complexity_distribution: HashMap<String, u32>,
    pub security_level_distribution: HashMap<String, u32>,
    pub health_score: f32,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub archival_candidate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartMergeRequest {
    pub source_collections: Vec<String>,
    pub similarity_threshold: f32,
    pub target_collection_name: Option<String>,
    pub preserve_metadata: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleManagementRequest {
    pub collection_name: String,
    pub inactivity_threshold_days: u32,
    pub size_threshold_gb: f32,
    pub fragmentation_threshold: f32,
    pub auto_optimize: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedSmartAddRequest {
    pub documents: Vec<String>,
    pub titles: Option<Vec<String>>,
    pub force_collection: Option<String>,
    pub auto_classify: Option<bool>,
    pub enable_semantic_analysis: Option<bool>,
    pub generate_embeddings: Option<bool>,
    pub extract_metadata: Option<bool>,
    pub ids: Option<Vec<String>>,
    pub metadatas: Option<Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedSmartAddResponse {
    pub results: Vec<EnhancedDocumentResult>,
    pub collection_analytics: Vec<CollectionAnalytics>,
    pub processing_summary: ProcessingSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedDocumentResult {
    pub id: String,
    pub collection_name: String,
    pub classification: EnhancedClassificationResult,
    pub embedding_quality: f32,
    pub auto_generated_tags: Vec<String>,
    pub suggested_related_docs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingSummary {
    pub total_documents: usize,
    pub successful_classifications: usize,
    pub new_collections_created: usize,
    pub total_processing_time_ms: u64,
    pub average_confidence_score: f32,
    pub quality_metrics: QualityMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub avg_embedding_quality: f32,
    pub avg_readability_score: f32,
    pub avg_complexity_score: f32,
    pub security_coverage: f32,
    pub tech_diversity_score: f32,
}

pub async fn chroma_smart_add_documents(
    request: SmartAddDocumentsRequest,
) -> Result<SmartAddDocumentsResponse> {
    use crate::classifier::AutoClassifier;
    
    let mut results = Vec::new();
    let auto_classify = request.auto_classify.unwrap_or(true);
    let client = get_client();
    let classifier = AutoClassifier::new();
    
    for (i, document) in request.documents.iter().enumerate() {
        let doc_id = request.ids.as_ref()
            .and_then(|ids| ids.get(i))
            .cloned()
            .unwrap_or_else(|| format!("doc_{}", i));
            
        let title = request.titles.as_ref()
            .and_then(|titles| titles.get(i))
            .map(|s| s.as_str());
            
        let metadata = request.metadatas.as_ref()
            .and_then(|metas| metas.get(i))
            .cloned();
        
        let mut result = SmartAddDocumentResult {
            document_id: doc_id.clone(),
            collection_name: "unclassified".to_string(),
            classification: None,
            success: false,
            error: None,
        };
        
        let collection_name = if let Some(forced) = &request.force_collection {
            forced.clone()
        } else if auto_classify {
            match classifier.classify_content(document, title) {
                Ok(classification) => {
                    // Test enhanced features
                    let smart_tags = classifier.generate_smart_tags(document);
                    let dynamic_collections = classifier.suggest_dynamic_collections(document);
                    let adaptive_name = classifier.generate_adaptive_collection_name(&classification, document);
                    
                    result.classification = Some(serde_json::json!({
                        "suggested_collection": classification.suggested_collection,
                        "adaptive_collection": adaptive_name,
                        "dynamic_suggestions": dynamic_collections,
                        "smart_tags": smart_tags,
                        "metadata": classification.metadata,
                        "confidence_score": classification.confidence_score,
                        "reasoning": classification.reasoning,
                        "validation_passed": classification.validation_passed
                    }));
                    
                    if classification.validation_passed {
                        adaptive_name  // Use adaptive name instead of original
                    } else {
                        result.error = Some("Classification validation failed".to_string());
                        "unclassified".to_string()
                    }
                }
                Err(e) => {
                    result.error = Some(format!("Classification failed: {}", e));
                    "unclassified".to_string()
                }
            }
        } else {
            "unclassified".to_string()
        };
        
        result.collection_name = collection_name.clone();
        
        // Only add to collection if classification passed or was forced
        if result.error.is_none() {
            // Ensure collection exists
            if client.get_collection(&collection_name).is_err() {
                if let Err(e) = client.create_collection(&collection_name, None) {
                    result.error = Some(format!("Failed to create collection: {}", e));
                    results.push(result);
                    continue;
                }
            }
            
            // Add document to collection
            let collection = match client.get_collection(&collection_name) {
                Ok(col) => col,
                Err(e) => {
                    result.error = Some(format!("Failed to get collection: {}", e));
                    results.push(result);
                    continue;
                }
            };
            
            let mut final_metadata = metadata.unwrap_or_else(|| serde_json::json!({}));
            if let Some(classification_data) = &result.classification {
                if let Some(serde_json::Value::Object(class_map)) = classification_data.get("metadata") {
                    if let serde_json::Value::Object(ref mut final_map) = final_metadata {
                        for (key, value) in class_map {
                            final_map.insert(key.clone(), value.clone());
                        }
                    }
                }
                
                // Add smart tags to metadata
                if let Some(smart_tags) = classification_data.get("smart_tags") {
                    if let serde_json::Value::Object(ref mut final_map) = final_metadata {
                        final_map.insert("smart_tags".to_string(), smart_tags.clone());
                    }
                }
            }
            
            match collection.add(vec![document.clone()], None, Some(vec![final_metadata]), vec![doc_id.clone()]) {
                Ok(_) => {
                    result.success = true;
                }
                Err(e) => {
                    result.error = Some(format!("Failed to add document: {}", e));
                }
            }
        }
        
        results.push(result);
    }
    
    let successful_count = results.iter().filter(|r| r.success).count();
    let total_count = results.len();
    
    Ok(SmartAddDocumentsResponse {
        results,
        summary: format!(
            "Successfully processed {} out of {} documents. Auto-classification: {}",
            successful_count, total_count, auto_classify
        ),
    })
}

// Enhanced Smart Add Documents with ML Classification
pub async fn chroma_enhanced_smart_add_documents(
    request: EnhancedSmartAddRequest,
) -> Result<EnhancedSmartAddResponse> {
    let start_time = std::time::Instant::now();
    let classifier = AutoClassifier::new();
    let client = get_client();
    
    let mut results = Vec::new();
    let mut collections_created = 0;
    let mut successful_classifications = 0;
    let mut total_confidence = 0.0;
    let mut quality_metrics = QualityMetrics {
        avg_embedding_quality: 0.0,
        avg_readability_score: 0.0,
        avg_complexity_score: 0.0,
        security_coverage: 0.0,
        tech_diversity_score: 0.0,
    };

    for (i, document) in request.documents.iter().enumerate() {
        let title = request.titles.as_ref().and_then(|t| t.get(i));
        let id = request.ids.as_ref()
            .and_then(|ids| ids.get(i))
            .cloned()
            .unwrap_or_else(|| format!("doc_{}", uuid::Uuid::new_v4()));

        // Enhanced classification with semantic analysis
        let classification_result = if request.enable_semantic_analysis.unwrap_or(true) {
            classifier.enhanced_classify(document, title.map(|s| s.as_str()))?
        } else {
            let basic = classifier.classify_content(document, title.map(|s| s.as_str()))?;
            EnhancedClassificationResult {
                classification: basic,
                semantic_features: classifier.extract_semantic_features(document),
                performance_metrics: crate::classifier::PerformanceMetrics {
                    embedding_quality: 0.5,
                    classification_confidence: 0.5,
                    processing_time_ms: 0,
                    memory_usage_bytes: document.len() as u64,
                },
                lifecycle_info: crate::classifier::LifecycleInfo {
                    created_at: chrono::Utc::now(),
                    last_accessed: chrono::Utc::now(),
                    access_count: 1,
                    relevance_score: 0.5,
                    archival_candidate: false,
                },
            }
        };

        let collection_name = if let Some(forced) = &request.force_collection {
            forced.clone()
        } else if request.auto_classify.unwrap_or(true) {
            classifier.generate_adaptive_collection_name(&classification_result.classification, document)
        } else {
            "general_documents".to_string()
        };

        // Ensure collection exists
        let collections = client.list_collections(None, None)?;
        if !collections.contains(&collection_name) {
            client.create_collection(&collection_name, Some(serde_json::json!({
                "auto_created": true,
                "created_by": "enhanced_smart_add",
                "created_at": chrono::Utc::now(),
                "classification_metadata": classification_result.classification.metadata
            })))?;
            collections_created += 1;
        }

        // Generate enhanced metadata
        let mut enhanced_metadata = classification_result.classification.metadata.clone();
        if let Value::Object(ref mut map) = enhanced_metadata {
            map.insert("semantic_features".to_string(), serde_json::to_value(&classification_result.semantic_features)?);
            map.insert("performance_metrics".to_string(), serde_json::to_value(&classification_result.performance_metrics)?);
            map.insert("lifecycle_info".to_string(), serde_json::to_value(&classification_result.lifecycle_info)?);
            map.insert("auto_generated_tags".to_string(), serde_json::to_value(classifier.generate_smart_tags(document))?);
        }

        // Add document to collection
        let collection = client.get_collection(&collection_name)?;
        collection.add(
            vec![document.clone()],
            None, // Embeddings will be generated automatically
            Some(vec![enhanced_metadata.clone()]),
            vec![id.clone()],
        )?;

        // Generate auto tags and find related documents
        let auto_tags = classifier.generate_smart_tags(document);
        let suggested_related = find_related_documents(&collection_name, document, &classifier).await?;

        results.push(EnhancedDocumentResult {
            id,
            collection_name,
            classification: classification_result.clone(),
            embedding_quality: classification_result.performance_metrics.embedding_quality,
            auto_generated_tags: auto_tags,
            suggested_related_docs: suggested_related,
        });

        // Update metrics
        if classification_result.classification.validation_passed {
            successful_classifications += 1;
            total_confidence += classification_result.classification.confidence_score;
            quality_metrics.avg_embedding_quality += classification_result.performance_metrics.embedding_quality;
            quality_metrics.avg_readability_score += classification_result.semantic_features.readability_score;
            quality_metrics.avg_complexity_score += classification_result.semantic_features.complexity_score;
            
            if classification_result.semantic_features.security_level != "Minimal" {
                quality_metrics.security_coverage += 1.0;
            }
            
            quality_metrics.tech_diversity_score += classification_result.semantic_features.tech_stack.len() as f32;
        }
    }

    // Finalize metrics
    let doc_count = request.documents.len() as f32;
    if successful_classifications > 0 {
        let success_count = successful_classifications as f32;
        quality_metrics.avg_embedding_quality /= success_count;
        quality_metrics.avg_readability_score /= success_count;
        quality_metrics.avg_complexity_score /= success_count;
        quality_metrics.security_coverage /= doc_count;
        quality_metrics.tech_diversity_score /= success_count;
    }

    // Generate collection analytics
    let collection_analytics = generate_collection_analytics(&client).await?;

    let processing_time = start_time.elapsed().as_millis() as u64;
    let avg_confidence = if successful_classifications > 0 {
        total_confidence / successful_classifications as f32
    } else {
        0.0
    };

    Ok(EnhancedSmartAddResponse {
        results,
        collection_analytics,
        processing_summary: ProcessingSummary {
            total_documents: request.documents.len(),
            successful_classifications,
            new_collections_created: collections_created,
            total_processing_time_ms: processing_time,
            average_confidence_score: avg_confidence,
            quality_metrics,
        },
    })
}

// Collection Lifecycle Management
pub async fn manage_collection_lifecycle(
    request: LifecycleManagementRequest,
) -> Result<String> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    
    let document_count = collection.count()?;
    let analytics = analyze_collection_health(&request.collection_name).await?;
    
    let mut actions_taken = Vec::new();
    
    // Check for archival candidacy
    if analytics.last_accessed < chrono::Utc::now() - chrono::Duration::days(request.inactivity_threshold_days as i64) {
        archive_collection(&request.collection_name).await?;
        actions_taken.push("archived due to inactivity".to_string());
    }
    
    // Check fragmentation
    if analytics.health_score < request.fragmentation_threshold && request.auto_optimize {
        optimize_collection(&request.collection_name).await?;
        actions_taken.push("optimized for fragmentation".to_string());
    }
    
    // Check size limits
    let estimated_size_gb = (document_count * 1024) as f32 / (1024.0 * 1024.0 * 1024.0); // Rough estimate
    if estimated_size_gb > request.size_threshold_gb {
        split_large_collection(&request.collection_name).await?;
        actions_taken.push("split due to size limit".to_string());
    }
    
    Ok(format!(
        "Lifecycle management completed for collection '{}'. Actions taken: [{}]",
        request.collection_name,
        actions_taken.join(", ")
    ))
}

// Smart Collection Merge
pub async fn smart_merge_collections(
    request: SmartMergeRequest,
) -> Result<String> {
    let _client = get_client();
    let classifier = AutoClassifier::new();
    
    if request.source_collections.len() < 2 {
        return Err(anyhow!("Need at least 2 collections to merge"));
    }
    
    let mut similarity_matrix: HashMap<(String, String), f32> = HashMap::new();
    
    // Calculate similarities between collections
    for i in 0..request.source_collections.len() {
        for j in i+1..request.source_collections.len() {
            let coll1 = &request.source_collections[i];
            let coll2 = &request.source_collections[j];
            
            let similarity = calculate_collection_similarity(coll1, coll2, &classifier).await?;
            similarity_matrix.insert((coll1.clone(), coll2.clone()), similarity);
        }
    }
    
    // Find collections that exceed similarity threshold
    let mut merge_groups = Vec::new();
    for ((coll1, coll2), similarity) in &similarity_matrix {
        if *similarity > request.similarity_threshold {
            merge_groups.push(vec![coll1.clone(), coll2.clone()]);
        }
    }
    
    if merge_groups.is_empty() {
        return Ok("No collections found with sufficient similarity for merging".to_string());
    }
    
    // Perform merges
    let mut merged_count = 0;
    for group in merge_groups {
        let target_name = request.target_collection_name.clone()
            .unwrap_or_else(|| format!("merged_{}", chrono::Utc::now().timestamp()));
        
        merge_collections_group(&group, &target_name, request.preserve_metadata).await?;
        merged_count += 1;
    }
    
    Ok(format!("Successfully merged {} collection groups", merged_count))
}

// Helper functions for enhanced operations
async fn find_related_documents(
    collection_name: &str,
    document: &str,
    classifier: &AutoClassifier,
) -> Result<Vec<String>> {
    let client = get_client();
    let collection = client.get_collection(collection_name)?;
    
    // Use semantic features to find related documents
    let features = classifier.extract_semantic_features(document);
    let query_terms: Vec<String> = features.entities.into_iter()
        .chain(features.topics.into_iter())
        .chain(features.tech_stack.into_iter())
        .take(5)
        .collect();
    
    if query_terms.is_empty() {
        return Ok(Vec::new());
    }
    
    let results = collection.query(
        query_terms,
        3,
        None,
        None,
        vec!["documents".to_string()],
    )?;
    
    // Extract document IDs from results
    if let Some(ids_array) = results.get("ids").and_then(|v| v.as_array()) {
        if let Some(first_batch) = ids_array.get(0).and_then(|v| v.as_array()) {
            return Ok(first_batch.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect());
        }
    }
    
    Ok(Vec::new())
}

async fn generate_collection_analytics(
    client: &std::sync::Arc<crate::client::ChromaClient>,
) -> Result<Vec<CollectionAnalytics>> {
    let collections = client.list_collections(None, None)?;
    let mut analytics = Vec::new();
    
    for collection_name in collections {
        let health = analyze_collection_health(&collection_name).await?;
        analytics.push(health);
    }
    
    Ok(analytics)
}

async fn analyze_collection_health(collection_name: &str) -> Result<CollectionAnalytics> {
    let client = get_client();
    let collection = client.get_collection(collection_name)?;
    
    let document_count = collection.count()?;
    let sample_docs = collection.peek(10)?;
    
    // Analyze sample documents for metrics
    let mut tech_stack_dist = HashMap::new();
    let mut complexity_dist = HashMap::new();
    let mut security_dist = HashMap::new();
    let mut total_quality = 0.0;
    
    if let Some(docs_array) = sample_docs.get("documents").and_then(|v| v.as_array()) {
        let classifier = AutoClassifier::new();
        
        for doc_value in docs_array {
            if let Some(doc_text) = doc_value.as_str() {
                let features = classifier.extract_semantic_features(doc_text);
                total_quality += classifier.calculate_embedding_quality(doc_text);
                
                // Update distributions
                for tech in &features.tech_stack {
                    *tech_stack_dist.entry(tech.clone()).or_insert(0) += 1;
                }
                
                let complexity_level = if features.complexity_score > 0.7 {
                    "High"
                } else if features.complexity_score > 0.4 {
                    "Medium"
                } else {
                    "Low"
                };
                *complexity_dist.entry(complexity_level.to_string()).or_insert(0) += 1;
                *security_dist.entry(features.security_level.clone()).or_insert(0) += 1;
            }
        }
    }
    
    let sample_size = std::cmp::min(document_count, 10);
    let avg_quality = if sample_size > 0 { 
        total_quality / (sample_size as f32)
    } else { 
        0.0 
    };
    
    let health_score = calculate_health_score(document_count, avg_quality, &tech_stack_dist);
    
    Ok(CollectionAnalytics {
        name: collection_name.to_string(),
        document_count,
        avg_embedding_quality: avg_quality,
        dominant_topics: extract_dominant_topics(&tech_stack_dist),
        tech_stack_distribution: tech_stack_dist,
        complexity_distribution: complexity_dist,
        security_level_distribution: security_dist,
        health_score,
        last_accessed: chrono::Utc::now(), // In real implementation, this would be tracked
        archival_candidate: health_score < 0.3 || document_count == 0,
    })
}

async fn calculate_collection_similarity(
    coll1: &str,
    coll2: &str,
    classifier: &AutoClassifier,
) -> Result<f32> {
    let client = get_client();
    
    let collection1 = client.get_collection(coll1)?;
    let collection2 = client.get_collection(coll2)?;
    
    let sample1 = collection1.peek(5)?;
    let sample2 = collection2.peek(5)?;
    
    // Extract features from both collections
    let features1 = extract_collection_features(&sample1, classifier);
    let features2 = extract_collection_features(&sample2, classifier);
    
    // Calculate similarity based on overlapping features
    let common_features = features1.intersection(&features2).count() as f32;
    let total_features = features1.union(&features2).count() as f32;
    
    if total_features == 0.0 {
        return Ok(0.0);
    }
    
    Ok(common_features / total_features)
}

fn extract_collection_features(
    sample_docs: &Value,
    classifier: &AutoClassifier,
) -> std::collections::HashSet<String> {
    let mut features = std::collections::HashSet::new();
    
    if let Some(docs_array) = sample_docs.get("documents").and_then(|v| v.as_array()) {
        for doc_value in docs_array {
            if let Some(doc_text) = doc_value.as_str() {
                let semantic_features = classifier.extract_semantic_features(doc_text);
                features.extend(semantic_features.entities);
                features.extend(semantic_features.topics);
                features.extend(semantic_features.tech_stack);
            }
        }
    }
    
    features
}

fn calculate_health_score(
    document_count: usize,
    avg_quality: f32,
    tech_distribution: &HashMap<String, u32>,
) -> f32 {
    let size_factor = if document_count == 0 { 
        0.0 
    } else { 
        (document_count as f32 / 100.0).min(1.0) 
    };
    
    let diversity_factor = if tech_distribution.is_empty() { 
        0.0 
    } else { 
        (tech_distribution.len() as f32 / 10.0).min(1.0) 
    };
    
    (size_factor * 0.4 + avg_quality * 0.4 + diversity_factor * 0.2).min(1.0)
}

fn extract_dominant_topics(tech_dist: &HashMap<String, u32>) -> Vec<String> {
    let mut sorted_tech: Vec<_> = tech_dist.iter().collect();
    sorted_tech.sort_by(|a, b| b.1.cmp(a.1));
    sorted_tech.into_iter().take(5).map(|(k, _)| k.clone()).collect()
}

async fn archive_collection(collection_name: &str) -> Result<()> {
    let client = get_client();
    let archived_name = format!("archived_{}_{}", collection_name, chrono::Utc::now().timestamp());
    
    // In a real implementation, this would move the collection to archive storage
    // For now, we'll just rename it
    let collection = client.get_collection(collection_name)?;
    collection.modify(Some(archived_name), Some(serde_json::json!({
        "archived": true,
        "archived_at": chrono::Utc::now(),
        "original_name": collection_name
    })))?;
    
    Ok(())
}

async fn optimize_collection(collection_name: &str) -> Result<()> {
    let client = get_client();
    let collection = client.get_collection(collection_name)?;
    
    // In a real implementation, this would perform index optimization, compaction, etc.
    // For now, we'll just update metadata to indicate optimization
    collection.modify(None, Some(serde_json::json!({
        "last_optimized": chrono::Utc::now(),
        "optimization_version": "1.0"
    })))?;
    
    Ok(())
}

async fn split_large_collection(collection_name: &str) -> Result<()> {
    let client = get_client();
    let collection = client.get_collection(collection_name)?;
    
    // Get all documents
    let all_docs = collection.get(None, None, None, vec!["documents".to_string(), "metadatas".to_string()], None, None)?;
    
    if let (Some(docs), Some(metadatas)) = (
        all_docs.get("documents").and_then(|v| v.as_array()),
        all_docs.get("metadatas").and_then(|v| v.as_array())
    ) {
        let mid_point = docs.len() / 2;
        
        // Create two new collections
        let part1_name = format!("{}_part1", collection_name);
        let part2_name = format!("{}_part2", collection_name);
        
        client.create_collection(&part1_name, Some(serde_json::json!({
            "split_from": collection_name,
            "part": 1,
            "split_at": chrono::Utc::now()
        })))?;
        
        client.create_collection(&part2_name, Some(serde_json::json!({
            "split_from": collection_name,
            "part": 2,
            "split_at": chrono::Utc::now()
        })))?;
        
        // Move documents to new collections
        let part1_collection = client.get_collection(&part1_name)?;
        let part2_collection = client.get_collection(&part2_name)?;
        
        // Add first half to part1
        if mid_point > 0 {
            let docs1: Vec<String> = docs.iter().take(mid_point)
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            let metadatas1: Vec<Value> = metadatas.iter().take(mid_point).cloned().collect();
            let ids1: Vec<String> = (0..docs1.len()).map(|i| format!("doc_{}", i)).collect();
            
            part1_collection.add(docs1, None, Some(metadatas1), ids1)?;
        }
        
        // Add second half to part2
        if docs.len() > mid_point {
            let docs2: Vec<String> = docs.iter().skip(mid_point)
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            let metadatas2: Vec<Value> = metadatas.iter().skip(mid_point).cloned().collect();
            let ids2: Vec<String> = (0..docs2.len()).map(|i| format!("doc_{}", i)).collect();
            
            part2_collection.add(docs2, None, Some(metadatas2), ids2)?;
        }
    }
    
    Ok(())
}

async fn merge_collections_group(
    collections: &[String],
    target_name: &str,
    preserve_metadata: bool,
) -> Result<()> {
    let client = get_client();
    
    // Create target collection
    client.create_collection(target_name, Some(serde_json::json!({
        "merged_from": collections,
        "merged_at": chrono::Utc::now(),
        "preserve_metadata": preserve_metadata
    })))?;
    
    let target_collection = client.get_collection(target_name)?;
    
    // Merge all documents from source collections
    for source_name in collections {
        let source_collection = client.get_collection(source_name)?;
        let all_docs = source_collection.get(None, None, None, vec!["documents".to_string(), "metadatas".to_string()], None, None)?;
        
        if let (Some(docs), metadatas) = (
            all_docs.get("documents").and_then(|v| v.as_array()),
            all_docs.get("metadatas").and_then(|v| v.as_array())
        ) {
            let docs_vec: Vec<String> = docs.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            
            let metadatas_vec = if preserve_metadata && metadatas.is_some() {
                Some(metadatas.unwrap().iter().cloned().collect())
            } else {
                None
            };
            
            let ids_vec: Vec<String> = (0..docs_vec.len())
                .map(|i| format!("{}_{}", source_name, i))
                .collect();
            
            if !docs_vec.is_empty() {
                target_collection.add(docs_vec, None, metadatas_vec, ids_vec)?;
            }
        }
    }
    
    Ok(())
}


