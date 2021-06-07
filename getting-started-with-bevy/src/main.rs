use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

struct Name(String);
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaine Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("John Doe".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));

}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}
