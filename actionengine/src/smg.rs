//! State Machine Graph (SMG) - Core data structures for GUI state representation
//!
//! This module provides the fundamental types for representing GUI states,
//! UI elements, and actions in a graph structure that extends Terraphim's RoleGraph.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for a GUI state
pub type StateId = String;

/// Unique identifier for a UI element
pub type ElementId = String;

/// Unique identifier for an action
pub type ActionId = String;

/// Represents a clickable/input-able UI element on a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// Unique element identifier
    pub id: ElementId,
    /// Role: button, input, link, checkbox, etc.
    pub role: String,
    /// Accessible name
    pub name: String,
    /// Associated label text
    pub label: Option<String>,
    /// Placeholder text for inputs
    pub placeholder: Option<String>,
    /// Element selectors (CSS, XPath, ARIA)
    pub selectors: Selectors,
    /// Bounding box on screen
    pub bounding_box: Option<BoundingBox>,
    /// Current state: visible, enabled, checked, etc.
    pub state: ElementState,
    /// Semantic roles for Terraphim matching
    pub semantic_roles: Vec<String>,
    /// Text content
    pub text: Option<String>,
    /// Href for links
    pub href: Option<String>,
    /// Parent element ID
    pub parent_id: Option<ElementId>,
    /// Child element IDs
    pub children: Vec<ElementId>,
}

impl UIElement {
    /// Create a new UI element
    pub fn new(role: &str, name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: role.to_string(),
            name: name.to_string(),
            label: None,
            placeholder: None,
            selectors: Selectors::default(),
            bounding_box: None,
            state: ElementState::default(),
            semantic_roles: Vec::new(),
            text: None,
            href: None,
            parent_id: None,
            children: Vec::new(),
        }
    }

    /// Get primary selector
    pub fn primary_selector(&self) -> Option<&String> {
        self.selectors.css.as_ref()
            .or(self.selectors.xpath.as_ref())
            .or(self.selectors.aria.as_ref())
    }
}

/// Element selectors for locating the UI element
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Selectors {
    /// CSS selector
    pub css: Option<String>,
    /// XPath selector
    pub xpath: Option<String>,
    /// ARIA selector
    pub aria: Option<String>,
    /// Text content selector
    pub text: Option<String>,
    /// Role-based selector
    pub role: Option<String>,
    /// JavaScript selector
    pub js: Option<String>,
}

/// Bounding box for element positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Element state flags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ElementState {
    pub visible: bool,
    pub enabled: bool,
    pub checked: bool,
    pub focused: bool,
    pub required: bool,
    pub disabled: bool,
    pub readonly: bool,
}

/// Represents a complete GUI state (page/frame/dialog)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUIState {
    /// Unique state identifier
    pub id: StateId,
    /// URL of the page
    pub url: String,
    /// Page title
    pub title: String,
    /// Visible UI elements
    pub elements: Vec<UIElement>,
    /// Viewport dimensions
    pub viewport: Viewport,
    /// Accessibility tree snapshot
    pub accessibility_tree: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Loading state
    pub is_loading: bool,
    /// Error message if any
    pub error: Option<String>,
    /// Metadata for Terraphim
    pub metadata: HashMap<String, String>,
}

impl GUIState {
    /// Create a new GUI state
    pub fn new(url: &str, title: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url: url.to_string(),
            title: title.to_string(),
            elements: Vec::new(),
            viewport: Viewport::default(),
            accessibility_tree: None,
            timestamp: Utc::now(),
            status_code: None,
            is_loading: false,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Add an element to the state
    pub fn add_element(&mut self, element: UIElement) {
        self.elements.push(element);
    }

    /// Get element by ID
    pub fn get_element(&self, id: &str) -> Option<&UIElement> {
        self.elements.iter().find(|e| e.id == id)
    }

    /// Get all clickable elements
    pub fn get_clickable_elements(&self) -> Vec<&UIElement> {
        self.elements.iter()
            .filter(|e| e.role == "button" || e.role == "link" || e.state.enabled)
            .collect()
    }

    /// Get all input elements
    pub fn get_input_elements(&self) -> Vec<&UIElement> {
        self.elements.iter()
            .filter(|e| e.role == "textbox" || e.role == "searchbox" || e.role == "combobox")
            .collect()
    }
}

/// Viewport dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self { width: 1920, height: 1080 }
    }
}

