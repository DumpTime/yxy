//! Wrappers of API procedures

use crate::bind::*;
use crate::error::Error;

/// Authorize
///
/// Wrapper of Authorization procedure.
///
/// just input uid to get authorization :) .
/// This could be a vulnerability of yxy API.
///
/// returns a tuple of (Session Token, [`app::auth::UserInfo`])
pub async fn app_auth(uid: &str) -> Result<(String, app::auth::UserInfo), Error> {
    // Init non-redirect client to catch redirect response
    let client = build_non_redirect_client()?;

    let oauth_code = app::auth::get_oauth_code(&client, uid).await?;

    let (ses, user) = app::auth::authorize(&client, &oauth_code).await?;

    Ok((ses, user))
}

/// Query electricity binding info
///
/// Wrapper of user's electricity binding info query procedure.
///
/// If no binding info, return Err([`Error::NoBind`])
pub async fn query_ele_bind(session: &str) -> Result<app::electricity::EleBindInfo, Error> {
    let handler = app::AppHandler::build(session)?;

    let bind = handler.query_electricity_binding().await?;

    Ok(bind)
}

/// Query electricity
///
/// Wrapper of query electricity procedure.
///
/// Default query electricity by user's electricity(room) binding info.
///
/// If no binding info, return Err([`Error::NoBind`])
pub async fn query_ele(session: &str) -> Result<app::electricity::ElectricityInfo, Error> {
    // Init authorized handler
    let handler = app::AppHandler::build(session)?;

    // Query Bind Info
    let bind_info = handler.query_electricity_binding().await?;

    // Query Electricity Info
    handler.query_electricity(&bind_info.into()).await
}

/// Query electricity by [`app::electricity::RoomInfo`]
///
/// Wrapper of query electricity procedure.
pub async fn query_ele_by_room_info(
    session: &str,
    room_info: &app::electricity::RoomInfo,
) -> Result<app::electricity::ElectricityInfo, Error> {
    // Init authorized handler
    let handler = app::AppHandler::build(session)?;

    handler.query_electricity(room_info).await
}
