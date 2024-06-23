//! Уменьшает отступ всех строк в файле

pub fn unindent(file: &[&str]) -> String {
    // Находим минимальный отступ
    let min_indent = file.iter().map(|l| count_start_spaces(l)).min().unwrap();
    let mut new_file = vec![];
    for line in file {
        let new_line = line[min_indent..line.len()].to_string();
        new_file.push(new_line);
    }
    new_file.join("\n")
}

fn count_start_spaces(line: &str) -> usize {
    line.chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .map(|ch| ch.len_utf8())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file = vec![
            "            line1",
            "        line2",
            "    line3",
            "        line4",
        ];
        let new_file = unindent(&file);
        assert_eq!(new_file, "        line1\n    line2\nline3\n    line4")
    }
}
