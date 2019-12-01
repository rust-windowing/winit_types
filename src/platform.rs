pub use self::platform::*;

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
mod platform;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[path = "platform/unix.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod platform;
#[cfg(target_os = "android")]
#[path = "platform/android.rs"]
mod platform;
#[cfg(target_os = "ios")]
#[path = "platform/ios.rs"]
mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "platform/web.rs"]
mod platform;

#[cfg(all(
    not(target_os = "ios"),
    not(target_os = "windows"),
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "android"),
    not(target_os = "dragonfly"),
    not(target_os = "freebsd"),
    not(target_os = "netbsd"),
    not(target_os = "openbsd"),
    not(target_arch = "wasm32"),
))]
compile_error!("The platform you're compiling for is not supported by winit_types");
