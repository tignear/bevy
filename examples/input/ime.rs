use bevy::{prelude::*, input::ime::Ime};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(ime_control)
        .add_system(ime_print)
        .run();
}

fn ime_control(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_ime_allowed(true);
}

fn ime_print(mut ime_event: EventReader<Ime>){
    for event in ime_event.iter() {
        info!("{:?}", event);
    }
}