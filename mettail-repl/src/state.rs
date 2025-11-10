use crate::theory::{AscentResults, Term, Theory};
use anyhow::Result;

/// The current state of the REPL session
pub struct ReplState {
    /// The name of the currently loaded theory
    theory_name: Option<String>,
    
    /// The current term being explored
    current_term: Option<Box<dyn Term>>,
    
    /// The ID of the current term in the rewrite graph (may differ from term.term_id())
    current_graph_id: Option<u64>,
    
    /// Navigation history
    history: Vec<HistoryEntry>,
    
    /// Current position in history
    history_idx: usize,
    
    /// Cached Ascent results
    ascent_results: Option<AscentResults>,
}

/// An entry in the navigation history
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub term_id: u64,
    pub display: String,
    pub rewrite_applied: Option<String>,
}

impl ReplState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self {
            theory_name: None,
            current_term: None,
            current_graph_id: None,
            history: Vec::new(),
            history_idx: 0,
            ascent_results: None,
        }
    }
    
    /// Load a theory by name
    pub fn load_theory(&mut self, name: String) {
        self.theory_name = Some(name);
        self.current_term = None;
        self.current_graph_id = None;
        self.history.clear();
        self.history_idx = 0;
        self.ascent_results = None;
    }
    
    /// Get the name of the current theory
    pub fn theory_name(&self) -> Option<&str> {
        self.theory_name.as_deref()
    }
    
    /// Set the current term (without running Ascent - that's done externally now)
    pub fn set_term(&mut self, term: Box<dyn Term>, results: AscentResults) -> Result<()> {
        let graph_id = term.term_id();
        self.set_term_with_id(term, results, graph_id)
    }
    
    /// Set the current term with an explicit graph ID
    pub fn set_term_with_id(&mut self, term: Box<dyn Term>, results: AscentResults, graph_id: u64) -> Result<()> {
        // Update state
        self.current_term = Some(term.clone_box());
        self.current_graph_id = Some(graph_id);
        self.ascent_results = Some(results);
        
        // Add to history
        let entry = HistoryEntry {
            term_id: graph_id,
            display: format!("{}", term),
            rewrite_applied: None,
        };
        self.history.push(entry);
        self.history_idx = self.history.len() - 1;
        
        Ok(())
    }
    
    /// Get the current term's ID in the rewrite graph
    pub fn current_graph_id(&self) -> Option<u64> {
        self.current_graph_id
    }
    
    /// Get the current term
    pub fn current_term(&self) -> Option<&dyn Term> {
        self.current_term.as_ref().map(|b| b.as_ref())
    }
    
    /// Get the Ascent results
    pub fn ascent_results(&self) -> Option<&AscentResults> {
        self.ascent_results.as_ref()
    }
    
    /// Get the history
    pub fn history(&self) -> &[HistoryEntry] {
        &self.history
    }
    
    /// Get the current history index
    pub fn history_index(&self) -> usize {
        self.history_idx
    }
    
    /// Navigate back in history
    pub fn go_back(&mut self) -> Option<&HistoryEntry> {
        if self.history_idx > 0 {
            self.history_idx -= 1;
            Some(&self.history[self.history_idx])
        } else {
            None
        }
    }
    
    /// Navigate forward in history
    pub fn go_forward(&mut self) -> Option<&HistoryEntry> {
        if self.history_idx + 1 < self.history.len() {
            self.history_idx += 1;
            Some(&self.history[self.history_idx])
        } else {
            None
        }
    }
    
    /// Jump to a specific history entry
    pub fn goto(&mut self, idx: usize) -> Option<&HistoryEntry> {
        if idx < self.history.len() {
            self.history_idx = idx;
            Some(&self.history[self.history_idx])
        } else {
            None
        }
    }
    
    /// Clear the history
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_idx = 0;
    }
}

impl Default for ReplState {
    fn default() -> Self {
        Self::new()
    }
}

