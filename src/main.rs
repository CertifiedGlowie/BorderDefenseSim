use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use rand::Rng;

#[derive(Component)]
struct Player {
    blocking: Option<Timer>,
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HpText;

#[derive(Resource)]
pub struct GateHealth(pub u8);

#[derive(Component)]
struct Enemy {
    health: u16,
    speed: f32,
    state: EnemyState, // Add an enum to represent the state of enemy movement
    damage: u8,
    niggertype: NiggerVariations,
    previous_translation: Vec3,
}

#[derive(Clone, Copy)]
enum EnemyState {
    Up,
    Right,
}

#[derive(Resource)]
pub struct Shells(pub u8);

#[derive(Component)]
struct ReloadWarning;

fn get_touch(touches: Res<Touches>, window: Query<&Window, With<PrimaryWindow>>) -> bool {
    let window = window.get_single().unwrap();
    let window_width = window.width();
    let shooting_area_width = window_width / 4.0; // Define the width of the shooting area

    // Check for touch input
    for touch in touches.iter() {
        let touch_x = touch.position().x / window_width;

        // Implement shooting logic in the shooting area
        if touch_x > 1.0 - shooting_area_width / window_width {
            return true;
        }
    }
    false
}

fn spawn_projectile(
    mut commands: Commands,
    asset: Res<ImageAssets>,
    audio_asset: Res<AudioAssets>,
    input: (
        Res<ButtonInput<KeyCode>>,
        Res<ButtonInput<MouseButton>>,
        Res<Touches>,
    ),
    mut shells: ResMut<Shells>,
    mut player: Query<(&mut Transform, &mut Player)>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Some((player_transform, mut player)) = player.iter_mut().next() {
        if let Some(timer) = &mut player.blocking {
            timer.tick(time.delta());

            if timer.finished() {
                player.blocking = None
            }
        }

        let (keyboard_input, mouse_input, touchinput) = input;
        if !mouse_input.just_pressed(MouseButton::Left)
            && !keyboard_input.just_pressed(KeyCode::Space)
            && !keyboard_input.just_pressed(KeyCode::Enter)
            && !get_touch(touchinput, window)
        {
            return;
        }

        if player.blocking.is_none() {
            match shells.0 {
                0 => {
                    shells.0 = 5;
                    player.blocking = Some(Timer::from_seconds(0.5, TimerMode::Once));
                }
                1 => {
                    player.blocking = Some(Timer::from_seconds(2.5, TimerMode::Once));
                }
                _ => {
                    player.blocking = Some(Timer::from_seconds(0.5, TimerMode::Once));
                }
            }
            shells.0 -= 1;
            println!("1 Shot fired! Remaining ammo: {:?}", shells.0);

            let texture = asset.tank_round.clone();
            let projectile_velocity = 80.0;

            let direction = player_transform
                .rotation
                .mul_vec3(Vec3::new(0.0, -1.0, 0.0));

            let spawn_position = player_transform.translation + direction * 20.0;

            commands.spawn(AudioBundle {
                source: audio_asset.tankshot.clone(),
                settings: PlaybackSettings::ONCE,
            });
            commands
                .spawn(SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: spawn_position,
                        rotation: player_transform.rotation,
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(Projectile {
                    damage: 100,
                    velocity: projectile_velocity,
                    direction,
                });
        }
    }
}

fn move_projectiles(mut projectiles: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in &mut projectiles {
        // Use the direction stored in the Projectile component to move it
        transform.translation += projectile.direction * projectile.velocity * time.delta_seconds();
    }
}

fn handle_player_animation(
    mut player: Query<(&mut Handle<Image>, &Player)>,
    asset: Res<ImageAssets>,
) {
    for (mut sprite, player) in &mut player.iter_mut() {
        if let Some(timer) = &player.blocking {
            if timer.elapsed() >= timer.duration() / 2 {
                *sprite = asset.turret_animation_one.clone();
            } else {
                *sprite = asset.turret_animation_two.clone();
            }
        } else {
            *sprite = asset.tankturret.clone();
        }
    }
}

fn handle_enemy_animation(
    mut enemies: Query<(&mut Handle<Image>, &Transform, &mut Enemy)>,
    asset: Res<ImageAssets>,
) {
    for (mut sprite, transform, mut enemy) in &mut enemies.iter_mut() {
        // Calculate distance traveled since the last animation change
        let distance_traveled = (transform.translation - enemy.previous_translation).length();

        // Check if the distance traveled exceeds a threshold
        const ANIMATION_THRESHOLD: f32 = 5.0;
        if distance_traveled >= ANIMATION_THRESHOLD {
            // Handle animation based on enemy type
            match enemy.niggertype {
                NiggerVariations::Normal => {
                    *sprite = if *sprite == asset.normal_nigger_step {
                        asset.normal_nigger.clone()
                    } else {
                        asset.normal_nigger_step.clone()
                    };
                }
                NiggerVariations::Speedy => {
                    *sprite = if *sprite == asset.speedy_nigger_step {
                        asset.speedy_nigger.clone()
                    } else {
                        asset.speedy_nigger_step.clone()
                    };
                }
                NiggerVariations::Buff => {
                    *sprite = if *sprite == asset.buff_nigger_step {
                        asset.buff_nigger.clone()
                    } else {
                        asset.buff_nigger_step.clone()
                    };
                }
                NiggerVariations::Arab => {
                    *sprite = if *sprite == asset.arab_nigger_step {
                        asset.arab_nigger.clone()
                    } else {
                        asset.arab_nigger_step.clone()
                    };
                }
            }
            enemy.previous_translation = transform.translation;
        }
    }
}

fn character_rotate(
    mut characters: Query<(&mut Transform, &Player)>,
    touch_input: Res<Touches>,
    input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.get_single().unwrap();
    let window_width = window.width();
    let shooting_area_width = window_width / 4.0; // Define the width of the shooting area

    for (mut transform, _player) in &mut characters.iter_mut() {
        // Rotate player
        if input.pressed(KeyCode::ArrowRight) {
            transform.rotation *= Quat::from_rotation_z(std::f32::consts::PI / 650.0);
        }

        if input.pressed(KeyCode::KeyD) {
            transform.rotation *= Quat::from_rotation_z(std::f32::consts::PI / 250.0);
        }

        if input.pressed(KeyCode::ArrowLeft) {
            transform.rotation *= Quat::from_rotation_z(-std::f32::consts::PI / 650.0);
        }

        if input.pressed(KeyCode::KeyA) {
            transform.rotation *= Quat::from_rotation_z(-std::f32::consts::PI / 250.0);
        }

        // Check for touch input
        for touch in touch_input.iter() {
            let touch_x = touch.position().x / window_width;

            // Rotate the character based on touch input in the rotation area
            if touch_x < 1.0 - shooting_area_width / window_width {
                let angle = std::f32::consts::PI * (2.0 * touch_x - 1.0);
                transform.rotation = Quat::from_rotation_z(angle);
            }
            // Implement shooting logic in the shooting area
            else if touch_x > 1.0 - shooting_area_width / window_width {
                return;
            }
        }
    }
}

fn stop_game(
    mut commands: Commands,
    sprites: Query<Entity, With<Sprite>>,
    asset_server: Res<AssetServer>,
) {
    // Despawn all sprites
    for entity in sprites.iter() {
        commands.entity(entity).despawn();
    }

    // Reset gate health
    commands.insert_resource(GateHealth(100));

    // Reset shells
    commands.insert_resource(Shells(5));
    commands.insert_resource(Score(0));

    commands.spawn(
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "GAME OVER!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/aovel_sans.ttf"),
                font_size: 128.0,
                color: Color::RED,
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(40.0),   // Center the text horizontally
            right: Val::Percent(40.0),  // Center the text horizontally
            top: Val::Percent(40.0),    // Center the text vertically
            bottom: Val::Percent(40.0), // Center the text vertically
            ..default()
        }),
    );
}

