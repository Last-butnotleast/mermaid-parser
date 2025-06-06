mod types;

use pest::Parser;
use pest_derive::Parser;
use crate::types::Diagram;

#[derive(Parser)]
#[grammar = "grammar/mermaid.pest"]
pub struct MermaidParser;

pub fn parse_diagram(input: &str) -> Result<Diagram, pest::error::Error<Rule>> {
    let res =  MermaidParser::parse(Rule::file, input)?; 
    println!("{:#?}", res);
    
    Ok(Diagram::default())
}