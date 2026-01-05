//! Main builder implementation

use crate::backend::{BuildBackend, CloudBackend, DockerBackend, WslBackend};
use crate::config::{BuildBackendPreference, BuildConfig, BuildResult};
use crate::error::{Error, Result};
use crate::project::Project;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

/// Solana program builder
pub struct Builder {
    config: BuildConfig,
    backend: Option<Arc<dyn BuildBackend>>,
}

impl Builder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: BuildConfig::default(),
            backend: None,
        }
    }

    /// Set project directory
    pub fn project_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.project_dir = path.into();
        self
    }

    /// Set output directory
    pub fn output_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.output_dir = path.into();
        self
    }

    /// Enable release mode
    pub fn release(mut self, release: bool) -> Self {
        self.config.release = release;
        self
    }

    /// Set backend
    pub fn backend(mut self, backend: BuildBackendPreference) -> Self {
        self.config.backend = backend;
        self
    }

    /// Add feature
    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.config.features.push(feature.into());
        self
    }

    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.config.verbose = true;
        self
    }

    /// Build the project
    pub async fn build(mut self) -> Result<BuildResult> {
        let start = Instant::now();

        // Load project
        let project = Project::load(&self.config.project_dir)?;

        if self.config.verbose {
            println!("🔍 Detected project: {} ({:?})", project.name, project.project_type);
        }

        // Select backend
        let backend = self.select_backend().await?;
        self.backend = Some(backend.clone());

        if self.config.verbose {
            println!("🔧 Using backend: {}", backend.name());
        }

        // Build based on project type
        let so_path = if project.is_anchor() {
            backend.build_anchor(&project, &self.config).await?
        } else {
            backend.build_native(&project, &self.config).await?
        };

        // Get program ID
        let program_id = self.get_program_id(&project, &so_path)?;

        // Calculate size
        let size_bytes = std::fs::metadata(&so_path)
            .map(|m| m.len())
            .unwrap_or(0);

        // Check for IDL
        let idl_path = project.deploy_dir()
            .join(format!("{}.json", project.name))
            .exists()
            .then(|| project.deploy_dir().join(format!("{}.json", project.name)));

        let duration_secs = start.elapsed().as_secs_f64();

        Ok(BuildResult {
            so_path,
            program_id,
            duration_secs,
            size_bytes,
            idl_path,
            backend: backend.name().to_string(),
        })
    }

    /// Select the appropriate backend
    async fn select_backend(&self) -> Result<Arc<dyn BuildBackend>> {
        match self.config.backend {
            BuildBackendPreference::Docker => {
                if DockerBackend::is_available().await {
                    Ok(Arc::new(DockerBackend::new(&self.config.docker_image)))
                } else {
                    Err(Error::DockerNotAvailable)
                }
            }
            BuildBackendPreference::Wsl => {
                if WslBackend::is_available().await {
                    Ok(Arc::new(WslBackend::new()))
                } else {
                    Err(Error::ToolchainNotFound)
                }
            }
            BuildBackendPreference::Cloud => {
                let url = self.config.cloud_url.clone()
                    .unwrap_or_else(|| "https://build.kawai.dev".to_string());
                Ok(Arc::new(CloudBackend::new(&url)))
            }
            BuildBackendPreference::Local => {
                // Check if local toolchain exists
                Err(Error::UnsupportedPlatform)
            }
            BuildBackendPreference::Auto => {
                // Try backends in order
                if DockerBackend::is_available().await {
                    return Ok(Arc::new(DockerBackend::new(&self.config.docker_image)));
                }
                if WslBackend::is_available().await {
                    return Ok(Arc::new(WslBackend::new()));
                }
                // Fall back to cloud
                let url = self.config.cloud_url.clone()
                    .unwrap_or_else(|| "https://build.kawai.dev".to_string());
                Ok(Arc::new(CloudBackend::new(&url)))
            }
        }
    }

    /// Get the program ID from the built artifact
    fn get_program_id(&self, project: &Project, so_path: &PathBuf) -> Result<String> {
        // Check if there's a keypair file
        let keypair_path = so_path.with_extension("json");
        if keypair_path.exists() {
            // Read and parse keypair
            let content = std::fs::read_to_string(&keypair_path)?;
            let bytes: Vec<u8> = serde_json::from_str(&content)
                .map_err(|e| Error::InvalidProject(e.to_string()))?;

            // Derive pubkey from keypair
            if bytes.len() >= 64 {
                let pubkey_bytes = &bytes[32..64];
                return Ok(bs58::encode(pubkey_bytes).into_string());
            }
        }

        // Check project config for program ID
        if let Some(main) = project.main_program() {
            if let Some(id) = &main.program_id {
                return Ok(id.clone());
            }
        }

        // Generate a placeholder
        Ok("11111111111111111111111111111111".to_string())
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick build function
pub async fn build(project_dir: impl Into<PathBuf>) -> Result<BuildResult> {
    Builder::new()
        .project_dir(project_dir)
        .build()
        .await
}

/// Build with release mode
pub async fn build_release(project_dir: impl Into<PathBuf>) -> Result<BuildResult> {
    Builder::new()
        .project_dir(project_dir)
        .release(true)
        .build()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_builder_creation() {
        let builder = Builder::new()
            .project_dir("./test")
            .release(true);
        
        assert!(builder.config.release);
    }
}

