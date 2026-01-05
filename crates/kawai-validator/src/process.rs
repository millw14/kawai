//! Process management utilities for Windows

use crate::error::{Error, Result};
use std::process::Stdio;
use tokio::process::Command;

/// Check if a process is running by name
pub async fn is_process_running(name: &str) -> bool {
    #[cfg(windows)]
    {
        Command::new("tasklist")
            .args(["/FI", &format!("IMAGENAME eq {}", name)])
            .output()
            .await
            .map(|o| {
                let stdout = String::from_utf8_lossy(&o.stdout);
                stdout.contains(name)
            })
            .unwrap_or(false)
    }

    #[cfg(not(windows))]
    {
        Command::new("pgrep")
            .arg("-f")
            .arg(name)
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

/// Kill a process by name
pub async fn kill_process(name: &str) -> Result<()> {
    #[cfg(windows)]
    {
        let status = Command::new("taskkill")
            .args(["/F", "/IM", name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        if !status.success() {
            // Process might not exist
        }
    }

    #[cfg(not(windows))]
    {
        let _ = Command::new("pkill")
            .arg("-f")
            .arg(name)
            .status()
            .await;
    }

    Ok(())
}

/// Check if a TCP port is in use
pub fn is_port_in_use(port: u16) -> bool {
    std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).is_err()
}

/// Wait for a port to become available
pub async fn wait_for_port(port: u16, timeout_secs: u64) -> Result<()> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(timeout_secs);

    while start.elapsed() < timeout {
        if !is_port_in_use(port) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    Err(Error::Timeout(timeout_secs))
}

/// Wait for a port to be listening (service ready)
pub async fn wait_for_service(port: u16, timeout_secs: u64) -> Result<()> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(timeout_secs);

    while start.elapsed() < timeout {
        if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    Err(Error::Timeout(timeout_secs))
}

/// Get current memory usage of a process (Windows)
#[cfg(windows)]
pub async fn get_process_memory(pid: u32) -> Option<u64> {
    use std::mem::MaybeUninit;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::ProcessStatus::GetProcessMemoryInfo;
    use windows::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS;
    use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

    unsafe {
        let handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            pid,
        )
        .ok()?;

        let mut pmc = MaybeUninit::<PROCESS_MEMORY_COUNTERS>::uninit();
        let size = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        if GetProcessMemoryInfo(handle, pmc.as_mut_ptr(), size).is_ok() {
            let pmc = pmc.assume_init();
            let _ = CloseHandle(handle);
            Some(pmc.WorkingSetSize as u64)
        } else {
            let _ = CloseHandle(handle);
            None
        }
    }
}

#[cfg(not(windows))]
pub async fn get_process_memory(_pid: u32) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_check() {
        // Port 80 is likely in use or blocked
        // Port 65535 is likely free
        let _ = is_port_in_use(65535);
    }
}

