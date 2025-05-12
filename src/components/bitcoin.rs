//! # Bitcoin UI Components
//!
//! This module contains reusable UI components for Bitcoin-related functionality,
//! such as displaying amounts, scanning and rendering QR codes, and handling
//! numeric input for payments or wallet operations.

/// Component for displaying Bitcoin amounts with formatting handling.
pub mod amount_display;
pub use amount_display::*;

/// A numeric keypad component for entering Bitcoin amounts.
pub mod numeric_keypad;
pub use numeric_keypad::*;

/// Component for rendering Bitcoin-related QR codes (e.g., addresses, payment requests).
pub mod qr_code;
pub use qr_code::*;

/// Component for scanning QR codes with a camera.
pub mod qr_scanner;
pub use qr_scanner::*;
