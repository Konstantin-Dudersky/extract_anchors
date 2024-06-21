pub fn extract_anchor_name_from_line(line: &str, anchor: &str) -> Option<String> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let line = "// ANCHOR: inputs";
        let anchor_name = extract_anchor_name_from_line(line, "ANCHOR");
        let Some(anchor_name) = anchor_name else {
            panic!("None!");
        };
        assert_eq!(anchor_name, "inputs");
    }
}
