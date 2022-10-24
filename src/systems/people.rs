use bevy::prelude::*;
use crate::components::person::{Name, Person};
use crate::resources::GreetTimer;

pub fn add_people(mut commands: Commands) {
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

pub fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}