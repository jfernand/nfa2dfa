use crate::ast::{Expr, Operation, UnaryOp};
use petgraph::dot::Dot;
use petgraph::prelude::NodeIndex;
use petgraph::{Directed, Graph};

pub struct Construction {
    pub graph: Graph<String, String, Directed>,
    start: NodeIndex,
    end: NodeIndex,
    level: usize,
}

const EPSILON: char = 'ε';

fn to_construction(expr: &Expr) -> Construction {
    match expr {
        Expr::Char(c) => construct_char(c.to_string()),
        Expr::BinaryOp { left, op, right } => match op {
            Operation::Or => compose_or(left, right),
            Operation::And => compose_and(left, right),
        },
        Expr::UnaryOp { op, operand } => match op {
            UnaryOp::Kleene => compose_kleene(operand),
            UnaryOp::Plus => compose_plus(operand),
        },
    }
}

fn compose_plus(operand: &Box<Expr>) -> Construction {
    let c = to_construction(operand);
    let mut g = c.graph;
    let label = usize_to_char(c.level);
    let q0 = g.add_node(format!("{}{}", label, 0));
    let q3 = g.add_node(format!("{}{}", label, 3));
    let q1 = c.start;
    let q2 = c.end;
    g.add_edge(q2, q1, EPSILON.to_string());
    g.add_edge(q0, q1, EPSILON.to_string());
    g.add_edge(q2, q3, EPSILON.to_string());

    if let Some(weight) = g.node_weight_mut(q1) {
        *weight = format!("{}{}", label, 1);
    }
    if let Some(weight) = g.node_weight_mut(q2) {
        *weight = format!("{}{}", label, 2);
    }

    Construction {
        graph: g,
        start: c.start,
        end: c.end,
        level: c.level + 1,
    }
}

fn compose_kleene(operand: &Box<Expr>) -> Construction {
    let c = to_construction(operand);
    let mut g = c.graph;
    let label = usize_to_char(c.level);
    let q0 = g.add_node(format!("{}{}", label, 0));
    let q3 = g.add_node(format!("{}{}", label, 3));
    let q1 = c.start;
    let q2 = c.end;
    g.add_edge(q2, q1, EPSILON.to_string());
    g.add_edge(q0, q3, EPSILON.to_string());
    g.add_edge(q0, q1, EPSILON.to_string());
    g.add_edge(q2, q3, EPSILON.to_string());

    if let Some(weight) = g.node_weight_mut(q1) {
        *weight = format!("{}{}", label, 1);
    }
    if let Some(weight) = g.node_weight_mut(q2) {
        *weight = format!("{}{}", label, 2);
    }

    Construction {
        graph: g,
        start: c.start,
        end: c.end,
        level: c.level + 1,
    }
}

fn compose_and<'a>(left: &Box<Expr>, right: &Box<Expr>) -> Construction {
    todo!()
}

fn compose_or<'a>(left: &Box<Expr>, right: &Box<Expr>) -> Construction {
    todo!()
    //     let left = to_construction(left);
    //     let right = to_construction(right);
    //     let mut left_graph = left.graph;
    //     let mut right_graph = right.graph;
    //     let n_left_nodes = left_graph.node_count();
    //     let q0 = g.add_node("q0".to_string());
    //     let q3 = g.add_node("q3".to_string());
    //     let q1 = c.start;
    //     let q2 = c.end;
    //     g.add_edge(q2, q1, EPSILON.to_string());
    //     g.add_edge(q0, q3, EPSILON.to_string());
    //     g.add_edge(q0, q1, EPSILON.to_string());
    //     g.add_edge(q2, q3, EPSILON.to_string());
    //
    //     if let Some(weight) = g.node_weight_mut(q1) {
    //         *weight = "q1".to_string();
    //     }
    //     if let Some(weight) = g.node_weight_mut(q2) {
    //         *weight = "q2".to_string();
    //     }
    //
    //     Construction{
    //         graph: g,
    //         start: c.start,
    //         end: c.end,
    //     }
}

// pub fn construct_and<'a>(a: String, b: String) -> Construction<'a> {
//     let mut g = Graph::<_, _, Directed>::new();
//     let start = g.add_node(&'S');
//     let end = g.add_node(&'E');
//     let mid = g.add_node(&'m');
//     g.add_edge(start, mid, a);
//     g.add_edge(mid, end, &b);
//     Construction {
//         graph: g,
//         start,
//         end,
//     }
// }

// pub fn construct_or<'a>(a: &'a char, b: &'a char) -> Construction<'a> {
//     let mut g = Graph::new();
//     let start = g.add_node(&'S');
//     let end = g.add_node(&'E');
//     let a_node = g.add_node(&'a');
//     let b_node = g.add_node(&'b');
//     g.add_edge(start, a_node, a);
//     g.add_edge(start, b_node, b);
//     g.add_edge(a_node, end, &EPSILON);
//     g.add_edge(b_node, end, &EPSILON);
//     Construction {
//         graph: g,
//         start,
//         end,
//     }
// }

pub fn construct_char(a: String) -> Construction {
    let mut g = Graph::new();
    let start = g.add_node("q0".to_string());
    let end = g.add_node("q1".to_string());
    g.add_edge(start, end, a);
    Construction {
        graph: g,
        start,
        end,
        level: 0,
    }
}

impl Construction {
    pub fn to_file(&self, file_path: &str) -> std::io::Result<()> {
        save_to_file(Dot::new(&self.graph).to_string().as_str(), file_path)
    }
}
fn save_to_file(content: &str, file_path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn usize_to_char(index: usize) -> char {
    (b'a' + (index as u8)) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char() {
        let expr = crate::ast::char('a');
        let c = to_construction(&expr).graph;
        let g6 = petgraph::graph6::get_graph6_representation(&c);
        assert_eq!(c.node_count(), 2, "The graph should have 2 nodes");
        assert_eq!(g6, "A_")
    }

    #[test]
    fn test_kleene() {
        let expr = crate::ast::star(crate::ast::char('a'));
        let c = to_construction(&expr);
        c.to_file("graphs/kleene.dot").unwrap();
        let g6 = petgraph::graph6::get_graph6_representation(&c.graph);
        println!("Kleene: {}", g6);
        assert_eq!(
            c.graph.node_count(),
            4,
            "The kleene graph should have 4 nodes"
        );
        assert_eq!(
            c.graph.edge_count(),
            5,
            "The kleene graph should have 5 edges"
        );
        assert_eq!(g6, "Cb")
    }

    #[test]
    fn test_plus() {
        let expr = crate::ast::plus(crate::ast::char('a'));
        let c = to_construction(&expr);
        c.to_file("graphs/plus.dot").unwrap();
        let g6 = petgraph::graph6::get_graph6_representation(&c.graph);
        println!("Plus: {}", g6);
        assert_eq!(
            c.graph.node_count(),
            4,
            "The plus graph should have 4 nodes"
        );
        assert_eq!(
            c.graph.edge_count(),
            4,
            "The plus graph should have 4 edges"
        );
        assert_eq!(g6, "Ca")
    }
}
