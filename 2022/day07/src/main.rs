mod fs_object;
use fs_object::FileSystemObject;

fn main() {
    let raw = include_str!("../input.txt");
    let fso = FileSystemObject::from_iter(raw.lines());
    println!("A: {}", fso.part_a());
    println!("B: {}", fso.part_b());
}
