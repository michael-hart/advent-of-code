mod handheld;
mod instruction;
mod instruction_type;

use handheld::Handheld;

fn main() {
    let mut handheld = Handheld::from(include_str!("../input.txt"));
    let accum = handheld.get_accum_at_loop_restart();
    println!("Acc at restart of loop is {}", accum);
}
