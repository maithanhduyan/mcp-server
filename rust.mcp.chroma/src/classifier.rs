use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// Enhanced classification with ML-like features
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnhancedClassificationResult {
    pub classification: ClassificationResult,
    pub semantic_features: SemanticFeatures,
    pub performance_metrics: PerformanceMetrics,
    pub lifecycle_info: LifecycleInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticFeatures {
    pub entities: Vec<String>,
    pub topics: Vec<String>,
    pub sentiment_score: f32,
    pub readability_score: f32,
    pub tech_stack: Vec<String>,
    pub security_level: String,
    pub complexity_score: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMetrics {
    pub embedding_quality: f32,
    pub classification_confidence: f32,
    pub processing_time_ms: u64,
    pub memory_usage_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LifecycleInfo {
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u32,
    pub relevance_score: f32,
    pub archival_candidate: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoClassifier {
    pub categories: HashMap<String, Vec<String>>,
    pub purposes: Vec<String>,
    pub scopes: Vec<String>,
    pub difficulty_levels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassificationResult {
    pub suggested_collection: String,
    pub metadata: Value,
    pub confidence_score: f32,
    pub reasoning: String,
    pub validation_passed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslatedResult {
    pub original_text: String,
    pub translated_text: String,
    pub detected_language: String,
    pub target_language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryResult {
    pub documents: Vec<String>,
    pub translated_documents: Option<Vec<String>>,
    pub metadata: Vec<Value>,
    pub distances: Vec<f32>,
    pub query_language: String,
    pub auto_translated: bool,
}

impl AutoClassifier {
    pub fn new() -> Self {
        let mut categories = HashMap::new();

        categories.insert(
            "lap_trinh".to_string(),
            vec![
                "code".to_string(), "programming".to_string(), "algorithm".to_string(),
                "function".to_string(), "class".to_string(), "variable".to_string(),
                "debug".to_string(), "test".to_string(), "development".to_string(),
                "software".to_string(), "application".to_string(), "framework".to_string()
            ]
        );

        categories.insert(
            "co_so_du_lieu".to_string(),
            vec![
                "database".to_string(), "sql".to_string(), "query".to_string(),
                "table".to_string(), "index".to_string(), "schema".to_string(),
                "nosql".to_string(), "mongodb".to_string(), "mysql".to_string(),
                "postgresql".to_string(), "data".to_string(), "storage".to_string()
            ]
        );

        categories.insert(
            "web_development".to_string(),
            vec![
                "html".to_string(), "css".to_string(), "javascript".to_string(),
                "react".to_string(), "vue".to_string(), "angular".to_string(),
                "frontend".to_string(), "backend".to_string(), "api".to_string(),
                "rest".to_string(), "http".to_string(), "server".to_string()
            ]
        );

        categories.insert(
            "devops".to_string(),
            vec![
                "docker".to_string(), "kubernetes".to_string(), "ci/cd".to_string(),
                "deployment".to_string(), "infrastructure".to_string(), "cloud".to_string(),
                "aws".to_string(), "azure".to_string(), "monitoring".to_string(),
                "automation".to_string(), "pipeline".to_string(), "container".to_string()
            ]
        );

        categories.insert(
            "bao_mat".to_string(),
            vec![
                "security".to_string(), "encryption".to_string(), "authentication".to_string(),
                "authorization".to_string(), "vulnerability".to_string(), "ssl".to_string(),
                "tls".to_string(), "firewall".to_string(), "penetration".to_string(),
                "audit".to_string(), "compliance".to_string(), "privacy".to_string()
            ]
        );

        categories.insert(
            "ai_ml".to_string(),
            vec![
                "machine learning".to_string(), "artificial intelligence".to_string(),
                "neural network".to_string(), "deep learning".to_string(), "model".to_string(),
                "training".to_string(), "prediction".to_string(), "algorithm".to_string(),
                "data science".to_string(), "tensorflow".to_string(), "pytorch".to_string(),
                "nlp".to_string()
            ]
        );

        categories.insert(
            "tong_quat".to_string(),
            vec![
                "general".to_string(), "tutorial".to_string(), "guide".to_string(),
                "documentation".to_string(), "reference".to_string(), "example".to_string(),
                "how-to".to_string(), "tips".to_string(), "best practices".to_string(),
                "introduction".to_string(), "overview".to_string(), "basics".to_string()
            ]
        );

        Self {
            categories,
            purposes: vec![
                "tai_lieu_hoc".to_string(),
                "tham_khao".to_string(),
                "tieu_chuan".to_string(),
                "phuong_phap".to_string(),
                "best_practices".to_string(),
                "xu_ly_su_co".to_string(),
            ],
            scopes: vec![
                "frontend".to_string(),
                "backend".to_string(),
                "fullstack".to_string(),
                "mobile".to_string(),
                "devops".to_string(),
                "tong_quat".to_string(),
            ],
            difficulty_levels: vec![
                "co_ban".to_string(),
                "trung_binh".to_string(),
                "nang_cao".to_string(),
                "chuyen_gia".to_string(),
            ],
        }
    }

    // Enhanced classification with ML-like features
    pub fn enhanced_classify(
        &self,
        content: &str,
        title: Option<&str>,
    ) -> Result<EnhancedClassificationResult> {
        let start_time = std::time::Instant::now();
        
        // 1. Basic classification
        let basic_classification = self.classify_content(content, title)?;
        
        // 2. Extract semantic features
        let semantic_features = self.extract_semantic_features(content);
        
        // 3. Calculate performance metrics
        let processing_time = start_time.elapsed().as_millis() as u64;
        let performance_metrics = PerformanceMetrics {
            embedding_quality: self.calculate_embedding_quality(content),
            classification_confidence: basic_classification.confidence_score,
            processing_time_ms: processing_time,
            memory_usage_bytes: content.len() as u64 * 2, // Rough estimate
        };
        
        // 4. Initialize lifecycle info
        let now = Utc::now();
        let lifecycle_info = LifecycleInfo {
            created_at: now,
            last_accessed: now,
            access_count: 1,
            relevance_score: self.calculate_relevance_score(content, &semantic_features),
            archival_candidate: false,
        };
        
        Ok(EnhancedClassificationResult {
            classification: basic_classification,
            semantic_features,
            performance_metrics,
            lifecycle_info,
        })
    }

    pub fn classify_content(
        &self,
        content: &str,
        title: Option<&str>,
    ) -> Result<ClassificationResult> {
        let content_lower = content.to_lowercase();
        let title_lower = title.map(|t| t.to_lowercase()).unwrap_or_default();
        let combined_text = format!("{} {}", title_lower, content_lower);

        // Score each category using multilingual keywords
        let mut category_scores: HashMap<String, f32> = HashMap::new();
        let multilingual_keywords = self.get_multilingual_keywords();

        for category in self.categories.keys() {
            let mut score = 0.0;
            
            // Use both original and multilingual keywords
            if let Some(keywords) = self.categories.get(category) {
                for keyword in keywords {
                    if combined_text.contains(&keyword.to_lowercase()) {
                        score += self.get_keyword_weight(keyword);
                    }
                }
            }
            
            // Add multilingual keyword scores
            if let Some(ml_keywords) = multilingual_keywords.get(category) {
                for keyword in ml_keywords {
                    if combined_text.contains(&keyword.to_lowercase()) {
                        score += self.get_keyword_weight(keyword) * 1.2; // Boost multilingual matches
                    }
                }
            }
            
            category_scores.insert(category.clone(), score);
        }

        // Find best category
        let best_category = category_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| k.clone())
            .unwrap_or_else(|| "tong_quat".to_string());

        // Suggest purpose
        let purpose = self.suggest_purpose(&combined_text);

        // Suggest scope
        let scope = self.suggest_scope(&combined_text);

        // Suggest difficulty
        let difficulty = self.suggest_difficulty(&combined_text);

        // Extract keywords
        let keywords = self.extract_keywords(&combined_text);

        // Generate collection name
        let collection_name = format!(
            "{}_{}_{}_{}", 
            best_category, purpose, scope, difficulty
        );

        // Calculate confidence
        let max_score = category_scores
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let confidence = (max_score / (content.len() as f32 / 100.0)).min(1.0);

        // Generate metadata
        let metadata = json!({
            "danh_muc": best_category,
            "muc_dich": purpose,
            "pham_vi": scope,
            "do_kho": difficulty,
            "keywords": keywords,
            "language": self.detect_language(&combined_text),
            "cap_nhat_cuoi": chrono::Utc::now().format("%Y-%m-%d").to_string(),
            "auto_classified": true,
            "confidence_score": confidence,
            "version": "1.0"
        });

        // Validation using Darwin-Musk principles
        let validation_passed = self.validate_classification(&combined_text, &best_category, confidence);

        let reasoning = format!(
            "Đã phân loại vào danh mục '{}' với độ tin cậy {:.2}%. Dựa trên {} từ khóa phù hợp và {} yếu tố ngữ cảnh.",
            best_category, confidence * 100.0, keywords.len(), category_scores.len()
        );

        Ok(ClassificationResult {
            suggested_collection: collection_name,
            metadata,
            confidence_score: confidence,
            reasoning,
            validation_passed,
        })
    }    // Extract comprehensive semantic features
    pub fn extract_semantic_features(&self, content: &str) -> SemanticFeatures {
        SemanticFeatures {
            entities: self.extract_entities(content),
            topics: self.detect_topics(content),
            sentiment_score: self.analyze_sentiment(content),
            readability_score: self.calculate_readability(content),
            tech_stack: self.detect_tech_stack(content),
            security_level: self.assess_security_level(content),
            complexity_score: self.calculate_complexity(content),
        }
    }
    
    // Extract named entities from content
    fn extract_entities(&self, content: &str) -> Vec<String> {
        let mut entities = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        
        for window in words.windows(2) {
            let phrase = window.join(" ");
            // Simple pattern matching for common entities
            if phrase.contains("API") || phrase.contains("HTTP") || phrase.contains("REST") {
                entities.push(phrase);
            }
        }
        
        // Look for programming languages
        let languages = ["Python", "JavaScript", "Rust", "Java", "Go", "C++", "TypeScript"];
        for lang in &languages {
            if content.to_lowercase().contains(&lang.to_lowercase()) {
                entities.push(lang.to_string());
            }
        }
        
        entities.into_iter().take(10).collect()
    }
    
    // Detect main topics in content
    fn detect_topics(&self, content: &str) -> Vec<String> {
        let mut topics = Vec::new();
        let content_lower = content.to_lowercase();
        
        let topic_keywords = [
            ("Machine Learning", vec!["ml", "model", "training", "neural", "ai"]),
            ("Web Development", vec!["html", "css", "javascript", "frontend", "backend"]),
            ("Database", vec!["sql", "database", "query", "table", "index"]),
            ("DevOps", vec!["docker", "kubernetes", "ci/cd", "deployment", "infrastructure"]),
            ("Security", vec!["security", "encryption", "authentication", "vulnerability"]),
            ("Performance", vec!["optimization", "performance", "speed", "efficiency"]),
        ];
        
        for (topic, keywords) in &topic_keywords {
            let matches = keywords.iter().filter(|&kw| content_lower.contains(kw)).count();
            if matches >= 2 {
                topics.push(topic.to_string());
            }
        }
        
        topics
    }
    
    // Analyze sentiment of content
    fn analyze_sentiment(&self, content: &str) -> f32 {
        let positive_words = ["good", "great", "excellent", "amazing", "perfect", "love"];
        let negative_words = ["bad", "terrible", "awful", "hate", "problem", "issue"];
        
        let content_lower = content.to_lowercase();
        let pos_count = positive_words.iter().filter(|&w| content_lower.contains(w)).count() as f32;
        let neg_count = negative_words.iter().filter(|&w| content_lower.contains(w)).count() as f32;
        
        if pos_count + neg_count == 0.0 {
            return 0.0; // Neutral
        }
        
        (pos_count - neg_count) / (pos_count + neg_count)
    }
    
    // Calculate readability score
    fn calculate_readability(&self, content: &str) -> f32 {
        let sentences = content.split(&['.', '!', '?']).filter(|s| !s.trim().is_empty()).count() as f32;
        let words = content.split_whitespace().count() as f32;
        let syllables = content.chars().filter(|c| "aeiouAEIOU".contains(*c)).count() as f32;
        
        if sentences == 0.0 || words == 0.0 {
            return 0.0;
        }
        
        // Simplified Flesch Reading Ease formula
        let score = 206.835 - (1.015 * (words / sentences)) - (84.6 * (syllables / words));
        (score / 100.0).max(0.0).min(1.0)
    }
    
    // Detect technology stack
    fn detect_tech_stack(&self, content: &str) -> Vec<String> {
        let tech_keywords = [
            ("React", vec!["react", "jsx", "component"]),
            ("Node.js", vec!["node", "npm", "express"]),
            ("Django", vec!["django", "python", "models"]),
            ("Docker", vec!["docker", "container", "dockerfile"]),
            ("PostgreSQL", vec!["postgres", "postgresql", "psql"]),
            ("Redis", vec!["redis", "cache", "session"]),
        ];
        
        let content_lower = content.to_lowercase();
        let mut detected = Vec::new();
        
        for (tech, keywords) in &tech_keywords {
            if keywords.iter().any(|&kw| content_lower.contains(kw)) {
                detected.push(tech.to_string());
            }
        }
        
        detected
    }
    
    // Assess security level
    fn assess_security_level(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();
        let high_security_keywords = ["encryption", "authentication", "authorization", "security", "ssl", "tls"];
        let medium_security_keywords = ["validation", "sanitization", "token", "session"];
        let low_security_keywords = ["password", "user", "login"];
        
        let high_matches = high_security_keywords.iter().filter(|&kw| content_lower.contains(kw)).count();
        let medium_matches = medium_security_keywords.iter().filter(|&kw| content_lower.contains(kw)).count();
        let low_matches = low_security_keywords.iter().filter(|&kw| content_lower.contains(kw)).count();
        
        if high_matches >= 2 {
            "High".to_string()
        } else if medium_matches >= 2 || high_matches >= 1 {
            "Medium".to_string()
        } else if low_matches >= 1 {
            "Low".to_string()
        } else {
            "Minimal".to_string()
        }
    }
    
    // Calculate complexity score
    fn calculate_complexity(&self, content: &str) -> f32 {
        let complexity_indicators = [
            ("algorithm", 0.8),
            ("optimization", 0.7),
            ("architecture", 0.6),
            ("design pattern", 0.9),
            ("scalability", 0.7),
            ("performance", 0.6),
        ];
        
        let content_lower = content.to_lowercase();
        let mut complexity_score = 0.0;
        let mut indicator_count = 0;
        
        for (indicator, weight) in &complexity_indicators {
            if content_lower.contains(indicator) {
                complexity_score += weight;
                indicator_count += 1;
            }
        }
        
        if indicator_count == 0 {
            // Base complexity based on content length and structure
            let word_count = content.split_whitespace().count() as f32;
            let code_indicators = content.matches(&['{', '}', '(', ')', '[', ']']).count() as f32;
            (word_count / 1000.0 + code_indicators / 100.0).min(1.0)
        } else {
            (complexity_score / indicator_count as f32).min(1.0)
        }
    }
      // Calculate embedding quality
    pub fn calculate_embedding_quality(&self, content: &str) -> f32 {
        let word_count = content.split_whitespace().count() as f32;
        let unique_words = content.split_whitespace()
            .map(|w| w.to_lowercase())
            .collect::<std::collections::HashSet<_>>()
            .len() as f32;
        
        if word_count == 0.0 {
            return 0.0;
        }
        
        // Quality based on vocabulary diversity
        let diversity = unique_words / word_count;
        let length_factor = (word_count / 100.0).min(1.0);
        
        (diversity * length_factor).min(1.0)
    }
    
    // Calculate relevance score
    fn calculate_relevance_score(&self, _content: &str, features: &SemanticFeatures) -> f32 {
        let mut score = 0.0;
        
        // Factor in various features
        score += features.entities.len() as f32 * 0.1;
        score += features.topics.len() as f32 * 0.15;
        score += features.tech_stack.len() as f32 * 0.2;
        score += features.complexity_score * 0.3;
        score += features.readability_score * 0.25;
        
        score.min(1.0)
    }

    fn get_keyword_weight(&self, keyword: &str) -> f32 {
        // Higher weight for more specific keywords
        match keyword.len() {
            1..=3 => 0.5,
            4..=6 => 1.0,
            7..=10 => 1.5,
            _ => 2.0,
        }
    }

    pub fn suggest_purpose(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();
        if content_lower.contains("standard") || content_lower.contains("rule") {
            "tieu_chuan".to_string()
        } else if content_lower.contains("method") || content_lower.contains("approach") {
            "phuong_phap".to_string()
        } else if content_lower.contains("best practice") || content_lower.contains("recommendation") {
            "best_practices".to_string()
        } else if content_lower.contains("troubleshoot") || content_lower.contains("fix") {
            "xu_ly_su_co".to_string()
        } else if content_lower.contains("reference") || content_lower.contains("cheat") {
            "tham_khao".to_string()
        } else if content_lower.contains("tutorial") || content_lower.contains("guide") {
            "tai_lieu_hoc".to_string()
        } else {
            "tong_quat".to_string()
        }
    }

    pub fn suggest_scope(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();
        let languages = ["python", "javascript", "java", "rust", "go", "c++"];
        for lang in &languages {
            if content_lower.contains(lang) {
                return format!("ngon_ngu_{}", lang);
            }
        }

        let frameworks = ["react", "django", "flask", "vue", "angular"];
        for fw in &frameworks {
            if content_lower.contains(fw) {
                return format!("framework_{}", fw);
            }
        }

        let domains = ["frontend", "backend", "devops", "mobile"];
        for domain in &domains {
            if content_lower.contains(domain) {
                return domain.to_string();
            }
        }

        let roles = ["developer", "architect", "manager", "qa"];
        for role in &roles {
            if content_lower.contains(role) {
                return format!("vai_tro_{}", role);
            }
        }

        "tong_quat".to_string()
    }

    pub fn suggest_difficulty(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();
        let advanced_terms = ["advanced", "expert", "complex", "enterprise", "scalable"];
        for term in &advanced_terms {
            if content_lower.contains(term) {
                return "nang_cao".to_string();
            }
        }

        let basic_terms = ["basic", "beginner", "introduction", "getting started"];
        for term in &basic_terms {
            if content_lower.contains(term) {
                return "co_ban".to_string();
            }
        }

        "trung_binh".to_string()
    }

    fn extract_keywords(&self, content: &str) -> Vec<String> {
        // Extract technical terms and important words
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut keywords = Vec::new();

        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if clean_word.len() > 3 && !clean_word.chars().all(|c| c.is_ascii_lowercase()) {
                keywords.push(clean_word);
            }
        }

        keywords.sort();
        keywords.dedup();
        keywords.into_iter().take(10).collect()
    }

    pub fn detect_language(&self, content: &str) -> String {
        let vietnamese_chars = content
            .chars()
            .filter(|c| "àáạảãâầấậẩẫăằắặẳẵèéẹẻẽêềếệểễìíịỉĩòóọỏõôồốộổỗơờớợởỡùúụủũưừứựửữỳýỵỷỹđĐ".contains(*c))
            .count();
        let total_chars = content.chars().filter(|c| c.is_alphabetic()).count();

        if vietnamese_chars as f32 / total_chars as f32 > 0.1 {
            "vietnamese".to_string()
        } else {
            "english".to_string()
        }
    }

    fn validate_classification(&self, content: &str, category: &str, confidence: f32) -> bool {
        // Darwin-Musk validation principles

        // Evidence requirement (Darwin)
        if confidence < 0.3 {
            return false;
        }

        // Content size validation (Musk: eliminate if too small)
        if content.len() < 100 {
            return false;
        }

        // Specificity check (Musk: simplify)
        let word_count = content.split_whitespace().count();
        if word_count < 20 {
            return false;
        }

        // Category relevance (Darwin: question assumptions)
        let empty_vec = vec![];
        let category_keywords = self.categories.get(category).unwrap_or(&empty_vec);
        let matches = category_keywords
            .iter()
            .filter(|&keyword| content.to_lowercase().contains(&keyword.to_lowercase()))
            .count();

        if matches == 0 {
            return false;
        }

        true
    }

    // Helper function to improve cross-language keyword matching
    fn get_multilingual_keywords(&self) -> HashMap<String, Vec<String>> {
        let mut categories = HashMap::new();

        categories.insert(
            "lap_trinh".to_string(),
            vec![
                "lập trình".to_string(), "mã nguồn".to_string(), "thuật toán".to_string(),
                "hàm số".to_string(), "lớp".to_string(), "biến".to_string(),
                "gỡ lỗi".to_string(), "kiểm thử".to_string(), "phát triển".to_string()
            ]
        );

        categories.insert(
            "co_so_du_lieu".to_string(),
            vec![
                "cơ sở dữ liệu".to_string(), "truy vấn".to_string(), "bảng".to_string(),
                "chỉ mục".to_string(), "lược đồ".to_string(), "dữ liệu".to_string(),
                "lưu trữ".to_string()
            ]
        );

        categories.insert(
            "web_development".to_string(),
            vec![
                "phát triển web".to_string(), "trang web".to_string(), "ứng dụng web".to_string(),
                "giao diện".to_string(), "máy chủ".to_string(), "api".to_string()
            ]
        );

        categories.insert(
            "devops".to_string(),
            vec![
                "triển khai".to_string(), "hạ tầng".to_string(), "đám mây".to_string(),
                "giám sát".to_string(), "tự động hóa".to_string(), "container".to_string()
            ]
        );

        categories.insert(
            "bao_mat".to_string(),
            vec![
                "bảo mật".to_string(), "mã hóa".to_string(), "xác thực".to_string(),
                "phân quyền".to_string(), "lỗ hổng".to_string(), "tường lửa".to_string()
            ]
        );

        categories.insert(
            "ai_ml".to_string(),
            vec![
                "học máy".to_string(), "trí tuệ nhân tạo".to_string(), "mạng nơ-ron".to_string(),
                "học sâu".to_string(), "mô hình".to_string(), "huấn luyện".to_string(),
                "dự đoán".to_string(), "khoa học dữ liệu".to_string()
            ]
        );

        categories
    }

    // Translation methods for cross-language support
    pub fn translate_text(&self, text: &str, target_language: &str) -> Result<TranslatedResult> {
        let detected_language = self.detect_language(text);
        
        if detected_language == target_language {
            return Ok(TranslatedResult {
                original_text: text.to_string(),
                translated_text: text.to_string(),
                detected_language,
                target_language: target_language.to_string(),
            });
        }

        let translated = self.simple_translate(text, &detected_language, target_language);
        
        Ok(TranslatedResult {
            original_text: text.to_string(),
            translated_text: translated,
            detected_language,
            target_language: target_language.to_string(),
        })
    }

    fn simple_translate(&self, text: &str, from_lang: &str, to_lang: &str) -> String {
        // Simple word-by-word translation using lookup tables
        let translations = if from_lang == "vietnamese" && to_lang == "english" {
            self.get_vi_to_en_translations()
        } else if from_lang == "english" && to_lang == "vietnamese" {
            self.get_en_to_vi_translations()
        } else {
            return text.to_string();
        };

        let words: Vec<&str> = text.split_whitespace().collect();
        let translated_words: Vec<String> = words
            .iter()
            .map(|word| {
                let clean_word = word.to_lowercase();
                translations.get(&clean_word).cloned().unwrap_or_else(|| word.to_string())
            })
            .collect();

        translated_words.join(" ")
    }

    fn get_en_to_vi_translations(&self) -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert("programming".to_string(), "lập trình".to_string());
        translations.insert("database".to_string(), "cơ sở dữ liệu".to_string());
        translations.insert("security".to_string(), "bảo mật".to_string());
        translations.insert("development".to_string(), "phát triển".to_string());
        translations.insert("algorithm".to_string(), "thuật toán".to_string());
        translations.insert("function".to_string(), "hàm số".to_string());
        translations.insert("variable".to_string(), "biến".to_string());
        translations.insert("class".to_string(), "lớp".to_string());
        translations.insert("method".to_string(), "phương thức".to_string());
        translations.insert("server".to_string(), "máy chủ".to_string());
        translations.insert("client".to_string(), "máy khách".to_string());
        translations.insert("network".to_string(), "mạng".to_string());
        translations.insert("internet".to_string(), "internet".to_string());
        translations.insert("website".to_string(), "trang web".to_string());
        translations.insert("application".to_string(), "ứng dụng".to_string());
        translations.insert("software".to_string(), "phần mềm".to_string());
        translations.insert("hardware".to_string(), "phần cứng".to_string());
        translations.insert("system".to_string(), "hệ thống".to_string());
        translations.insert("data".to_string(), "dữ liệu".to_string());
        translations.insert("information".to_string(), "thông tin".to_string());
        translations
    }

    fn get_vi_to_en_translations(&self) -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert("lập trình".to_string(), "programming".to_string());
        translations.insert("cơ sở dữ liệu".to_string(), "database".to_string());
        translations.insert("bảo mật".to_string(), "security".to_string());
        translations.insert("phát triển".to_string(), "development".to_string());
        translations.insert("thuật toán".to_string(), "algorithm".to_string());
        translations.insert("hàm số".to_string(), "function".to_string());
        translations.insert("biến".to_string(), "variable".to_string());
        translations.insert("lớp".to_string(), "class".to_string());
        translations.insert("phương thức".to_string(), "method".to_string());
        translations.insert("máy chủ".to_string(), "server".to_string());
        translations.insert("máy khách".to_string(), "client".to_string());
        translations.insert("mạng".to_string(), "network".to_string());
        translations.insert("trang web".to_string(), "website".to_string());
        translations.insert("ứng dụng".to_string(), "application".to_string());
        translations.insert("phần mềm".to_string(), "software".to_string());
        translations.insert("phần cứng".to_string(), "hardware".to_string());
        translations.insert("hệ thống".to_string(), "system".to_string());
        translations.insert("dữ liệu".to_string(), "data".to_string());
        translations.insert("thông tin".to_string(), "information".to_string());
        translations
    }

    pub fn translate_query_results(
        results: QueryResult,
        target_language: &str,
        query_language: &str,
    ) -> Result<QueryResult> {
        if query_language == target_language {
            return Ok(results);
        }

        let classifier = AutoClassifier::new();
        let mut translated_documents = Vec::new();

        for doc in &results.documents {
            let translation_result = classifier.translate_text(doc, target_language)?;
            translated_documents.push(translation_result.translated_text);
        }

        Ok(QueryResult {
            documents: results.documents,
            translated_documents: Some(translated_documents),
            metadata: results.metadata,
            distances: results.distances,
            query_language: query_language.to_string(),
            auto_translated: true,
        })
    }

    // Dynamic collection suggestion based on content analysis
    pub fn suggest_dynamic_collections(&self, content: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let content_lower = content.to_lowercase();

        // Analyze content patterns
        if content_lower.contains("tutorial") || content_lower.contains("guide") {
            suggestions.push("tutorials_and_guides".to_string());
        }

        if content_lower.contains("error") || content_lower.contains("bug") {
            suggestions.push("troubleshooting".to_string());
        }

        if content_lower.contains("best practice") || content_lower.contains("pattern") {
            suggestions.push("best_practices".to_string());
        }

        if content_lower.contains("api") || content_lower.contains("documentation") {
            suggestions.push("api_documentation".to_string());
        }

        if content_lower.contains("performance") || content_lower.contains("optimization") {
            suggestions.push("performance_optimization".to_string());
        }

        suggestions
    }

    // Smart tagging based on content analysis
    pub fn generate_smart_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let content_lower = content.to_lowercase();

        // Technology tags
        let technologies = [
            "react", "vue", "angular", "nodejs", "python", "java", "rust", "go",
            "docker", "kubernetes", "aws", "azure", "mongodb", "postgresql", "redis"
        ];

        for tech in &technologies {
            if content_lower.contains(tech) {
                tags.push(format!("tech:{}", tech));
            }
        }

        // Complexity tags
        if content_lower.contains("beginner") || content_lower.contains("basic") {
            tags.push("level:beginner".to_string());
        } else if content_lower.contains("advanced") || content_lower.contains("expert") {
            tags.push("level:advanced".to_string());
        } else {
            tags.push("level:intermediate".to_string());
        }

        // Content type tags
        if content_lower.contains("tutorial") {
            tags.push("type:tutorial".to_string());
        } else if content_lower.contains("reference") {
            tags.push("type:reference".to_string());
        } else if content_lower.contains("example") {
            tags.push("type:example".to_string());
        }

        tags
    }

    // Adaptive collection naming with context
    pub fn generate_adaptive_collection_name(&self, classification: &ClassificationResult, content: &str) -> String {
        let base_name = &classification.suggested_collection;
        let smart_tags = self.generate_smart_tags(content);
        
        // Extract primary technology
        let tech_tag = smart_tags.iter()
            .find(|tag| tag.starts_with("tech:"))
            .map(|tag| tag.replace("tech:", ""))
            .unwrap_or_else(|| "general".to_string());

        // Extract level
        let level_tag = smart_tags.iter()
            .find(|tag| tag.starts_with("level:"))
            .map(|tag| tag.replace("level:", ""))
            .unwrap_or_else(|| "intermediate".to_string());

        format!("{}_{}_{}_{}", 
            base_name,
            tech_tag,
            level_tag,
            chrono::Utc::now().format("%Y%m")
        )
    }
}

impl Default for AutoClassifier {
    fn default() -> Self {
        Self::new()
    }
}
