use serde::Serialize;

#[derive(Serialize)]
struct Person {
    age: f64,
    name: String,
    friends: Vec<Person>,
}

fn main() {
    let alex = Person {
        age: 30.,
        name: "sander".to_owned(),
        friends: vec![Person {
            age: 55.33333,
            name: "Horst Schlämmer".to_owned(),
            friends: Vec::new()
        }]
    };

    std::fs::write("a.spa-json", spa_json::spa_json_serializer::to_string(&alex).unwrap()).unwrap()
}
