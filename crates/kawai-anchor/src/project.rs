//! Anchor project management

use crate::config::{create_anchor_toml, AnchorConfig};
use crate::error::{Error, Result};
use crate::idl::Idl;
use kawai_build::{BuildConfig, Builder};
use kawai_sdk::prelude::*;
use kawai_validator::Validator;
use std::fs;
use std::path::{Path, PathBuf};

/// Anchor project manager
pub struct Anchor {
    /// Project root
    root: PathBuf,
    /// Configuration
    config: AnchorConfig,
}

impl Anchor {
    /// Load an existing Anchor project
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        let config = AnchorConfig::load(&root)?;

        Ok(Self { root, config })
    }

    /// Initialize a new Anchor project
    pub async fn init(name: &str) -> Result<Self> {
        let root = PathBuf::from(name);

        if root.exists() {
            return Err(Error::ConfigError(format!(
                "Directory '{}' already exists",
                name
            )));
        }

        // Create project structure
        Self::create_project_structure(&root, name)?;

        // Load the created project
        Self::load(&root)
    }

    /// Create project directory structure
    fn create_project_structure(root: &Path, name: &str) -> Result<()> {
        // Create directories
        fs::create_dir_all(root.join("programs").join(name).join("src"))?;
        fs::create_dir_all(root.join("tests"))?;
        fs::create_dir_all(root.join("migrations"))?;
        fs::create_dir_all(root.join("app"))?;

        // Create Anchor.toml
        let program_id = "11111111111111111111111111111111"; // Placeholder
        let config = create_anchor_toml(name, program_id);
        config.save(root)?;

        // Create program Cargo.toml
        let program_cargo = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
description = "Created with Kawai"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]
name = "{}"

[features]
no-entrypoint = []
no-idl = []
no-log-ix-name = []
cpi = ["no-entrypoint"]
default = []

[dependencies]
anchor-lang = "0.29.0"
"#,
            name,
            name.replace("-", "_")
        );
        fs::write(
            root.join("programs").join(name).join("Cargo.toml"),
            program_cargo,
        )?;

        // Create lib.rs
        let lib_rs = format!(
            r#"use anchor_lang::prelude::*;

declare_id!("{}");

#[program]
pub mod {} {{
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {{
        msg!("🌸 Kawai: Program initialized!");
        Ok(())
    }}
}}

#[derive(Accounts)]
pub struct Initialize {{}}
"#,
            program_id,
            name.replace("-", "_")
        );
        fs::write(
            root.join("programs").join(name).join("src").join("lib.rs"),
            lib_rs,
        )?;

        // Create workspace Cargo.toml
        let workspace_cargo = format!(
            r#"[workspace]
members = [
    "programs/*"
]

[profile.release]
overflow-checks = true
lto = "fat"
codegen-units = 1
[profile.release.build-override]
opt-level = 3
incremental = false
codegen-units = 1
"#
        );
        fs::write(root.join("Cargo.toml"), workspace_cargo)?;

        // Create test file
        let test_ts = format!(
            r#"import * as anchor from "@coral-xyz/anchor";
import {{ Program }} from "@coral-xyz/anchor";
import {{ {} }} from "../target/types/{}";

describe("{}", () => {{
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.{} as Program<{}>;

  it("Is initialized!", async () => {{
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  }});
}});
"#,
            to_pascal_case(name),
            name.replace("-", "_"),
            name,
            to_pascal_case(name),
            to_pascal_case(name)
        );
        fs::write(root.join("tests").join(format!("{}.ts", name)), test_ts)?;

        // Create package.json
        let package_json = format!(
            r#"{{
  "scripts": {{
    "lint:fix": "prettier */*.js \"*/**/*{{.js,.ts}}\" -w",
    "lint": "prettier */*.js \"*/**/*{{.js,.ts}}\" --check"
  }},
  "dependencies": {{
    "@coral-xyz/anchor": "^0.29.0"
  }},
  "devDependencies": {{
    "chai": "^4.3.4",
    "mocha": "^9.0.3",
    "ts-mocha": "^10.0.0",
    "@types/bn.js": "^5.1.0",
    "@types/chai": "^4.3.0",
    "@types/mocha": "^9.0.0",
    "typescript": "^4.3.5",
    "prettier": "^2.6.2"
  }}
}}
"#
        );
        fs::write(root.join("package.json"), package_json)?;

        // Create tsconfig.json
        let tsconfig = r#"{
  "compilerOptions": {
    "types": ["mocha", "chai"],
    "typeRoots": ["./node_modules/@types"],
    "lib": ["es2015"],
    "module": "commonjs",
    "target": "es6",
    "esModuleInterop": true
  }
}
"#;
        fs::write(root.join("tsconfig.json"), tsconfig)?;

        // Create .gitignore
        let gitignore = r#".anchor
