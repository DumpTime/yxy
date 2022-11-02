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

#[tokio::test]
async fn application() {
    use yxy::bind::app::*;

    let handler = Arc::new(AppHandler::build(&CONFIG.session_token).unwrap());

    let h = handler.clone();
    let task1 = tokio::spawn(async move {
        let bind_info = h.binding_info().await.unwrap();
        println!("{:#?}", bind_info);

        let h1 = h.clone();
        let task1 = tokio::spawn(async move {
            let ele_info = h1.surplus(&bind_info.into()).await.unwrap();
            println!("{:#?}", ele_info);
        });

        let h1 = h.clone();
        let task2 = tokio::spawn(async move {
            let record = h1.my_recharge_records(1).await.unwrap();
            println!("{:#?}", record);
        });

        try_join!(task1, task2).unwrap();
    });

    let h = handler.clone();
    let task2 = tokio::spawn(async move {
        let user_info = h.user_info().await.unwrap();
        println!("{:#?}", user_info);
    });

    try_join!(task1, task2).unwrap();
}

#[tokio::test]
async fn auth() -> Result {
    use yxy::wrapper::app_auth;

    let (session_token, user_info) = app_auth(&CONFIG.uid).await?;
    println!("session_token: {session_token}, {user_info:#?}");

    Ok(())
}
