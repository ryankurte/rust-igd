//! This module implements the same features as the main crate, but using async io.

mod gateway;
mod search;
mod soap;

pub use self::gateway::Gateway;
pub use self::search::{search_gateway, search_gateway_from, search_gateway_from_timeout, search_gateway_timeout};
