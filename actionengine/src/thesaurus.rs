//! UI Thesaurus - Semantic mapping for UI elements
//!
//! Provides synonym and semantic relationship mapping for UI elements,
//! extending Terraphim's thesaurus for GUI automation.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// UI Thesaurus - maps semantic concepts to UI element terminology
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UIThesaurus {
    /// Action synonyms: "submit" ↔ "save" ↔ "confirm" ↔ "apply"
    pub action_synonyms: HashMap<String, Vec<String>>,
    /// Element synonyms: "button" ↔ "clickable" ↔ "control"
    pub element_synonyms: HashMap<String, Vec<String>>,
    /// Field synonyms: "email" ↔ "mail" ↔ "e-mail"
    pub field_synonyms: HashMap<String, Vec<String>>,
    /// Intent to action mapping
    pub intent_actions: HashMap<String, Vec<String>>,
    /// Standard UI patterns
    pub patterns: Vec<UIPattern>,
}

impl UIThesaurus {
    /// Create a new UI thesaurus with default mappings
    pub fn new() -> Self {
        let mut thesaurus = Self::default();
        thesaurus.load_defaults();
        thesaurus
    }

    /// Load default UI element mappings
    fn load_defaults(&mut self) {
        // Action synonyms
        self.add_action_synonym("submit", vec!["save", "confirm", "apply", "done", "ok"]);
        self.add_action_synonym("cancel", vec!["close", "back", "dismiss", "abort"]);
        self.add_action_synonym("delete", vec!["remove", "trash", "destroy", "clear"]);
        self.add_action_synonym("edit", vec!["modify", "change", "update", "alter"]);
        self.add_action_synonym("search", vec!["find", "lookup", "query", "filter"]);
        self.add_action_synonym("login", vec!["sign in", "log in", "authenticate", "sign on"]);
        self.add_action_synonym("logout", vec!["sign out", "log off", "sign off", "exit"]);
        self.add_action_synonym("upload", vec!["upload", "import", "add file", "attach"]);
        self.add_action_synonym("download", vec!["export", "save as", "get", "retrieve"]);
        self.add_action_synonym("next", vec!["proceed", "continue", "forward", "advance"]);
        self.add_action_synonym("previous", vec!["back", "prior", "last", "return"]);
        self.add_action_synonym("refresh", vec!["reload", "update", "sync", "renew"]);

        // Element synonyms
        self.add_element_synonym("button", vec!["clickable", "control", "action", "widget"]);
        self.add_element_synonym("input", vec!["field", "textbox", "entry", "control"]);
        self.add_element_synonym("link", vec!["anchor", "hyperlink", "navigation"]);
        self.add_element_synonym("checkbox", vec!["check", "toggle", "option"]);
        self.add_element_synonym("dropdown", vec!["select", "combobox", "picker", "menu"]);
        self.add_element_synonym("modal", vec!["dialog", "popup", "overlay", "alert"]);
        self.add_element_synonym("menu", vec!["nav", "navigation", "sidebar", "drawer"]);
        self.add_element_synonym("table", vec!["grid", "data", "list", "rows"]);
        self.add_element_synonym("form", vec!["fields", "inputs", "entry"]);

        // Field synonyms
        self.add_field_synonym("email", vec!["mail", "e-mail", "email-address", "contact"]);
        self.add_field_synonym("password", vec!["pass", "pwd", "secret"]);
        self.add_field_synonym("username", vec!["user", "login", "account", "name"]);
        self.add_field_synonym("phone", vec!["telephone", "mobile", "cell", "contact-number"]);
        self.add_field_synonym("address", vec!["location", "street", "postal", "zip"]);
        self.add_field_synonym("name", vec!["full-name", "fullname", "first-last"]);
        self.add_field_synonym("search", vec!["query", "keywords", "find", "filter"]);
        self.add_field_synonym("date", vec!["when", "day", "time", "timestamp"]);
        self.add_field_synonym("price", vec!["cost", "amount", "total", "fee", "charge"]);
        self.add_field_synonym("description", vec!["details", "info", "about", "notes", "comment"]);
        self.add_field_synonym("title", vec!["heading", "header", "label", "name"]);

        // Intent to actions mapping
        self.add_intent_action("book", vec!["click_reserve", "click_book", "submit_booking"]);
        self.add_intent_action("buy", vec!["click_add_to_cart", "click_buy", "submit_order"]);
        self.add_intent_action("contact", vec!["type_message", "submit_contact", "click_send"]);
        self.add_intent_action("login", vec!["type_username", "type_password", "click_login"]);
        self.add_intent_action("register", vec!["type_email", "type_password", "click_signup"]);
        self.add_intent_action("search", vec!["type_search", "click_search", "press_enter"]);
        self.add_intent_action("upload", vec!["click_upload", "drag_file", "select_file"]);
        self.add_intent_action("download", vec!["click_download", "click_export"]);
        self.add_intent_action("subscribe", vec!["type_email", "click_subscribe", "check_optin"]);
        self.add_intent_action("filter", vec!["select_filter", "click_apply", "type_filter"]);

        // Common UI patterns
        self.patterns = vec![
            UIPattern {
                id: Uuid::new_v4().to_string(),
                name: "login_form".to_string(),
                description: "Standard login form with username and password".to_string(),
                elements: vec!["username_input".to_string(), "password_input".to_string(), "login_button".to_string()],
                sequence: Some(vec!["type_username".to_string(), "type_password".to_string(), "click_login".to_string()]),
            },
            UIPattern {
                id: Uuid::new_v4().to_string(),
                name: "search_bar".to_string(),
                description: "Search input with search button".to_string(),
                elements: vec!["search_input".to_string(), "search_button".to_string()],
                sequence: Some(vec!["type_search".to_string(), "click_search".to_string()]),
            },
            UIPattern {
                id: Uuid::new_v4().to_string(),
                name: "contact_form".to_string(),
                description: "Contact form with fields and submit".to_string(),
                elements: vec!["name_input".to_string(), "email_input".to_string(), "message_textarea".to_string(), "submit_button".to_string()],
                sequence: Some(vec!["type_name".to_string(), "type_email".to_string(), "type_message".to_string(), "click_submit".to_string()]),
            },
            UIPattern {
                id: Uuid::new_v4().to_string(),
                name: "modal_confirmation".to_string(),
                description: "Confirmation dialog with OK/Cancel".to_string(),
                elements: vec!["modal".to_string(), "ok_button".to_string(), "cancel_button".to_string()],
                sequence: Some(vec!["click_ok".to_string(), "click_cancel".to_string()]),
            },
            UIPattern {
                id: Uuid::new_v4().to_string(),
                name: "pagination".to_string(),
                description: "Page navigation with prev/next".to_string(),
                elements: vec!["prev_button".to_string(), "next_button".to_string(), "page_numbers".to_string()],
                sequence: Some(vec!["click_next".to_string(), "click_prev".to_string()]),
            },
            UIPattern {
                id: Uuid::new_v4().to_string(),
                name: "filter_sort".to_string(),
                description: "Filter and sort controls".to_string(),
                elements: vec!["filter_dropdown".to_string(), "sort_dropdown".to_string(), "apply_button".to_string()],
                sequence: Some(vec!["select_filter".to_string(), "select_sort".to_string(), "click_apply".to_string()]),
            },
        ];
    }

