//! A module for using States in a Deterministic Finite Automata (DFA).
use std::collections::HashMap;

/// State
#[derive(Debug, PartialEq, Eq, Default)]
pub struct State {
    /// A map of all valid transitions from the state.
    transitions: HashMap<char, i32>,
}

impl State {
    /// Add the specified transition to this state.
    ///
    /// No validation is made that `new_state` exists until after
    /// all of the states have been added to the DFA.
    pub fn add_transition(&mut self, w: char, new_state: i32) {
        self.transitions.insert(w, new_state);
    }

    /// Return the transition from this state for the specified input.
    pub fn transition_for(&self, w: &char) -> Option<&i32> {
        self.transitions.get(w)
    }

    /// Return the number of transitions from this state.
    pub fn num_transitions(&self) -> usize {
        self.transitions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_state_initial() {
        let initial = State::default();

        assert_eq!(initial.num_transitions(), 0);
    }

    #[test]
    fn construct_state_with_transitions() {
        let mut initial = State::default();
        initial.add_transition('a', 1);
        initial.add_transition('b', 2);

        assert_eq!(initial.num_transitions(), 2);
    }
}
