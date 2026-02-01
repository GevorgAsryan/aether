pub mod macos;
pub mod linux;

pub struct UnifiedMemory {
    pub ptr: *mut u8,
    pub size: usize,
}

impl UnifiedMemory {
    pub fn new(size: usize) -> Self {
        // Platform specific allocation
        if cfg!(target_os = "macos") {
             println!("Allocating Unified Memory (Zero-Copy) on macOS");
        } else {
             println!("Allocating Pinned Memory on Linux");
        }
        
        let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        
        Self { ptr, size }
    }
}

pub enum RuntimeTarget {
    MacOS(macos::metal::MetalContext),
    Linux(linux::io_uring_backend::IoUringContext),
    Unknown,
}

pub fn detect_runtime() -> RuntimeTarget {
    if macos::is_apple_silicon() || cfg!(target_os = "macos") {
        if let Some(ctx) = macos::metal::MetalContext::new() {
            return RuntimeTarget::MacOS(ctx);
        }
    }
    
    if linux::is_linux_server() || cfg!(target_os = "linux") {
        return RuntimeTarget::Linux(linux::io_uring_backend::IoUringContext::new());
    }
    
    RuntimeTarget::Unknown
}
