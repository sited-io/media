pub mod api;
mod auth;
mod commerce;
pub mod db;
pub mod files;
pub mod logging;
mod model;
mod quota;
mod services;

pub use auth::init_jwks_verifier;
pub use commerce::CommerceService;
pub use quota::QuotaService;
pub use services::*;

pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| {
        panic!("ERROR: Missing environment variable '{var}'")
    })
}
