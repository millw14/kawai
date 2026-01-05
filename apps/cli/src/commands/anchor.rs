//! Anchor command handlers

use crate::AnchorCommands;
use colored::*;
use kawai_anchor::{Anchor, idl::Idl};

pub async fn handle(cmd: AnchorCommands) {
    match cmd {
        AnchorCommands::Init { name } => {
            init_anchor(&name).await;
        }
        AnchorCommands::Build { path, verbose } => {
            build_anchor(&path, verbose).await;
        }
        AnchorCommands::Test { path, skip_build } => {
            test_anchor(&path, skip_build).await;
        }
        AnchorCommands::Deploy { path, cluster } => {
            deploy_anchor(&path, &cluster).await;
        }
        AnchorCommands::Idl { path, output } => {
            generate_idl(&path, output).await;
        }
    }
}

async fn init_anchor(name: &str) {
    println!("{} Initializing Anchor project: {}", "⚓".bright_yellow(), name.bright_cyan());
    println!();

    match Anchor::init(name).await {
        Ok(project) => {
            println!("{} Project created!", "✅".bright_green());
            println!();
            println!("   {} {}", "Location:".bright_cyan(), project.root().display());
            println!();
            println!("{}", "Next steps:".bright_yellow());
            println!("   cd {}", name);
            println!("   kawai anchor build");
            println!("   kawai anchor test");
        }
        Err(e) => {
            println!("{} Failed to create project: {}", "❌".bright_red(), e);
        }
    }
    println!();
}

async fn build_anchor(path: &str, verbose: bool) {
    println!("{} Building Anchor project...", "⚓".bright_yellow());
    println!();

    println!("   {} {}", "Project:".bright_cyan(), path);
    println!();

    match Anchor::build(path).await {
        Ok(result) => {
            println!("{} Build successful!", "✅".bright_green());
            println!();
            println!("   {} {}", "Output:".bright_cyan(), result.so_path.display());
            println!("   {} {}", "Program ID:".bright_cyan(), result.program_id);
            println!("   {} {:.2}s", "Duration:".bright_cyan(), result.duration_secs);
            println!("   {} {} bytes", "Size:".bright_cyan(), result.size_bytes);
            
            if let Some(idl) = result.idl_path {
                println!("   {} {}", "IDL:".bright_cyan(), idl.display());
            }
        }
        Err(e) => {
            println!("{} Build failed!", "❌".bright_red());
            println!();
            println!("{}", e);
        }
    }
    println!();
}

async fn test_anchor(path: &str, skip_build: bool) {
    println!("{} Running Anchor tests...", "🧪".bright_yellow());
    println!();

    if !skip_build {
        println!("   Building first...");
        if let Err(e) = Anchor::build(path).await {
            println!("{} Build failed: {}", "❌".bright_red(), e);
            return;
        }
        println!("   Build complete!");
        println!();
    }

    match Anchor::load(path) {
        Ok(project) => {
            match project.test().await {
                Ok(result) => {
                    if result.passed {
                        println!("{} Tests passed!", "✅".bright_green());
                    } else {
                        println!("{} Tests failed!", "❌".bright_red());
                    }
                    println!();
                    println!("{}", result.output);
                }
                Err(e) => {
                    println!("{} Test error: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Failed to load project: {}", "❌".bright_red(), e);
        }
    }
}

async fn deploy_anchor(path: &str, cluster: &str) {
    println!("{} Deploying Anchor program to {}...", "🚢".bright_yellow(), cluster.bright_cyan());
    println!();

    match Anchor::deploy(path, cluster).await {
        Ok(result) => {
            println!("{} Deployment successful!", "✅".bright_green());
            println!();
            println!("   {} {}", "Program ID:".bright_cyan(), result.program_id);
            println!("   {} {}", "Cluster:".bright_cyan(), result.cluster);
            println!("   {} {}", "Signature:".bright_cyan(), result.signature.dimmed());
        }
        Err(e) => {
            println!("{} Deployment failed: {}", "❌".bright_red(), e);
        }
    }
    println!();
}

async fn generate_idl(path: &str, output: Option<String>) {
    println!("{} Generating IDL...", "📋".bright_yellow());
    println!();

    match Anchor::load(path) {
        Ok(project) => {
            match project.idl() {
                Ok(Some(idl)) => {
                    if let Some(output_path) = output {
                        match idl.save(&output_path) {
                            Ok(()) => {
                                println!("{} IDL saved to {}", "✅".bright_green(), output_path);
                            }
                            Err(e) => {
                                println!("{} Failed to save: {}", "❌".bright_red(), e);
                            }
                        }
                    } else {
                        // Print to stdout
                        match serde_json::to_string_pretty(&idl) {
                            Ok(json) => println!("{}", json),
                            Err(e) => println!("{} Serialization error: {}", "❌".bright_red(), e),
                        }
                    }
                }
                Ok(None) => {
                    println!("{} No IDL found. Build the project first:", "⚠️".bright_yellow());
                    println!("   kawai anchor build");
                }
                Err(e) => {
                    println!("{} Failed to load IDL: {}", "❌".bright_red(), e);
                }
            }
        }
        Err(e) => {
            println!("{} Not an Anchor project: {}", "❌".bright_red(), e);
        }
    }
}

