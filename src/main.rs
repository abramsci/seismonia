/* SEISMONIA application - core program logic and lifecycle.

Build with Bevy game engine <https://bevyengine.org/>.
*/
use bevy::{color::palettes::css::*, prelude::*};


/*                              GLOBAL CONSTANTS                              */
const BG_COLOR: ClearColor = ClearColor(Color::srgb(0.2, 0.2, 0.5));
const SPLASH_COLOR: Srgba = ORANGE;
const SPLASH_KEY: KeyCode = KeyCode::Space;


/*                               CORE FUNCTIONS                               */
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn change(input: Res<ButtonInput<KeyCode>>, mut color: ResMut<ClearColor>) {
    if input.just_pressed(SPLASH_KEY) {
        color.0 = SPLASH_COLOR.into();
    }
}


/*                                APP LIFECYCLE                               */
fn main() {
    App::new()
        .insert_resource(BG_COLOR)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, change)
        .run();
}
/******************************************************************************/