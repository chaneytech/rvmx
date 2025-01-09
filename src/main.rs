pub mod assembler;
pub mod instruction;
mod instruction_test;
mod repl;
pub mod vm;
mod vm_test;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
