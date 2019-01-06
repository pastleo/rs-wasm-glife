mod hello;
mod universe;

fn main() {
    hello::hello();
    println!("");

    let mut universe = universe::Universe::new(8, 8);

    println!("universe:\n{}", universe);
    println!("universe.get_index(1,3) = {}", universe.get_index(1,3));
    println!("universe.get(1,3) = {}", universe.get(1,3));
    println!("universe.live_neighbor_cnt(1,3) = {}", universe.live_neighbor_cnt(1,3));
    println!("universe.sum() = {}", universe.sum());

    println!("");
    universe.tick();
    println!("universe.tick()");
    println!("universe:\n{}", universe);
}
