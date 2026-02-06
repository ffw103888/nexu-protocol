package main

/*
#cgo LDFLAGS: -L/root/nexu-protocol/nexu-core/target/x86_64-unknown-linux-gnu/release -lnexu_core
#include "/root/nexu-protocol/nexu-core/include/nexu_core.h"
// 手动声明C字符串长度计算函数（辅助Go传参）
#include <string.h>
*/
import "C"
import (
    "fmt"
    "unsafe"
)

// 暴露Go函数供外部调用（适配C-ABI，处理参数和内存释放）
//export NexuCalculateHash
func NexuCalculateHash(input *C.char) *C.char {
    // 1. 计算输入字符串长度（匹配nexu_hash需要的unsigned int len参数）
    inputLen := C.unsigned(C.strlen(input))
    
    // 2. 调用正确的C函数：nexu_hash（替换错误的nexu_sha256_hash）
    hashResult := C.nexu_hash(input, inputLen)
    
    // 注意：若上层应用需要手动释放内存，需调用C.nexu_free(hashResult)
    // 此处返回结果，由调用方负责释放，避免提前释放导致空指针
    return hashResult
}

// 暴露内存释放函数（供外部调用，释放哈希结果）
//export NexuFreeHash
func NexuFreeHash(ptr *C.char) {
    C.nexu_free(ptr)
}

func main() {
    // 测试示例（可选，验证编译后功能）
    testInput := C.CString("test_nexu_hash")
    defer C.free(unsafe.Pointer(testInput)) // 释放测试输入的内存
    
    hash := NexuCalculateHash(testInput)
    fmt.Printf("SHA256 Hash Result: %s\n", C.GoString(hash))
    
    // 释放哈希结果内存
    NexuFreeHash(hash)
    
    fmt.Println("Nexu Go SDK compiled successfully")
}
