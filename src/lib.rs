#[deny(missing_docs)]
use std::io;

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
    /// Create a new DFABuilder.
    pub fn new() -> Self {
        Self {
            final_states: Vec::new(),
            states: Vec::new(),
        }
    }

    /// Create a builder from the specified BufRead.
    pub fn from<T: io::BufRead>(read: T) -> io::Result<Self> {
        let builder = DFABuilder::new();
        let mut lines = read.lines()
            .map(|r| r.unwrap())
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_owned());

        let line = lines.next();

        if let Some(line) = line {
            for final_state in line.split(' ').map(|a| a.parse::<i32>()) {
                if let Ok(state) = final_state {
                    println!("Final State: {}", state);
                } else {
                    println!("Non integral state found");
                }
            }
        }

        Ok(builder)
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

    /// Return the states loaded for this DFA.
    pub fn states(&self) -> &[State] {
        &self.states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfa_builder_from_string() {
        // Try to build a DFA for (ab)*
        let builder = DFABuilder::from(&mut io::Cursor::new(
            r#"
        0
        0 a 1
        0 b 2
        1 a 2
        1 b 0
        2 a 2
        2 b 2
        "#,
        ));

        assert_eq!(true, builder.is_ok());
    }
}
