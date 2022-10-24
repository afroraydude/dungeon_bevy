use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable};

mod components;
mod entities;
mod systems;

#[derive(Component)]
struct Person;

#[derive(Component, Inspectable)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Bob".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("James".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Natalie".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(HelloPlugin)
        .run();
}
