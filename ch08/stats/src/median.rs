// ベクタ使ってない。使ってなくない？
pub fn median(xs: &mut [i32]) -> Option<i32> {
    xs.sort_unstable();
    let &x = xs.get(xs.len() / 2)?;
    Some(x)
}

#[test]
fn test_median() {
    assert_eq!(median(&mut []), None);
    assert_eq!(median(&mut [1, 1, 4]), Some(1));
    assert_eq!(median(&mut [5, 1, 4]), Some(4));
}
