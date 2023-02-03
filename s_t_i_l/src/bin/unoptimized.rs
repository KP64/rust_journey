#![allow(unused)]
use std::mem;

#[derive(Debug)]
#[rustfmt::skip]
struct User {
    email        : String,
    username     : String,
    active       : bool,
    sign_in_count: usize,
}

impl Default for User {
    fn default() -> Self {
        Self {
            email: Default::default(),
            username: Default::default(),
            active: true,
            sign_in_count: 1,
        }
    }
}

// * Interfaces + Implementations for User

impl User {
    const fn new(email: String, username: String) -> Self {
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

impl User {
    fn set_name(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    fn set_email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    fn set_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl User {
    fn get_name(self) -> Self {
        println!("SelfUser: {}", self.username);
        self
    }
    fn get_email(self) -> Self {
        println!("SelfEmaiL: {}", self.email);
        self
    }
    fn get_active(self) -> Self {
        println!("SelfEmaiL: {}", self.active);
        self
    }
    fn get_sign_in_count(self) -> Self {
        println!("SelfEmaiL: {}", self.sign_in_count);
        self
    }
}

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let email1 = String::from("karamalsadeh@hotmail.com");
    let name1 = String::from("Karam");

    let email2 = String::from("alsadehjamal@gmail.com");
    let name2 = String::from("Arij");

    let us1 = User::new(email1, name1).get_name().get_email();
    println!("{us1:?}");
    let us2 = User::default()
        .set_email(email2)
        .set_name(name2)
        .get_name()
        .get_email();
    println!("{us2:?}");

    println!("Us1Mb: {}", mem::size_of_val(&us1));
    println!("Us2Mb: {}", mem::size_of_val(&us2));
}
