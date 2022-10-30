struct WillSayGoodbye<'a>(&'a str);

impl<'a> Drop for WillSayGoodbye<'a> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

const GOODBYE_IN_GERMAN: WillSayGoodbye = WillSayGoodbye("Auf Wiedersehen!");

pub fn german_goodbye() {
    println!("German Goodbye:");
    let _goodbye_sayer = GOODBYE_IN_GERMAN;
}
