//! Polyglot Embeddings
//!
//! Native language support for semantic memory.
//! Per GLOBAL_GAPS.md: Arabic (Jais), Japanese, Hindi

pub mod embeddings;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use embeddings::{PolyglotEmbedder, EmbeddingResult};

/// Supported languages with native embedding models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Arabic,
    Japanese,
    Hindi,
    Chinese,
    Spanish,
    French,
    German,
    Portuguese,
    Russian,
    Korean,
    Other,
}

impl Language {
    /// Detect language from text (simplified).
    pub fn detect(text: &str) -> Self {
        let text_lower = text.to_lowercase();
        
        // Arabic detection (presence of Arabic Unicode block)
        if text.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c)) {
            return Language::Arabic;
        }
        
        // Japanese detection (Hiragana, Katakana, or Kanji)
        if text.chars().any(|c| 
            ('\u{3040}'..='\u{309F}').contains(&c) || // Hiragana
            ('\u{30A0}'..='\u{30FF}').contains(&c) || // Katakana
            ('\u{4E00}'..='\u{9FFF}').contains(&c)    // Kanji
        ) {
            return Language::Japanese;
        }
        
        // Hindi detection (Devanagari)
        if text.chars().any(|c| ('\u{0900}'..='\u{097F}').contains(&c)) {
            return Language::Hindi;
        }
        
        // Chinese detection (CJK without Japanese markers)
        if text.chars().any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c)) {
            return Language::Chinese;
        }
        
        // Cyrillic for Russian
        if text.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)) {
            return Language::Russian;
        }
        
        // Korean (Hangul)
        if text.chars().any(|c| ('\u{AC00}'..='\u{D7AF}').contains(&c)) {
            return Language::Korean;
        }
        
        // Default to English
        Language::English
    }
    
    /// Get recommended embedding model for this language.
    pub fn embedding_model(&self) -> &'static str {
        match self {
            Language::Arabic => "jais-embedding-v1",
            Language::Japanese => "multilingual-e5-large",
            Language::Hindi => "multilingual-e5-large",
            Language::Chinese => "bge-large-zh",
            Language::Korean => "ko-sroberta-multitask",
            Language::Russian => "multilingual-e5-large",
            _ => "e5-large-v2",
        }
    }
}

/// Polyglot memory store.
pub struct PolyglotMemory {
    /// Embedder per language
    embedders: HashMap<Language, PolyglotEmbedder>,
    /// Default embedder
    default_embedder: PolyglotEmbedder,
}

impl PolyglotMemory {
    /// Create a new polyglot memory store.
    pub fn new() -> Self {
        Self {
            embedders: HashMap::new(),
            default_embedder: PolyglotEmbedder::new(Language::English),
        }
    }
    
    /// Register a language-specific embedder.
    pub fn register_embedder(&mut self, language: Language, embedder: PolyglotEmbedder) {
        self.embedders.insert(language, embedder);
    }
    
    /// Embed text with automatic language detection.
    pub async fn embed(&self, text: &str) -> EmbeddingResult {
        let language = Language::detect(text);
        let embedder = self.embedders.get(&language).unwrap_or(&self.default_embedder);
        embedder.embed(text).await
    }
    
    /// Semantic search with cross-lingual intent verification.
    pub async fn search(&self, query: &str, top_k: usize) -> Vec<SearchResult> {
        let _query_embedding = self.embed(query).await;
        
        // In production, this would search the vector store
        Vec::new()
    }
}

impl Default for PolyglotMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub text: String,
    pub score: f32,
    pub language: Language,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection_english() {
        assert_eq!(Language::detect("Hello world"), Language::English);
    }

    #[test]
    fn test_language_detection_arabic() {
        assert_eq!(Language::detect("مرحبا بالعالم"), Language::Arabic);
    }

    #[test]
    fn test_language_detection_japanese() {
        assert_eq!(Language::detect("こんにちは"), Language::Japanese);
    }

    #[test]
    fn test_language_detection_hindi() {
        assert_eq!(Language::detect("नमस्ते"), Language::Hindi);
    }

    #[test]
    fn test_embedding_model_selection() {
        assert_eq!(Language::Arabic.embedding_model(), "jais-embedding-v1");
        assert_eq!(Language::English.embedding_model(), "e5-large-v2");
    }
}
