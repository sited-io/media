pub mod api;
mod auth;
mod credentials;
pub mod db;
pub mod files;
pub mod logging;
mod model;
mod payment;
mod quota;
mod services;
pub mod subscribers;

pub use auth::init_jwks_verifier;
pub use credentials::CredentialsService;
pub use payment::PaymentService;
pub use quota::QuotaService;
pub use services::*;

pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| {
        panic!("ERROR: Missing environment variable '{var}'")
    })
}
