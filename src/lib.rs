//! # Remote Settings Client
//!
//! A library for fetching Remote Settings data.
//!
//! ## Example
//!
//! ```rust
//!   use remote_settings_client::Client;
//!   pub use viaduct::set_backend;
//!   pub use viaduct_reqwest::ReqwestBackend;
//!
//!   fn main() {
//!     set_backend(&ReqwestBackend).unwrap();
//!
//!     let client = Client::builder()
//!       .bucket_name("main-preview")
//!       .collection_name("search-config")
//!       .build();
//!
//!     match client.get() {
//!       Ok(records) => println!("{:?}", records),
//!       Err(error) => println!("Error fetching/verifying records: {:?}", error),
//!     };
//!   }
//! ```
//!
//! See [`Client`] for more infos.
pub mod client;

pub use client::Client;
pub use client::Collection;
pub use client::SignatureError;
pub use client::Verification;

pub use client::DEFAULT_BUCKET_NAME;
pub use client::DEFAULT_SERVER_URL;
