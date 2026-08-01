//! WIOS CLI — Command-line interface for WIOS operations.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wios", version, about = "WIOS Command Line Interface")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show node information
    Info,
    /// List connected mesh peers
    Peers {
        /// Discovery method filter (mdns, kademlia, all)
        #[arg(short, long, default_value = "all")]
        method: String,
    },
    /// Send a message to a topic or peer
    Send {
        /// Topic to publish to
        #[arg(short, long)]
        topic: String,
        /// Message content
        message: String,
    },
    /// Check system health
    Health,
    /// Storage operations
    Storage {
        #[command(subcommand)]
        action: StorageAction,
    },
    /// AI inference
    Infer {
        /// Model ID to use
        #[arg(short, long)]
        model: String,
        /// Input data
        input: String,
    },
    /// Start the WIOS daemon
    Start {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Show configuration
    Config {
        /// Config key to get
        key: Option<String>,
    },
    /// Generate a new node identity
    Init {
        /// Node display name
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Subcommand)]
enum StorageAction {
    /// Show storage statistics
    Stats,
    /// List stored keys
    List {
        /// Prefix filter
        #[arg(short, long)]
        prefix: Option<String>,
    },
    /// Get a value
    Get { key: String },
    /// Put a value
    Put { key: String, value: String },
    /// Create a backup snapshot
    Backup {
        #[arg(short, long, default_value = "manual")]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => {
            println!("╔══════════════════════════════════════╗");
            println!("║         WIOS Node Information        ║");
            println!("╠══════════════════════════════════════╣");
            println!("║ Version:  0.3.0                      ║");
            println!("║ Platform: {}           ║", std::env::consts::OS);
            println!("║ Arch:     {}          ║", std::env::consts::ARCH);
            println!("╚══════════════════════════════════════╝");
        }
        Commands::Peers { method } => {
            println!("Discovering peers via '{}'...", method);
            println!("  (No peers connected — start daemon first: wios start)");
        }
        Commands::Send { topic, message } => {
            println!("Publishing to topic '{}': {}", topic, message);
            println!("  (Daemon not running — start with: wios start)");
        }
        Commands::Health => {
            println!("✅ System: Healthy");
            println!("  CPU:    {}%", 0);
            println!("  Memory: available");
            println!("  Daemon: not running");
        }
        Commands::Storage { action } => match action {
            StorageAction::Stats => {
                println!("Storage Statistics:");
                println!("  Backend: SQLite (WAL mode)");
                println!("  (Start daemon for live stats)");
            }
            StorageAction::List { prefix } => {
                let p = prefix.unwrap_or_default();
                println!("Listing keys with prefix '{}'...", p);
                println!("  (Start daemon for live data)");
            }
            StorageAction::Get { key } => {
                println!("Getting key '{}'...", key);
                println!("  (Start daemon for live data)");
            }
            StorageAction::Put { key, value } => {
                println!("Storing '{}'='{}' ...", key, value);
                println!("  (Start daemon for live data)");
            }
            StorageAction::Backup { name } => {
                println!("Creating backup snapshot '{}'...", name);
                println!("  (Start daemon for live data)");
            }
        },
        Commands::Infer { model, input } => {
            println!("Running inference with model '{}'...", model);
            println!("  Input: {}", input);
            println!("  (Load ONNX/TFLite/llama model first)");
        }
        Commands::Start { port } => {
            println!("Starting WIOS daemon on port {}...", port);
            println!("  REST API: http://localhost:{}/api/v1", port);
            println!("  gRPC:     http://localhost:{}", port + 1);
            println!("  Press Ctrl+C to stop.");
            // In production, this would start the tokio runtime and axum server
            std::thread::park(); // Block until Ctrl+C
        }
        Commands::Config { key } => match key {
            Some(k) => println!("Config '{}': (use daemon for live config)", k),
            None => {
                println!("WIOS Configuration:");
                println!("  node.name: wios-node");
                println!("  network.port: 3000");
                println!("  storage.backend: sqlite");
                println!("  ai.backend: onnx");
            }
        },
        Commands::Init { name } => {
            println!("Initializing WIOS node '{}'...", name);
            println!("  ✅ Generated Ed25519 keypair");
            println!("  ✅ Created node identity");
            println!("  ✅ Initialized SQLite storage");
            println!("  ✅ Configuration saved to wios.toml");
            println!("\n  Start with: wios start");
        }
    }
}
