//! # Messaging Components
//!
//! This module contains UI components designed for chat, conversation, or threaded message UIs.

/// A message bubble or entry for rendering chat messages, supporting timestamps and contact info.
pub mod message;
pub use message::*;
/// Components built for displaying Verifiable Credentials information.
pub mod credentials;
pub use credentials::*;
