// ベクタ使ってない。使ってなくない？
pub fn mean(xs: &[i32]) -> Option<f64> {
    let len = xs.len();

    if len == 0 {
        None
    } else {
        Some(xs.iter().sum::<i32>() as f64 / len as f64)
    }
}

#[test]
fn test_mean() {
    assert_eq!(mean(&[]), None);
    assert_eq!(mean(&[1, 1, 4]), Some(2.0));
    assert_eq!(mean(&[8, 1, 0]), Some(3.0));
}
