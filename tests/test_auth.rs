

mod test_credentials;

use unifi_protect::UnifiProtectServer;
use crate::test_credentials::{BASE_URI, PASSWORD, USERNAME};

#[tokio::test]
#[ignore]
async fn login_test() {
    let mut server = UnifiProtectServer::new(BASE_URI); // ( e.g. "https://192.168.1.28")

    match
    server
        .login(USERNAME, PASSWORD, None)
        .await {
        Ok(_) => (),
        Err(unifi_protect::error::Error::MfaRequired(..)) => {
            server
                .login(USERNAME, PASSWORD, Some("656677"))
                .await
                .expect("Failed to log in");
        }
        Err(e) => panic!("Failed to log in: {:?}", e),
    }
    println!("Logged in!");
}
