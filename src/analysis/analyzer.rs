use bytes::{Bytes, BytesMut};
use anyhow::Result;
use tracing::info;

pub struct TrafficAnalyzer;

impl TrafficAnalyzer {
    pub fn analyze_payload(data: &Bytes) -> AnalysisReport {
        let mut report = AnalysisReport::default();
        report.payload_len = data.len();
        
        // Calculate entropy for encrypted vs plaintext detection
        report.entropy = Self::calculate_entropy(data);
        report.is_encrypted = report.entropy > 7.5;
        
        // Check for common protocol signatures
        if data.starts_with(b"HTTP") {
            report.protocol = Some("HTTP".to_string());
        } else if data.starts_with(b"\x16\x03") {
            report.protocol = Some("TLS".to_string());
        }
        
        report
    }

    fn calculate_entropy(data: &Bytes) -> f64 {
        if data.is_empty() { return 0.0; }
        let mut counts = [0usize; 256];
        for &b in data { counts[b as usize] += 1; }
        
        let mut entropy = 0.0;
        for &count in &counts {
            if count > 0 {
                let p = count as f64 / data.len() as f64;
                entropy -= p * p.log2();
            }
        }
        entropy
    }
}

#[derive(Default, Debug)]
pub struct AnalysisReport {
    pub payload_len: usize,
    pub entropy: f64,
    pub is_encrypted: bool,
    pub protocol: Option<String>,
}
