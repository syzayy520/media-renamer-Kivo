// src-tauri/src/tmdb_search/transport/transport_gate.rs
// Transport Gate
// 职责：控制 TMDb live HTTP transport 的启用/禁用

use std::sync::atomic::{AtomicBool, Ordering};

/// Transport Gate 控制 live HTTP transport 的启用状态
///
/// 默认禁用，必须显式启用才能进行真实 HTTP 调用
/// 用于安全保护，防止意外的网络请求
pub struct TransportGate {
    /// 是否启用 live transport
    enabled: AtomicBool,
}

impl TransportGate {
    /// 创建新的 Transport Gate（默认禁用）
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
        }
    }

    /// 创建已启用的 Transport Gate
    pub fn enabled() -> Self {
        Self {
            enabled: AtomicBool::new(true),
        }
    }

    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// 启用 live transport
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// 禁用 live transport
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }
}

impl Default for TransportGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_gate_disabled() {
        let gate = TransportGate::new();
        assert!(!gate.is_enabled());
    }

    #[test]
    fn test_default_trait_disabled() {
        let gate = TransportGate::default();
        assert!(!gate.is_enabled());
    }

    #[test]
    fn test_enabled_constructor() {
        let gate = TransportGate::enabled();
        assert!(gate.is_enabled());
    }

    #[test]
    fn test_enable_disable() {
        let gate = TransportGate::new();
        assert!(!gate.is_enabled());

        gate.enable();
        assert!(gate.is_enabled());

        gate.disable();
        assert!(!gate.is_enabled());
    }

    #[test]
    fn test_toggle() {
        let gate = TransportGate::new();

        gate.enable();
        assert!(gate.is_enabled());

        gate.disable();
        assert!(!gate.is_enabled());

        gate.enable();
        assert!(gate.is_enabled());
    }
}
