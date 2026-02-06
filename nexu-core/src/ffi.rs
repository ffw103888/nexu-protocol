use libc::{c_char, c_uint, c_void};
use sha2::{Sha256, Digest};
use std::ffi::CStr;
use std::ptr;

/// C语言调用的SHA256哈希函数，返回C字符串指针（需手动调用nexu_free释放）
#[no_mangle]
pub extern "C" fn nexu_hash(input: *const c_char, len: c_uint) -> *mut c_char {
    // 空指针/空长度直接返回NULL（失败关闭原则，不静默处理）
    if input.is_null() || len == 0 {
        return ptr::null_mut();
    }

    // 安全转换C字符串到Rust字符串
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // 计算SHA256哈希并转为十六进制字符串
    let mut hasher = Sha256::new();
    hasher.update(input_str.as_bytes());
    let result = hasher.finalize();
    let hex_str = hex::encode(result);

    // 分配C兼容内存，存储十六进制结果（C字符串需以\0结尾）
    let c_hex = unsafe {
        libc::malloc((hex_str.len() + 1) * std::mem::size_of::<c_char>()) as *mut c_char
    };
    if c_hex.is_null() {
        return ptr::null_mut();
    }
    // 逐字节拷贝结果到C内存
    for (i, &byte) in hex_str.as_bytes().iter().enumerate() {
        unsafe { *c_hex.add(i) = byte as c_char; }
    }
    // C字符串末尾补0
    unsafe { *c_hex.add(hex_str.len()) = 0; }

    c_hex
}

/// C语言调用的内存释放函数，释放nexu_hash分配的C字符串指针
#[no_mangle]
pub extern "C" fn nexu_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { libc::free(ptr as *mut c_void); }
    }
}
