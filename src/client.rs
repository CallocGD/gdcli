
pub mod user;
pub mod level;
pub mod parser;
pub mod gdenums;

use std::collections::HashMap;
use std::fmt;
use std::process::exit;

use self::user::User;
use self::level::Level;


pub struct GDClient{
    client: reqwest::Client,
    host:String,
}

impl GDClient {
    pub (crate) fn new(gdps_host:String, connection_timeout: u64, proxy:&Option<String>) -> GDClient{
        let client = reqwest::Client::builder();

        if proxy.is_some() {
            return GDClient{client: client
                /* Cloudflare firewall will throw a fit unless the user-agent is removed */
                .user_agent("")
                .proxy(reqwest::Proxy::all(proxy.as_ref().unwrap().clone()).expect("Invalid Proxy Url"))
                .connect_timeout(std::time::Duration::from_secs(connection_timeout))
                .build()
                .expect("GDClient Failed to configure"), 
                host:gdps_host}
        }
        return GDClient{
                client: client
                .user_agent("")
                .connect_timeout(std::time::Duration::from_secs(connection_timeout))
                .build()
                .expect("GDClient Failed to configure"),
                host:gdps_host
            }
    }
    pub (crate) fn url(&self, endpoint:String) -> reqwest::Url {
        return reqwest::Url::parse(&self.host).unwrap().join(&endpoint).expect("url failed to parse");
    }
    
    pub (crate) async fn post(&self, endpoint:&str, fields:HashMap<&str, String>) -> Result<reqwest::Response, reqwest::Error> {
        return self.client.post(self.url(endpoint.to_string())).form(&fields).send().await
    }
    
    pub (crate) async fn fetch_user(&self, name_or_id:&String) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert("secret", "Wmfd2893gb7".to_string());
        fields.insert("str", name_or_id.to_string());
        return self.post("/database/getGJUsers20.php", fields).await
    }

    pub (crate) async fn get_user_info(&self, target_account_id:&String) -> Result<reqwest::Response, reqwest::Error> {
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert("secret", "Wmfd2893gb7".to_string());
        fields.insert("targetAccountID", target_account_id.to_string());
        return self.post("/database/getGJUserInfo20.php", fields).await
    }

    pub (crate) async fn download_level(&self, level_id:i64) -> Result<reqwest::Response, reqwest::Error>{
        let mut fields: HashMap<&str, String> = HashMap::new();
        fields.insert("secret", "Wmfd2893gb7".to_string());
        fields.insert("levelID", level_id.to_string());
        return self.post("/database/downloadGJLevel22.php", fields).await;
    }

}


pub fn create_client<'a>(mut gdps_host:Option<&'a str>, connection_timeout:Option<u64>, proxy:&Option<String>) -> GDClient{
    return GDClient::new(
        gdps_host.get_or_insert("http://www.boomlings.com").to_string(), 
        connection_timeout.unwrap_or(10), 
        proxy
    )
}


/* TODO: Boolean to request more user info (Which requires the second request...) */

pub async fn get_user(id:&String, connection_timeout:Option<u64>, proxy:&Option<String>) -> Result<User, String>{
    let client = create_client(Some("http://www.boomlings.com"), connection_timeout, proxy);
    let _resp = client.fetch_user(id).await;
    let resp = _resp.unwrap().error_for_status().unwrap();
    let text = resp.text().await.unwrap();
    if text.starts_with("-"){
        return Err(fmt::format(format_args!("User Does Not exist {}", id)));
    }

    let part1 = User::new(text);
    let user_str = client.get_user_info(&part1.account_id.to_string()).await.unwrap().error_for_status().unwrap().text().await.unwrap();
    return Ok(User::new(user_str));
}

pub async fn get_level(id:i64, connection_timeout:Option<u64>, proxy:&Option<String>) -> Level{
    if id < -2 {
        println!("ERROR ID \"{}\" is invalid ", id);
        exit(-1);
    }
    let client = create_client(Some("http://www.boomlings.com"), connection_timeout, proxy);
    let text = client.download_level(id).await.unwrap().error_for_status().unwrap().text().await.unwrap();
    if text == "-1" {
        println!("Level Requested does not exist");
        exit(-1);
    }
    return Level::new(text); 
}





