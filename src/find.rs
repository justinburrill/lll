use crate::paths::*;

pub fn match_string(q: &str, dir: Directory) -> Vec<FilePath> {
    let mut results: Vec<FilePath> = vec![];
    for fp in dir.children {
        if fp.get_item_name().contains(q) {
            results.push(fp);
        }
    }
    results
}
