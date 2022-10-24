use bevy::prelude::{App, Plugin, Timer};
use crate::resources::GreetTimer;
use crate::systems::people::{add_people, greet_people};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}