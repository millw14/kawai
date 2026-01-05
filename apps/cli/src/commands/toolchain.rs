//! Toolchain command handlers

use crate::ToolchainCommands;
use colored::*;
use kawai_build::toolchain::Toolchain;

pub async fn handle(cmd: ToolchainCommands) {
    match cmd {
        ToolchainCommands::Status => {
            status().await;
        }
        ToolchainCommands::InstallSolana { version } => {
            install_solana(&version).await;
        }
        ToolchainCommands::InstallAnchor { version } => {
            install_anchor(&version).await;
        }
        ToolchainCommands::PullDocker { image } => {
            pull_docker(&image).await;
        }
    }
}

async fn status() {
    println!("{} Kawai Toolchain Status", "🔧".bright_yellow());
    println!();

    let toolchain = Toolchain::new();
    let status = toolchain.status().await;

    println!("{}", "═══════════════════════════════════════════".bright_magenta());
    println!();

    // Docker
    print!("   {} Docker Desktop: ", if status.docker { "✅" } else { "❌" });
    if status.docker {
        println!("{}", "Installed".bright_green());
    } else {
        println!("{}", "Not found".bright_red());
        println!("      → Install from https://docker.com/products/docker-desktop");
    }
    println!();

    // WSL
    print!("   {} WSL2: ", if status.wsl { "✅" } else { "❌" });
    if status.wsl {
        println!("{}", "Available".bright_green());
        
        // Solana in WSL
        print!("      {} Solana: ", if status.wsl_solana.is_some() { "✅" } else { "❌" });
        if let Some(v) = &status.wsl_solana {
            println!("{}", v.bright_green());
        } else {
            println!("{}", "Not installed".bright_red());
            println!("         → Run: kawai toolchain install-solana");
        }

        // Anchor in WSL
        print!("      {} Anchor: ", if status.wsl_anchor.is_some() { "✅" } else { "⚪" });
        if let Some(v) = &status.wsl_anchor {
            println!("{}", v.bright_green());
        } else {
            println!("{}", "Not installed (optional)".dimmed());
            println!("         → Run: kawai toolchain install-anchor");
        }
    } else {
        println!("{}", "Not available".bright_red());
        println!("      → Enable with: wsl --install");
    }
    println!();

    // Cloud
    println!("   {} Cloud Build: {}", "✅", "Always available".bright_green());
    println!("      Uses remote compilation service");
    println!();

    println!("{}", "═══════════════════════════════════════════".bright_magenta());
    println!();

    // Summary
    if status.has_backend() {
        let best = status.best_backend().unwrap_or("Cloud");
        println!("{} Ready! Best backend: {}", "🎉".bright_green(), best.bright_cyan());
        println!();
        println!("   Try: kawai build program ./my-project");
    } else {
        println!("{} Setup required!", "⚠️".bright_yellow());
        println!();
        println!("   Choose one:");
        println!("   1. Install Docker Desktop (easiest)");
        println!("   2. Enable WSL2 and install Solana");
        println!("   3. Use cloud backend (works now, needs internet)");
    }
    println!();
}

async fn install_solana(version: &str) {
    println!("{} Installing Solana {} in WSL...", "📦".bright_yellow(), version);
    println!();

    let toolchain = Toolchain::new();

    // Check if WSL is available
    if !Toolchain::wsl_available().await {
        println!("{} WSL is not available!", "❌".bright_red());
        println!();
        println!("{}", "Enable WSL2 first:".bright_yellow());
        println!("   wsl --install");
        println!("   (Restart your computer after)");
        return;
    }

    match toolchain.install_wsl_solana(version).await {
        Ok(()) => {
            println!();
            println!("{} Solana {} installed!", "✅".bright_green(), version);
            println!();
            println!("{}", "You can now build programs with:".bright_cyan());
            println!("   kawai build program ./my-project --backend wsl");
        }
        Err(e) => {
            println!("{} Installation failed: {}", "❌".bright_red(), e);
        }
    }
}

async fn install_anchor(version: &str) {
    println!("{} Installing Anchor {} in WSL...", "⚓".bright_yellow(), version);
    println!();

    let toolchain = Toolchain::new();

    // Check if WSL is available
    if !Toolchain::wsl_available().await {
        println!("{} WSL is not available!", "❌".bright_red());
        return;
    }

    // Check if Solana is installed
    if toolchain.status().await.wsl_solana.is_none() {
        println!("{} Solana must be installed first!", "⚠️".bright_yellow());
        println!("   Run: kawai toolchain install-solana");
        return;
    }

    match toolchain.install_wsl_anchor(version).await {
        Ok(()) => {
            println!();
            println!("{} Anchor {} installed!", "✅".bright_green(), version);
            println!();
            println!("{}", "You can now:".bright_cyan());
            println!("   kawai anchor init my-project");
            println!("   kawai anchor build");
        }
        Err(e) => {
            println!("{} Installation failed: {}", "❌".bright_red(), e);
        }
    }
}

async fn pull_docker(image: &str) {
    println!("{} Pulling Docker image: {}...", "🐳".bright_yellow(), image);
    println!();

    // Check if Docker is available
    if !Toolchain::docker_available().await {
        println!("{} Docker is not available!", "❌".bright_red());
        println!();
        println!("{}", "Install Docker Desktop:".bright_yellow());
        println!("   https://docker.com/products/docker-desktop");
        return;
    }

    let status = tokio::process::Command::new("docker")
        .args(["pull", image])
        .status()
        .await;

    match status {
        Ok(s) if s.success() => {
            println!();
            println!("{} Image pulled successfully!", "✅".bright_green());
            println!();
            println!("{}", "You can now build with Docker:".bright_cyan());
            println!("   kawai build program ./my-project --backend docker");
        }
        Ok(_) => {
            println!("{} Failed to pull image", "❌".bright_red());
        }
        Err(e) => {
            println!("{} Error: {}", "❌".bright_red(), e);
        }
    }
}

