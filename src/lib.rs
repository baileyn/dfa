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
use std::io;

mod state;
use state::State;

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
            let mut current_state = (&0, self.get_state(&0));

            for c in s.chars() {
                let next_state = current_state.1.transition_for(&c);

                match next_state {
                    Some(next_state) => current_state = (next_state, self.get_state(next_state)),
                    None => return false,
                }                
            }

            self.final_states.contains(current_state.0)
        }
    }

    /// Return the `State` with the specified `state_id`. 
    fn get_state(&self, state_id: &i32) -> &State {
        // We can unwrap here safely because it's ensured in
        // `DFABuilder#build` that all states referenced in
        // the transitions exist.
        self.states.get(state_id).unwrap()
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
            .filter(|s| !s.is_empty()).collect();

        if lines.len() == 0 {
            return Err(DFABuilderError::EmptyStream);
        }

        let mut lines = lines.iter();

        let line = lines.next();

        if let Some(line) = line {
            for final_state in line.split(' ').map(|a| a.parse::<i32>()) {
                if let Ok(state) = final_state {
                    builder.add_final_state(state);
                } else {
                    return Err(DFABuilderError::NonIntegralFinalState);
                }
            }
        }

        for line in lines {
            let (from_state, w, to_state) = parse_line(&line)?;

            // We can just insert the character here because by the definition
            // of a set, it cannot have duplicates.
            builder.alphabet.insert(w);

            let mut state = builder.states.entry(from_state).or_insert_with(State::default);
            
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
        if self.states.get(&0).is_none() {
            return None;
        }
        
        // Ensure there's at least one final state.
        if self.final_states.len() < 1 {
            return None;
        }
        
        // Ensure all states have a branch for each item in the alphabet.
        for (_state_id, state) in &self.states {
            for i in &self.alphabet {
                if state.transition_for(i).is_none() {
                    return None;
                }
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
        Err(DFABuilderError::MalformedLine("Expected 3 components in transition line"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfa_builder_from_string() {
        // DFA for (ab)*
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

    #[test]
    fn builder_without_final_state_fails() {
        let builder = DFABuilder::from(&mut io::Cursor::new(
            r#"
                0 a 1
                0 b 1
                1 a 0
                1 b 0
            "#,
        ));

        assert_eq!(Err(DFABuilderError::NonIntegralFinalState), builder);
    }

    #[test]
    fn builder_without_content_fails() {
        let builder = DFABuilder::from(&mut io::Cursor::new(""));

        assert_eq!(Err(DFABuilderError::EmptyStream), builder);
    }

    #[test]
    fn builder_handles_excess_whitespace() {
        // DFA for (ab)*
        let builder = DFABuilder::from(&mut io::Cursor::new(
            r#"
        0
        0     a 1
        0 b     2
        1    a    2
        1       b     0
        2      a   2
        2   b      2
        "#,
        ));

        assert_eq!(true, builder.is_ok());
    }
}
