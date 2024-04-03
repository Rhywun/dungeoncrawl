use std::process::exit;

use crate::prelude::*;

/// A game player.
pub struct Player {
    /// The [Point] on the [Map] where this player is located.
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// Draws this [Player], centered in the [Camera].
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Use the layer above the base layer.
        ctx.set_active_console(1);

        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
    /// Moves this [Player] on the [Map] based on keyboard input.
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            eprintln!("{:?}", key);

            if key == VirtualKeyCode::X {
                // TODO: Quit for real somewhere, with e.g. `ctx.quitting`.
                eprintln!("Committing suicide");
                exit(0);
            }

            let delta = match key {
                VirtualKeyCode::Left
                | VirtualKeyCode::A
                | VirtualKeyCode::Numpad4 => Point::new(-1, 0),
                VirtualKeyCode::Right
                | VirtualKeyCode::D
                | VirtualKeyCode::Numpad6 => Point::new(1, 0),
                VirtualKeyCode::Up
                | VirtualKeyCode::W
                | VirtualKeyCode::Numpad8 => Point::new(0, -1),
                VirtualKeyCode::Down
                | VirtualKeyCode::S
                | VirtualKeyCode::Numpad2 => Point::new(0, 1),
                VirtualKeyCode::Numpad7 => Point::new(-1, -1),
                VirtualKeyCode::Numpad9 => Point::new(1, -1),
                VirtualKeyCode::Numpad1 => Point::new(-1, 1),
                VirtualKeyCode::Numpad3 => Point::new(1, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
