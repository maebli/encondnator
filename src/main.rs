use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (print_names, take_damage, despawn_dead).chain())
        .run();
}

#[derive(Debug, Component)]
struct Health(f32);

#[derive(Debug, Component)]
struct Enemy;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ground
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(1.0, 1.0))),
        material: materials.add(Color::WHITE),
        ..default()
    },));

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 2.0, 2.0),
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn((Name::new("J.C. Denton"), Health(100.0)));
    commands.spawn((Name::new("Bob Page"), Health(200.0), Enemy));
}

fn print_names(names: Query<(&Name, &Health)>) {
    for (name, health) in names.iter() {
        println!("hi there, {name} ({health})", health = health.0);
    }
}

fn take_damage(mut enemies: Query<&mut Health, With<Enemy>>) {
    for mut health in enemies.iter_mut() {
        health.0 = (health.0 - 10.0).max(0.0);
    }
}

fn despawn_dead(mut commands: Commands, healths: Query<(Entity, &Health)>) {
    for (entity, health) in healths.iter() {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
