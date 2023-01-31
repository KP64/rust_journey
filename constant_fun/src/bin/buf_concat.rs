/// Gets total length of all provided strings
const fn len(strs: &[&str]) -> usize {
    let mut result = 0;
    let mut remaining = strs;

    // Note that we cant use iter() here because
    // it is not a const expression. Therefore, we
    // have to iterate using slice deconstruction.
    while let [current, tail @ ..] = remaining {
        result += current.len();
        remaining = tail;
    }

    result
}

/// Helper struct for concatenation of strings in const fn.
#[derive(Debug)]
struct Buf<const N: usize>([u8; N]);

/// Concatenates all provided strings into a single string
/// ! *at compile time*.
const fn concat<const N: usize>(strs: &[&str]) -> Buf<N> {
    let mut buffer = [0; N];
    let mut position_in_buffer = 0;

    let mut remaining = strs;
    while let [current, tail @ ..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;

        // Note that we cant use copy_from_slice because
        // mutable references arent allowed in const functions.
        // buffe.copy_from_slice(x);

        // Note that for loop isnt (yet) allowed in const functions.
        // We have to use while instead.
        while i < x.len() {
            buffer[position_in_buffer] = x[i];
            position_in_buffer += 1;
            i += 1;
        }
        remaining = tail;
    }

    Buf(buffer)
}

fn main() {
    println!("Buf Concat:");

    const STRS: &[&str; 13] = &[
        "hi", " ", "there", " ", "my", " ", "name", " ", "is", " ", "KG", " ", "64",
    ];
    const LEN: usize = len(STRS);

    println!("Len: {LEN:?}");

    // * TurboFish Syntax
    let conc = concat::<LEN>(STRS);
    println!("TFS: {conc:?}");

    // * Explicit Type Syntax
    let conc: Buf<LEN> = concat(STRS);
    println!("ETS: {conc:?}");
}
