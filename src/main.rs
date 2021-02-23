use automata::*;

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
    rev_auto.print_language();

    let mut auto = Automaton::new();
    auto.add_state(State::new(vec![("0", 1), ("1", 0)], false));
    auto.add_state(State::new(vec![("0", 1), ("1", 0)], true));
    let rev_auto = ReverseAutomaton::from_automaton(&auto);
    rev_auto.print_language();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let success = auto.run(&buf.trim());
        println!("success: {}", success);
    }
}
