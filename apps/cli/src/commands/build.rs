//! Build command handlers

use crate::BuildCommands;
use colored::*;
use kawai_build::{Builder, config::BuildBackendPreference, toolchain::Toolchain};

pub async fn handle(cmd: BuildCommands) {
    match cmd {
        BuildCommands::Program { path, release, backend, verbose } => {
            build_program(&path, release, &backend, verbose).await;
        }
        BuildCommands::Status => {
            toolchain_status().await;
        }
        BuildCommands::InstallToolchain { backend } => {
            install_toolchain(&backend).await;
        }
    }
}

async fn build_program(path: &str, release: bool, backend: &str, verbose: bool) {
    println!("{} Building Solana program...", "🔨".bright_yellow());
    println!();

    let backend_pref = match backend {
        "docker" => BuildBackendPreference::Docker,
        "wsl" => BuildBackendPreference::Wsl,
        "cloud" => BuildBackendPreference::Cloud,
        _ => BuildBackendPreference::Auto,
    };

    let mut builder = Builder::new()
        .project_dir(path)
        .release(release)
        .backend(backend_pref);

    if verbose {
        builder = builder.verbose();
    }

    println!("   {} {}", "Project:".bright_cyan(), path);
    println!("   {} {}", "Mode:".bright_cyan(), if release { "release" } else { "debug" });
    println!("   {} {}", "Backend:".bright_cyan(), backend);
    println!();

    match builder.build().await {
        Ok(result) => {
            println!("{} Build successful!", "✅".bright_green());
            println!();
            println!("   {} {}", "Output:".bright_cyan(), result.so_path.display());
            println!("   {} {}", "Program ID:".bright_cyan(), result.program_id);
            println!("   {} {:.2}s", "Duration:".bright_cyan(), result.duration_secs);
            println!("   {} {} bytes", "Size:".bright_cyan(), result.size_bytes);
            println!("   {} {}", "Backend used:".bright_cyan(), result.backend);
            
            if let Some(idl) = result.idl_path {
                println!("   {} {}", "IDL:".bright_cyan(), idl.display());
            }
        }
        Err(e) => {
            println!("{} Build failed!", "❌".bright_red());
            println!();
            println!("{}", e);
            println!();
            println!("{}", "💡 Tips:".bright_yellow());
            println!("   • Make sure Docker is running (for Docker backend)");
            println!("   • Or ensure Solana tools are installed in WSL");
            println!("   • Run 'kawai build status' to check toolchain");
        }
    }
    println!();
}

async fn toolchain_status() {
    println!("{} Build Toolchain Status", "🔧".bright_yellow());
    println!();

    let toolchain = Toolchain::new();
    let status = toolchain.status().await;

    // Docker status
    if status.docker {
        println!("   {} Docker: {}", "✅".bright_green(), "Available".bright_green());
    } else {
        println!("   {} Docker: {}", "❌".bright_red(), "Not found".bright_red());
    }

    // WSL status
    if status.wsl {
        println!("   {} WSL: {}", "✅".bright_green(), "Available".bright_green());
        
        if let Some(v) = &status.wsl_solana {
            println!("      {} Solana: {}", "✅".bright_green(), v);
        } else {
            println!("      {} Solana: {}", "❌".bright_red(), "Not installed");
        }

        if let Some(v) = &status.wsl_anchor {
            println!("      {} Anchor: {}", "✅".bright_green(), v);
        } else {
            println!("      {} Anchor: {}", "⚠️".bright_yellow(), "Not installed");
        }
    } else {
        println!("   {} WSL: {}", "❌".bright_red(), "Not available".bright_red());
    }

    // Cloud
    println!("   {} Cloud: {}", "✅".bright_green(), "Always available (requires internet)".dimmed());

    println!();
    
    if status.has_backend() {
        println!("{} Ready to build! Best backend: {}", 
            "🎉".bright_green(), 
            status.best_backend().unwrap_or("Cloud").bright_cyan());
    } else {
        println!("{} No build backend available!", "⚠️".bright_yellow());
        println!("   Install Docker Desktop or enable WSL2");
    }
    println!();
}

async fn install_toolchain(backend: &str) {
    println!("{} Installing build toolchain for {}...", "📦".bright_yellow(), backend);
    println!();

    let toolchain = Toolchain::new();

    match backend {
        "docker" => {
            println!("   Pulling Anchor build image...");
            
            match toolchain.pull_docker_image("projectserum/build:v0.27.0").await {
                Ok(()) => {
                    println!();
                    println!("{} Docker build image ready!", "✅".bright_green());
                }
                Err(e) => {
                    println!("{} Failed: {}", "❌".bright_red(), e);
                }
            }
        }
        "wsl" => {
            println!("   Installing Solana in WSL...");
            
            match toolchain.install_wsl_solana("1.18.0").await {
                Ok(()) => {
                    println!();
                    println!("   Installing Anchor in WSL...");
                    
                    match toolchain.install_wsl_anchor("0.29.0").await {
                        Ok(()) => {
                            println!();
                            println!("{} WSL toolchain ready!", "✅".bright_green());
                        }
                        Err(e) => {
                            println!("{} Anchor installation failed: {}", "⚠️".bright_yellow(), e);
                            println!("   Solana is installed, Anchor is optional");
                        }
                    }
                }
                Err(e) => {
                    println!("{} Failed: {}", "❌".bright_red(), e);
                }
            }
        }
        _ => {
            println!("{} Unknown backend: {}", "❌".bright_red(), backend);
            println!("   Use 'docker' or 'wsl'");
        }
    }
}

