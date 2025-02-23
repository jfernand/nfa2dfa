use nfa2dfa::ast::{and, char, or, plus, star};
use nfa2dfa::graph::{construct_and, construct_char, construct_or};
use petgraph::graph6::get_graph6_representation;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let expr = star(or(and(char('a'), char('b')), char('b')));
    println!("{}", &expr.trim());
    let state = 1;
    &expr.walk(&mut |e: &i32| e.clone(), &state);

    let expr = plus(and(char('a'), char('b')));
    println!("{}", &expr.trim());
    &expr.walk(&mut |e: &i32| e.clone(), &state);

    let g = construct_char("a".to_string());
    // println!("{:?}", Dot::new(&g.graph));
    g.to_file("graphs/a.dot").unwrap();
    println!("{:?}", get_graph6_representation(&g.graph));

    // let g = construct_and(&'a', &'b');
    // // println!("{:?}", Dot::new(&g.graph));
    // g.to_file("graphs/a_and_b.dot").unwrap();
    // println!("{:?}", get_graph6_representation(&g.graph));

    // let g = construct_or(&'a', &'b');
    // // println!("{:?}", Dot::new(&g.graph));
    // g.to_file("graphs/a_or_b.dot").unwrap();
    // println!("{:?}", get_graph6_representation(&g.graph));
}
