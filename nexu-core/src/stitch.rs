use sha2::{Sha256, Digest};
use thiserror::Error;

/// Nexu协议核心错误类型，枚举所有可能的失败场景
#[derive(Error, Debug)]
pub enum NexuError {
    #[error("invalid input: empty or malformed string")]
    InvalidInput,
    #[error("stitch failed: causal chain hash mismatch")]
    ChainMismatch,
}

/// 因果链拼接核心函数：输入字符串列表→链式哈希结果
/// 规则：输入非空→逐个哈希→哈希结果再整体哈希→返回十六进制字符串
pub fn stitch_causal_chain(inputs: &[&str]) -> Result<String, NexuError> {
    // 空输入/包含空字符串，直接返回错误（失败关闭）
    if inputs.is_empty() || inputs.iter().any(|s| s.is_empty()) {
        return Err(NexuError::InvalidInput);
    }

    // 逐个计算输入的SHA256哈希
    let mut hashes = Vec::new();
    for input in inputs {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hashes.push(hasher.finalize());
    }

    // 拼接所有哈希结果，计算最终的因果链哈希
    let mut chain_hasher = Sha256::new();
    for hash in hashes {
        chain_hasher.update(hash);
    }
    let final_hash = chain_hasher.finalize();

    Ok(hex::encode(final_hash))
}

// 单元测试，验证核心逻辑正确性
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stitch_valid_input() {
        let inputs = &["cold-truth", "nexu-v2.3", "deterministic"];
        let result = stitch_causal_chain(inputs).unwrap();
        assert_ne!(result, "");
    }

    #[test]
    fn test_stitch_invalid_input() {
        let inputs = &["", "empty-input"];
        assert!(stitch_causal_chain(inputs).is_err());
    }
}
