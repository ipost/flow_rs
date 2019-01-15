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
  if it's sunday {
    thinking emoji;
  }
  exit;
}
some more;
while something is happening {
  do that other thing;
}
";

fn main() {
    let pairs = IdentParser::parse(Rule::all, STUFF).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        print_pair(pair, 0);
    }
}

fn print_pair(pair: Pair<Rule>, depth: usize) {
    let indent = " ".repeat(depth);
    //println!("{}Rule: {:?}\n{}      {:?}",
    //         indent, pair.as_rule(), indent, pair.as_str());
    match pair.as_rule() {
        Rule::all => println!("all:"),
        Rule::step => println!("{}step:", indent),
        Rule::process => println!("{}process:", indent),
        Rule::if_branch => println!("{}if_branch:", indent),
        Rule::while_loop => println!("{}while_loop:", indent),
        Rule::condition => println!("{}condition:", indent),
        Rule::expression => println!("{}expression: \"{}\"", indent, pair.as_str()),
        Rule::EOI => {},
        _ => unreachable!(),
    }

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
