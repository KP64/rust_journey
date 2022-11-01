use std::{fmt, mem};

#[derive(Copy, Clone, Debug)]
struct User<'u> {
    email: &'u str,
    username: &'u str,
    active: bool,
    sign_in_count: usize,
}

impl<'u> fmt::Display for User<'u> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            r#"{}:
    email: "{}",
    active: {},
    sign_in_count: {}"#,
            self.get_username(),
            self.get_email(),
            self.get_active(),
            self.get_sign_in_count()
        ))
    }
}

// * Interfaces + Implementations for User
trait Build<'b> {
    fn new(email: &'b str, username: &'b str) -> Self;

    fn empty_new() -> Self;
}
impl<'b> Build<'b> for User<'b> {
    fn new(email: &'b str, username: &'b str) -> Self {
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn empty_new() -> Self {
        Self {
            email: "",
            username: "",
            active: true,
            sign_in_count: 1,
        }
    }
}

trait SetInfo<'s> {
    fn set_username(self, username: &'s str) -> Self;
    fn set_email(self, email: &'s str) -> Self;
    fn set_active(self, active: bool) -> Self;
}
impl<'s> SetInfo<'s> for User<'s> {
    fn set_username(mut self, username: &'s str) -> Self {
        self.username = username;
        self
    }

    fn set_email(mut self, email: &'s str) -> Self {
        self.email = email;
        self
    }

    fn set_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

trait GetInfo<'g> {
    fn get_username(self) -> &'g str;
    fn get_email(self) -> &'g str;
    fn get_active(self) -> bool;
    fn get_sign_in_count(self) -> usize;
}

/*
! Note: This is not how a builder pattern works
? impl<'a> GetInfo for User<'a>
* Since lifetimes arent used in the functions
* lifetime can be anonymous:
? impl GetInfo for User<'_>
*/
/* impl GetInfo for User<'_> {
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
} */

impl<'g> GetInfo<'g> for User<'g> {
    fn get_username(self) -> &'g str {
        self.username
    }
    fn get_email(self) -> &'g str {
        self.email
    }
    fn get_active(self) -> bool {
        self.active
    }
    fn get_sign_in_count(self) -> usize {
        self.sign_in_count
    }
}

fn main() {
    let email = "karamalsadeh@hotmail.com";
    let name = "Karam";

    let user = User::new(email, name);
    /*
     * println!("{}", us.get_email());
     * println!("{}", us.get_username());
     * println!("{}", us.get_active());
     */
    println!("{}", user);
    println!(
        "User {} Mb: {}",
        user.get_username(),
        mem::size_of_val(&user)
    );

    println!();

    let email = "alsadehjamal@gmail.com";
    let name = "Arij";

    let user = User::empty_new()
        .set_email(email)
        .set_username(name)
        .set_active(false);
    println!("My_Display: {}", user);
    println!(
        "User {} Mb: {}",
        user.get_username(),
        mem::size_of_val(&user)
    );
}
