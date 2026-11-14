//! Screen recording permission. Only macOS gates screen capture behind a user
//! permission (System Settings → Privacy & Security → Screen Recording).

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    Granted,
    Denied,
    /// The platform has no such permission.
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    Unsupported,
}

#[cfg(target_os = "macos")]
mod mac {
    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C" {
        fn CGPreflightScreenCaptureAccess() -> bool;
        fn CGRequestScreenCaptureAccess() -> bool;
    }
    pub fn preflight() -> bool {
        unsafe { CGPreflightScreenCaptureAccess() }
    }
    pub fn request() -> bool {
        unsafe { CGRequestScreenCaptureAccess() }
    }
}

pub fn status() -> Status {
    #[cfg(target_os = "macos")]
    return if mac::preflight() { Status::Granted } else { Status::Denied };
    #[cfg(not(target_os = "macos"))]
    Status::Unsupported
}

/// Asks the OS to show its permission prompt (macOS shows it once per install).
/// Returns true when access is already granted.
pub fn request() -> bool {
    #[cfg(target_os = "macos")]
    return mac::request();
    #[cfg(not(target_os = "macos"))]
    true
}

/// Clears this app's Screen Recording entry so macOS asks again. An ad-hoc signed
/// app is identified by the hash of its code, so after an update or reinstall the old
/// grant still shows as enabled in System Settings but no longer applies to the app.
pub fn reset(bundle_id: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("/usr/bin/tccutil")
            .args(["reset", "ScreenCapture", bundle_id])
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
    }
    #[cfg(not(target_os = "macos"))]
    let _ = bundle_id;
    Ok(())
}

/// Deep link to the Screen Recording pane.
pub const SETTINGS_URL: &str = "x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture";
