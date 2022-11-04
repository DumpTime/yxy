//! Using [`reqwest`] **blocking** clients
//!
//! # Warning: Not actively maintained
//! Provides backward compatibility only, not recommended for new code.

pub mod bind;
pub mod wrapper;

pub use bind::app::auth::UserInfo;
pub use bind::app::electricity::{EleBindInfo, ElectricityInfo, RoomInfo};
pub use bind::app::AppHandler;
pub use bind::login::{LoginHandler, LoginInfo, SecurityTokenInfo};
