//! Структура содержит информацию о местоположении строк с ключевыми словами в файле

use std::collections::HashSet;

/// Структура с информацией о всех ключевых словах
#[derive(Debug)]
pub struct AnchorsInFile {
    filename: String,
    store: HashSet<SingleAnchorInFile>,
}

impl AnchorsInFile {
    /// Создание структуры AnchorsInFile
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            store: HashSet::new(),
        }
    }

    /// Добавление информации о строке с ключевым словом
    pub fn push(&mut self, name: &str, line_number: usize, kind: AnchorKind) {
        let anchor = SingleAnchorInFile {
            name: name.to_string(),
            line_number,
            kind,
        };
        self.store.insert(anchor);
    }

    /// Возвращает информацию о ключевых словах в файле
    pub fn info(self) -> crate::Result<Vec<(String, usize, usize)>> {
        let mut result = vec![];

        let mut names: HashSet<_> = HashSet::new();
        for a in self.store.iter() {
            names.insert(&a.name);
        }

        for anchor_name in names.iter() {
            let mut start = None;
            let mut end = None;
            for SingleAnchorInFile {
                name,
                line_number,
                kind,
            } in self.store.iter()
            {
                if name != anchor_name.as_str() {
                    continue;
                }
                match kind {
                    AnchorKind::Start => start = Some(line_number),
                    AnchorKind::End => end = Some(line_number),
                }
            }
            let start = check_start_anchor_exist(&self.filename, anchor_name, start)?;
            let end = check_end_anchor_exist(&self.filename, anchor_name, end)?;
            let start = start + 1;
            let end = end - 1;
            check_start_before_end(&self.filename, anchor_name, start, end)?;
            let result_line = (anchor_name.to_string(), start, end);
            result.push(result_line);
        }

        Ok(result)
    }
}

fn check_start_anchor_exist(
    file_name: &str,
    anchor_name: &str,
    start: Option<&usize>,
) -> crate::Result<usize> {
    let Some(start) = start else {
        let err = crate::Error::NoStartAnchor {
            file_name: file_name.to_string(),
            anchor_name: anchor_name.to_string(),
        };
        return Err(err);
    };
    Ok(*start)
}

fn check_end_anchor_exist(
    file_name: &str,
    anchor_name: &str,
    end: Option<&usize>,
) -> crate::Result<usize> {
    let Some(end) = end else {
        let err = crate::Error::NoEndAnchor {
            file_name: file_name.to_string(),
            anchor_name: anchor_name.to_string(),
        };
        return Err(err);
    };
    Ok(*end)
}

fn check_start_before_end(
    file_name: &str,
    anchor_name: &str,
    start: usize,
    end: usize,
) -> crate::Result<()> {
    if start > end {
        let err = crate::Error::EndAnchorBeforeStart {
            file_name: file_name.to_string(),
            anchor_name: anchor_name.to_string(),
        };
        return Err(err);
    }
    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct SingleAnchorInFile {
    pub name: String,
    pub line_number: usize,
    pub kind: AnchorKind,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AnchorKind {
    Start,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let mut file = AnchorsInFile::new("filename");

        file.push("example_1", 1, AnchorKind::Start);
        file.push("example_1", 10, AnchorKind::End);
        file.push("example_2", 3, AnchorKind::Start);
        file.push("example_2", 20, AnchorKind::End);

        let info = file.info().unwrap();
        assert_eq!(info.len(), 2);
        for i in info.iter() {
            match i.0.as_str() {
                "example_1" => assert_eq!(*i, ("example_1".to_string(), 2, 9)),
                "example_2" => assert_eq!(*i, ("example_2".to_string(), 4, 19)),
                _ => (),
            }
        }
    }
}
