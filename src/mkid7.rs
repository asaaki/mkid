// mkid7 - Quick UUID v7 generator
// Shortcut for: mkid uuid v7 (or just: mkid)

use uuid::Uuid;

fn main() {
    println!("{}", Uuid::now_v7());
}
