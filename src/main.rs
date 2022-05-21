mod examples;

fn main() {
    let example_to_load = "guessing_game";

    match example_to_load {
        "hello_world" => examples::hello_world::hello_world(),
        "guessing_game" => examples::guessing_game::guessing_game(),
        _ => println!("Example not found")
    }
    // hello_world();
}
