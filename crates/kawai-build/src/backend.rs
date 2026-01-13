//! Build backends for different compilation environments

use crate::config::BuildConfig;
use crate::error::{Error, Result};
use crate::project::Project;
use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

/// Trait for build backends
#[async_trait]
pub trait BuildBackend: Send + Sync {
    /// Build a native Solana program
    async fn build_native(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf>;

    /// Build an Anchor program
    async fn build_anchor(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf>;

    /// Get backend name
    fn name(&self) -> &'static str;
}

/// Docker-based build backend
pub struct DockerBackend {
    image: String,
}

impl DockerBackend {
    pub fn new(image: &str) -> Self {
        Self {
            image: image.to_string(),
        }
    }

    /// Check if Docker is available
    pub async fn is_available() -> bool {
        Command::new("docker")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Build docker run command
    fn docker_build_command(&self, project: &Project, config: &BuildConfig, cmd: &str) -> Command {
        let mut docker = Command::new("docker");
        docker.args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/workspace", project.root.display()),
            "-w",
            "/workspace",
            &self.image,
            "bash",
            "-c",
            cmd,
        ]);
        docker
    }
}

#[async_trait]
impl BuildBackend for DockerBackend {
    async fn build_native(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        let build_cmd = if config.release {
            "cargo build-sbf --release"
        } else {
            "cargo build-sbf"
        };

        let mut cmd = self.docker_build_command(project, config, build_cmd);
        
        if config.verbose {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CompilationError(stderr.to_string()));
        }

        // Find the .so file
        let so_name = format!("{}.so", project.name.replace("-", "_"));
        let so_path = project.deploy_dir().join(&so_name);

        if !so_path.exists() {
            return Err(Error::BuildFailed(format!(
                "Expected output not found: {}",
                so_path.display()
            )));
        }

        Ok(so_path)
    }

    async fn build_anchor(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        let build_cmd = "anchor build";

        let mut cmd = self.docker_build_command(project, config, build_cmd);
        
        if config.verbose {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CompilationError(stderr.to_string()));
        }

        // Find the .so file
        let main_program = project.main_program()
            .ok_or_else(|| Error::InvalidProject("No program found".to_string()))?;

        let so_name = format!("{}.so", main_program.name.replace("-", "_"));
        let so_path = project.deploy_dir().join(&so_name);

        if !so_path.exists() {
            return Err(Error::BuildFailed(format!(
                "Expected output not found: {}",
                so_path.display()
            )));
        }

        Ok(so_path)
    }

    fn name(&self) -> &'static str {
        "Docker"
    }
}

/// WSL-based build backend
pub struct WslBackend {
    solana_installed: bool,
}

impl WslBackend {
    pub fn new() -> Self {
        Self {
            solana_installed: false,
        }
    }

    /// Check if WSL is available
    pub async fn is_available() -> bool {
        Command::new("wsl")
            .arg("--status")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Check if Solana tools are installed in WSL
    async fn check_solana_installed(&self) -> bool {
        Command::new("wsl")
            .args(["which", "cargo-build-sbf"])
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Convert Windows path to WSL path
    fn to_wsl_path(path: &std::path::Path) -> String {
        let path_str = path.display().to_string();
        
        // Convert C:\Users\... to /mnt/c/Users/...
        if path_str.chars().nth(1) == Some(':') {
            let drive = path_str.chars().next().unwrap().to_lowercase();
            let rest = &path_str[2..].replace('\\', "/");
            format!("/mnt/{}{}", drive, rest)
        } else {
            path_str.replace('\\', "/")
        }
    }
}

impl Default for WslBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BuildBackend for WslBackend {
    async fn build_native(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        if !self.check_solana_installed().await {
            return Err(Error::ToolchainNotFound);
        }

        let wsl_path = Self::to_wsl_path(&project.root);
        let build_cmd = if config.release {
            format!("cd '{}' && cargo build-sbf --release", wsl_path)
        } else {
            format!("cd '{}' && cargo build-sbf", wsl_path)
        };

        let mut cmd = Command::new("wsl");
        cmd.args(["bash", "-c", &build_cmd]);

        if config.verbose {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CompilationError(stderr.to_string()));
        }

        // Find the .so file
        let so_name = format!("{}.so", project.name.replace("-", "_"));
        let so_path = project.deploy_dir().join(&so_name);

        if !so_path.exists() {
            return Err(Error::BuildFailed(format!(
                "Expected output not found: {}",
                so_path.display()
            )));
        }

        Ok(so_path)
    }

    async fn build_anchor(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        let wsl_path = Self::to_wsl_path(&project.root);
        let build_cmd = format!("cd '{}' && anchor build", wsl_path);

        let mut cmd = Command::new("wsl");
        cmd.args(["bash", "-c", &build_cmd]);

        if config.verbose {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
        }

        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CompilationError(stderr.to_string()));
        }

