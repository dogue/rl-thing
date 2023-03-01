use bevy::prelude::*;

pub struct TweenPlugin;

impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(process_tween);
    }
}

#[derive(Component, Debug)]
pub struct TweenTransform {
    pub start: Transform,
    pub end: Transform,
    pub duration: f32,
    pub counter: f32,
}

impl TweenTransform {
    pub fn new(start: Transform, end: Transform, duration: f32) -> Self {
        Self {
            start,
            end,
            duration,
            counter: 0.,
        }
    }
}

fn process_tween(
    mut query: Query<(&mut Transform, &mut TweenTransform, Entity), With<TweenTransform>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, mut tween, entity) in query.iter_mut() {
        let x = lerp(
            tween.start.translation.x,
            tween.end.translation.x,
            tween.counter,
        );

        let y = lerp(
            tween.start.translation.y,
            tween.end.translation.y,
            tween.counter,
        );

        // println!(
        //     "({}, {}) - {} - {} - {} - {}",
        //     x,
        //     y,
        //     tween.duration,
        //     time.delta_seconds(),
        //     tween.duration * time.delta_seconds(),
        //     (tween.duration * time.delta_seconds()) / 0.1
        // );

        transform.translation.x = x;
        transform.translation.y = y;

        tween.counter += (1. / tween.duration) * time.delta_seconds();

        if tween.counter >= 1. {
            transform.translation = tween.end.translation;
            commands.entity(entity).remove::<TweenTransform>();
        }
    }
}

fn lerp(start: f32, end: f32, time: f32) -> f32 {
    start + (end - start) * time
}
