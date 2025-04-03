#[cfg(test)]
mod tests {
    use std::println;

    use bevy_app::{App, Update};
    use bevy_ecs::{
        component::Component,
        system::{Query, Res},
        world::World,
    };
    use bevy_time::{Time, TimePlugin};

    fn t_s(dt: Res<Time>, w: &World) {
        let _ = dt;
        let _ = w;
        println!("t_s");
    }

    #[derive(Component)]
    struct Position {
        x: f32,
        y: f32,
    }
    #[derive(Component)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    // This system moves each entity with a Position and Velocity component
    fn movement(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in &mut query {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }

    #[test]
    fn check_app_bevy_update() {
        let mut app = App::new();
        app.add_plugins(TimePlugin)
            .add_systems(Update, (movement, t_s));

        app.update();
        app.update();
        app.update();
    }
}
