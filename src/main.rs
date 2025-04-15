// Identical to C++ 'include' importing
use bracket_lib::prelude::{BTerm, BTermBuilder, GameState};
use bracket_lib::terminal::*;

use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

// Macro that says 'from my basic data, please derive the boilder plate needed for x'
// In this case, x is a component
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

// Creating a new structure - similar to classes in other languages
struct State {
    ecs: World,
}
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

        // Asks the ECS for read access to the container it is using to store Position components
        let positions = self.ecs.read_storage::<Position>();
        // Asks the ECS for read access to the container it is using to store Renderables components
        let renderables = self.ecs.read_storage::<Renderable>();

        // .join() joins these two values so that only entities with a position and renderable are iterated
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    use crate::BTermBuilder;
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Creating a new game state from world
    // ::new() is a constructure for World type but without a reference from self
    // Can only be used to create new World objects
    let mut gs = State { ecs: World::new() };

    // Tells our World to look at the types we are giving it
    // and then create storage systems for each
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    // Tells the World (game state) that we'd like a new entity
    // The . methods is called builder pattern in Rust

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build();

    // For loop to create 10 entities (incl 10 would be 0..=10)
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 25 })
            .with(Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .build();
    }

    crate::main_loop(context, gs)
}
