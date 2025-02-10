//! # The Process Scanning API

// Compile the Windows version of the userscript API only on Windows.
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::ProcessApi;

// Compile the Linux version of the userscript API only on Linux.
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::ProcessApi;

/// Represents Mapped Process Memory
pub struct MemoryMap {
    /// The process PID
    pub pid: usize,

    /// Name of the process, if any.
    pub name: Option<String>,

    /// Base address of the memory map.
    pub start: usize,

    /// Ending address of the memory map.
    pub stop: usize,

    /// True if this memory segment is readable.
    pub readable: bool,

    /// True if this memory segment is writable.
    pub writable: bool,

    /// True if this memory segment is executable.
    pub executable: bool,
}

impl MemoryMap {
    /// Create a new [`MemoryMap`]
    pub fn new(pid: usize, name: Option<String>, start: usize, stop: usize, readable: bool, writable: bool, executable: bool) -> Self {
        Self { pid, name, start, stop, readable, writable, executable }
    }
}
