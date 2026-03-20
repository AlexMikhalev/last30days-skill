//! ActionScorer - Hybrid scoring for action selection
//!
//! Provides hybrid scoring combining Terraphim graph embeddings
//! with TF-IDF for ranking actions in the State Machine Graph.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::smg::{StateMachineGraph, GUIAction, ActionType};
use crate::thesaurus::UIThesaurus;

/// Weights for hybrid scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorerWeights {
    /// Weight for graph-based ranking (0.0-1.0)
    pub graph_weight: f32,
    /// Weight for TF-IDF scoring (0.0-1.0)
    pub tfidf_weight: f32,
    /// Weight for success rate (0.0-1.0)
    pub success_rate_weight: f32,
    /// Weight for recency (0.0-1.0)
    pub recency_weight: f32,
}

impl Default for ScorerWeights {
    fn default() -> Self {
        Self {
            graph_weight: 0.4,
            tfidf_weight: 0.3,
            success_rate_weight: 0.2,
            recency_weight: 0.1,
        }
    }
}

/// A scored action with breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredAction {
    /// Reference to the action
    pub action: GUIAction,
    /// Combined score (0.0-1.0)
    pub score: f32,
    /// Score breakdown
    pub breakdown: ScoreBreakdown,
    /// Reasoning for score
    pub reasoning: String,
}

/// Score breakdown by component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    /// Graph-based rank score
    pub graph_score: f32,
    /// TF-IDF relevance score
    pub tfidf_score: f32,
    /// Success rate score
    pub success_rate_score: f32,
    /// Recency score
    pub recency_score: f32,
}

/// ActionScorer - Hybrid scoring for ranking actions
#[derive(Debug, Clone)]
pub struct ActionScorer {
    weights: ScorerWeights,
    /// Term frequency map for current query
    term_freq: HashMap<String, f32>,
    /// Document frequency map
    doc_freq: HashMap<String, f32>,
    /// Total number of actions indexed
    total_actions: usize,
    /// Timestamp of last query (for recency)
    last_query_time: Option<i64>,
}

impl ActionScorer {
    /// Create a new scorer with weights
    pub fn new(graph_weight: f32, tfidf_weight: f32) -> Self {
        Self {
            weights: ScorerWeights {
                graph_weight,
                tfidf_weight,
                ..Default::default()
            },
            term_freq: HashMap::new(),
            doc_freq: HashMap::new(),
            total_actions: 0,
            last_query_time: None,
        }
    }

    /// Create with custom weights
    pub fn with_weights(weights: ScorerWeights) -> Self {
        Self {
            weights,
            term_freq: HashMap::new(),
            doc_freq: HashMap::new(),
            total_actions: 0,
            last_query_time: None,
        }
    }

    /// Score actions and return top K
    pub fn score_actions(
        &self,
        graph: &StateMachineGraph,
        query: &str,
        top_k: usize,
    ) -> Vec<ScoredAction> {
        let mut scored: Vec<ScoredAction> = graph.actions.values()
            .map(|action| self.score_single_action(graph, action, query))
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // Return top K
        scored.into_iter().take(top_k).collect()
    }

    /// Score a single action
    fn score_single_action(
        &self,
        graph: &StateMachineGraph,
        action: &GUIAction,
        query: &str,
    ) -> ScoredAction {
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        // Get source state for context
        let source_state = graph.get_state(&action.source_state);
        
        // 1. Graph score (based on connectivity and rank)
        let graph_score = self.calculate_graph_score(graph, action);

        // 2. TF-IDF score (semantic relevance)
        let tfidf_score = self.calculate_tfidf_score(action, &query_terms, source_state);

        // 3. Success rate score
        let success_rate_score = action.success_rate;

        // 4. Recency score (based on timestamp)
        let recency_score = self.calculate_recency_score(&action.timestamp);

        // Combine scores
        let combined = 
            (graph_score * self.weights.graph_weight) +
            (tfidf_score * self.weights.tfidf_weight) +
            (success_rate_score * self.weights.success_rate_weight) +
            (recency_score * self.weights.recency_weight);

        // Generate reasoning
        let reasoning = self.generate_reasoning(
            action,
            &query_terms,
            graph_score,
            tfidf_score,
            success_rate_score,
        );

        ScoredAction {
            action: action.clone(),
            score: combined,
            breakdown: ScoreBreakdown {
                graph_score,
                tfidf_score,
                success_rate_score,
                recency_score,
            },
            reasoning,
        }
    }