.DS_Store
target
**/*.rs.bk
node_modules
test-ledger
.yarn
"#;
        fs::write(root.join(".gitignore"), gitignore)?;

        Ok(())
    }

    /// Build the project
    pub async fn build(path: impl AsRef<Path>) -> Result<kawai_build::config::BuildResult> {
        let project = Self::load(&path)?;

        Builder::new()
            .project_dir(&project.root)
            .release(true)
            .build()
            .await
            .map_err(Error::from)
    }

    /// Deploy to a cluster
    pub async fn deploy(
        path: impl AsRef<Path>,
        cluster: &str,
    ) -> Result<DeployResult> {
        let project = Self::load(&path)?;

        // Build first
        let build_result = Self::build(&path).await?;

        // Connect to cluster
        let network: Network = cluster.parse()
            .map_err(|_| Error::DeployFailed(format!("Invalid cluster: {}", cluster)))?;

        let kawai = Kawai::connect(network).await
            .map_err(|e| Error::DeployFailed(e.to_string()))?;

        // Load the program keypair
        let keypair_path = build_result.so_path.with_extension("json");
        let program_keypair = if keypair_path.exists() {
            let content = fs::read_to_string(&keypair_path)?;
            let bytes: Vec<u8> = serde_json::from_str(&content)?;
            Some(bytes)
        } else {
            None
        };

        // TODO: Implement actual deployment using Solana's program deploy
        // This would require the BPF loader program interaction

        Ok(DeployResult {
            program_id: build_result.program_id,
            cluster: cluster.to_string(),
            signature: "deployment_signature_placeholder".to_string(),
        })
    }

    /// Run tests
    pub async fn test(&self) -> Result<TestResult> {
        // Start local validator
        let validator = Validator::auto().await
            .map_err(Error::from)?;
        
        validator.start().await
            .map_err(Error::from)?;

        // Run tests using the configured test script
        let test_script = self.config.scripts
            .get("test")
            .cloned()
            .unwrap_or_else(|| "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts".to_string());

        let output = tokio::process::Command::new("cmd")
            .args(["/C", &test_script])
            .current_dir(&self.root)
            .output()
            .await?;

        // Stop validator
        validator.stop().await.ok();

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(Error::TestFailed(format!("{}\n{}", stdout, stderr)));
        }

        Ok(TestResult {
            passed: true,
            output: stdout,
        })
    }

    /// Get the IDL
    pub fn idl(&self) -> Result<Option<Idl>> {
        let idl_dir = self.root.join("target").join("idl");
        
        if !idl_dir.exists() {
            return Ok(None);
        }

        // Find the first IDL file
        for entry in fs::read_dir(idl_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let idl = Idl::load(&path)?;
                return Ok(Some(idl));
            }
        }

        Ok(None)
    }

    /// Get project root
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get configuration
    pub fn config(&self) -> &AnchorConfig {
        &self.config
    }
}

/// Deployment result
#[derive(Debug, Clone)]
pub struct DeployResult {
    pub program_id: String,
    pub cluster: String,
    pub signature: String,
}

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub output: String,
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split(|c| c == '_' || c == '-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("my_program"), "MyProgram");
        assert_eq!(to_pascal_case("my-program"), "MyProgram");
    }
}

