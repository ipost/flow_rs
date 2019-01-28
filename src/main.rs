extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
#[grammar = "def.pest"]
struct IdentParser;

#[derive(Copy, Clone)]
enum NodeShape {
    Rectangle,
    Diamond,
}

#[derive(Clone)]
struct Node {
    pub id: String,
    pub label: Option<String>,
    pub shape: Option<NodeShape>,
    pub color: Option<String>,
}

impl Node {
    pub fn new(id: String) -> Self {
        Node {
            id: id,
            label: None,
            shape: None,
            color: None,
        }
    }

    pub fn labelled(mut self, l: String) -> Self {
        self.label = Some(l);
        self
    }

    pub fn diamond(mut self) -> Self {
        self.shape = Some(NodeShape::Diamond);
        self
    }

    pub fn rectangle(mut self) -> Self {
        self.shape = Some(NodeShape::Rectangle);
        self
    }

    pub fn red(mut self) -> Self {
        self.color = Some("red".to_string());
        self
    }

    pub fn color(&self) -> String {
        match &self.color {
            Some(s) => s.clone(),
            None => "black".to_string(),
        }
    }

    pub fn shape(&self) -> NodeShape {
        match &self.shape {
            Some(s) => s.clone(),
            None => NodeShape::Rectangle,
        }
    }

    pub fn label_str(&self) -> &str {
        match &self.label {
            Some(l) => l,
            None => &"",
        }
    }
}

#[derive(Clone)]
struct Edge {
    pub label: Option<String>,
    pub start_node: Node,
}

impl Edge {
    pub fn starting_at(n: Node) -> Self {
        Edge {
            start_node: n,
            label: None,
        }
    }

    pub fn labelled(mut self, l: String) -> Self {
        self.label = Some(l);
        self
    }

    pub fn start_id(&self) -> &str {
        &self.start_node.id
    }
}

struct DotWriter {
    dot: String,
}

impl DotWriter {
    pub fn new() -> DotWriter {
        DotWriter {
            dot: "".to_string(),
        }
    }

    pub fn write_edge(&mut self, edge: &Edge, end: &Node) {
        let mut writeable = format!("{} -> {}", edge.start_id(), end.id);
        if let Some(label) = &edge.label {
            writeable.push_str(&format!(" [label=\"{}\"]", label))
        }
        writeable.push_str(";");
        self.write_line(&writeable);
    }

    pub fn write_node(&mut self, node: &Node) {
        let writeable = format!(
            "{} [label=\"{}\",shape={},color={}];",
            node.id,
            node.label_str(),
            match node.shape() {
                NodeShape::Rectangle => "box",
                NodeShape::Diamond => "diamond",
            },
            node.color(),
        );
        self.write_line(&writeable);
    }

    pub fn write_line(&mut self, line: &str) {
        self.dot.push_str(&format!("{}\n", line.trim()));
    }

    pub fn consume(self) -> String {
        self.dot
    }
}

const STUFF: &str = "
 first step;
 if we should exit early {
 exit;
 }
step two is
a two-line step;
if some condition {
  do a thing;
  if it's sunday {
    thinking emoji;
    if we want to exit {
        exit;
    }
  }
}
some more;
while something is happening {
  do that other thing;
  if it's sunday {
    more thinking emoji;
  }
  if it's saturday {
    NO thinking emoji;
    some sunglasses emoji;
  }
}
if some_condition {
    the last step;
}
";

/*
 * ideas
 * =====================================
 * escape characters in node labels
 * case statement
 */

fn main() {
    let print_ast = false;
    let pairs = IdentParser::parse(Rule::all, STUFF).unwrap_or_else(|e| panic!("{}", e));

    if print_ast {
        for pair in pairs.clone() {
            print_pair(pair, 0);
        }
    }

    let dot = make_dot(pairs);

    let mut file = File::create("testout.dot").unwrap();
    file.write(&dot.as_bytes()).unwrap();

    println!("dot:\n{}", dot);
}

fn print_pair(pair: Pair<Rule>, depth: usize) {
    let indent = " ".repeat(depth);
    match pair.as_rule() {
        Rule::all => println!("all:"),
        Rule::step => println!("{}step:", indent),
        Rule::exit => println!("{}EXIT", indent),
        Rule::process => println!("{}process:", indent),
        Rule::if_branch => println!("{}if_branch:", indent),
        Rule::while_loop => println!("{}while_loop:", indent),
        Rule::condition => println!("{}condition:", indent),
        Rule::expression => println!("{}expression: \"{}\"", indent, pair.as_str()),
        Rule::EOI => {}
        _ => unreachable!(),
    }

    for inner_pair in pair.into_inner() {
        print_pair(inner_pair, 2 + depth);
    }
}

fn make_dot(pairs: Pairs<Rule>) -> String {
    let mut name_gen = make_id_generator();
    let mut dot = DotWriter::new();
    let dotified_all = dotify_all(pairs, &mut name_gen);
    dot.write_line("digraph F {");
    dot.write_line(&dotified_all);
    dot.write_line("}");
    dot.consume()
}

