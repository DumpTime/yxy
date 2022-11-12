mod common;

use common::Config;
use once_cell::sync::Lazy;

use std::{error::Error, path::PathBuf, sync::Arc};
use tokio::try_join;

const CONFIG_PATH: &str = "test.yaml";
static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(CONFIG_PATH);
    println!("{}", d.display());
    Config::parse_blocking(&d).unwrap()
});

type Result = std::result::Result<(), Box<dyn Error>>;

#[ignore]
#[tokio::test]
async fn application() {
    use yxy::bind::app::*;
    use yxy::RoomInfo;

    let handler = Arc::new(AppHandler::build(&CONFIG.session_token).unwrap());

    let h = handler.clone();
    let task1 = tokio::spawn(async move {
        let bind_info = h.binding_info().await.unwrap();
        println!("{:#?}", bind_info);
        let room_info = Arc::new(RoomInfo::from(bind_info));

        let h1 = h.clone();
        let ri = room_info.clone();
        let task1 = tokio::spawn(async move {
            let surplus = h1.surplus(&ri).await.unwrap();
            println!("{:#?}", surplus);
        });

        let h1 = h.clone();
        let task2 = tokio::spawn(async move {
            let record = h1.user_recharge_records(1).await.unwrap();
            println!("{:#?}", record);
        });

        let h1 = h.clone();
        let ri = room_info.clone();
        let task3 = tokio::spawn(async move {
            let record = h1.room_recharge_records(1, &ri).await.unwrap();
            println!("{:#?}", record);
        });

        let (h1, ri) = (h.clone(), room_info.clone());
        let task4 = tokio::spawn(async move {
            let record = h1.usage_records(&ri, None).await.unwrap();
            println!("{:#?}", record);
        });

        try_join!(task1, task2, task3, task4).unwrap();
    });

    let h = handler.clone();
    let task2 = tokio::spawn(async move {
        let user_info = h.user_info().await.unwrap();
        println!("{:#?}", user_info);
    });

    try_join!(task1, task2).unwrap();
}

#[ignore]
#[tokio::test]
async fn app_auth() -> Result {
    use yxy::wrapper::app_auth;

    let (session_token, user_info) = app_auth(&CONFIG.uid).await?;
    println!("session_token: {session_token}\n{user_info:#?}");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn campus() {
    use yxy::bind::campus::*;

    let handler = CampusHandler::build(
        &CONFIG.campus_token,
        &CONFIG.device_id,
        &CONFIG.uid,
        &CONFIG.school_code,
    )
    .unwrap();
    let record = handler.query_card_balance().await.unwrap();

    println!("{}", record);
}
