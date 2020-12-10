mod handheld;
mod instruction;
mod instruction_type;

use handheld::Handheld;

fn main() {
    let handheld = Handheld::from(include_str!("../input.txt"));
    let accum = handheld.get_accum_after_terminate();
    println!("Acc at restart of loop is {:?}", accum);
}
