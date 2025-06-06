// src/parser.rs
//! Grammar excerpt we rely on:
//! ```pest
//! diagram      = { SOI ~ "classDiagram" ~ NEWLINE+ ~ statement ~ EOI }
//! statement    = { ((class | relation_stmt | member_stmt | comment) ~ NEWLINE*)* }
//! class        = { "class" ~ class_identifier }
//! class_identifier = _{ ASCII_ALPHA ~ ( ("_"|"-")? ASCII_ALPHANUMERIC )* }
//! ```
//! ⇒ the *only* rule we care about is **`class`**; it contains exactly one
//! `class_identifier` child.
//!
//! # Public contract
//! `parse(&str) -> Result<Diagram>` still holds; downstream callers don’t need
//! to change.

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use crate::types::{Class, Diagram, Namespace, DEFAULT_NAMESPACE};

#[derive(Parser)]
#[grammar = "grammar/mermaid.pest"] // kept in sync with user’s grammar
struct MermaidParser;

/// Error surface for the parsing layer.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("pest: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
}

/// Parse a Mermaid _classDiagram_ snippet and return an in‑memory `Diagram`
/// filled with *empty* [`Class`] nodes (no members yet).
pub fn parse(src: &str) -> Result<Diagram, ParseError> {
    let mut diagram = Diagram::default();
    let pairs = MermaidParser::parse(Rule::diagram, src)?;

    for pair in pairs { // the single `diagram` pair
        walk(pair, &mut diagram)?;
    }
    Ok(diagram)
}

// ─────────────────────────────────────── internals ───────────────────────────

/// Depth‑first walk over the Pest tree; invoke `collect_class` when we hit a
/// `class` rule, otherwise recurse.
fn walk(pair: Pair<Rule>, diagram: &mut Diagram) -> Result<(), ParseError> {
    match pair.as_rule() {
        Rule::class => collect_class(pair, diagram)?,
        _ => {
            for inner in pair.into_inner() {
                walk(inner, diagram)?;
            }
        }
    }
    Ok(())
}

/// Insert a single `Class` with **no members** into the `diagram`.
fn collect_class(pair: Pair<Rule>, diagram: &mut Diagram) -> Result<(), ParseError> {
    // `class` ::= "class" ~ class_identifier
    let mut id: Option<String> = None;
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::class_identifier {
            id = Some(inner.as_str().trim().to_owned());
            break;
        }
    }
    let fq_name = match id {
        Some(n) => n,
        None => return Ok(()), // malformed class; ignore in this slice
    };

    let (namespace, simple_name) = split_namespace(&fq_name);
    let class = Class {
        name: fq_name.clone(),
        generic: None,
        annotations: vec![],
        members: Vec::new(),
        namespace: namespace.to_owned(),
    };

    diagram
        .namespaces
        .entry(namespace.to_owned())
        .or_insert_with(|| Namespace {
            name: namespace.to_owned(),
            ..Default::default()
        })
        .classes
        .insert(simple_name.to_owned(), class);

    Ok(())
}

/// Split `module::Foo` into `(module, Foo)` – future‑proof although current
/// grammar only yields simple identifiers.
fn split_namespace(fq_name: &str) -> (&str, &str) {
    match fq_name.rfind("::") {
        Some(idx) => (&fq_name[..idx], &fq_name[idx + 2..]),
        None => (DEFAULT_NAMESPACE, fq_name),
    }
}
