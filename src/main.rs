use bevy::prelude::*;

#[derive(Resource)]
struct HelloTimer(Timer);

fn init() {
    println!("initializing...")
}

fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world");
    }
}

fn goodbye_world() {
    println!("goodbye world")
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(HelloTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, init)
            .add_systems(Update, (hello_world, goodbye_world));
    }
}


fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin))
    .run();
}