//! ActionEngine: State Machine Memory for GUI Agents
//!
//! This module provides the foundation for programmatic GUI automation using
//! Terraphim-powered knowledge graph embeddings.
//!
//! # Architecture
//!
//! - **StateMachineGraph (SMG)**: Graph of GUI states and actions
//! - **UIThesaurus**: Semantic mapping for UI elements
//! - **ActionScorer**: Hybrid scoring (graph rank + TF-IDF)
//! - **Storage**: JSON persistence with Terraphim cache
//!
//! # Example
//!
//! ```rust
//! use actionengine::StateMachineGraph;
//!
//! let mut graph = StateMachineGraph::new("my-app");
//! // Add states and actions...
//! ```

pub mod smg;
pub mod thesaurus;
pub mod scorer;
pub mod storage;

pub use smg::{
    GUIState, UIElement, GUIAction, ActionType, Selectors, BoundingBox,
    ElementState, StateMachineGraph, ActionTemplate,
};
pub use thesaurus::UIThesaurus;
pub use scorer::{ActionScorer, ScorerWeights, ScoredAction};
pub use storage::{GraphStorage, StorageError};

use terraphim_rolegraph::RoleGraph;

/// Configuration for ActionEngine
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionEngineConfig {
    /// Name of the application being automated
    pub app_name: String,
    /// Path to Terraphim knowledge graph
    pub kg_path: Option<String>,
    /// Enable auto-linking of UI elements
    pub auto_linking: bool,
    /// Graph rank weight (0.0-1.0)
    pub graph_weight: f32,
    /// TF-IDF weight (0.0-1.0)
    pub tfidf_weight: f32,
}

impl Default for ActionEngineConfig {
    fn default() -> Self {
        Self {
            app_name: "default".to_string(),
            kg_path: None,
            auto_linking: true,
            graph_weight: 0.7,
            tfidf_weight: 0.3,
        }
    }
}

/// Main ActionEngine for GUI automation
pub struct ActionEngine {
    /// State machine graph
    pub graph: StateMachineGraph,
    /// UI Thesaurus for semantic matching
    pub thesaurus: UIThesaurus,
    /// Action scorer with hybrid ranking
    pub scorer: ActionScorer,
    /// Configuration
    pub config: ActionEngineConfig,
}

impl ActionEngine {
    /// Create a new ActionEngine
    pub fn new(app_name: &str) -> Self {
        let config = ActionEngineConfig {
            app_name: app_name.to_string(),
            ..Default::default()
        };

        Self {
            graph: StateMachineGraph::new(app_name),
            thesaurus: UIThesaurus::default(),
            scorer: ActionScorer::new(config.graph_weight, config.tfidf_weight),
            config,
        }
    }

    /// Create with explicit configuration
    pub fn with_config(config: ActionEngineConfig) -> Self {
        Self {
            graph: StateMachineGraph::new(&config.app_name),
            thesaurus: UIThesaurus::default(),
            scorer: ActionScorer::new(config.graph_weight, config.tfidf_weight),
            config,
        }
    }

    /// Integrate with existing Terraphim RoleGraph
    pub fn integrate_terraphim(&mut self, role_graph: RoleGraph) {
        self.graph.integrate_rolegraph(role_graph);
    }

    /// Search for relevant actions given a task
    pub fn search_actions(&self, task: &str, top_k: usize) -> Vec<ScoredAction> {
        // Use scorer to rank actions
        self.scorer.score_actions(&self.graph, task, top_k)
    }

    /// Get all available actions from current state
    pub fn get_available_actions(&self, state_id: &str) -> Vec<&GUIAction> {
        self.graph.get_state_actions(state_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_engine_creation() {
        let engine = ActionEngine::new("test-app");
        assert_eq!(engine.config.app_name, "test-app");
    }

    #[test]
    fn test_default_config() {
        let config = ActionEngineConfig::default();
        assert_eq!(config.graph_weight, 0.7);
        assert_eq!(config.tfidf_weight, 0.3);
    }
}
