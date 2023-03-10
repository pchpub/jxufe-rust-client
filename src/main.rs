use jxufe_pch::mods::types::JxufeClient;

#[tokio::main]
async fn main() {
    let username = "";
    let password = "";
    let user_agent = "";
    let proxy_url = "";
    
    let mut client = match JxufeClient::new(username, password,"", proxy_url, user_agent) {
        Ok(value) => value,
        Err(value) => {
            println!("[Error] {}", value);
            std::process::exit(1);
        }
    };

    client.login().await.unwrap();
}
