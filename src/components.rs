//! - [`general`] — Basic reusable UI components (buttons, inputs, etc).
//! - [`bitcoin`] — Components specific to Bitcoin UIs (wallets, balances, QR, etc).
//! - [`messages`] — Components for displaying and interacting with messages (chat UIs, etc).

pub mod general;
pub use general::*;

pub mod bitcoin;
pub use bitcoin::*;

pub mod messages;
pub use messages::*;