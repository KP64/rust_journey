// * Trait: Array ordered?

trait IsOrdered {
    fn is_ordered(&self) -> bool;
}

// Const generics
impl<T, const N: usize> IsOrdered for [T; N]
where
    T: Ord + Copy,
{
    fn is_ordered(&self) -> bool {
        if self.is_empty() || self.len() == 1 {
            return true;
        }

        let mut prev = None;
        for item in self {
            if Some(item) < prev {
                return false;
            }
            prev = Some(item);
        }
        true
    }
}

pub(crate) fn ordered_arr() {
    println!("Ordered Array:");
    let numbers = [1, 3, 2, 45];
    println!("{}", numbers.is_ordered())
}
