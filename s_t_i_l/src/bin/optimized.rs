use std::{fmt, mem};

#[derive(Copy, Clone, Debug)]
struct User<'u> {
    email: &'u str,
    username: &'u str,
    active: bool,
    sign_in_count: usize,
}

impl Default for User<'_> {
    fn default() -> Self {
        Self {
            email: Default::default(),
            username: Default::default(),
            active: true,
            sign_in_count: 1,
        }
    }
}

impl<'u> fmt::Display for User<'u> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl<'b> User<'b> {
    const fn new(email: &'b str, username: &'b str) -> Self {
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

impl<'s> User<'s> {
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

impl<'g> User<'g> {
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

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let email = "karamalsadeh@hotmail.com";
    let name = "Karam";

    let user = User::new(email, name);
    /*
     * println!("{}", us.get_email());
     * println!("{}", us.get_username());
     * println!("{}", us.get_active());
     */
    println!("{user}");
    println!(
        "User {} Mb: {}",
        user.get_username(),
        mem::size_of_val(&user)
    );

    println!();

    let email = "alsadehjamal@gmail.com";
    let name = "Arij";

    let user = User::default()
        .set_email(email)
        .set_username(name)
        .set_active(false);
    println!("My_Display: {user}");
    println!(
        "User {} Mb: {}",
        user.get_username(),
        mem::size_of_val(&user)
    );
}
