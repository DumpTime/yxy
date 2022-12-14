//! # yxy
//!
//! YXY(yiSchool) platform API binding, written in rust.
//!
//! Uses [`reqwest`](https://crates.io/crates/reqwest) to perform async HTTP requests.
//!
//! ## Authorize
//! You should authorize before using any application API.
//!
//! Simply using the [`wrapper::app_auth`] wrapper:
//!
//! ```rust
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let (session, user_info): (String, yxy::UserInfo) = yxy::wrapper::app_auth("your_user_id").await?;
//! #    Ok(())
//! # }
//! ```
//! returns session and [`UserInfo`] struct.
//!
//! ## Query electricity binding
//!
//! ```rust
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let (session, user_info) = yxy::wrapper::app_auth("your_user_id").await?; // Authorize
//!
//! let bind_info: yxy::BindInfo = yxy::wrapper::query_ele_bind(&session).await?;
//!
//! let room_info: yxy::RoomInfo = bind_info.into();
//! # Ok(())
//! # }
//! ```
//!
//! ## Query Electricity
//!
//! 1. Use [`wrapper::query_ele`] wrapper to query by user's electricity binding info.
//!
//! ```rust
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let (session, user_info) = yxy::wrapper::app_auth("your_user_id").await?; // Authorize
//!
//! let info: yxy::SurplusInfo = yxy::wrapper::query_ele(&session).await?;
//!
//! let surplus = &info.surplus_list[0]; // take the first element
//!
//! println!(
//!     "
//! Electricity Info:
//! -----------------
//! Room: {}
//! Status: {}
//!
//! Total Surplus: {} kW·h
//! Total Amount: ￥{}
//!
//! Basic: {} kW·h | ￥{}
//! Subsidy : {} kW·h | ￥{}
//! ",
//!     info.display_room_name,
//!     surplus.room_status,
//!     info.soc,
//!     info.total_soc_amount,
//!     surplus.surplus,
//!     surplus.amount,
//!     surplus.subsidy,
//!     surplus.subsidy_amount,
//! );
//! #   Ok(())
//! # }
//! ```
//!
//! 2. Using [`wrapper::query_ele_by_room_info`] wrapper. Query by specific [`RoomInfo`].
//!
//! ```rust
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let (session, user_info) = yxy::wrapper::app_auth("your_user_id").await?; // Authorize
//!
//! // you can query binding previously, and it is reusable.
//! let bind_info: yxy::BindInfo = yxy::wrapper::query_ele_bind(&session).await?;
//! let room_info: yxy::RoomInfo = bind_info.into();
//!
//! let info: yxy::SurplusInfo = yxy::wrapper::query_ele_by_room_info(&session, &room_info).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## App login
//!
//! Example of getting [`LoginInfo`] procedure
//!
//! ```rust
//! # async fn login(verbose: bool) -> Result<(), yxy::error::Error> {
//! let phone_num = "1234567890";
//! let handler = yxy::LoginHandler::new()?;
//!
//! println!("Querying security token...");
//! let security_token: yxy::SecurityTokenInfo = handler.security_token().await?;
//! if verbose {
//!     println!("Success: {:?}", security_token);
//! }
//!
//! let mut captcha = String::new();
//! if security_token.level != 0 {
//!     // image captcha required
//!     println!("Image captcha required.");
//!     let result = handler.captcha_image(&security_token.security_token).await?;
//!
//!     println!("Captcha: {}", result);
//!
//!     println!("Please input the captcha: ");
//!     std::io::stdin().read_line(&mut captcha)?;
//! }
//!
//! println!("Sending verification code...");
//! let user_exists = handler.send_verification_code(
//!     phone_num,
//!     &security_token.security_token,
//!     if security_token.level == 0 {
//!         None
//!     } else {
//!         Some(&captcha)
//!     },
//! ).await?;
//!
//! if user_exists == false {
//!     eprintln!("Current user is not registered");
//! }
//!
//! // Get code from stdin
//! let mut code = String::new();
//! println!("Send SMS successfully, please enter the verification code:");
//! std::io::stdin().read_line(&mut code)?;
//!
//! println!("Login...");
//! let result: yxy::LoginInfo = handler.login_by_code(phone_num, &code).await?;
//! if verbose {
//!     println!("Login response: {:?}", result);
//! }
//! println!("Login successfully. Here is your UID and other information:");
//!
//! // stdout infos
//! println!(
//!     "
//!
//! UID: {}
//! Token: {}
//! Login by device id: {}
//! ----------------------------
//! Job no: {}
//! ID card: {}
//! Bind card status: {}
//! Last login Time: {}
//!
//! ",
//!     result.id,
//!     result.token,
//!     result.device_id,
//!     result.job_no.unwrap_or_default(),
//!     result.user_idcard.unwrap_or_default(),
//!     result.bind_card_status,
//!     result.last_login,
//! );
//!
//! Ok(())
//! # }
//! ```
//!
//!

pub mod bind;
pub mod error;
pub mod url;
pub mod utils;
pub mod wrapper;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use bind::app::auth::UserInfo;
pub use bind::app::electricity::{
    BindInfo, RechargeRecord, RoomInfo, SurplusInfo, UsageRecord, UserRechargeRecord,
};
pub use bind::app::AppHandler;
pub use bind::campus::login::{LoginHandler, LoginInfo, SecurityTokenInfo};
