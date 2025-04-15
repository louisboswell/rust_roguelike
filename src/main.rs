// Identical to C++ 'include' importing
use bracket_lib::prelude::{BTerm, BTermBuilder, GameState};
use bracket_lib::terminal::*;

// Creating a new structure - similar to classes in other languages
struct State {}
// Telling Rust that our State structure implements the trait GameState
// Similar to base classes / inheritance
impl GameState for State {
    // This is a function definition of a function called tick
    // This is inside the trait implementation scope, so must match the type
    // declared by the trait

    // No -> type means it returns void
    // &mut self means this function requires access to the parent structure (state)
    // &mut means it can change variables inside State
    // cts: &mut BTerm means pass in a variable called ctx (short for Context)
    // & means pass a reference i.e. pointers
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> BError {
    use crate::BTermBuilder;
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    crate::main_loop(context, gs)
}