fn move_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform, &mut Enemy)>,
    time: Res<Time>,
    mut health: ResMut<GateHealth>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if health.0 == 0 {
        game_state.set(GameState::GameOver)
    }
    for (entity, mut transform, mut enemy) in &mut enemies.iter_mut() {
        match enemy.state {
            EnemyState::Up => {
                transform.translation.y += enemy.speed * time.delta_seconds();
            }
            EnemyState::Right => {
                transform.translation.x += enemy.speed * time.delta_seconds();
            }
        }
        if transform.translation.y >= -30.0 && transform.translation.y <= -25.0
            || transform.translation.y >= -5.0
        {
            enemy.state = EnemyState::Right;
        }

        if transform.translation.x >= -40.0 && transform.translation.y <= -10.0
            || transform.translation.x >= 80.0
        {
            enemy.state = EnemyState::Up;
        }

        if transform.translation.x > 79.0 && transform.translation.y > 54.0 {
            commands.entity(entity).despawn();
            if enemy.damage >= health.0 {
                // Enemy damage exceeds gate health, set gate health to 0
                health.0 = 0;
            } else {
                // Subtract enemy damage from gate health
                health.0 -= enemy.damage;
            }
        }
    }
}

#[derive(Component)]
struct Projectile {
    damage: u16,
    velocity: f32,
    direction: Vec3, // Add this line
}

