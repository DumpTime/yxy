//! # yxy-cli
//!
//! CLI for YXY

use clap::Parser;
use yxy::*;

mod arg;
mod conf;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<yxy::error::Error>> {
    let opts = arg::Options::parse();

    if let Some(v) = opts.command {
        match v {
            arg::Commands::Query { query: q, arg: a } => match q {
                arg::Query::Uid => {
                    query_uid(&a, opts.verbose).await?;
                }
                arg::Query::Electricity => {
                    let (result, _session) = query_ele(&a, None, opts.verbose).await?;
                    print_ele(&result);
                }
            },
        }
    } else {
        let conf_path = match &opts.config {
            Some(c) => c,
            None => "./conf.yaml",
        };

        let conf = match conf::Config::parse(conf_path).await {
            Ok(v) => v,
            Err(e) => {
                return Err(Box::new(yxy::error::Error::Runtime(format!(
                    "Read/Parse conf.yaml file error: {}",
                    e
                ))));
            }
        };

        // Read the session cache
        let session = match &conf.cookie_file {
            None => None,
            Some(cookie_file) => match tokio::fs::read_to_string(cookie_file).await {
                Ok(v) => {
                    if opts.verbose {
                        println!("Using cached session id: {}", v);
                    }
                    Some(v)
                }
                Err(e) => {
                    eprintln!("Session cache file reading error: {}", e);
                    None
                }
            },
        };

        // Default query electricity
        let (result, session) = query_ele(&conf.uid, session, opts.verbose).await?;

        // Cache the session
        if let Some(cookie_file) = &conf.cookie_file {
            if let Err(e) = yxy::utils::file_write(cookie_file, &session.unwrap()) {
                eprintln!("Fail to cache the session id: {}", e);
            } else if opts.verbose {
                println!("Session cached.")
            }
        }

        // Notification
        if opts.notify {
            // Message push service
            if let Some(sc) = conf.server_chan {
                println!("Pushing message to ServerChan channel...");
                if result.soc < sc.warning_threshold {
                    utils::push_message(
                        &sc.key,
                        &format!("{}{}", &sc.warning_title, &result.soc),
                        &fmt_ele_md(&result),
                    )
                    .await?;
                } else if sc.log_level == 0 {
                    utils::push_message(
                        &sc.key,
                        &format!("{}{}", &sc.title, &result.soc),
                        &fmt_ele_md(&result),
                    )
                    .await?;
                } else {
                    println!("Nothing to do.");
                }
                println!("Success.")
            } else {
                eprintln!("No message push config found");
            }
        } else {
            print_ele(&result);
        }
    }

    Ok(())
}

/// fmt & print electricity info
fn print_ele(info: &yxy::SurplusInfo) {
    let surplus = &info.surplus_list[0];
    println!(
        "
Electricity Info: 
-----------------
Room: {}
Status: {}

Total Surplus: {} kW·h
Total Amount: ￥{}

Basic: {} kW·h | ￥{}
Subsidy : {} kW·h | ￥{}
",
        info.display_room_name,
        surplus.room_status,
        info.soc,
        info.total_soc_amount,
        surplus.surplus,
        surplus.amount,
        surplus.subsidy,
        surplus.subsidy_amount,
    );
}

/// fmt electricity info in markdown style
pub fn fmt_ele_md(info: &yxy::SurplusInfo) -> String {
    let surplus = &info.surplus_list[0];
    format!(
        "\
# Electricity Info
-----------------
- Room: **{}**
- Status: **{}**

- Total Surplus: **{}** kW·h
- Total Amount: **￥{}**

- Basic: **{}** kW·h | **￥{}**
- Subsidy : **{}** kW·h | **￥{}**
",
        info.display_room_name,
        surplus.room_status,
        info.soc,
        info.total_soc_amount,
        surplus.surplus,
        surplus.amount,
        surplus.subsidy,
        surplus.subsidy_amount,
    )
}

