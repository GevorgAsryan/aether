pub mod io_uring_backend;

pub fn is_linux_server() -> bool {
    #[cfg(target_os = "linux")]
    {
        true
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}
