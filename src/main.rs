use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Write;

#[derive(Debug)]
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
struct State<'a> {
    /// Mapping from one word to the index of the index of the corresponding
    /// `State`
    transitions: BTreeMap<&'a str, usize>,
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

#[derive(Debug)]
struct ReverseAutomaton<'a> {
    states: Vec<ReverseState<'a>>,
    end_states: Vec<usize>,
}

#[derive(Debug, Clone)]
struct ReverseState<'a> {
    parent_states: Vec<(usize, &'a str)>,
}

impl<'a> ReverseAutomaton<'a> {
    fn from_automaton(automaton: &Automaton<'a>) -> Self {
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

    fn print_language(&self) {
        let mut paths: Vec<_> = self.end_states.iter().map(|i| (*i, String::new())).collect();
        while !paths.is_empty() {
            let mut idcs_to_remove = BTreeSet::new();
            for i in 0..paths.len() {
                let state_idx = paths[i].0;
                if state_idx == 0 {
                    idcs_to_remove.insert(i);
                }
                let mut iter = self.states[state_idx].parent_states.iter();
                let first = iter.next();
                for (parent, word) in iter {
                    fork_path(&mut paths, i, (*parent, word.to_string()));
                }
                if let Some(tail) = first {
                    let mut tail = (tail.0, tail.1.to_owned());
                    if i == 0 {
                        fork_path(&mut paths, i, tail.clone());
                    }
                    if !idcs_to_remove.contains(&i) {
                        tail.1.push_str(&paths[i].1);
                        paths[i] = tail;
                    }
                }
            }
            println!("{:?}", paths);
            println!("{:?}", idcs_to_remove);
            std::io::stdin().read_line(&mut String::new());
            for i in idcs_to_remove.iter().rev() {
                println!("{}", paths.swap_remove(*i).1);
            }
        }
    }
}

fn fork_path(
    paths: &mut Vec<(usize, String)>,
    path_idx: usize,
    mut new_tail: (usize, String),
) {
    let old_tail = &paths[path_idx];
    new_tail.1.push_str(&old_tail.1);
    paths.push(new_tail);
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

fn nums_to_dates<I>(nums: I, next_state: usize) -> impl Iterator<Item = (String, usize)>
where
    I: Iterator<Item = usize>,
{
    nums.map(|n| format!("{:02}.", n))
        .map(move |s| (s, next_state))
}

fn main() {
    let mut auto = Automaton::new();
    let mut s0_trans: Vec<_> = nums_to_dates(1..=28, 1).collect();
    s0_trans.extend(nums_to_dates(29..=30, 2));
    s0_trans.extend(nums_to_dates(31..=31, 3));
    auto.add_state(State::new(s0_trans.iter().map(|(s, i)| (&**s, *i)), false));
    let s1_trans: Vec<_> = nums_to_dates(1..=12, 4).collect();
    auto.add_state(State::new(s1_trans.iter().map(|(s, i)| (&**s, *i)), false));
    let s2_trans: Vec<_> = s1_trans.iter().filter(|(s, _)| s != "02.").collect();
    auto.add_state(State::new(s2_trans.iter().map(|(s, i)| (&**s, *i)), false));
    let mut s3_trans: Vec<_> = s1_trans.iter().take(7).step_by(2).collect();
    s3_trans.extend(s1_trans.iter().skip(7).step_by(2));
    auto.add_state(State::new(s3_trans.iter().map(|(s, i)| (&**s, *i)), false));
    auto.add_state(State::new(vec![("2006", 5)], false));
    auto.add_state(State::new(vec![], true));
    let rev_auto = ReverseAutomaton::from_automaton(&auto);
    //rev_auto.print_language();

    let mut auto = Automaton::new();
    auto.add_state(State::new(vec![("0", 1)], false));
    auto.add_state(State::new(vec![("1", 0)], true));
    let rev_auto = ReverseAutomaton::from_automaton(&auto);
    println!("{:?}", rev_auto);
    rev_auto.print_language();
    ///*
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let success = auto.run(&buf.trim());
        println!("success: {}", success);
    }
    //*/
}
