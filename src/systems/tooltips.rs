use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
) {
    // Includes parent Entity in the search results
    let mut positions = <(Entity, &Point, &Name)>::query();

    // Draw tooltips
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset; // (1)
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos) // (2)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4; // (3)
            let display = if let Ok(health) = ecs
                .entry_ref(*entity) // (4)
                .unwrap()
                .get_component::<Health>()
            {
                format!("{} : {} hp", &name.0, health.current) // (5)
            } else {
                // (6)
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
