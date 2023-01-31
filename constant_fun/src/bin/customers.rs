#[derive(Debug, Copy, Clone)]
struct Customer<'a> {
    _name: &'a str,
    _age: u8,
}

impl<'a> Customer<'a> {
    const fn new(name: &'a str, age: u8) -> Self {
        Self {
            _name: name,
            _age: age,
        }
    }
}

const fn nth<T, const N: usize>(items: [T; N], index: usize) -> T
where
    T: Copy,
{
    items[index]
}

fn main() {
    println!("Customers:");

    const CUSTOMERS: [Customer; 2] = [Customer::new("John", 30), Customer::new("Jane", 25)];

    const NTH_CUSTOMER: Customer = nth(CUSTOMERS, 1);
    println!("{NTH_CUSTOMER:?}");
}
