use crate::pathj::path::*;

// TODO: maybe i do need traits?
pub fn match_string(q: &str, dir: Path<Directory>) -> Vec<Path<T>> {}

// pub fn match_string(q: &str, dir: Directory) -> Vec<Box<dyn HasPath>> {
//     // Working example of two different objects stored in the same vec
//     // let d = Directory::new();
//     // let f = File::new();
//     // let v: Vec<Box<dyn HasPath>> = vec![Box::new(d), Box::new(f)];

//     let mut results: Vec<Box<dyn HasPath>> = vec![];
//     // let subfiles = &mut dir
//     //     .subfiles
//     //     .into_iter()
//     //     .map(|x| Box::new(x))
//     //     .collect::<Vec<Box<dyn HasPath>>>();
//     // let subdirs: &mut dyn Iterator<Item = Box<dyn HasPath>> =
//     //     &mut dir.subdirs.into_iter().map(|x| Box::new(x));
//     // // let children: &mut dyn Iterator<Item = Box<dyn HasPath>> = todo!();
//     // for path in subfiles.chain(subdirs) {
//     //     if path.get_item_name().contains(q) {
//     //         results.push(path);
//     //     }
//     // }

//     // newest version below
//     // let mut files = dir
//     //     .subfiles
//     //     .clone()
//     //     .into_iter()
//     //     .map(|x| Box::new(x))
//     //     .collect::<Vec<_>>();
//     // let mut dirs = dir.subdirs.clone();
//     // results.append(&mut files);

//     todo!();
//     // results
// }
