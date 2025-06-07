#[cfg(test)]
mod tests {
    use mermaid_parser::types::{Member, Visibility, DEFAULT_NAMESPACE};
    #[test]
    fn parse_class_with_members() {
        let mermaid = include_str!("./mermaid/test.mmd");

        let diagram = mermaid_parser::parser::parse(mermaid).unwrap();
        let ns = diagram.namespaces.get(DEFAULT_NAMESPACE).unwrap();
        println!("{:?}", diagram);
    }
}
