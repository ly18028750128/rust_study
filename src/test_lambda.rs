pub fn exec_lambda(a: f64, b: f64, c: f64) -> f64 {
    let calculate = |a, b, c| a * b + c;
    let mut idx:i64 = 0;
    while idx < 100 {
        // 循环体
        calculate(a, b, c);
        idx += 1;
    }
    calculate(a, b, c)
}