    /// Add action synonym group
    pub fn add_action_synonym(&mut self, canonical: &str, synonyms: Vec<&str>) {
        let mut all = vec![canonical.to_string()];
        all.extend(synonyms.into_iter().map(|s| s.to_string()));
        self.action_synonyms.insert(canonical.to_string(), all);
    }

    /// Add element synonym group
    pub fn add_element_synonym(&mut self, canonical: &str, synonyms: Vec<&str>) {
        let mut all = vec![canonical.to_string()];
        all.extend(synonyms.into_iter().map(|s| s.to_string()));
        self.element_synonyms.insert(canonical.to_string(), all);
    }

    /// Add field synonym group
    pub fn add_field_synonym(&mut self, canonical: &str, synonyms: Vec<&str>) {
        let mut all = vec![canonical.to_string()];
        all.extend(synonyms.into_iter().map(|s| s.to_string()));
        self.field_synonyms.insert(canonical.to_string(), all);
    }

    /// Add intent to action mapping
    pub fn add_intent_action(&mut self, intent: &str, actions: Vec<&str>) {
        self.intent_actions.insert(
            intent.to_string(),
            actions.into_iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Get canonical form for an action term
    pub fn get_canonical_action(&self, term: &str) -> Option<String> {
        let term_lower = term.to_lowercase();
        
        for (canonical, synonyms) in &self.action_synonyms {
            if synonyms.iter().any(|s| s.to_lowercase() == term_lower) {
                return Some(canonical.clone());
            }
        }
        None
    }

    /// Get canonical form for an element term
    pub fn get_canonical_element(&self, term: &str) -> Option<String> {
        let term_lower = term.to_lowercase();
        
        for (canonical, synonyms) in &self.element_synonyms {
            if synonyms.iter().any(|s| s.to_lowercase() == term_lower) {
                return Some(canonical.clone());
            }
        }
        None
    }

    /// Get canonical form for a field term
    pub fn get_canonical_field(&self, term: &str) -> Option<String> {
        let term_lower = term.to_lowercase();
        
        for (canonical, synonyms) in &self.field_synonyms {
            if synonyms.iter().any(|s| s.to_lowercase() == term_lower) {
                return Some(canonical.clone());
            }
        }
        None
    }

    /// Get all synonyms for a term
    pub fn get_synonyms(&self, term: &str) -> Vec<String> {
        let term_lower = term.to_lowercase();
        
        // Check action synonyms
        for synonyms in self.action_synonyms.values() {
            if synonyms.iter().any(|s| s.to_lowercase() == term_lower) {
                return synonyms.clone();
            }
        }
        
        // Check element synonyms
        for synonyms in self.element_synonyms.values() {
            if synonyms.iter().any(|s| s.to_lowercase() == term_lower) {
                return synonyms.clone();
            }
        }
        
        // Check field synonyms
        for synonyms in self.field_synonyms.values() {
            if synonyms.iter().any(|s| s.to_lowercase() == term_lower) {
                return synonyms.clone();
            }
        }
        
        vec![term.to_string()]
    }

    /// Get actions for an intent
    pub fn get_actions_for_intent(&self, intent: &str) -> Vec<String> {
        self.intent_actions
            .get(&intent.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }

    /// Find matching pattern
    pub fn find_pattern(&self, elements: &[String]) -> Option<&UIPattern> {
        for pattern in &self.patterns {
            let match_count = pattern.elements.iter()
                .filter(|e| elements.contains(e))
                .count();
            
            if match_count >= pattern.elements.len() / 2 {
                return Some(pattern);
            }
        }
        None
    }

    /// Normalize a UI element description
    pub fn normalize_element(&self, description: &str) -> String {
        let desc_lower = description.to_lowercase();
        
        // Try to find canonical form
        if let Some(canonical) = self.get_canonical_element(&desc_lower) {
            return canonical;
        }
        
        if let Some(canonical) = self.get_canonical_field(&desc_lower) {
            return canonical;
        }
        
        description.to_string()
    }

    /// Normalize an action description
    pub fn normalize_action(&self, action: &str) -> String {
        if let Some(canonical) = self.get_canonical_action(action) {
            return canonical;
        }
        action.to_string()
    }

    /// Build semantic query from natural language
    pub fn build_semantic_query(&self, query: &str) -> SemanticQuery {
        let query_lower = query.to_lowercase();
        let words: Vec<&str> = query_lower.split_whitespace().collect();
        let mut actions = HashSet::new();
        let mut elements = HashSet::new();
        let mut fields = HashSet::new();
        
        for word in &words {
            if let Some(canonical) = self.get_canonical_action(word) {
                actions.insert(canonical);
            }
            if let Some(canonical) = self.get_canonical_element(word) {
                elements.insert(canonical);
            }
            if let Some(canonical) = self.get_canonical_field(word) {
                fields.insert(canonical);
            }
        }
        
        SemanticQuery {
            original: query.to_string(),
            actions: actions.into_iter().collect(),
            elements: elements.into_iter().collect(),
            fields: fields.into_iter().collect(),
        }
    }
}

/// UI Pattern - common UI composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub elements: Vec<String>,
    pub sequence: Option<Vec<String>>,
}

/// Semantic query built from natural language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticQuery {
    pub original: String,
    pub actions: Vec<String>,
    pub elements: Vec<String>,
    pub fields: Vec<String>,
}

impl SemanticQuery {
    /// Check if query contains specific action
    pub fn has_action(&self, action: &str) -> bool {
        self.actions.iter().any(|a| a == action)
    }

