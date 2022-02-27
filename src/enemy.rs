use bevy::{prelude::*, core::FixedTimestep};
use rand::{Rng, thread_rng};
use crate::{WinSize, SpriteImages, ActiveEnemies, Enemy};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(3.0))
                .with_system(enemy_spawn)
        );
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WinSize>,
    sprite_images: Res<SpriteImages>
) {
    if active_enemies.0 < 1 {
        // Get random spawn coords
        let mut rng = thread_rng();
        let w_spawn = win_size.w / 2. - 100.;
        let h_spawn = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_spawn..w_spawn) as f32;
        let y = rng.gen_range(-h_spawn..h_spawn) as f32;
        // spawn the enemy
        commands.spawn_bundle(SpriteBundle {
            texture: sprite_images.enemy.0.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
            })
            // Custom components
            .insert(Enemy);

        active_enemies.0 += 1;
    }
}