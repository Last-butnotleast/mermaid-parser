use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/mermaid.pest"]
pub struct MermaidParser;

pub fn parse_diagram(input: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    MermaidParser::parse(Rule::file, input)
}