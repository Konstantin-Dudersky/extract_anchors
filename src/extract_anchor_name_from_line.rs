pub fn extract_anchor_name_from_line(line: &str, anchor: &str) -> Option<String> {
    let split = line.split(anchor).collect::<Vec<&str>>();
    if split.len() != 2 {
        return None;
    }
    let name = split[1].trim();
    Some(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start() {
        let line = "// ANCHOR: inputs";
        let anchor_name = extract_anchor_name_from_line(line, "ANCHOR:");
        assert_eq!(anchor_name, Some("inputs".to_owned()));
    }

    #[test]
    fn end() {
        let line = "// ANCHOR_END: inputs";
        let anchor_name = extract_anchor_name_from_line(line, "ANCHOR_END:");
        assert_eq!(anchor_name, Some("inputs".to_owned()));
    }
}
