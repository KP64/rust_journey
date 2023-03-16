use rand::{thread_rng, Rng};

const OPTIONS: [&str; 3] = ["MK8", "MKW", "Monopoly"];
fn main() {
    let rng = thread_rng().gen_range(0..OPTIONS.len());
    dbg!(OPTIONS[rng]);
}
