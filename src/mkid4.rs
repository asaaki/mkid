// mkid4 - Quick UUID v4 generator
// Shortcut for: mkid uuid v4

use uuid::Uuid;

fn main() {
    println!("{}", Uuid::new_v4());
}
