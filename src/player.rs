use bevy::prelude::*;
use crate::{SpriteImages, WinSize, Player, Speed, PlayerReadyFire, TIME_STEP, Bullet, TO_RAD, Level, image_helper::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn))
        .add_system(player_movement)
        .add_system(player_fire)
        .add_system(bullet_movement)
        .add_system(player_level_collide);
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
            translation: Vec3::new(0., bottom + 400., 10.),
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

        let old_pos = transform.translation;
        transform.translation.x += dir * speed.0 * TIME_STEP;
        transform.translation.y -= 9.82 * TIME_STEP;

        let pos = &mut transform.translation;
        let area = Vec3::new(win_size.w / 2., win_size.h / 2., 0.);
        
        if pos.x > area.x || pos.x < -area.x
        {
            pos.x = old_pos.x;
        }
        if pos.y > area.y || pos.y < -area.y
        {
            pos.y = old_pos.y;
        }
    }
}

fn player_level_collide(
    mut images: ResMut<Assets<Image>>,
    mut player_query: Query<(&mut Transform, With<Player>), Without<Level>>,
    level_query: Query<(&Handle<Image>, &Transform, With<Level>), Without<Player>>
){
    // Fetch player transform and image handle+transform of level
    if let ( Ok((mut player_transform, _)), Ok((img_handle, level_transform, _)) ) = 
           ( player_query.get_single_mut(), level_query.get_single()) {

        let size = images.get(img_handle).unwrap().texture_descriptor.size;
        let size = Vec2::new(size.width as f32, size.height as f32);
        let player_wpos = player_transform.translation;
        let to_level_pos = level_transform.compute_matrix().inverse();
        let player_lpos = to_level_pos.transform_point3(player_wpos);
        let player_ppos = UVec2::new((player_lpos.x + size.x * 0.5) as u32,(-player_lpos.y + size.y * 0.5) as u32);


        if get_pixel(&player_ppos, images.get(img_handle).unwrap()).r() <= 0.0 {
            set_pixel(&player_ppos, &Color::RED, images.get_mut(img_handle).unwrap());
        }
        else {
            set_pixel(&player_ppos, &Color::BLUE, images.get_mut(img_handle).unwrap());
        }
        /*
        transform.translation.x += dir * speed.0 * TIME_STEP;
        transform.translation.y -= 9.82 * TIME_STEP;

        let pos = &mut transform.translation;
        let area = Vec3::new(win_size.w / 2., win_size.h / 2., 0.);
        
        if pos.x > area.x || pos.x < -area.x
        {
            pos.x = old_pos.x;
        }
        if pos.y > area.y || pos.y < -area.y
        {
            pos.y = old_pos.y;
        }
        */
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