/// Query UID procedure
async fn query_uid(phone_num: &str, verbose: bool) -> Result<(), yxy::error::Error> {
    let handler = yxy::bind::campus::login::LoginHandler::new()?;

    println!("Querying security token...");
    let security_token = handler.security_token().await?;
    if verbose {
        println!("Success: {:?}", security_token);
    }

    let mut captcha = String::new();
    if security_token.level != 0 {
        // image captcha required
        println!("Image captcha required.");
        let result = handler
            .captcha_image(&security_token.security_token)
            .await?;

        println!("Captcha: {}", result);

        println!("Please input the captcha: ");
        std::io::stdin().read_line(&mut captcha)?;
    }

    println!("Sending verification code...");
    let user_exists = handler
        .send_verification_code(
            phone_num,
            &security_token.security_token,
            if security_token.level == 0 {
                None
            } else {
                Some(&captcha)
            },
        )
        .await?;

    if !user_exists {
        eprintln!("Current user is not registered");
    }

    // Get code from stdin
    let mut code = String::new();
    println!("Send SMS successfully, please enter the verification code:");
    std::io::stdin().read_line(&mut code)?;

    println!("Login...");
    let result = handler.login_by_code(phone_num, &code).await?;
    if verbose {
        println!("Login response: {:?}", result);
    }
    println!("Login successfully. Here is your uid & other information:");

    // stdout infos
    println!(
        "

    UID: {}
    Token: {}
    Login by device id: {}
    ----------------------------
    Job no: {}
    ID card: {}
    Bind card status: {}
    Last login Time: {}

    ",
        result.id,
        result.token,
        result.device_id,
        result.job_no.unwrap_or_default(),
        result.user_idcard.unwrap_or_default(),
        result.bind_card_status,
        result.last_login,
    );

    Ok(())
}

/// Procedure of query electricity
async fn query_ele(
    uid: &str,
    mut session: Option<String>,
    verbose: bool,
) -> Result<(SurplusInfo, Option<String>), error::Error> {
    let mut tried = false;
    loop {
        if session.is_none() {
            let (ses, _) = app_auth(uid, verbose).await?;
            session.replace(ses);
        }
        match app_query_ele(session.as_ref().unwrap(), verbose).await {
            Err(e) => {
                // Handle errors
                match e {
                    error::Error::Auth(_) => {
                        if tried {
                            return Err(error::Error::Auth(
                                "Maximum auth retry number reached.".into(),
                            ));
                        }
                        session.take();
                        if verbose {
                            eprintln!("Auth may expired, trying to reauthorize.")
                        }
                    }
                    _ => return Err(e),
                }
                tried = true;
            }
            Ok(v) => {
                return Ok((v, session));
            }
        }
    }
}

/// Authorization sub-procedure
async fn app_auth(id: &str, verbose: bool) -> Result<(String, UserInfo), error::Error> {
    let client = bind::build_non_redirect_client()?;

    if verbose {
        println!("Trying to get oauth code...");
        let oauth_code = bind::app::auth::get_oauth_code(&client, id).await?;
        println!("OAuth Code: {}", oauth_code);

        println!("Trying to auth...");
        let (ses, user) = bind::app::auth::authorize(&client, &oauth_code).await?;
        println!("Authorized, the session id is: {}", ses);

        Ok((ses, user))
    } else {
        let oauth_code = bind::app::auth::get_oauth_code(&client, id).await?;

        let (ses, user) = bind::app::auth::authorize(&client, &oauth_code).await?;

        Ok((ses, user))
    }
}

/// Application sub-procedure
async fn app_query_ele(session: &str, verbose: bool) -> Result<SurplusInfo, error::Error> {
    // Init authorized handler
    let handler = bind::app::AppHandler::build(session)?;

    // Query Bind Info
    if verbose {
        println!("Querying bind info...");
        let bind_info = handler.binding_info().await?;
        println!("Bind info: {:?}", bind_info);

        // Query Electricity Info
        println!("Query electricity info...");

        let electricity_info = handler.surplus(&yxy::RoomInfo::from(bind_info)).await?;
        println!("Electricity info: {:?}", electricity_info);

        Ok(electricity_info)
    } else {
        let bind_info = handler.binding_info().await?;

        // Query Electricity Info
        let electricity_info = handler.surplus(&yxy::RoomInfo::from(bind_info)).await?;

        Ok(electricity_info)
    }
}
