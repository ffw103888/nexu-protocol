// 导出C-ABI交互模块和因果链核心模块
pub mod ffi;
pub mod stitch;

// 对外暴露核心函数和错误类型，供上层调用
pub use stitch::{stitch_causal_chain, NexuError};
