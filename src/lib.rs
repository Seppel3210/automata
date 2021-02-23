use std::{collections::BTreeMap, convert::TryFrom, mem};
use std::str::FromStr;

#[derive(Debug)]
pub struct Automaton<'a> {
    states: Vec<State<'a>>,
}

impl<'a> Automaton<'a> {
    pub fn new() -> Self {
        Automaton { states: vec![] }
    }

    pub fn add_state(&mut self, state: State<'a>) {
        self.states.push(state);
    }

    pub fn run(&self, mut input: &str) -> bool {
        let mut current_state = &self.states[0];
        let mut current_state_index = 0;
        while input.len() != 0 {
            let mut matched = false;
            for i in 0..input.len() {
                let word = &input[..=i];
                if let Some(state_index) = current_state.transitions.get(word) {
                    input = &input[word.len()..];
                    current_state = &self.states[*state_index];
                    current_state_index = *state_index;
                    matched = true;
                    break;
                }
            }
            if !matched {
                eprintln!("No match for {:?} in state {}", input, current_state_index);
                return false;
            }
        }
        current_state.end_state
    }
}

#[derive(Debug)]
pub struct AutomatonParseError(usize);

impl<'a> TryFrom<&'a str> for Automaton<'a> {
    type Error = AutomatonParseError;
    fn try_from(s: &'a str) -> Result<Self, AutomatonParseError> {
        use AutomatonParseError as APE;
        let mut state_idx: Option<usize> = None;
        let mut transitions = vec![];
        let mut final_state = false;
        let mut states = vec![];

        for (i, line) in s.lines().enumerate() {
            if line.contains(':') {
                // push old state
                if let Some(state_idx) = state_idx {
                    states.push((state_idx, State::new(mem::take(&mut transitions), final_state)));
                }
                // prepare for new one
                let mut words = line.split(':');
                let idx = words.next().ok_or(APE(i))?.trim().parse().map_err(|_| APE(i))?;
                state_idx = Some(idx);
                final_state = words.next().map_or(false, |s| s.trim() == "final");
            } else {
                let mut words = line.split("->");
                let key = match words.next() {
                    Some(key) => key.trim(),
                    None => continue,
                };
                let value = match words.next().map(|s| s.trim().parse()) {
                    Some(Ok(value)) => value,
                    _ => continue,
                };
                transitions.push((key, value));
            }
        }
        // push final state
        states.push((state_idx.ok_or(APE(0))?, State::new(transitions, final_state)));
        let mut this = Self::new();
        states.sort_by_key(|s| s.0);
        for (_, state) in states {
            this.add_state(state);
        }
        Ok(this)
    }
}


#[derive(Debug)]
pub struct State<'a> {
    /// Mapping from one word to the index of the index of the corresponding
    /// `State`
    transitions: BTreeMap<&'a str, usize>,
    end_state: bool,
}

impl<'a> State<'a> {
    pub fn new<T>(transitions: T, end_state: bool) -> Self
    where
        T: IntoIterator<Item = (&'a str, usize)>,
    {
        let transitions = transitions.into_iter().collect();
        State {
            transitions,
            end_state,
        }
    }
}

#[derive(Debug)]
pub struct ReverseAutomaton<'a> {
    states: Vec<ReverseState<'a>>,
    end_states: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct ReverseState<'a> {
    parent_states: Vec<(usize, &'a str)>,
}

impl<'a> ReverseAutomaton<'a> {
    pub fn from_automaton(automaton: &Automaton<'a>) -> Self {
        let mut states = vec![ReverseState::new(); automaton.states.len()];
        let mut end_states = vec![];
        for (state_idx, state) in automaton.states.iter().enumerate() {
            if state.end_state {
                end_states.push(state_idx);
            }
            for (word, &child) in &state.transitions {
                states[child].add_parent(state_idx, word);
            }
        }

        ReverseAutomaton { states, end_states }
    }

    pub fn print_language(&self) {
        let mut paths: Vec<_> = self
            .end_states
            .iter()
            .map(|i| (*i, String::new()))
            .collect();
        while !paths.is_empty() {
            paths = paths
                .into_iter()
                .flat_map(|(state_idx, words)| {
                    if state_idx == 0 {
                        println!("{}", words);
                    }
                    self.states[state_idx]
                        .parent_states
                        .iter()
                        .map( move |(parent_idx, word)| {
                            let mut word = (*word).to_owned();
                            word.push_str(&words);
                            (*parent_idx, word)
                        })
                })
                .collect();
        }
    }
}

impl<'a> ReverseState<'a> {
    fn new() -> Self {
        ReverseState {
            parent_states: vec![],
        }
    }

    fn add_parent(&mut self, state_idx: usize, word: &'a str) {
        self.parent_states.push((state_idx, word));
    }
}
