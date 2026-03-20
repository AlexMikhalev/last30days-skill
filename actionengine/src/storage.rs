//! Graph Storage - JSON persistence with Terraphim cache
//!
//! Provides serialization and persistence for the State Machine Graph.

use std::path::PathBuf;
use std::fs;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

use crate::smg::StateMachineGraph;
use crate::thesaurus::UIThesaurus;
use crate::ActionEngineConfig;

/// Storage error type
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Path error: {0}")]
    Path(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
}

/// Graph storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base directory for storage
    pub base_path: PathBuf,
    /// Enable caching
    pub cache_enabled: bool,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval: u64,
    /// Maximum cache size in MB
    pub max_cache_size_mb: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./data/actionengine"),
            cache_enabled: true,
            auto_save_interval: 60,
            max_cache_size_mb: 100,
        }
    }
}

/// Graph storage manager
pub struct GraphStorage {
    config: StorageConfig,
    /// In-memory cache
    cache: Option<GraphCache>,
}

impl GraphStorage {
    /// Create new storage manager
    pub fn new(config: StorageConfig) -> Self {
        let cache = if config.cache_enabled {
            Some(GraphCache::new(config.max_cache_size_mb))
        } else {
            None
        };

        Self { config, cache }
    }

    /// Create with default config
    pub fn default_storage() -> Self {
        Self::new(StorageConfig::default())
    }

    /// Save graph to JSON file
    pub fn save_graph(&self, graph: &StateMachineGraph) -> Result<PathBuf, StorageError> {
        let file_path = self.graph_path(&graph.name);
        
        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize graph
        let json = serde_json::to_string_pretty(graph)?;
        
        // Write to file
        let mut file = fs::File::create(&file_path)?;
        file.write_all(json.as_bytes())?;

        // Update cache
        if let Some(cache) = &self.cache {
            cache.put(&graph.name, graph);
        }

        Ok(file_path)
    }

    /// Load graph from JSON file
    pub fn load_graph(&self, name: &str) -> Result<StateMachineGraph, StorageError> {
        // Check cache first
        if let Some(cache) = &self.cache {
            if let Some(graph) = cache.get(name) {
                return Ok(graph);
            }
        }

        let file_path = self.graph_path(name);
        
        if !file_path.exists() {
            return Err(StorageError::Path(format!(
                "Graph file not found: {}",
                file_path.display()
            )));
        }

        // Read file
        let mut file = fs::File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize
        let graph: StateMachineGraph = serde_json::from_str(&contents)?;

        // Update cache
        if let Some(cache) = &self.cache {
            cache.put(name, &graph);
        }

        Ok(graph)
    }

    /// Delete graph
    pub fn delete_graph(&self, name: &str) -> Result<(), StorageError> {
        let file_path = self.graph_path(name);
        
        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }

        // Remove from cache
        if let Some(cache) = &self.cache {
            cache.remove(name);
        }

