use std::f64::consts::PI;

/// 精度阈值：当项小于此值时停止迭代
const EPSILON: f64 = 1e-15;
/// 最大迭代次数：防止特殊情况下进入死循环
const MAX_ITER: usize = 100;

/// 使用泰勒级数计算 sin(x)
/// 公式: sin(x) = x - x^3/3! + x^5/5! - ...
pub fn sin(mut x: f64) -> f64 {
    // 周期规约到 [-PI, PI]
    x %= 2.0 * PI;
    if x > PI { x -= 2.0 * PI; }
    if x < -PI { x += 2.0 * PI; }

    let mut res = 0.0;
    let mut term = x;
    let x_sq = x * x;

    for n in 1..MAX_ITER {
        res += term;
        // 下一项系数: -x^2 / (2n * (2n + 1))
        term *= -x_sq / ((2 * n) * (2 * n + 1)) as f64;
        if term.abs() < EPSILON { break; }
    }
    res
}

/// 使用泰勒级数计算 cos(x)
/// 公式: cos(x) = 1 - x^2/2! + x^4/4! - ...
pub fn cos(mut x: f64) -> f64 {
    // 周期规约到 [-PI, PI]
    x %= 2.0 * PI;
    if x > PI { x -= 2.0 * PI; }
    if x < -PI { x += 2.0 * PI; }

    let mut res = 0.0;
    let mut term = 1.0;
    let x_sq = x * x;

    for n in 1..MAX_ITER {
        res += term;
        // 下一项系数: -x^2 / ((2n - 1) * 2n)
        term *= -x_sq / ((2 * n - 1) * (2 * n)) as f64;
        if term.abs() < EPSILON { break; }
    }
    res
}

/// 使用泰勒级数计算 e^x
/// 公式: e^x = 1 + x + x^2/2! + x^3/3! + ...
pub fn exp(x: f64) -> f64 {
    let mut res = 1.0;
    let mut term = 1.0;

    for n in 1..MAX_ITER {
        term *= x / n as f64;
        res += term;
        if term.abs() < EPSILON { break; }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_accuracy() {
        let x = 1.0;
        // 验证 sin
        assert!((sin(x) - x.sin()).abs() < 1e-10);
        // 验证 cos
        assert!((cos(x) - x.cos()).abs() < 1e-10);
        // 验证 exp
        assert!((exp(x) - x.exp()).abs() < 1e-10);
    }

    #[test]
    fn test_large_angles() {
        let x = 10.0 * PI; // 测试规约能力
        assert!((sin(x) - 0.0).abs() < 1e-10);
    }
}