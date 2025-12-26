//! VeriMantle CLI
//!
//! Single binary that runs anywhere.
//!
//! Usage:
//!   verimantle run     # Start with auto-detection
//!   verimantle detect  # Show detected environment
//!   verimantle config  # Show auto-generated config

use verimantle_runtime::{detect_environment, auto_configure, VERSION};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("run");
    
    match command {
        "run" => {
            println!("VeriMantle v{}", VERSION);
            println!("The Universal AI Agent Kernel");
            println!();
            
            if let Err(e) = verimantle_runtime::run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        
        "detect" => {
            let env = detect_environment();
            println!("Detected Environment:");
            println!("{:#?}", env);
        }
        
        "config" => {
            let env = detect_environment();
            let config = auto_configure(&env);
            println!("Auto-Generated Configuration:");
            println!("{:#?}", config);
        }
        
        "version" | "-v" | "--version" => {
            println!("VeriMantle v{}", VERSION);
        }
        
        "help" | "-h" | "--help" => {
            print_help();
        }
        
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("VeriMantle - The Universal AI Agent Kernel");
    println!();
    println!("USAGE:");
    println!("  verimantle <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("  run      Start VeriMantle with auto-detection");
    println!("  detect   Show detected environment");
    println!("  config   Show auto-generated configuration");
    println!("  version  Show version");
    println!("  help     Show this help");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("  PORT             HTTP port (default: 3000)");
    println!("  GRPC_PORT        gRPC port (default: 50051)");
    println!("  BIND_ADDRESS     Bind address (default: 0.0.0.0)");
    println!("  DATABASE_URL     Database connection URL");
    println!("  CACHE_URL        Cache connection URL");
    println!();
    println!("VeriMantle auto-detects:");
    println!("  - Container (Docker, Podman)");
    println!("  - Kubernetes");
    println!("  - Serverless");
    println!("  - Edge devices");
    println!("  - Bare metal servers");
}
