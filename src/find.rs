use crate::filepath::*;
use crate::directory::*;

pub fn match_string(q: &str, dir: Directory) -> Vec<FilePath> {
    let mut results: Vec<FilePath> = vec![];
    let children:Vec<Box<dyn Pathj>> = dir.subdirs.append(dir.subfiles);
    for fp in children {
        if fp.get_item_name().contains(q) {
            results.push(fp);
        }
    }
    results
}
