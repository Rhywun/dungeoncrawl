use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        // Debug
        eprintln!("{:?}", key);

        // // NOTE: Debug - commit suicide
        // if *key == VirtualKeyCode::X {
        //     // TODO: Quit
        //     return;
        // }

        // Commit suicide
        if *key == VirtualKeyCode::X {
            // TODO: Quit for real somewhere, with e.g. `ctx.quitting`.
            eprintln!("Committing suicide");
            std::process::exit(0);
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

        if delta.x != 0 || delta.y != 0 {
            // This means something like "get all the points that have the
            // player."
            let mut players =
                <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
