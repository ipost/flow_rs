#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::io::*;
use std::path::PathBuf;

mod attr;
mod dot_writer;
mod edge;
mod node;
mod params;

use dot_writer::DotWriter;
use edge::Edge;
use node::Node;
use params::parameters;

#[derive(Parser)]
#[grammar = "def.pest"]
struct IdentParser;

/*
 * ideas
 * =====================================
 * case statement
 * render implicit exits on false
 */

fn main() {
    let parameters = parameters();
    let print_ast = parameters.print_ast;
    let input = get_input_string(parameters.in_file);

    let pairs = IdentParser::parse(Rule::all, &input).unwrap_or_else(|e| panic!("{}", e));

    if print_ast {
        for pair in pairs.clone() {
            print_pair(pair, 0);
        }
    }

    let dot = make_dot(pairs);

    let mut output = get_output_handle(parameters.out_file);
    output.write(&dot.as_bytes()).unwrap();
}

fn print_pair(pair: Pair<Rule>, depth: usize) {
    let indent = " ".repeat(depth);
    match pair.as_rule() {
        Rule::all => println!("all:"),
        Rule::step => println!("{}step:", indent),
        Rule::exit => println!("{}EXIT", indent),
        Rule::process => println!("{}process:", indent),
        Rule::if_branch => println!("{}if_branch:", indent),
        Rule::else_branch => println!("{}else_branch:", indent),
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
    dot.write_line("strict digraph {");
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
        dot.write_node(&exit_point.start_node.clone().terminal());
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
                        .terminal();
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
            match pairs.peek() {
                Some(pair) => match pair.as_rule() {
                    Rule::else_branch => {
                        let mut pairs = pair.into_inner();
                        let (process_dot, else_process_exits) = dotify_process(
                            pairs.next().unwrap(),
                            vec![condition_edge.clone().labelled("False".to_string())],
                            id_generator,
                        );
                        dot.write_line(&process_dot.consume());
                        process_exits.extend(else_process_exits);
                    }
                    _ => unreachable!(),
                },
                None => process_exits.push(condition_edge.labelled("False".to_string())),
            };
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

fn get_input_string(file_path: Option<PathBuf>) -> String {
    match file_path {
        Some(f) => {
            let mut file = match File::open(f) {
                Ok(f) => f,
                Err(_e) => panic!("cannot open file"),
            };
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => contents,
                Err(_e) => panic!("cannot read file"),
            }
        }
        None => {
            let mut input = String::new();
            match std::io::stdin().read_to_string(&mut input) {
                Ok(_) => input,
                Err(_e) => panic!("cannot read STDIN"),
            }
        }
    }
}

fn get_output_handle(file_path: Option<PathBuf>) -> Box<Write> {
    match file_path {
        Some(f) => {
            println!(
                "writing to {}",
                f.clone().into_os_string().to_string_lossy()
            );
            Box::new(File::create(f).unwrap())
        }
        None => {
            println!("writing to stdout");
            Box::new(stdout())
        }
    }
}
