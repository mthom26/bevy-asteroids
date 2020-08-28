use bevy::prelude::*;
use bevy_rapier2d::physics::EventQueue;

pub fn handle_physics_events_system(events: Res<EventQueue>) {
    let mut recieved_event = false;

    while let Ok(event) = events.proximity_events.pop() {
        println!("Proximity event: {:?}", event);
        recieved_event = true;
    }

    while let Ok(event) = events.contact_events.pop() {
        println!("Contact event: {:?}", event);
        recieved_event = true;
    }

    if recieved_event {
        println!();
    }
}
