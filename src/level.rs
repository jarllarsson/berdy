use bevy::{prelude::*, core::FixedTimestep};
use rand::{thread_rng, Rng};
use crate::{SpriteImages, WinSize, TIME_STEP, TO_RAD, Level};

pub struct LevelPlugin;

impl Plugin for LevelPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_stage(
            "game_setup_level",
            SystemStage::single(level_spawn))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.2))
                .with_system(level_destroy)
        );
    }
}

fn level_spawn(
    mut commands: Commands,
    sprite_images: Res<SpriteImages>,
    window_size: Res<WinSize>
) {
    // Spawn sprite
    let bottom = -window_size.h / 2.;
    // Create entity and add to entity bundle
    commands.spawn_bundle(SpriteBundle {
        texture: sprite_images.level.0.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(3., 3., 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    // Custom components
    .insert(Level);
}

fn level_destroy(
    win_size: Res<WinSize>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&Handle<Image>, With<Level>)>
){
    if let Ok((img_handle, _)) = query.get_single_mut() {
        let mut rng = thread_rng();
        if let Some(img) = images.get_mut(img_handle) {
            let size = img.texture_descriptor.size;
            let hole_pos = UVec2::new(rng.gen_range(0..size.width), rng.gen_range(0..size.height));
            let mut idx = 0;
            let radius = 5.;
            for _pixel in img.data.chunks_exact_mut(4) { // RGBA
                if _pixel[0] as u16 + _pixel[1] as u16 + _pixel[2] as u16 != 0 { // if not black pixel
                    let pixel_pos = UVec2::new(idx % size.width, idx / size.width);
                    // Random pixel
                    // _pixel[0] = rng.gen_range(0..255) as u8;
                    // _pixel[1] = rng.gen_range(0..255) as u8;
                    // _pixel[2] = rng.gen_range(0..255) as u8;

                    // Random hole
                    if (Vec2::new(pixel_pos.x as f32, pixel_pos.y as f32) - Vec2::new(hole_pos.x as f32, hole_pos.y as f32)).length_squared() < radius * radius {
                        _pixel[0] = rng.gen_range(0..255) as u8;
                        _pixel[1] = rng.gen_range(0..255) as u8;
                        _pixel[2] = rng.gen_range(0..255) as u8;
                    }
                }
                idx += 1;
            }
        }
    }
}

/*
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
}*/