#[derive(Component)]
struct ExplosionTimer(Timer);

fn projectile_collision(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &Transform, &Projectile)>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy)>,
    asset: Res<ImageAssets>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut Sprite)>,
    mut score: ResMut<Score>,
) {
    for (projectile_entity, projectile_transform, projectile) in &mut projectiles.iter_mut() {
        for (enemy_entity, enemy_transform, mut enemy) in &mut enemies.iter_mut() {
            let distance = Vec3::distance(
                projectile_transform.translation,
                enemy_transform.translation,
            );
            let collision_radius = 5.0;
            if distance < collision_radius * 2.0 {
                // Spawn explosion entity
                let _explosion_entity = commands.spawn((
                    SpriteBundle {
                        texture: asset.explosion.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                enemy_transform.translation.x,
                                enemy_transform.translation.y,
                                1.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ExplosionTimer(Timer::from_seconds(0.2, TimerMode::Once)), // Set the explosion duration
                ));
                if enemy.health <= projectile.damage {
                    // Projectile damage exceeds enemy health, set enemy health to 0
                    enemy.health = 0;
                } else {
                    // Subtract projectile damage from enemy health
                    enemy.health -= projectile.damage;
                }

                if enemy.health == 0 {
                    // Enemy is destroyed, despawn it and update score
                    commands.entity(enemy_entity).despawn();
                    score.0 += 1;
                    println!("{}", score.0);
                }

                // Despawn projectile entity
                commands.entity(projectile_entity).despawn();
            }
        }
    }

    // Update and despawn explosions
    for (entity, mut timer, _sprite) in &mut query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "tank/tankturret.png")]
    pub tankturret: Handle<Image>,
    #[asset(path = "tank/tankbase.png")]
    pub tankbase: Handle<Image>,

    #[asset(path = "niggers/normal.png")]
    pub normal_nigger: Handle<Image>,
    #[asset(path = "niggers/step/normal.png")]
    pub normal_nigger_step: Handle<Image>,

    #[asset(path = "niggers/speedy.png")]
    pub speedy_nigger: Handle<Image>,

    #[asset(path = "niggers/step/speedy.png")]
    pub speedy_nigger_step: Handle<Image>,

    #[asset(path = "niggers/buff.png")]
    pub buff_nigger: Handle<Image>,

    #[asset(path = "niggers/step/buff.png")]
    pub buff_nigger_step: Handle<Image>,

    #[asset(path = "niggers/arab.png")]
    pub arab_nigger: Handle<Image>,

    #[asset(path = "niggers/step/arab.png")]
    pub arab_nigger_step: Handle<Image>,

    #[asset(path = "tank/tank_round.png")]
    pub tank_round: Handle<Image>,
    #[asset(path = "explosion.png")]
    pub explosion: Handle<Image>,
    #[asset(path = "tank/retract_turret_one.png")]
    pub turret_animation_one: Handle<Image>,
    #[asset(path = "tank/retract_turret_two.png")]
    pub turret_animation_two: Handle<Image>,
    #[asset(path = "map.png")]
    pub map: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/tankshot.ogg")]
    tankshot: Handle<AudioSource>,

    #[asset(path = "audio/nigga-stole-my-bike.ogg")]
    niggabike: Handle<AudioSource>,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

fn update_display_score(
    mut query_set: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<HpText>>,
    )>,
    score: Res<Score>,
    health: Res<GateHealth>,
) {
    // Access the first query (ScoreText)
    for mut text in query_set.p0().iter_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
    }

    // Access the second query (HpText)
    for mut text in query_set.p1().iter_mut() {
        text.sections[0].value = format!("Health: {}", health.0);
    }
}

