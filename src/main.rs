use std::path::Path;

mod lib;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let a_path = args.get(1);
    let b_path = args.get(2);

    match (a_path, b_path) {
        (Some(a), Some(b)) => lib::diff(Path::new(a), Path::new(b)),
        _ => println!("Usage: basic_diff <src_path> <dst_path>"),
    }
}