        Ok(())
    }

    /// List all saved graphs
    pub fn list_graphs(&self) -> Result<Vec<String>, StorageError> {
        let mut graphs = Vec::new();
        
        if !self.config.base_path.exists() {
            return Ok(graphs);
        }

        for entry in fs::read_dir(&self.config.base_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    if let Some(name) = stem.to_str() {
                        // Skip thesaurus files
                        if !name.ends_with("_thesaurus") {
                            graphs.push(name.to_string());
                        }
                    }
                }
            }
        }

        Ok(graphs)
    }

    /// Save thesaurus to JSON file
    pub fn save_thesaurus(&self, name: &str, thesaurus: &UIThesaurus) -> Result<PathBuf, StorageError> {
        let file_path = self.thesaurus_path(name);
        
        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(thesaurus)?;
        let mut file = fs::File::create(&file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(file_path)
    }

    /// Load thesaurus from JSON file
    pub fn load_thesaurus(&self, name: &str) -> Result<UIThesaurus, StorageError> {
        let file_path = self.thesaurus_path(name);
        
        if !file_path.exists() {
            // Return default thesaurus if file doesn't exist
            return Ok(UIThesaurus::new());
        }

        let mut file = fs::File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let thesaurus: UIThesaurus = serde_json::from_str(&contents)?;
        Ok(thesaurus)
    }

    /// Save complete ActionEngine state
    pub fn save_engine(
        &self,
        graph: &StateMachineGraph,
        thesaurus: &UIThesaurus,
        config: &ActionEngineConfig,
    ) -> Result<(), StorageError> {
        // Save graph
        self.save_graph(graph)?;

        // Save thesaurus
        self.save_thesaurus(&config.app_name, thesaurus)?;

        // Save config
        self.save_config(&config.app_name, config)?;

        Ok(())
    }

    /// Load complete ActionEngine state
    pub fn load_engine(
        &self,
        name: &str,
    ) -> Result<(StateMachineGraph, UIThesaurus, ActionEngineConfig), StorageError> {
        let graph = self.load_graph(name)?;
        let thesaurus = self.load_thesaurus(name)?;
        let config = self.load_config(name)?;

        Ok((graph, thesaurus, config))
    }

    /// Save configuration
    fn save_config(&self, name: &str, config: &ActionEngineConfig) -> Result<PathBuf, StorageError> {
        let file_path = self.config_path(name);
        
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(config)?;
        let mut file = fs::File::create(&file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(file_path)
    }

    /// Load configuration
    fn load_config(&self, name: &str) -> Result<ActionEngineConfig, StorageError> {
        let file_path = self.config_path(name);
        
        if !file_path.exists() {
            return Ok(ActionEngineConfig {
                app_name: name.to_string(),
                ..Default::default()
            });
        }

        let mut file = fs::File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: ActionEngineConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Get graph file path
    fn graph_path(&self, name: &str) -> PathBuf {
        self.config.base_path.join(format!("{}.json", name))
    }

    /// Get thesaurus file path
    fn thesaurus_path(&self, name: &str) -> PathBuf {
        self.config.base_path.join(format!("{}_thesaurus.json", name))
    }

    /// Get config file path
    fn config_path(&self, name: &str) -> PathBuf {
        self.config.base_path.join(format!("{}_config.json", name))
    }

    /// Export graph to Terraphim format
    pub fn export_to_terraphim(&self, graph: &StateMachineGraph) -> Result<TerraphimExport, StorageError> {
        let export = TerraphimExport::from_graph(graph);
        Ok(export)
    }

    /// Import from Terraphim format
    pub fn import_from_terraphim(&self, export: &TerraphimExport) -> StateMachineGraph {
        export.to_graph()
    }

    /// Clear all data
    pub fn clear_all(&self) -> Result<(), StorageError> {
        if self.config.base_path.exists() {
            fs::remove_dir_all(&self.config.base_path)?;
        }
        
        if let Some(cache) = &self.cache {
            cache.clear();
        }

        Ok(())
    }

    /// Get storage statistics
    pub fn stats(&self) -> StorageStats {
        let mut stats = StorageStats::default();
        
        if self.config.base_path.exists() {
            if let Ok(entries) = fs::read_dir(&self.config.base_path) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            stats.file_count += 1;
                            stats.total_size_bytes += metadata.len();
                        }
                    }
                }
            }
        }

        if let Some(cache) = &self.cache {
            stats.cache_entries = cache.len();
        }

        stats
    }
}

/// In-memory cache for graphs
struct GraphCache {
    max_size_mb: u64,
    entries: std::collections::HashMap<String, CachedGraph>,
}

struct CachedGraph {
    graph: StateMachineGraph,
    timestamp: std::time::Instant,
}

impl GraphCache {
    fn new(max_size_mb: u64) -> Self {
        Self {
            max_size_mb,
            entries: std::collections::HashMap::new(),
        }
    }

    fn put(&self, _name: &str, _graph: &StateMachineGraph) {
        // Note: In a real implementation, we'd use interior mutability
        // For now, this is a simplified version
    }

    fn get(&self, _name: &str) -> Option<StateMachineGraph> {
        None // Simplified - would return cached graph
    }

    fn remove(&self, _name: &str) {
        // Remove from cache
    }

