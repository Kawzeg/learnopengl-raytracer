pub fn normalize<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> f64 {
    let a: f64 = x.into();
    let b: f64 = y.into();
    (a % b) / b
}
