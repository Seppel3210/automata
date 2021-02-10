use std::collections::HashMap;

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
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let success = auto.run(&buf.trim());
        println!("success: {}", success);
    }
}
