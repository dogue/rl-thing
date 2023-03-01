use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::player::ProcessTurn;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_mob).add_system(mob_movement);
    }
}

#[derive(Component)]
pub struct Mob;

fn spawn_mob(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(32.)),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        })
        .insert(Mob);
}

fn mob_movement(mut mobs: Query<&mut Transform, With<Mob>>, events: EventReader<ProcessTurn>) {
    if !events.is_empty() {
        for mut mob in mobs.iter_mut() {
            let mut rng = rand::thread_rng();
            let movement: Vec2 = match rng.gen_range(0..4) {
                0 => Vec2::X * -32.,
                1 => Vec2::X * 32.,
                2 => Vec2::Y * 32.,
                3 => Vec2::Y * -32.,
                _ => Vec2::ZERO,
            };

            mob.translation += movement.extend(0.);
        }

        events.clear();
    }
}
