//! A windowing shell for Iced, on top of [`winit`].
//!
//! ![`iced_winit` crate graph](https://github.com/hecrj/iced/blob/cae26cb7bc627f4a5b3bcf1cd023a0c552e8c65e/docs/graphs/winit.png?raw=true)
//!
//! `iced_winit` offers some convenient abstractions on top of [`iced_native`]
//! to quickstart development when using [`winit`].
//!
//! It exposes a renderer-agnostic [`Application`] trait that can be implemented
//! and then run with a simple call. The use of this trait is optional.
//!
//! Additionally, a [`conversion`] module is available for users that decide to
//! implement a custom event loop.
//!
//! [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
//! [`winit`]: https://github.com/rust-windowing/winit
//! [`Application`]: trait.Application.html
//! [`conversion`]: conversion
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![forbid(rust_2018_idioms)]

#[doc(no_inline)]
pub use iced_native::*;
pub use winit;

pub mod application;
pub mod conversion;
pub mod settings;

mod clipboard;
mod context;
mod mode;
mod proxy;

pub use application::Application;
pub use clipboard::Clipboard;
pub use context::Context;
pub use mode::Mode;
pub use proxy::Proxy;
pub use settings::Settings;

pub use iced_graphics::Viewport;
