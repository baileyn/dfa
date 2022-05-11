//! Construct a DFA from the contents of a file.
//!
//! # Example
//! ```
//! # use dfa::DFABuilder;
//! # use std::io;
//! // Try to build a DFA for (ab)*
//! let builder = DFABuilder::from(&mut io::Cursor::new(
//!     r#"
//!     0
//!     0 a 1
//!     0 b 2
//!     1 a 2
//!     1 b 0
//!     2 a 2
//!     2 b 2
//!     "#,
//! ));
//!
//! # assert_eq!(true, builder.is_ok());
//! let builder = builder.unwrap().build();
//! # assert_eq!(true, builder.is_some());
//! let dfa = builder.unwrap();
//!
//! assert_eq!(true, dfa.is_valid_string("abab"));
//! assert_eq!(false, dfa.is_valid_string("ababa"));
//! ```

#![deny(missing_docs)]

use std::collections::HashMap;
use std::collections::HashSet;

mod state;
use state::State;
use std::io;

#[cfg(test)]
mod tests;

/// DFA represents a Deterministic Finite Automata.
#[derive(Debug)]
pub struct DFA {
    /// A vector of all the states that are considered a final state.
    final_states: Vec<i32>,

    /// A vector of all of the states for this DFA.
    states: HashMap<i32, State>,
}

impl DFA {
    /// Return `true` if `s` is a valid string in the language represented
    /// by this `DFA`.
    pub fn is_valid_string<'a>(&self, s: &'a str) -> bool {
        // Check trivial case of the empty string.
        if s.is_empty() {
            self.final_states.contains(&0)
        } else {
            println!("Possible final states: {:?}", self.final_states);
            // Start at the initial state.
            let mut current_state = (&0, self.get_state(&0));
            let mut transition_count = 1;

            // Loop through the letters of the supplied string.
            for c in s.chars() {
                let next_state = current_state.1.transition_for(&c);

                // Check if the next state exists.
                match next_state {
                    // If it does, set it as the current state.
                    Some(next_state) => {
                        println!("Transition {}: {} {} {}", transition_count, current_state.0, c, next_state);
                        transition_count += 1;
                        current_state = (next_state, self.get_state(next_state))
                    },
                    // Otherwise, end the check.
                    None => return false,
                }
            }

            println!("{} {} a final state.", current_state.0, 
                if self.final_states.contains(current_state.0) {
                    "is"
                } else {
                    "is not"
                }
            );
            
            self.final_states.contains(current_state.0)
        }
    }

    /// Return the `State` with the specified `state_id`.
    fn get_state(&self, state_id: &i32) -> &State {
        // We can unwrap here safely because it's ensured in
        // `DFABuilder#build` that all states referenced in
        // the transitions exist.
        &self.states[state_id]
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
/// `DFABuilder` is used to create a DFA with validation.
pub struct DFABuilder {
    /// A vector of all the states that are considered a final state.
    final_states: Vec<i32>,

    /// A vector of all of the states for the `DFA`.
    states: HashMap<i32, State>,

    /// The vector for the alphabet the `DFA` will operate under.
    alphabet: HashSet<char>,
}

#[derive(Debug, PartialEq, Eq)]
/// Represents an error when parsing a DFA from file.
pub enum DFABuilderError {
    /// Represents a malformed line in the DFA file.
    MalformedLine(&'static str),
    /// Represents when the DFA encounters a non-integral final state.
    NonIntegralFinalState,
    /// Represents the DFA data was empty.
    EmptyStream,
    /// Represents when the DFA builder expects a single character but received more.
    ExpectedChar,
    /// Represents when the DFA expected an integer but was given something else.
    ExptectedInt,
}

impl From<std::num::ParseIntError> for DFABuilderError {
    fn from(_err: std::num::ParseIntError) -> DFABuilderError {
        DFABuilderError::ExptectedInt
    }
}

impl DFABuilder {
    /// Create a builder from the specified BufRead.
    pub fn from<T: io::BufRead>(read: T) -> Result<Self, DFABuilderError> {
        let mut builder = DFABuilder::default();
        let lines: Vec<String> = read.lines()
            .map(|r| r.unwrap())
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect();

        // If the content we're reading from is empty, return an error.
        if lines.is_empty() {
            return Err(DFABuilderError::EmptyStream);
        }

        let mut lines = lines.iter();

        // We can safely unwrap this value because at this point, the length of
        // lines is guaranteed to be at least 1.
        let line = lines.next().unwrap();

        // Split the line up by spaces and map them into integers.
        for final_state in line.split(' ').map(|a| a.parse::<i32>()) {
            if let Ok(state) = final_state {
                // If the value was successfully parsed into an integer, put it as a final state.
                builder.add_final_state(state);
            } else {
                // Otherwise, return an error because this line should _only_ be for final states.
                return Err(DFABuilderError::NonIntegralFinalState);
            }
        }

        // Proceed through the rest of the lines in the data.
        for line in lines {
            // Attempt to parse the data in the line to (from_state, w, to_state),
            // if this was not possible, error will be returned (because of ? syntax).
            let (from_state, w, to_state) = parse_line(line)?;

            // We can just insert the character here because by the definition
            // of a set, it cannot have duplicates.
            builder.alphabet.insert(w);

            // Get, or insert, the `from_state`.
            let mut state = builder
                .states
                .entry(from_state)
                .or_insert_with(State::default);

            // Add the transition to the state.
            state.add_transition(w, to_state);
        }

        Ok(builder)
    }

    /// Add the specified state id as a final state.
    ///
    /// `state_id` the ID of the state that is considered a final state.
    pub fn add_final_state(&mut self, state_id: i32) {
        self.final_states.push(state_id);
    }

    /// Return the states loaded for this DFA.
    pub fn states(&self) -> &HashMap<i32, State> {
        &self.states
    }

    /// Attempt to build the `DFA` specified in this `DFABuilder`.
    pub fn build(self) -> Option<DFA> {
        // Ensure there's an initial state.
        self.states.get(&0)?;

        // Ensure there's at least one final state.
        if self.final_states.len() < 1 {
            return None;
        }

        // Ensure all states have a branch for each item in the alphabet.
        for state in self.states.values() {
            for w in &self.alphabet {
                // Get the transition for the current item in the alphabet.
                let transition = state.transition_for(w)?;

                // Ensure that the state being transitioned to actually exists.
                self.states.get(transition)?;
            }
        }

        Some(DFA {
            states: self.states,
            final_states: self.final_states,
        })
    }
}

/// Parse the specified `line`.
///
/// Returns a tuple of the form: `(from_state_id, transition_letter, to_state_id)`
fn parse_line(line: &str) -> Result<(i32, char, i32), DFABuilderError> {
    let components: Vec<_> = line.split(' ').filter(|x| !x.trim().is_empty()).collect();

    if components.len() != 3 {
        // There's not 3 components, so the line is misformed.
        Err(DFABuilderError::MalformedLine(
            "Expected 3 components in transition line",
        ))
    } else {
        // Parse the state this transition is for.
        let from_state = components[0].parse::<i32>()?;
        let w = components[1];
        if w.len() > 1 {
            return Err(DFABuilderError::ExpectedChar);
        }
        let to_state = components[2].parse::<i32>()?;
        Ok((from_state, w.chars().next().unwrap(), to_state))
    }
}