    /// Check if query contains specific element
    pub fn has_element(&self, element: &str) -> bool {
        self.elements.iter().any(|e| e == element)
    }

    /// Check if query contains specific field
    pub fn has_field(&self, field: &str) -> bool {
        self.fields.iter().any(|f| f == field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thesaurus_creation() {
        let thesaurus = UIThesaurus::new();
        assert!(!thesaurus.action_synonyms.is_empty());
        assert!(!thesaurus.element_synonyms.is_empty());
    }

    #[test]
    fn test_action_normalization() {
        let thesaurus = UIThesaurus::new();
        
        assert_eq!(thesaurus.normalize_action("submit"), "submit");
        assert_eq!(thesaurus.normalize_action("save"), "submit");
        assert_eq!(thesaurus.normalize_action("confirm"), "submit");
    }

    #[test]
    fn test_element_normalization() {
        let thesaurus = UIThesaurus::new();
        
        assert_eq!(thesaurus.normalize_element("button"), "button");
        assert_eq!(thesaurus.normalize_element("clickable"), "button");
    }

    #[test]
    fn test_synonyms() {
        let thesaurus = UIThesaurus::new();
        
        let synonyms = thesaurus.get_synonyms("save");
        assert!(synonyms.contains(&"submit".to_string()));
        assert!(synonyms.contains(&"save".to_string()));
    }

    #[test]
    fn test_semantic_query() {
        let thesaurus = UIThesaurus::new();
        
        let query = thesaurus.build_semantic_query("login to my account");
        assert!(query.has_action("login"));
        assert!(query.has_element("input") || query.has_element("button"));
    }

    #[test]
    fn test_pattern_matching() {
        let thesaurus = UIThesaurus::new();
        
        let elements = vec![
            "username_input".to_string(),
            "password_input".to_string(),
            "login_button".to_string(),
        ];
        
        let pattern = thesaurus.find_pattern(&elements);
        assert!(pattern.is_some());
        assert_eq!(pattern.unwrap().name, "login_form");
    }
}
