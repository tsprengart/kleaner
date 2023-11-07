use std::cmp::Ordering;
use jwalk::WalkDirGeneric;

fn check(path: String) {
    let walk_dir = WalkDirGeneric::<((usize), (bool))>::new(path); 
}
fn main() {
    check("/home/tsprengart/jobs".to_string());
}
