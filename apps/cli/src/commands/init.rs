//! Project initialization command handler

use colored::*;
use std::fs;
use std::path::Path;

pub async fn handle(name: String, template: String) {
    println!("{} Initializing Solana project: {}", "🚀".bright_yellow(), name.bright_cyan());
    println!();

    let project_path = Path::new(&name);
    
    if project_path.exists() {
        println!("{} Directory '{}' already exists", "❌".bright_red(), name);
        return;
    }

    match fs::create_dir_all(&project_path) {
        Ok(()) => {}
        Err(e) => {
            println!("{} Failed to create directory: {}", "❌".bright_red(), e);
            return;
        }
    }

    match template.as_str() {
        "basic" => create_basic_project(&name, project_path),
        "anchor" => create_anchor_project(&name, project_path),
        "token" => create_token_project(&name, project_path),
        _ => {
            println!("{} Unknown template: {}", "❌".bright_red(), template);
            println!("   Available: basic, anchor, token");
            return;
        }
    }

    println!();
    println!("{} Project created successfully!", "✅".bright_green());
    println!();
    println!("{}", "Next steps:".bright_yellow());
    println!("   cd {}", name);
    println!("   cargo build");
    println!();
}

fn create_basic_project(name: &str, path: &Path) {
    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
kawai-sdk = "0.1"
tokio = {{ version = "1", features = ["full"] }}
"#, name);

    fs::write(path.join("Cargo.toml"), cargo_toml).ok();

    // Create src/main.rs
    let main_rs = r#"use kawai_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌸 Welcome to your Kawai Solana project!");
    
    // Connect to devnet
    let kawai = Kawai::devnet().await?;
    
    // Get network info
    let stats = kawai.stats().await?;
    println!("📊 Current slot: {}", stats.slot);
    
    Ok(())
}
"#;

    fs::create_dir_all(path.join("src")).ok();
    fs::write(path.join("src/main.rs"), main_rs).ok();

    // Create .gitignore
    let gitignore = r#"/target/
Cargo.lock
*.pem
*.json
"#;
    fs::write(path.join(".gitignore"), gitignore).ok();

    println!("   {} Created basic Solana project", "📁".bright_cyan());
}

fn create_anchor_project(name: &str, path: &Path) {
    // Create basic structure first
    create_basic_project(name, path);

    // Add Anchor-specific files
    let anchor_toml = format!(r#"[features]
seeds = false
skip-lint = false

[programs.devnet]
{} = "YOUR_PROGRAM_ID"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
"#, name.replace("-", "_"));

    fs::write(path.join("Anchor.toml"), anchor_toml).ok();

    // Create programs directory
    fs::create_dir_all(path.join("programs").join(name).join("src")).ok();

    let lib_rs = r#"use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🌸 Hello from Kawai!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
"#;

    fs::write(path.join("programs").join(name).join("src/lib.rs"), lib_rs).ok();

    println!("   {} Created Anchor project structure", "⚓".bright_cyan());
}

fn create_token_project(name: &str, path: &Path) {
    // Create basic structure first
    create_basic_project(name, path);

    // Create token-specific main.rs
    let main_rs = r#"use kawai_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌸 Token Project Template");
    println!();
    
    // Connect to devnet
    let kawai = Kawai::devnet().await?;
    
    // TODO: Add token creation logic
    // - Create mint account
    // - Create token accounts
    // - Mint tokens
    // - Transfer tokens
    
    println!("💡 This template helps you create SPL tokens on Solana");
    
    Ok(())
}
"#;

    fs::write(path.join("src/main.rs"), main_rs).ok();

    println!("   {} Created token project template", "🪙".bright_cyan());
}