fn dotify_all(mut pairs: Pairs<Rule>, mut id_generator: &mut Box<FnMut() -> String>) -> String {
    let pair = pairs.next().unwrap();
    let (mut dot, exit_points) = match pair.as_rule() {
        Rule::all => dotify_process(pair.into_inner().next().unwrap(), vec![], &mut id_generator),
        _ => unreachable!(),
    };
    for exit_point in exit_points.iter() {
        dot.write_node(&exit_point.start_node.clone().red());
    }
    dot.consume()
}

fn dotify_process(
    pair: Pair<Rule>,
    entry_points: Vec<Edge>,
    id_generator: &mut Box<FnMut() -> String>,
) -> (DotWriter, Vec<Edge>) {
    let mut dot = DotWriter::new();
    let exit_points = match pair.as_rule() {
        Rule::process => {
            let pairs = pair.into_inner();
            pairs.fold(entry_points, |entry_points, pair| {
                let (new_dot, exit_points) = match pair.as_rule() {
                    Rule::step => dotify_step(pair, entry_points, id_generator),
                    Rule::if_branch => dotify_if(pair, entry_points, id_generator),
                    Rule::while_loop => dotify_while(pair, entry_points, id_generator),
                    _ => unreachable!(),
                };
                dot.write_line(&new_dot.consume());
                exit_points
            })
        }
        _ => unreachable!(),
    };
    (dot, exit_points)
}

fn dotify_step(
    pair: Pair<Rule>,
    entry_points: Vec<Edge>,
    id_generator: &mut Box<FnMut() -> String>,
) -> (DotWriter, Vec<Edge>) {
    let mut dot = DotWriter::new();
    let exits = match pair.as_rule() {
        Rule::step => {
            let mut pairs = pair.into_inner();
            let pair = pairs.next().unwrap();
            let exit_points = match pair.as_rule() {
                Rule::expression => {
                    let exit_node = Node::new(id_generator())
                        .labelled(pair.as_str().to_string())
                        .rectangle();
                    dot.write_node(&exit_node);
                    for entry_point in entry_points.iter() {
                        dot.write_edge(entry_point, &exit_node);
                    }
                    vec![Edge::starting_at(exit_node)]
                }
                Rule::exit => {
                    let exit_node = Node::new(id_generator())
                        .labelled("Exit".to_string())
                        .rectangle()
                        .red();
                    dot.write_node(&exit_node);
                    for entry_point in entry_points.iter() {
                        dot.write_edge(entry_point, &exit_node);
                    }
                    vec![]
                }
                _ => unreachable!(),
            };
            exit_points
        }
        _ => unreachable!(),
    };
    (dot, exits)
}

fn dotify_if(
    pair: Pair<Rule>,
    entry_points: Vec<Edge>,
    id_generator: &mut Box<FnMut() -> String>,
) -> (DotWriter, Vec<Edge>) {
    let mut dot = DotWriter::new();
    let exits = match pair.as_rule() {
        Rule::if_branch => {
            let mut pairs = pair.into_inner();
            let (condition_dot, condition_edge) =
                dotify_condition(pairs.next().unwrap(), &entry_points, id_generator);
            dot.write_line(&condition_dot.consume());
            let (process_dot, mut process_exits) = dotify_process(
                pairs.next().unwrap(),
                vec![condition_edge.clone().labelled("True".to_string())],
                id_generator,
            );
            dot.write_line(&process_dot.consume());
            process_exits.push(condition_edge.labelled("False".to_string()));
            process_exits
        }
        _ => unreachable!(),
    };
    (dot, exits)
}

fn dotify_while(
    pair: Pair<Rule>,
    entry_points: Vec<Edge>,
    id_generator: &mut Box<FnMut() -> String>,
) -> (DotWriter, Vec<Edge>) {
    let mut dot = DotWriter::new();
    let exits = match pair.as_rule() {
        Rule::while_loop => {
            let mut pairs = pair.into_inner();
            let (condition_dot, condition_edge) =
                dotify_condition(pairs.next().unwrap(), &entry_points, id_generator);
            dot.write_line(&condition_dot.consume());
            let (process_dot, process_exits) = dotify_process(
                pairs.next().unwrap(),
                vec![condition_edge.clone().labelled("True".to_string())],
                id_generator,
            );
            dot.write_line(&process_dot.consume());
            for process_exit in process_exits {
                dot.write_edge(&process_exit, &condition_edge.start_node);
            }
            vec![condition_edge.labelled("False".to_string())]
        }
        _ => unreachable!(),
    };
    (dot, exits)
}

fn dotify_condition(
    pair: Pair<Rule>,
    entry_points: &Vec<Edge>,
    id_generator: &mut Box<FnMut() -> String>,
) -> (DotWriter, Edge) {
    let mut dot = DotWriter::new();
    let condition_name = match pair.as_rule() {
        Rule::condition => {
            let condition_node = Node::new(id_generator())
                .labelled(pair.as_str().to_string())
                .diamond();
            dot.write_node(&condition_node);
            for entry_point in entry_points {
                dot.write_edge(&entry_point, &condition_node);
            }
            condition_node
        }
        _ => unreachable!(),
    };
    (dot, Edge::starting_at(condition_name))
}

fn make_id_generator() -> Box<FnMut() -> String> {
    let mut suffix = 0;
    Box::new(move || {
        let name = format!("n{}", suffix);
        suffix += 1;
        name
    })
}
