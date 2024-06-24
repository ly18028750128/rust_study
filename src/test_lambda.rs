pub fn exec_lambda(a: u32, b: u32, c: u32) -> u32 {
    let calculate = |a, b, c| a * b + c;
    let mut idx:i64 = 0;
    while idx < 100_0000_0000 {
        // 循环体
        calculate(a, b, c);
        idx += 1;
    }
    calculate(a, b, c)
}
