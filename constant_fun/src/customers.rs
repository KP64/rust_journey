#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
struct Customer<'a> {
    name: &'a str,
    age: u8,
}

const fn nth<T, const N: usize>(items: [T; N], index: usize) -> T
where
    T: Copy,
{
    items[index]
}

pub(crate) fn customers() {
    println!("Customers:");

    const CUSTOMERS: [Customer; 2] = [
        Customer {
            name: "John",
            age: 30,
        },
        Customer {
            name: "Jane",
            age: 25,
        },
    ];

    const NTH_CUSTOMER: Customer = nth(CUSTOMERS, 1);
    println!("{:?}", NTH_CUSTOMER);
}
