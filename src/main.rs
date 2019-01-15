extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "def.pest"]
struct IdentParser;

const STUFF: &str = "
 first step;
step two is
a two-line step;
if some condition {
  do a thing;
  exit;
}
do the rest;
";

fn main() {
    let pairs = IdentParser::parse(Rule::process, STUFF).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        print_pair(pair, 0);
    }
}

fn print_pair(pair: Pair<Rule>, depth: usize) {
    let span = pair.clone().into_span();
    println!("{}Rule: {:?}", " ".repeat(depth), pair.as_rule());
    println!("{}Span: {:?}", " ".repeat(depth), span);
    //println!("{}Text: {}", " ".repeat(depth), span.as_str());

    for inner_pair in pair.into_inner() {
        print_pair(inner_pair, 2 + depth);
        // let inner_span = inner_pair.clone().into_span();
        // match inner_pair.as_rule() {
        //     Rule::step => println!("step:       {}", inner_span.as_str()),
        //     Rule::process => println!("process:    {}", inner_span.as_str()),
        //     Rule::if_branch => println!("if_branch:  {}", inner_span.as_str()),
        //     Rule::expression => println!("expression: {}", inner_span.as_str()),
        //     _ => unreachable!(),
        // };
    }
}
