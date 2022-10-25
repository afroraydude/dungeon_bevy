pub mod assets;

use bevy::prelude::Timer;

pub struct GreetTimer(pub Timer);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum MyStates {
    AssetLoading,
    Next,
}