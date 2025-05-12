/// A constant indicating whether the current target platform is a mobile OS.
///
/// This is set to `true` when compiling for `iOS` or `Android`, and `false` for all other targets.
/// Useful for platform-specific UI logic or conditional compilation.
#[cfg(any(target_os = "ios", target_os = "android"))]
pub const IS_MOBILE: bool = true;

/// A constant indicating whether the current target platform is a mobile OS.
///
/// This is set to `true` when compiling for `iOS` or `Android`, and `false` for all other targets.
/// Useful for platform-specific UI logic or conditional compilation.
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub const IS_MOBILE: bool = false;