/// Type of GUI action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Click,
    Type,
    Select,
    Scroll,
    Hover,
    Drag,
    DoubleClick,
    RightClick,
    Submit,
    Navigate,
    Refresh,
    GoBack,
    GoForward,
    Wait,
    Execute,
    Screenshot,
    Custom,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Click => write!(f, "click"),
            ActionType::Type => write!(f, "type"),
            ActionType::Select => write!(f, "select"),
            ActionType::Scroll => write!(f, "scroll"),
            ActionType::Hover => write!(f, "hover"),
            ActionType::Drag => write!(f, "drag"),
            ActionType::DoubleClick => write!(f, "double_click"),
            ActionType::RightClick => write!(f, "right_click"),
            ActionType::Submit => write!(f, "submit"),
            ActionType::Navigate => write!(f, "navigate"),
            ActionType::Refresh => write!(f, "refresh"),
            ActionType::GoBack => write!(f, "go_back"),
            ActionType::GoForward => write!(f, "go_forward"),
            ActionType::Wait => write!(f, "wait"),
            ActionType::Execute => write!(f, "execute"),
            ActionType::Screenshot => write!(f, "screenshot"),
            ActionType::Custom => write!(f, "custom"),
        }
    }
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Click
    }
}

/// Pre/post condition for an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub element_id: Option<ElementId>,
    pub property: String,
    pub operator: String,  // eq, ne, gt, lt, contains, etc.
    pub value: String,
}

/// Represents an action that transitions between GUI states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUIAction {
    /// Unique action identifier
    pub id: ActionId,
    /// Source state ID
    pub source_state: StateId,
    /// Target state ID
    pub target_state: StateId,
    /// Type of action
    pub action_type: ActionType,
    /// Reference to UI element
    pub element_ref: ElementId,
    /// Input data (for type, select actions)
    pub input_data: Option<String>,
    /// Selectors for the element
    pub selectors: Selectors,
    /// Preconditions that must be met
    pub preconditions: Vec<Condition>,
    /// Postconditions expected after action
    pub postconditions: Vec<Condition>,
    /// Success rate (0.0-1.0)
    pub success_rate: f32,
    /// Number of times executed
    pub execution_count: u32,
    /// Average execution time in ms
    pub avg_duration_ms: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Is this a recovery action
    pub is_recovery: bool,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl GUIAction {
    /// Create a new action
    pub fn new(
        source: &str,
        target: &str,
        action_type: ActionType,
        element_ref: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_state: source.to_string(),
            target_state: target.to_string(),
            action_type,
            element_ref: element_ref.to_string(),
            input_data: None,
            selectors: Selectors::default(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            success_rate: 1.0,
            execution_count: 0,
            avg_duration_ms: 0,
            timestamp: Utc::now(),
            is_recovery: false,
            tags: Vec::new(),
        }
    }

    /// Record successful execution
    pub fn record_success(&mut self, duration_ms: u64) {
        self.execution_count += 1;
        // Running average
        self.avg_duration_ms = ((self.avg_duration_ms * (self.execution_count - 1) as u64) + duration_ms) 
            / self.execution_count as u64;
    }

    /// Record failed execution
    pub fn record_failure(&mut self) {
        self.execution_count += 1;
        // Decrease success rate
        let successes = (self.success_rate * self.execution_count as f32) as u32;
        self.success_rate = successes as f32 / self.execution_count as f32;
    }
}

/// Action template for program synthesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTemplate {
    /// Template ID
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Action type
    pub action_type: ActionType,
    /// Target element type
    pub target: String,  // button, input, link, etc.
    /// Context elements (surrounding UI)
    pub context: Vec<String>,
    /// Template parameters
    pub parameters: Vec<String>,
    /// Example usage
    pub example: Option<String>,
}

impl ActionTemplate {
    /// Create a new action template
    pub fn new(description: &str, action_type: ActionType, target: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.to_string(),
            action_type,
            target: target.to_string(),
            context: Vec::new(),
            parameters: Vec::new(),
            example: None,
        }
    }
}

/// State Machine Graph - extends Terraphim RoleGraph for GUI automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineGraph {
    /// Graph name
    pub name: String,
    /// GUI states (nodes)
    pub states: HashMap<StateId, GUIState>,
    /// GUI actions (edges)
    pub actions: HashMap<ActionId, GUIAction>,
    /// Action templates
    pub action_templates: Vec<ActionTemplate>,
    /// State index by URL
    url_index: HashMap<String, Vec<StateId>>,
    /// Element index
    element_index: HashMap<String, Vec<ElementId>>,
    /// Terraphim RoleGraph integration
    #[serde(skip)]
    role_graph: Option<terraphim_rolegraph::RoleGraph>,
}

