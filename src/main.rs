use std::{convert::TryInto, fs};
use automata::*;

fn main() {
    let auto_file = fs::read_to_string("./automaton.txt").unwrap();
    let automaton: Automaton = (&*auto_file).try_into().unwrap();
    println!("{:?}", automaton);
    let rev_auto = ReverseAutomaton::from_automaton(&automaton);
    println!("{:?}", rev_auto);
    rev_auto.print_language();
}
