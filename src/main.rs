use std::collections::HashMap;

struct Automaton<'a> {
    states: Vec<State<'a>>,
}

impl<'a> Automaton<'a> {
    pub fn new() -> Self {
        Automaton { states: vec![] }
    }

    pub fn add_state(&mut self, state: State<'a>) {
        self.states.push(state);
    }

    pub fn run(&self, mut input: &str) {
        let mut current_state = &self.states[0];
        while input.len() != 0 {
            for i in 0..input.len() {
                let word = &input[..=i];
                if let Some(state_index) = current_state.transitions.get(word) {
                    input = &input[word.len()..];
                    current_state = &self.states[*state_index];
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct State<'a> {
    /// Mapping from one word to the index of the index of the corresponding
    /// `State`
    transitions: HashMap<&'a str, usize>,
    end_state: bool,
}

impl<'a> State<'a> {
    fn new<T>(transitions: T, end_state: bool) -> Self
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

fn main() {
    let mut auto = Automaton::new();
    auto.add_state(State::new([("a", 0), ("b", 1)].iter().map(|v| *v), false));
    auto.add_state(State::new([("a", 1)].iter().map(|v| *v), true));
    auto.run("aaaaba");
}
