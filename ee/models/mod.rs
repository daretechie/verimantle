//! Frontier Model Adapters
//!
//! Unified interface for frontier AI models (Nova, Claude, GPT, Gemini)
//! Technology-focused, vendor-neutral design

pub mod adapter;
pub mod cost_optimizer;

pub use adapter::{FrontierModel, ModelConfig, ModelResponse, InferenceRequest};
pub use cost_optimizer::{ThinkingBudget, CostOptimizer};
