use crate::{despawn_screen, AppState};
use bevy::prelude::*;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub fn plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `AppState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(AppState::Splash), setup)
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(AppState::Splash), despawn_screen::<OnSplashScreen>);
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Component)]
struct CountDownText;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup(mut commands: Commands) {
    // Display the logo
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        children![
            (
                Text::new("Cordon"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
            ),
            (
                Text::new("Press a key or button... "),
                children![(TextSpan::default(), CountDownText,),]
            )
        ],
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    query: Query<&mut TextSpan, With<CountDownText>>,
) {
    if timer.tick(time.delta()).finished() {
        app_state.set(AppState::Menu);
    }

    for mut span in query {
        let elapsed_secs = timer.elapsed_secs_f64();
        **span = format!("{elapsed_secs:.1}s");
    }
}
