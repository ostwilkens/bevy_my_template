use web_sys::{CustomEvent, CustomEventInit, Window, EventTarget};
use bevy::prelude::*;

fn trigger_custom_event(event_name: &str) {
    let window = web_sys::window().expect("could not get window");

    let event = CustomEvent::new_with_event_init_dict(
        event_name,
        &CustomEventInit::new().bubbles(true).cancelable(true),
    ).expect("Could not create custom event");

    let target: &EventTarget = window.as_ref();
    target.dispatch_event(&event).expect("Could not dispatch custom event");
}

pub fn send_loaded_event() {
    info!("Sending AssetsLoaded event");
    trigger_custom_event("AssetsLoaded");
}