use std::mem;

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: usize,
}

// * Interfaces + Implementations for User
trait Build {
    fn new(email: String, username: String) -> Self;

    fn empty() -> Self;
}
impl Build for User {
    fn new(email: String, username: String) -> Self {
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn empty() -> Self {
        Self {
            email: String::from(""),
            username: String::from(""),
            active: true,
            sign_in_count: 1,
        }
    }
}

trait SetInfo {
    fn set_name(self, username: String) -> Self;
    fn set_email(self, email: String) -> Self;
    fn set_active(self, active: bool) -> Self;
}
impl SetInfo for User {
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

trait GetInfo {
    fn get_name(self) -> Self;
    fn get_email(self) -> Self;
    fn get_active(self) -> Self;
    fn get_sign_in_count(self) -> Self;
}
impl GetInfo for User {
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
    println!("{:?}", us1);
    let us2 = User::empty()
        .set_email(email2)
        .set_name(name2)
        .get_name()
        .get_email();
    println!("{:?}", us2);

    println!("Us1Mb: {}", mem::size_of_val(&us1));
    println!("Us2Mb: {}", mem::size_of_val(&us2));
}
