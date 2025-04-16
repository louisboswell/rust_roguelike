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

#[derive(Component)]
struct LeftMover {}

// Defines empty walker structure
struct LeftWalker {}

// 'a are lifetime specifiers - meaning the components must exist long enough for the system to run
impl<'a> System<'a> for LeftWalker {
    // Defining that the system needs read access to LeftMover components and write access to Position components
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    // Actual trait implementation
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        // Iterater finding objects with both LeftMover and Position components
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // Gains write access to both Position and Player components
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    // Only works on entities with both Player and Position
    for (_players, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
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
        self.run_system();
        player_input(self, ctx);

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

// Means we would like to implement functionality for State
impl State {
    // Defining a function that is mutable (able to change things)
    fn run_system(&mut self) {
        // Makes a new instance of LeftWalker
        let mut lw = LeftWalker {};
        // Tells the system to run and a reference to the ECS
        lw.run_now(&self.ecs);
        // Tells specs that if any changes were queued up they should apply now
        self.ecs.maintain();
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
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

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
        .with(Player {})
        .build();

    // For loop to create 10 entities (incl 10 would be 0..=10)
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 25 })
            .with(Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            // Adds an extra component to the ecs
            .with(LeftMover {})
            .build();
    }

    crate::main_loop(context, gs)
}