fn setup(
    mut commands: Commands,
    asset: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    audio_asset: Res<AudioAssets>,
) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let map_scale_factor = 0.2; // Adjust the scale factor as needed
    let map_translation = Vec3::new(0.0, 0.0, -2.0); // Adjust the Z value to make sure the map is behind other objects

    // Spawn the map with proper scaling and translation
    let _map = commands.spawn(SpriteBundle {
        texture: asset.map.clone(),
        transform: Transform {
            translation: map_translation,
            scale: Vec3::splat(map_scale_factor),
            ..Default::default()
        },
        ..Default::default()
    });
    let base_translation = Vec3::new(3.0, 60.0, -1.0); // Adjust the translation of the base as needed

    // Spawn the tank base first
    let _player_base = commands.spawn(SpriteBundle {
        texture: asset.tankbase.clone(),
        transform: Transform {
            translation: base_translation,
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Score: 0",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/aovel_sans.ttf"),
                    font_size: 48.0,
                    color: Color::RED,
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                left: Val::Px(5.0),
                ..default()
            }),
        ))
        .insert(ScoreText);

    commands
        .spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "HP: 100",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/aovel_sans.ttf"),
                    font_size: 48.0,
                    color: Color::LIME_GREEN,
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),

                ..default()
            }),
        ))
        .insert(HpText);

    commands.spawn(AudioBundle {
        source: audio_asset.niggabike.clone(), // Assuming reparations is the default song
        settings: PlaybackSettings::LOOP,
    });

    // Ensure the turret is spawned after the base, so it overlays it
    let _player_character = commands
        .spawn(SpriteBundle {
            texture: asset.tankturret.clone(),
            transform: Transform {
                translation: base_translation + Vec3::new(0.0, 1.5, 1.0), // Adjust the Z value if necessary
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { blocking: None });

    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
}

enum NiggerVariations {
    Normal,
    Speedy,
    Buff,
    Arab,
}

fn spawn_enemies(
    mut commands: Commands,
    mut timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    asset: Res<ImageAssets>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let mut rng = rand::thread_rng();
        let random_value: f32 = rng.gen::<f32>();

        let position = Vec3::new(rand::thread_rng().gen_range(-105.0..=-95.0), -70.0, 0.0);

        let variation = if random_value < 0.7 {
            NiggerVariations::Normal
        } else if random_value < 0.9 {
            NiggerVariations::Speedy
        } else if random_value < 0.95 {
            NiggerVariations::Buff
        } else {
            NiggerVariations::Arab
        };

        match variation {
            NiggerVariations::Normal => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset.normal_nigger.clone(),
                        transform: Transform::from_xyz(position.x, position.y, position.z),
                        ..Default::default()
                    })
                    .insert(Enemy {
                        health: 100,
                        speed: 25.0,
                        state: EnemyState::Up,
                        damage: 25,
                        niggertype: NiggerVariations::Normal,
                        previous_translation: position,
                    });
            }

            NiggerVariations::Speedy => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset.speedy_nigger.clone(),
                        transform: Transform::from_xyz(position.x, position.y, position.z),
                        ..Default::default()
                    })
                    .insert(Enemy {
                        health: 100,
                        speed: 60.0,
                        state: EnemyState::Up,
                        damage: 25,
                        niggertype: NiggerVariations::Speedy,
                        previous_translation: position,
                    });
            }

            NiggerVariations::Buff => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset.buff_nigger.clone(),
                        transform: Transform::from_xyz(position.x, position.y, position.z),
                        ..Default::default()
                    })
                    .insert(Enemy {
                        health: 200,
                        speed: 35.0,
                        state: EnemyState::Up,
                        damage: 75,
                        niggertype: NiggerVariations::Buff,
                        previous_translation: position,
                    });
            }

            NiggerVariations::Arab => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset.arab_nigger.clone(),
                        transform: Transform::from_xyz(position.x, position.y, position.z),
                        ..Default::default()
                    })
                    .insert(Enemy {
                        health: 100,
                        speed: 20.0,
                        state: EnemyState::Up,
                        damage: 100,
                        niggertype: NiggerVariations::Arab,
                        previous_translation: position,
                    });
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Running,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Border Defense Simulator".to_owned(),
                        focused: true,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running)
                .load_collection::<ImageAssets>()
                .load_collection::<AudioAssets>(),
        )
        .insert_resource(Shells(5))
        .insert_resource(GateHealth(100))
        .insert_resource(Score(0))
        .add_systems(OnEnter(GameState::Running), setup)
        .add_systems(
            Update,
            (
                character_rotate,
                spawn_projectile,
                move_projectiles,
                projectile_collision,
                spawn_enemies,
                handle_player_animation,
                handle_enemy_animation,
                move_enemies,
                update_display_score,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(OnEnter(GameState::GameOver), stop_game)
        .run();
}
