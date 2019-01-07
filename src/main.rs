mod hello;
mod universe;
use universe::Universe;

fn main() {
    hello::hello();
    println!("");

    let mut universe = Universe::new(8, 8);

    println!("universe:\n{}", universe);
    println!("universe.get_index(1,3) = {}", universe.get_index(1,3));
    println!("universe.get(1,3) = {}", universe.get(1,3));
    println!("universe.sum() = {}", universe.sum());

    println!("");
    universe.tick();
    println!("universe.tick()");
    println!("universe:\n{}", universe);
}
