use crate::resources::meyer_cross::MeyerCross;
use bevy::{app::Plugin, prelude::*};

pub struct MeyerCrossPlugin;

impl Plugin for MeyerCrossPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeyerCross::new());
    }
}