        // Find the .so file
        let main_program = project.main_program()
            .ok_or_else(|| Error::InvalidProject("No program found".to_string()))?;

        let so_name = format!("{}.so", main_program.name.replace("-", "_"));
        let so_path = project.deploy_dir().join(&so_name);

        Ok(so_path)
    }

    fn name(&self) -> &'static str {
        "WSL"
    }
}

/// Cloud-based build backend
pub struct CloudBackend {
    api_url: String,
}

impl CloudBackend {
    pub fn new(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
        }
    }

    /// Upload project and get build result
    async fn remote_build(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        // Create a zip of the project
        let zip_path = self.create_project_archive(project).await?;

        // Upload to build service
        let client = reqwest::Client::new();
        let file_bytes = tokio::fs::read(&zip_path).await?;

        let form = reqwest::multipart::Form::new()
            .part(
                "project",
                reqwest::multipart::Part::bytes(file_bytes)
                    .file_name("project.zip")
            )
            .text("release", config.release.to_string());

        let response = client
            .post(&format!("{}/build", self.api_url))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(Error::CloudError(error));
        }

        // Download the result
        let result: serde_json::Value = response.json().await?;
        let download_url = result["artifact_url"]
            .as_str()
            .ok_or_else(|| Error::CloudError("No artifact URL".to_string()))?;

        let so_response = client.get(download_url).send().await?;
        let so_bytes = so_response.bytes().await?;

        // Save to target/deploy
        let so_name = format!("{}.so", project.name.replace("-", "_"));
        let so_path = project.deploy_dir().join(&so_name);
        
        tokio::fs::create_dir_all(project.deploy_dir()).await?;
        tokio::fs::write(&so_path, so_bytes).await?;

        // Clean up
        let _ = tokio::fs::remove_file(zip_path).await;

        Ok(so_path)
    }

    /// Create a zip archive of the project
    async fn create_project_archive(&self, project: &Project) -> Result<PathBuf> {
        use std::io::Write;
        use walkdir::WalkDir;
        use zip::write::FileOptions;
        use zip::ZipWriter;

        let zip_path = std::env::temp_dir().join(format!("{}_build.zip", project.name));
        let file = std::fs::File::create(&zip_path)?;
        let mut zip = ZipWriter::new(file);

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for entry in WalkDir::new(&project.root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                // Skip target, .git, node_modules
                !name.starts_with('.') 
                    && name != "target" 
                    && name != "node_modules"
            })
        {
            let entry = entry.map_err(|e| Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string()
            )))?;
            
            let path = entry.path();
            let relative = path.strip_prefix(&project.root)
                .map_err(|e| Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string()
                )))?;

            if path.is_file() {
                zip.start_file(relative.to_string_lossy(), options)?;
                let content = std::fs::read(path)?;
                zip.write_all(&content)?;
            }
        }

        zip.finish()?;
        Ok(zip_path)
    }
}

#[async_trait]
impl BuildBackend for CloudBackend {
    async fn build_native(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        self.remote_build(project, config).await
    }

    async fn build_anchor(&self, project: &Project, config: &BuildConfig) -> Result<PathBuf> {
        self.remote_build(project, config).await
    }

    fn name(&self) -> &'static str {
        "Cloud"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_docker_available() {
        // Just check the function runs
        let _ = DockerBackend::is_available().await;
    }

    #[tokio::test]
    async fn test_wsl_available() {
        let _ = WslBackend::is_available().await;
    }
}

