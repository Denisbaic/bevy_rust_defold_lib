use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_time::prelude::*;
use bevy_transform::components::Transform;

const GRAVITY_VALUE: f32 = -9.81;

#[derive(Component)]
pub struct Speed {
    value: bevy_math::Vec3,
}

#[derive(Component)]
pub struct Friction {
    value: f32,
}

#[derive(Component)]
pub struct ScaleSpeed {
    value: bevy_math::Vec3,
}

#[derive(Component)]
pub struct RotationSpeed {
    value: bevy_math::Quat,
}

#[derive(Component)]
pub struct RotationFriction {
    value: f32,
}

#[derive(Component)]
pub struct LifetimeRemaining {
    life_remaining: Timer,
}

#[derive(Component)]
pub struct GravityScale {
    value: f32,
}

fn rotation_friction_system(
    mut query: Query<(&mut RotationSpeed, &RotationFriction)>,
    time: Res<Time>,
) {
    for (mut rotation_speed, rotation_friction) in &mut query {
        let friction_value = rotation_friction.value * time.delta_secs();
        rotation_speed.value = rotation_speed
            .value
            .slerp(Quat::IDENTITY, friction_value.clamp(0.0, 1.0));
    }
}

fn friction_system(mut query: Query<(&mut Speed, &Friction)>, time: Res<Time>) {
    for (mut speed, friction) in &mut query {
        let friction_value = (friction.value * time.delta_secs()).clamp(0.0, 1.0);
        speed.value *= 1.0 - friction_value;
    }
}

fn gravity_system(mut query: Query<(&mut Speed, &GravityScale)>, time: Res<Time>) {
    for (mut speed, gravity_scale) in &mut query {
        speed.value.y += gravity_scale.value * GRAVITY_VALUE * time.delta_secs();
    }
}

fn lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut LifetimeRemaining)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.life_remaining.tick(time.delta());
        if lifetime.life_remaining.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn move_system(time: Res<Time>, mut query: Query<(&mut Transform, &Speed)>) {
    for (mut transform, speed) in &mut query {
        transform.translation += speed.value * time.delta_secs();
    }
}

fn scale_system(mut query: Query<(&mut Transform, &ScaleSpeed)>, time: Res<Time>) {
    for (mut transform, scale_speed) in &mut query {
        transform.scale += scale_speed.value * time.delta_secs();
        transform.scale = Vec3::new(
            transform.scale.x.max(0.0),
            transform.scale.y.max(0.0),
            transform.scale.z.max(0.0),
        );
    }
}

fn rotate_entities_system(mut query: Query<(&mut Transform, &RotationSpeed)>, time: Res<Time>) {
    for (mut transform, rotation_speed) in &mut query {
        let delta_angle = rotation_speed.value * time.delta_secs();
        transform.rotation *= delta_angle;
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ParticleSet;

pub struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_system,
                scale_system,
                rotate_entities_system,
                gravity_system,
                friction_system,
                rotation_friction_system,
                lifetime_system,
            )
                .in_set(ParticleSet),
        );
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use bevy_time::TimePlugin;

    use super::*;

    #[test]
    fn test_lifetime_system() {
        // Создаем тестовый мир
        let mut app = App::new();

        app.add_plugins((ParticlesPlugin, TimePlugin));

        // Создаем сущность с компонентом `LifetimeRemaining`
        let entity = app
            .world_mut()
            .spawn(LifetimeRemaining {
                life_remaining: Timer::from_seconds(1.0, TimerMode::Once),
            })
            .id();

        // Проверяем, что сущность еще не удалена
        assert!(app.world().get_entity(entity).is_ok());

        while app.world().resource::<Time>().elapsed().as_secs_f32() < 1.0 {
            app.update();
            let time_res = app.world().resource::<Time>();
            println!("Time: {}", time_res.elapsed().as_secs_f32());
        }
        assert!(app.world().get_entity(entity).is_err());
    }
}
