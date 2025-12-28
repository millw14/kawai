// Build script to handle platform-specific compilation
// This helps catch platform issues early

fn main() {
    // Print build information
    println!("cargo:rerun-if-changed=build.rs");
    
    // Warn if building on Windows for production
    #[cfg(windows)]
    {
        println!("cargo:warning=Building on Windows - Solana validators run on Linux");
        println!("cargo:warning=Consider using WSL or cross-compiling for Linux target");
    }
}