impl StateMachineGraph {
    /// Create a new state machine graph
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            states: HashMap::new(),
            actions: HashMap::new(),
            action_templates: Vec::new(),
            url_index: HashMap::new(),
            element_index: HashMap::new(),
            role_graph: None,
        }
    }

    /// Add a GUI state
    pub fn add_state(&mut self, state: GUIState) {
        let id = state.id.clone();
        let url = state.url.clone();
        
        // Index by URL
        self.url_index.entry(url).or_insert_with(Vec::new).push(id.clone());
        
        // Index elements
        for element in &state.elements {
            self.element_index
                .entry(element.role.clone())
                .or_insert_with(Vec::new)
                .push(element.id.clone());
        }
        
        self.states.insert(id, state);
    }

    /// Add a GUI action
    pub fn add_action(&mut self, action: GUIAction) {
        self.actions.insert(action.id.clone(), action);
    }

    /// Get state by ID
    pub fn get_state(&self, id: &str) -> Option<&GUIState> {
        self.states.get(id)
    }

    /// Get state by URL
    pub fn get_state_by_url(&self, url: &str) -> Option<&GUIState> {
        self.url_index
            .get(url)
            .and_then(|ids| ids.first())
            .and_then(|id| self.states.get(id))
    }

    /// Get actions from a state
    pub fn get_state_actions(&self, state_id: &str) -> Vec<&GUIAction> {
        self.actions
            .values()
            .filter(|a| a.source_state == state_id)
            .collect()
    }

    /// Get actions to a state
    pub fn get_target_actions(&self, state_id: &str) -> Vec<&GUIAction> {
        self.actions
            .values()
            .filter(|a| a.target_state == state_id)
            .collect()
    }

    /// Get element across all states
    pub fn get_element(&self, element_id: &str) -> Option<(StateId, &UIElement)> {
        for (state_id, state) in &self.states {
            if let Some(element) = state.get_element(element_id) {
                return Some((state_id.clone(), element));
            }
        }
        None
    }

    /// Integrate Terraphim RoleGraph
    pub fn integrate_rolegraph(&mut self, role_graph: terraphim_rolegraph::RoleGraph) {
        self.role_graph = Some(role_graph);
    }

    /// Get the Terraphim RoleGraph
    pub fn role_graph(&self) -> Option<&terraphim_rolegraph::RoleGraph> {
        self.role_graph.as_ref()
    }

    /// Add action template
    pub fn add_template(&mut self, template: ActionTemplate) {
        self.action_templates.push(template);
    }

    /// Find path between two states
    pub fn find_path(&self, start: &str, end: &str) -> Option<Vec<ActionId>> {
        // BFS path finding
        let mut queue = vec![(start.to_string(), Vec::new())];
        let mut visited = std::collections::HashSet::new();
        
        while let Some((current, path)) = queue.pop() {
            if current == end {
                return Some(path);
            }
            
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            
            for action in self.get_state_actions(&current) {
                let mut new_path = path.clone();
                new_path.push(action.id.clone());
                queue.push((action.target_state.clone(), new_path));
            }
        }
        
        None
    }

    /// Get statistics
    pub fn stats(&self) -> GraphStats {
        GraphStats {
            state_count: self.states.len(),
            action_count: self.actions.len(),
            template_count: self.action_templates.len(),
            avg_elements_per_state: if self.states.is_empty() {
                0.0
            } else {
                self.states.values()
                    .map(|s| s.elements.len())
                    .sum::<usize>() as f64 / self.states.len() as f64
            },
        }
    }
}

/// Graph statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub state_count: usize,
    pub action_count: usize,
    pub template_count: usize,
    pub avg_elements_per_state: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_state_creation() {
        let state = GUIState::new("https://example.com", "Example");
        assert_eq!(state.url, "https://example.com");
        assert_eq!(state.title, "Example");
    }

    #[test]
    fn test_ui_element() {
        let element = UIElement::new("button", "Submit");
        assert_eq!(element.role, "button");
        assert_eq!(element.name, "Submit");
    }

    #[test]
    fn test_gui_action() {
        let action = GUIAction::new("state1", "state2", ActionType::Click, "element1");
        assert_eq!(action.source_state, "state1");
        assert_eq!(action.action_type, ActionType::Click);
    }

    #[test]
    fn test_state_machine_graph() {
        let mut graph = StateMachineGraph::new("test");
        
        let mut state1 = GUIState::new("https://example.com/page1", "Page 1");
        state1.add_element(UIElement::new("button", "Next"));
        
        let state2 = GUIState::new("https://example.com/page2", "Page 2");
        
        graph.add_state(state1);
        graph.add_state(state2);
        
        let mut action = GUIAction::new("state1", "state2", ActionType::Click, "btn-next");
        action.selectors.css = Some("#next-button".to_string());
        graph.add_action(action);
        
        let stats = graph.stats();
        assert_eq!(stats.state_count, 2);
        assert_eq!(stats.action_count, 1);
    }
}
