//! Library half of the probe so that `tests/` can build the router and call
//! `Router::handle` without opening a socket.

pub mod app;
pub mod components;
