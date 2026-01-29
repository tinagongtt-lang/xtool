# xtool 🛠️

`xtool` 是一个用 Rust 编写的高性能工具函数库。它的结构类似于 Python 的多级包管理，通过主库统一导出各个子模块。

目前已实现：
* **xmath**: 基础数学函数库（包含泰勒级数实现的三角函数）。

## 🚀 安装

在你的 Rust 项目的 `Cargo.toml` 中添加以下依赖（类似于 Python 的 `pip install git+...`）：

```toml
[dependencies]
xtool = { git = "[https://github.com/你的用户名/xtool.git](https://github.com/你的用户名/xtool.git)" }
```
## 📖 使用示例
你可以像在 Python 中使用 from xtool.xmath import taylor_sin 一样调用它：
```rust
use xtool::xmath;

fn main() {
    let radians = 1.0;
    // 使用泰勒展开计算正弦值
    let result = xmath::taylor_sin(radians, 15);
    
    println!("sin({}) 的计算结果为: {}", radians, result);
}
```
## 🧪 模块详情
xmath
该模块专注于数学计算，不依赖于标准库以外的复杂包。

 - taylor_sin(x: f64, precision: usize) -> f64

    - x: 弧度值。

    - precision: 迭代次数，建议值 10-20 以达到最佳精度平衡。

## 🛠️ 开发与测试
如果你想克隆本项目并运行测试：
```bash
git clone [https://github.com/tinagongtt-lang/xtool.git](https://github.com/tinagongtt-lang/xtool.git)
cd xtool
cargo test -p xmath
```
## 📄 开源协议
MIT
[LICENSE](https://github.com/tinagongtt-lang/xtool/blob/main/LICENSE "LICENSE")