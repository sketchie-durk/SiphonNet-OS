pub mod core;
pub mod plugins;
pub mod analysis;

use crate::core::engine::Engine;
use bytes::BytesMut;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;

async fn handle_connection(mut socket: TcpStream, engine: std::sync::Arc<Engine>) -> Result<()> {
    let mut buf = [0u8; 4096];
    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 { break; }

        let mut data = bytes::Bytes::copy_from_slice(&buf[..n]);
        
        // Phase 3: Integration of Traffic Analysis
        let report = crate::analysis::analyzer::TrafficAnalyzer::analyze_payload(&data);
        tracing::info!("Traffic Report: {:?}", report);

        if engine.process_request(&mut data).await? {
            socket.write_all(&data).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut engine = Engine::new();
    
    // Phase 2: Plugin Integration
    engine.add_plugin(crate::plugins::siphon::PacketSiphon::new(b"GET"));
    
    let engine_arc = std::sync::Arc::new(engine);
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("⚡ SiphonNet-OS Operational on 127.0.0.1:8080");
    
    loop {
        let (socket, _) = listener.accept().await?;
        let eng = engine_arc.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, eng).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}
