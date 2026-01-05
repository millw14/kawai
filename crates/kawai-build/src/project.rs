//! Project detection and management

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Project type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectType {
    /// Standard Solana program (using solana-program crate)
    Native,
    /// Anchor framework project
    Anchor,
    /// Unknown/custom
    Unknown,
}

/// Solana project
#[derive(Debug, Clone)]
pub struct Project {
    /// Project root directory
    pub root: PathBuf,

    /// Project name
    pub name: String,

    /// Project type
    pub project_type: ProjectType,

    /// Program directories (for workspaces)
    pub programs: Vec<ProgramInfo>,

    /// Has IDL
    pub has_idl: bool,
}

/// Information about a program in the project
#[derive(Debug, Clone)]
pub struct ProgramInfo {
    /// Program name
    pub name: String,

    /// Program directory
    pub path: PathBuf,

    /// Program ID (if specified in code or config)
    pub program_id: Option<String>,
}

impl Project {
    /// Load a project from a directory
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let root = path.as_ref().to_path_buf();

        if !root.exists() {
            return Err(Error::ProjectNotFound(root.display().to_string()));
        }

        // Check for Cargo.toml
        let cargo_toml = root.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(Error::CargoTomlNotFound);
        }

        // Parse Cargo.toml
        let cargo_content = fs::read_to_string(&cargo_toml)?;
        let cargo: toml::Value = toml::from_str(&cargo_content)?;

        let name = cargo
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("unknown")
            .to_string();

        // Detect project type
        let project_type = Self::detect_type(&root, &cargo);

        // Find programs
        let programs = Self::find_programs(&root, project_type)?;

        // Check for IDL
        let has_idl = root.join("target/idl").exists() 
            || root.join("idl").exists();

        Ok(Self {
            root,
            name,
            project_type,
            programs,
            has_idl,
        })
    }

    /// Detect project type
    fn detect_type(root: &Path, cargo: &toml::Value) -> ProjectType {
        // Check for Anchor.toml
        if root.join("Anchor.toml").exists() {
            return ProjectType::Anchor;
        }

        // Check dependencies for solana-program
        if let Some(deps) = cargo.get("dependencies") {
            if deps.get("solana-program").is_some() {
                return ProjectType::Native;
            }
            if deps.get("anchor-lang").is_some() {
                return ProjectType::Anchor;
            }
        }

        ProjectType::Unknown
    }

    /// Find programs in the project
    fn find_programs(root: &Path, project_type: ProjectType) -> Result<Vec<ProgramInfo>> {
        let mut programs = Vec::new();

        match project_type {
            ProjectType::Anchor => {
                // Check Anchor.toml for programs
                let anchor_toml = root.join("Anchor.toml");
                if anchor_toml.exists() {
                    let content = fs::read_to_string(&anchor_toml)?;
                    let anchor: toml::Value = toml::from_str(&content)?;

                    if let Some(progs) = anchor.get("programs") {
                        if let Some(table) = progs.as_table() {
                            for (network, network_progs) in table {
                                if let Some(progs_table) = network_progs.as_table() {
                                    for (name, id) in progs_table {
                                        programs.push(ProgramInfo {
                                            name: name.clone(),
                                            path: root.join("programs").join(name),
                                            program_id: id.as_str().map(|s| s.to_string()),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                // Also check programs directory
                let programs_dir = root.join("programs");
                if programs_dir.exists() {
                    for entry in fs::read_dir(&programs_dir)? {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_dir() && path.join("Cargo.toml").exists() {
                            let name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();

                            // Check if already added
                            if !programs.iter().any(|p| p.name == name) {
                                programs.push(ProgramInfo {
                                    name,
                                    path,
                                    program_id: None,
                                });
                            }
                        }
                    }
                }
            }
            ProjectType::Native | ProjectType::Unknown => {
                // Single program in root
                let cargo_toml = root.join("Cargo.toml");
                if cargo_toml.exists() {
                    let content = fs::read_to_string(&cargo_toml)?;
                    let cargo: toml::Value = toml::from_str(&content)?;

                    let name = cargo
                        .get("package")
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("program")
                        .to_string();

                    programs.push(ProgramInfo {
                        name,
                        path: root.to_path_buf(),
                        program_id: None,
                    });
                }
            }
        }

        Ok(programs)
    }

    /// Get the main program
    pub fn main_program(&self) -> Option<&ProgramInfo> {
        self.programs.first()
    }

    /// Check if this is an Anchor project
    pub fn is_anchor(&self) -> bool {
        self.project_type == ProjectType::Anchor
    }

    /// Get the target directory
    pub fn target_dir(&self) -> PathBuf {
        self.root.join("target")
    }

    /// Get the deploy directory
    pub fn deploy_dir(&self) -> PathBuf {
        self.root.join("target").join("deploy")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_project_type() {
        // Would need test fixtures
    }
}

