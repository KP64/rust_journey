#![feature(const_trait_impl, const_mut_refs, const_swap)]

pub mod sorting {

    #[allow(unused)]
    pub fn selection_sort<T>(arr: &mut [T])
    where
        T: Ord,
    {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
    }

    #[allow(unused)]
    pub fn bubble_sort<T>(arr: &mut [T])
    where
        T: Ord,
    {
        let len = arr.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    pub mod const_fun {

        #[allow(unused)]
        pub const fn const_selection_sort<T>(arr: &mut [T])
        where
            T: ~const std::cmp::PartialOrd,
        {
            let len = arr.len();
            let mut i = 0;
            while i < len {
                let mut min_idx = i;
                let mut j = i + 1;
                while j < len {
                    if arr[j] < arr[min_idx] {
                        min_idx = j;
                    }
                    j += 1;
                }
                arr.swap(min_idx, i);
                i += 1;
            }
        }

        #[allow(unused)]
        pub const fn const_bubble_sort<T>(arr: &mut [T])
        where
            T: ~const std::cmp::PartialOrd,
        {
            let len = arr.len();
            let mut i = 0;
            while i < len {
                let mut j = 0;
                while j < len - i - 1 {
                    if arr[j] > arr[j + 1] {
                        arr.swap(j, j + 1);
                    }
                    j += 1;
                }
                i += 1;
            }
        }
    }
}

pub mod searching {

    #[allow(unused)]
    pub fn binary_search<T, V>(arr: V, key: T) -> Option<usize>
    where
        T: Ord,
        V: AsRef<[T]>,
    {
        let arr = arr.as_ref();
        let mut low = 0;
        let mut high = arr.len();

        while low < high {
            let mid = low + (high - low) / 2;
            match arr[mid].cmp(&key) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Greater => high = mid,
            };
        }
        None
    }

    pub mod const_fun {

        #[allow(unused)]
        pub const fn const_binary_search<T>(arr: &[T], key: &T) -> Option<usize>
        where
            T: ~const Ord,
        {
            let mut low = 0;
            let mut high = arr.len();

            while low < high {
                let mid = low + (high - low) / 2;
                match arr[mid].cmp(key) {
                    std::cmp::Ordering::Less => low = mid + 1,
                    std::cmp::Ordering::Equal => return Some(mid),
                    std::cmp::Ordering::Greater => high = mid,
                }
            }
            None
        }
    }
}
