use bevy::prelude::*;
use crate::{SpriteImages, WinSize, Player, Speed, PlayerReadyFire, TIME_STEP, Bullet, TO_RAD};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn))
        .add_system(player_movement)
        .add_system(player_fire)
        .add_system(bullet_movement);
    }
}

fn player_spawn(
    mut commands: Commands,
    sprite_images: Res<SpriteImages>,
    window_size: Res<WinSize>
) {
    // Spawn sprite
    let bottom = -window_size.h / 2.;
    // Create entity and add to entity bundle
    commands.spawn_bundle(SpriteBundle {
        texture: sprite_images.player.0.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 67. / 4. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    // Custom components
    .insert(Player) // Unit struct to define player type
    .insert(Speed::default())
    .insert(PlayerReadyFire(true));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>
){
    if let Ok((speed, mut transform, _)) = query.get_single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.0
        };

        let old_pos = transform.translation.x;
        transform.translation.x += dir * speed.0 * TIME_STEP;

        let pos = &mut transform.translation.x;
        let area = Vec3::new(win_size.w / 2., win_size.h / 2., 0.);
        
        if *pos > area.x || *pos < -area.x
        {
            *pos = old_pos;
        }
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    sprite_images: ResMut<SpriteImages>,
    mut query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>
)
{
    if let Ok((player_transform, mut ready_fire, _)) = query.get_single_mut() {
        if ready_fire.0 && keyboard_input.pressed(KeyCode::Space) {
            ready_fire.0 = false;
            let pos = player_transform.translation;
            let mut spawn_bullet = |x_offset: f32| {
                // Create entity and add to entity bundle
                commands.spawn_bundle(SpriteBundle {
                    texture: sprite_images.bullet.0.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos.x + x_offset, pos.y + 5., 0.),
                        scale: Vec3::new(0.5, 0.5, 1.),
                        rotation: Quat::from_rotation_z(90. * TO_RAD),
                        ..Default::default()
                    },
                    ..Default::default()
                    })
                    // Custom components
                    .insert(Bullet) // Unit struct to define player type
                    .insert(Speed::default());
            };

            spawn_bullet(30.);
            spawn_bullet(-30.);

        };
        if keyboard_input.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn bullet_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Bullet>)>
){
    for (bullet_entity, speed, mut bullet_transform, _) in query.iter_mut() {
        let pos = &mut bullet_transform.translation;
        let area = Vec3::new(win_size.w / 2., win_size.h / 2., 0.);
        
        let dir = Vec3::new(0., 1., 0.);
        *pos += dir * speed.0 * TIME_STEP;

        if pos.y > area.y || pos.y < -area.y || 
        pos.x > area.x || pos.x < -area.x
        {
          commands.entity(bullet_entity).despawn();
        }

        // if (transform.translation.)
    }
}