#[allow(dead_code)]
struct Human<'a> {
    name: &'a str,
    age: u8,
    favorite_animal: &'a dyn Animal,
}
impl<'a> Human<'a> {
    const fn new(name: &'a str, age: u8, favorite_animal: &'a dyn Animal) -> Self {
        Self {
            name,
            age,
            favorite_animal,
        }
    }
}

// ! Const functions arent allowed to exist in traits
// ! Which is why const fn in Human impl is possible
// ! In contrast to the Animal Impls
trait Animal {
    fn make_sound<'a>(&self) -> &'a str;
}

struct Cat;
impl Animal for Cat {
    fn make_sound<'a>(&self) -> &'a str {
        "meow"
    }
}

struct Dog;
impl Animal for Dog {
    fn make_sound<'a>(&self) -> &'a str {
        "woof"
    }
}

#[allow(unused)]
const fn can_purr<'a>(can_purr: bool) -> &'a dyn Animal {
    match can_purr {
        true => &Cat,
        false => &Dog,
    }
}

// ! Dont forget that const functions can be
// ! used for non-const variables
pub(crate) fn animals() {
    println!("Animals:");

    // ? Const
    const JOHNNY: Human = Human::new("John", 36, &Dog);
    println!(
        "{}s favorite_animal makes {}",
        JOHNNY.name,
        JOHNNY.favorite_animal.make_sound()
    );

    // ? Non-Const
    let malica: Human = Human::new("Malica", 36, &Cat);
    println!(
        "{}s favorite_animal makes {}",
        malica.name,
        malica.favorite_animal.make_sound()
    );
}