    /// Calculate graph-based score
    fn calculate_graph_score(&self, graph: &StateMachineGraph, action: &GUIAction) -> f32 {
        let mut score = 0.5; // Base score

        // Check if source state has high connectivity
        let source_actions = graph.get_state_actions(&action.source_state);
        let target_actions = graph.get_target_actions(&action.target_state);
        
        // More outgoing edges = more explored state = higher confidence
        let connectivity_factor = (source_actions.len() as f32).min(10.0) / 10.0;
        score += connectivity_factor * 0.3;

        // Check if target state has incoming edges (it's been reached before)
        if !target_actions.is_empty() {
            score += 0.2;
        }

        // Recovery actions get lower base score (prefer tried paths)
        if action.is_recovery {
            score *= 0.8;
        }

        score.min(1.0)
    }

    /// Calculate TF-IDF score
    fn calculate_tfidf_score(
        &self,
        action: &GUIAction,
        query_terms: &[&str],
        source_state: Option<&crate::smg::GUIState>,
    ) -> f32 {
        if query_terms.is_empty() {
            return 0.5;
        }

        let mut score = 0.0;
        let mut match_count = 0;

        // Check action type match
        let action_type_str = action.action_type.to_string();
        for term in query_terms {
            if action_type_str.contains(term) {
                score += 1.0;
                match_count += 1;
            }
        }

        // Check element match if we have source state
        if let Some(state) = source_state {
            if let Some(element) = state.get_element(&action.element_ref) {
                // Check role
                if query_terms.iter().any(|t| element.role.contains(t)) {
                    score += 0.8;
                    match_count += 1;
                }
                // Check name
                if query_terms.iter().any(|t| element.name.to_lowercase().contains(t)) {
                    score += 0.8;
                    match_count += 1;
                }
                // Check text content
                if let Some(text) = &element.text {
                    if query_terms.iter().any(|t| text.to_lowercase().contains(t)) {
                        score += 0.5;
                        match_count += 1;
                    }
                }
            }
        }

        // Check tags
        for tag in &action.tags {
            if query_terms.iter().any(|t| tag.to_lowercase().contains(t)) {
                score += 0.3;
                match_count += 1;
            }
        }

        // Normalize by query length and match count
        let query_len = query_terms.len() as f32;
        if match_count > 0 {
            (score / query_len).min(1.0)
        } else {
            0.1 // Low base score for no matches
        }
    }

    /// Calculate recency score based on timestamp
    fn calculate_recency_score(&self, timestamp: &chrono::DateTime<chrono::Utc>) -> f32 {
        let now = chrono::Utc::now();
        let age_seconds = (now - *timestamp).num_seconds() as f32;
        
        // Decay over 30 days
        let decay_factor = 30.0 * 24.0 * 60.0 * 60.0; // 30 days in seconds
        let score = 1.0 - (age_seconds / decay_factor).min(1.0);
        
        score.max(0.1) // Minimum score
    }

    /// Generate human-readable reasoning
    fn generate_reasoning(
        &self,
        action: &GUIAction,
        _query_terms: &[&str],
        graph_score: f32,
        tfidf_score: f32,
        success_rate: f32,
    ) -> String {
        let mut reasons = Vec::new();

        // Graph reasoning
        if graph_score > 0.7 {
            reasons.push("well-connected state".to_string());
        } else if graph_score < 0.4 {
            reasons.push("less explored path".to_string());
        }

        // Semantic reasoning
        if tfidf_score > 0.6 {
            reasons.push("semantically relevant".to_string());
        }

        // Success rate reasoning
        if success_rate > 0.9 {
            reasons.push("high success rate".to_string());
        } else if success_rate < 0.5 {
            reasons.push("risky action".to_string());
        }

        // Action type
        reasons.push(format!("action: {}", action.action_type));

        if reasons.is_empty() {
            "Standard action".to_string()
        } else {
            reasons.join(", ")
        }
    }

    /// Update weights dynamically
    pub fn set_weights(&mut self, weights: ScorerWeights) {
        self.weights = weights;
    }

    /// Get current weights
    pub fn weights(&self) -> &ScorerWeights {
        &self.weights
    }

