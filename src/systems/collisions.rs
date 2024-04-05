use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get the player position
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    // Get all enemy positions
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    // Remove enemies whose position matches the player's position
    enemies.iter(ecs).filter(|(_, pos)| **pos == player_pos).for_each(|(entity, _)| {
        commands.remove(*entity);
    });
}
