trait Animal {
    fn talk(&self);
}
struct Cat;
impl Animal for Cat {
    fn talk(&self) {
        println!("meow");
    }
}

struct Dog;
impl Animal for Dog {
    fn talk(&self) {
        println!("bark");
    }
}

/// ? Will generate 2 Functions for each Animal.
/// ? Achieves this with function name dangling (for each type).
/// ! It Allows the impl Animal to be a reference (&impl Animal).
fn static_animal_talk(a: &impl Animal) {
    a.talk();
}

/// ? Will just accept "anything" that implements the Animal Trait.
/// ? Achieves this by only accepting a reference (&, Box, Rc) since
/// ? References are bound to be valid (point to valid memory).
fn dyn_animal_talk(a: &dyn Animal) {
    a.talk();
}

fn main() {
    println!("Static vs Dynamic Dispatch:");
    let d = Dog;
    let c = Cat;
    dyn_animal_talk(&d);
    dyn_animal_talk(&c);

    static_animal_talk(&d);
    static_animal_talk(&c);
}
