use bevy::{prelude::*, time::FixedTimesteps};

pub fn debug_time(time: Res<FixedTimesteps>) {
    match time.get("foo") {
        None => panic!("Time does not exist"),
        Some(state) => println!("{:.2}%", state.overstep_percentage() * 100.0),
    }
}