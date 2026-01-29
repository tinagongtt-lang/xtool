// 文件: xtool/src/lib.rs

// 1. 引入 xmath crate
// 2. 使用 `pub use` 将其重导出
// 这就像 Python 的: from . import xmath
pub use xmath;

// 如果你想让用户直接用 xtool::taylor_sin (而不是 xtool::xmath::taylor_sin)
// 你可以写: pub use xmath::taylor_sin; 
// 但为了保持结构清晰，通常建议保留模块名，如下：

/// 这是一个示例函数，证明 xtool 本身也可以有自己的功能
pub fn version() -> &'static str {
    "xtool v0.0.1"
}