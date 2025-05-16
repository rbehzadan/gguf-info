use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;
use gguf_rs::get_gguf_container;
use serde_json::json;

/// Display key GGUF model info or raw metadata in JSON format
#[derive(Parser)]
#[command(name = "llminfo")]
#[command(version = "0.2.0")]
#[command(about = "Inspect GGUF model metadata", long_about = None)]
struct Args {
    /// Path to the GGUF model
    #[arg(short, long)]
    model: PathBuf,

    /// Output full metadata section as JSON
    #[arg(long)]
    metadata: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut container = match get_gguf_container(args.model.to_str().unwrap()) {
        Ok(c) => c,
        Err(e) => {
            let error = json!({ "error": format!("Failed to load GGUF: {}", e) });
            println!("{}", serde_json::to_string_pretty(&error)?);
            return Ok(());
        }
    };

    let model = match container.decode() {
        Ok(m) => m,
        Err(e) => {
            let error = json!({ "error": format!("Failed to decode model: {}", e) });
            println!("{}", serde_json::to_string_pretty(&error)?);
            return Ok(());
        }
    };

    let metadata = model.metadata();

    if args.metadata {
        let out = json!({ "metadata": metadata });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    // Utility closure to extract a value from known prefixes
    let get = |suffix: &str| {
        for prefix in ["qwen3", "llama", "mistral", "gemma", "bloom"] {
            let key = format!("{prefix}.{suffix}");
            if let Some(value) = metadata.get(&key) {
                return Some(value.clone());
            }
        }
        None
    };

    let output = json!({
        "model_architecture": model.model_family(),
        "number_of_parameters": model.model_parameters(),
        "file_type": model.file_type(),
        "num_tensors": model.num_tensor(),
        "context_length": get("context_length").unwrap_or(json!(null)),
        "embedding_length": get("embedding_length").unwrap_or(json!(null)),
        "block_count": get("block_count").unwrap_or(json!(null)),
        "attention_head_count": get("attention.head_count").unwrap_or(json!(null)),
        "feed_forward_length": get("feed_forward_length").unwrap_or(json!(null)),
        "rope_freq_base": get("rope.freq_base").unwrap_or(json!(null)),
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