    /// Filter actions by type
    pub fn filter_by_action_type<'a>(
        &self,
        graph: &'a StateMachineGraph,
        action_type: ActionType,
    ) -> Vec<&'a GUIAction> {
        graph.actions.values()
            .filter(|a| a.action_type == action_type)
            .collect()
    }

    /// Find actions matching semantic query
    pub fn find_by_intent(
        &self,
        graph: &StateMachineGraph,
        thesaurus: &UIThesaurus,
        intent: &str,
        top_k: usize,
    ) -> Vec<ScoredAction> {
        // Get canonical actions for intent
        let intent_actions = thesaurus.get_actions_for_intent(intent);
        
        // Get actions that match
        let mut scored: Vec<ScoredAction> = graph.actions.values()
            .filter(|a| {
                let action_desc = format!("{} {}", a.action_type, a.element_ref);
                intent_actions.iter().any(|ia| action_desc.to_lowercase().contains(ia))
            })
            .map(|a| {
                let breakdown = ScoreBreakdown {
                    graph_score: self.calculate_graph_score(graph, a),
                    tfidf_score: 0.8, // High because matched intent
                    success_rate_score: a.success_rate,
                    recency_score: self.calculate_recency_score(&a.timestamp),
                };
                let score = 
                    (breakdown.graph_score * self.weights.graph_weight) +
                    (breakdown.tfidf_score * self.weights.tfidf_weight) +
                    (breakdown.success_rate_score * self.weights.success_rate_weight) +
                    (breakdown.recency_score * self.weights.recency_weight);
                
                ScoredAction {
                    action: a.clone(),
                    score,
                    breakdown,
                    reasoning: format!("matches intent: {}", intent),
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(top_k).collect()
    }

    /// Recommend alternative actions on failure
    pub fn recommend_alternatives(
        &self,
        graph: &StateMachineGraph,
        failed_action: &GUIAction,
    ) -> Vec<ScoredAction> {
        // Get all actions from the same source state
        let source_actions = graph.get_state_actions(&failed_action.source_state);
        
        // Score them excluding the failed one
        let mut alternatives: Vec<ScoredAction> = source_actions
            .into_iter()
            .filter(|a| a.id != failed_action.id)
            .map(|a| {
                let mut scored = self.score_single_action(
                    graph, 
                    a, 
                    &format!("{} {}", a.action_type, a.element_ref),
                );
                // Boost score for alternative actions
                scored.reasoning.push_str(" (fallback option)");
                scored
            })
            .collect();

        alternatives.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        alternatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smg::{GUIState, UIElement, Selectors, ElementState, GUIAction, ActionType};
    use chrono::Utc;

    fn create_test_graph() -> StateMachineGraph {
        let mut graph = StateMachineGraph::new("test");
        
        let mut state1 = GUIState::new("https://example.com", "Home");
        let element1 = UIElement {
            id: "btn-login".to_string(),
            role: "button".to_string(),
            name: "Login".to_string(),
            label: None,
            placeholder: None,
            selectors: Selectors { css: Some("#login".to_string()), ..Default::default() },
            bounding_box: None,
            state: ElementState { visible: true, enabled: true, ..Default::default() },
            semantic_roles: vec!["action".to_string()],
            text: Some("Login".to_string()),
            href: None,
            parent_id: None,
            children: vec![],
        };
        state1.add_element(element1);
        
        let state2 = GUIState::new("https://example.com/dashboard", "Dashboard");
        
        graph.add_state(state1);
        graph.add_state(state2);
        
        let mut action = GUIAction::new("state1", "state2", ActionType::Click, "btn-login");
        action.success_rate = 0.95;
        action.timestamp = Utc::now();
        action.tags = vec!["auth".to_string(), "navigation".to_string()];
        
        graph.add_action(action);
        
        graph
    }

    #[test]
    fn test_scorer_creation() {
        let scorer = ActionScorer::new(0.7, 0.3);
        assert_eq!(scorer.weights.graph_weight, 0.7);
        assert_eq!(scorer.weights.tfidf_weight, 0.3);
    }

    #[test]
    fn test_score_actions() {
        let scorer = ActionScorer::new(0.5, 0.5);
        let graph = create_test_graph();
        
        let results = scorer.score_actions(&graph, "login button", 5);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_recency_score() {
        let scorer = ActionScorer::default();
        
        // Recent action
        let recent = chrono::Utc::now();
        let recent_score = scorer.calculate_recency_score(&recent);
        assert!(recent_score > 0.9);
        
        // Old action (1 year ago)
        let old = chrono::Utc::now() - chrono::Duration::days(400);
        let old_score = scorer.calculate_recency_score(&old);
        assert!(old_score < 0.1);
    }
}
