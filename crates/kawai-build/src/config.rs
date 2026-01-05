//! Build configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Project directory
    pub project_dir: PathBuf,

    /// Output directory for artifacts
    pub output_dir: PathBuf,

    /// Build in release mode
    pub release: bool,

    /// Backend preference
    pub backend: BuildBackendPreference,

    /// Docker image for building
    pub docker_image: String,

    /// Cloud build service URL
    pub cloud_url: Option<String>,

    /// Features to enable
    pub features: Vec<String>,

    /// Skip verification after build
    pub skip_verify: bool,

    /// Verbose output
    pub verbose: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            project_dir: PathBuf::from("."),
            output_dir: PathBuf::from("target/deploy"),
            release: true,
            backend: BuildBackendPreference::Auto,
            docker_image: "projectserum/build:v0.27.0".to_string(),
            cloud_url: None,
            features: Vec::new(),
            skip_verify: false,
            verbose: false,
        }
    }
}

impl BuildConfig {
    /// Create a new build configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set project directory
    pub fn project_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.project_dir = path.into();
        self
    }

    /// Set output directory
    pub fn output_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.output_dir = path.into();
        self
    }

    /// Enable release mode
    pub fn release(mut self, release: bool) -> Self {
        self.release = release;
        self
    }

    /// Set backend preference
    pub fn backend(mut self, backend: BuildBackendPreference) -> Self {
        self.backend = backend;
        self
    }

    /// Add a feature
    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    /// Enable verbose output
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

/// Build backend preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BuildBackendPreference {
    /// Auto-detect best backend
    #[default]
    Auto,
    /// Use Docker
    Docker,
    /// Use WSL
    Wsl,
    /// Use cloud service
    Cloud,
    /// Use local toolchain (if available)
    Local,
}

/// Build result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    /// Path to the compiled .so file
    pub so_path: PathBuf,

    /// Program ID (derived from keypair or specified)
    pub program_id: String,

    /// Build duration
    pub duration_secs: f64,

    /// Output size in bytes
    pub size_bytes: u64,

    /// IDL path (if Anchor program)
    pub idl_path: Option<PathBuf>,

    /// Backend used
    pub backend: String,
}

