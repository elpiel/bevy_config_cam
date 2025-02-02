use bevy::{prelude::*, text::Text2dSize};
use bevy_config_cam::{ConfigCam, MovementSettings};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 80.0,          // default: 12.0
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(animate_translation.system())
        .add_system(animate_rotation.system())
        .add_system(animate_scale.system())
        .add_system(animate_text.system())
        .run();
}

struct AnimateTranslation;
struct AnimateRotation;
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    // Demonstrate changing translation
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("translation", text_style.clone(), text_alignment),
            ..Default::default()
        })
        .insert(AnimateTranslation);
    // Demonstrate changing rotation
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("rotation", text_style.clone(), text_alignment),
            ..Default::default()
        })
        .insert(AnimateRotation);
    // Demonstrate changing scale
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("scale", text_style, text_alignment),
            ..Default::default()
        })
        .insert(AnimateScale);
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32 - 400.0;
        transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(time.seconds_since_startup().cos() as f32);
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in query.iter_mut() {
        transform.translation = Vec3::new(400.0, 0.0, 0.0);
        transform.scale = Vec3::splat((time.seconds_since_startup().sin() as f32 + 1.1) * 2.0);
    }
}

fn animate_text(
    time: Res<Time>,
    mut query: Query<&mut Text, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut text in query.iter_mut() {
        let a = time.last_update().unwrap().elapsed();
        let mut ts = text.sections[0].clone();
        ts.value = a.as_micros().to_string();
        text.sections[0] = ts;
    }
}
