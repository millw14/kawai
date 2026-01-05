//! Validator runtime - slot progression and timing

use super::*;
use super::bank::Bank;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::interval;

/// Slot timer that advances the bank
pub struct SlotTimer {
    bank: Arc<Bank>,
    slot_time: Duration,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl SlotTimer {
    pub fn new(bank: Arc<Bank>, slot_time_ms: u64) -> Self {
        Self {
            bank,
            slot_time: Duration::from_millis(slot_time_ms),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Start the slot timer
    pub async fn start(&self, mut shutdown: broadcast::Receiver<()>) {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        let mut ticker = interval(self.slot_time);
        
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if !self.running.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    self.bank.advance_slot().await;
                }
                _ = shutdown.recv() => {
                    break;
                }
            }
        }
    }
    
    /// Stop the timer
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
}

