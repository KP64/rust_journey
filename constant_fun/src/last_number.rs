trait HasNumbers {
    // * Constant without value.
    const NUMBERS: [i32; 5];
    // * Constant with default values
    const LAST_NUMBER: i32 = 5;
}

#[derive(Debug)]
struct IHaveOtherNumbers;
impl HasNumbers for IHaveOtherNumbers {
    // * Here we override the default value from the trait.
    const LAST_NUMBER: i32 = 6;

    const NUMBERS: [i32; 5] = [1, 2, 3, 4, Self::LAST_NUMBER];
}

// ! Runs Code in Compile time instead of runtime
// ? Rarely Used
const _: () = {
    use std::marker::PhantomData;
    struct ImplementsMyTrait<T: HasNumbers>(PhantomData<T>);
    let _ = ImplementsMyTrait(PhantomData::<IHaveOtherNumbers>);
};

pub(crate) fn last_number() {
    println!("Last Number:");
    println!("{:?}", IHaveOtherNumbers::NUMBERS);
}