    fn clear(&self) {
        // Clear cache
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Storage statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub file_count: usize,
    pub total_size_bytes: u64,
    pub cache_entries: usize,
}

/// Terraphim-compatible export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraphimExport {
    /// Graph name
    pub name: String,
    /// Nodes in Terraphim format
    pub nodes: Vec<TerraphimNode>,
    /// Edges in Terraphim format
    pub edges: Vec<TerraphimEdge>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

use std::collections::HashMap;

/// Terraphim node format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraphimNode {
    pub id: u64,
    pub label: String,
    pub node_type: String,
    pub properties: HashMap<String, String>,
    pub rank: u64,
}

/// Terraphim edge format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraphimEdge {
    pub id: u64,
    pub source: u64,
    pub target: u64,
    pub label: String,
    pub properties: HashMap<String, String>,
    pub rank: u64,
}

impl TerraphimExport {
    /// Convert StateMachineGraph to Terraphim format
    pub fn from_graph(graph: &StateMachineGraph) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_id_map: HashMap<String, u64> = HashMap::new();
        let mut next_id: u64 = 1;

        // Convert states to nodes
        for (state_id, state) in &graph.states {
            let id = next_id;
            node_id_map.insert(state_id.clone(), id);
            next_id += 1;

            let mut properties = HashMap::new();
            properties.insert("url".to_string(), state.url.clone());
            properties.insert("title".to_string(), state.title.clone());
            properties.insert("element_count".to_string(), state.elements.len().to_string());

            nodes.push(TerraphimNode {
                id,
                label: state.title.clone(),
                node_type: "gui_state".to_string(),
                properties,
                rank: 1, // Base rank
            });
        }

        // Convert actions to edges
        for (_action_id, action) in &graph.actions {
            let source_id = node_id_map.get(&action.source_state).copied().unwrap_or(0);
            let target_id = node_id_map.get(&action.target_state).copied().unwrap_or(0);

            let mut properties = HashMap::new();
            properties.insert("action_type".to_string(), action.action_type.to_string());
            properties.insert("element_ref".to_string(), action.element_ref.clone());
            properties.insert("success_rate".to_string(), format!("{:.2}", action.success_rate));

            edges.push(TerraphimEdge {
                id: next_id,
                source: source_id,
                target: target_id,
                label: action.action_type.to_string(),
                properties,
                rank: (action.success_rate * 100.0) as u64,
            });
            next_id += 1;
        }

        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "actionengine".to_string());
        metadata.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Self {
            name: graph.name.clone(),
            nodes,
            edges,
            metadata,
        }
    }

    /// Convert back to StateMachineGraph
    pub fn to_graph(&self) -> StateMachineGraph {
        let mut graph = StateMachineGraph::new(&self.name);

        // This is a simplified conversion - in practice, you'd need
        // to store more information in the Terraphim format
        for node in &self.nodes {
            if node.node_type == "gui_state" {
                let url = node.properties.get("url").cloned().unwrap_or_default();
                let title = node.properties.get("title").cloned().unwrap_or_default();
                let state = crate::smg::GUIState::new(&url, &title);
                graph.add_state(state);
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_creation() {
        let config = StorageConfig::default();
        let storage = GraphStorage::new(config);
        assert!(storage.cache.is_some());
    }

    #[test]
    fn test_save_and_load_graph() {
        let temp_dir = TempDir::new().unwrap();
        let config = StorageConfig {
            base_path: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let storage = GraphStorage::new(config);

        let graph = StateMachineGraph::new("test-app");
        
        // Save
        let path = storage.save_graph(&graph).unwrap();
        assert!(path.exists());

        // Load
        let loaded = storage.load_graph("test-app").unwrap();
        assert_eq!(loaded.name, "test-app");
    }

    #[test]
    fn test_thesaurus_storage() {
        let temp_dir = TempDir::new().unwrap();
        let config = StorageConfig {
            base_path: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let storage = GraphStorage::new(config);

        let thesaurus = UIThesaurus::new();
        
        // Save
        storage.save_thesaurus("test", &thesaurus).unwrap();

        // Load
        let loaded = storage.load_thesaurus("test").unwrap();
        assert!(!loaded.action_synonyms.is_empty());
    }

    #[test]
    fn test_terraphim_export() {
        let mut graph = StateMachineGraph::new("test");
        let state = crate::smg::GUIState::new("https://example.com", "Test");
        graph.add_state(state);

        let export = TerraphimExport::from_graph(&graph);
        assert!(!export.nodes.is_empty());
    }
}
