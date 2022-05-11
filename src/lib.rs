#[deny(missing_docs)]

pub mod state;
use state::State;

/// DFA represents a Deterministic Finite Automata.
pub struct DFA {
    /// A vector of all the states that are considered a final state.
    final_states: Vec<i32>,

    /// A vector of all of the states for this DFA.
    states: Vec<State>,
}

pub struct DFABuilder {
    /// A vector of all the states that are considered a final state.
    final_states: Vec<i32>,

    /// A vector of all of the states for this DFA.
    states: Vec<State>,
}

impl DFABuilder {
    pub fn new() -> Self {
        Self {
            final_states: Vec::new(),
            states: Vec::new(),
        }
    }

    /// Add the specified state id as a final state.
    /// 
    /// `state_id` the ID of the state that is considered a final state.
    pub fn add_final_state(&mut self, state_id: i32) {
        self.final_states.push(state_id);
    }

    /// Add the specified state to the DFA.
    /// 
    /// `state` the state to add to the DFA.
    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
    }
}
