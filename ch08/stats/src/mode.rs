use std::collections::{HashMap, HashSet};

pub fn mode(xs: &[i32]) -> HashSet<i32> {
    let mut map = HashMap::new();

    for x in xs {
        *map.entry(x).or_insert(0) += 1;
    }

    let max = match map.iter().max_by_key(|&(_, v)| v) {
        None => return HashSet::new(),
        Some((_, &v)) => v,
    };

    map.into_iter()
        .filter(|&(_, v)| v == max)
        .map(|(&k, _)| k)
        .collect()
}

#[test]
fn test_mode() {
    assert_eq!(mode(&[]), HashSet::new());
    assert_eq!(mode(&[1, 1, 4]), HashSet::from([1]));
    assert_eq!(mode(&[5, 1, 4]), HashSet::from([5, 1, 4]));
}
