use core::hash::Hash;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Cacher<T, U, V>
where
    T: Fn(&U) -> V,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: Eq + Hash,
{
    pub fn new(calculation: T) -> Self {
        Self {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> &V {
        match self.values.entry(arg) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
                entry.insert(v)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    #[test]
    fn it_works() {
        let count = RefCell::new(0);

        let mut cacher = Cacher::new(|x: &i32| {
            *count.borrow_mut() += 1;
            *x
        });

        cacher.value(0);

        assert_eq!(*count.borrow(), 1);

        cacher.value(0);

        assert_eq!(*count.borrow(), 1);

        cacher.value(1);

        assert_eq!(*count.borrow(), 2);
    }
}
