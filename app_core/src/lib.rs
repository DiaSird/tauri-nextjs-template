//! # Core
//!
//! ## Examples
//!
//! ```no_run
//! use anyhow::Result;
//! use core::add;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     add(1, 1).await?;
//!     Ok(())
//! }
//! ```
//!
mod app;
pub mod error;

pub use crate::app::add;
