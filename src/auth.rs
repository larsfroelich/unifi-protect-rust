use crate::UnifiProtectServer;
use crate::error::Error;
use reqwest::Client;
use serde_json::json;

impl UnifiProtectServer {
    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), Error> {
        // Already logged in?
        if self.headers.contains_key("Cookie") && self.headers.contains_key("X-CSRF-Token") {
            return Ok(());
        }

        // Make sure we have a CSRF token, or get one if needed.
        let _ = self.acquire_csrf_token().await;

        // Log in
        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .post(&(self.uri.clone() + "/api/auth/login"))
            .headers(self.headers.clone())
            .json(&json!({
                "password": password,
                "rememberMe": true,
                "username": username,
                "token": ""
            }))
            .send()
            .await?;

        // Something went wrong with the login call, possibly a controller reboot or failure.
        if !response.status().is_success() {
            return Err(Error::LoginFailed(format!(
                "Status: {}, URL: {}",
                response.status(),
                response.url()
            )));
        }

        // We're logged in. Let's configure our headers.
        let csrf_token = response
            .headers()
            .get("X-CSRF-Token")
            .and_then(|value| value.to_str().ok());

        let cookie = response
            .headers()
            .get("Set-Cookie")
            .and_then(|value| value.to_str().ok());

        if let Some(token) = csrf_token {
            self.headers
                .insert("X-CSRF-Token", token.parse()?);
        }

        // Save the refreshed cookie
        if let Some(c) = cookie {
            self.headers.insert("Cookie", c.parse()?);
            return Ok(());
        }

        Err(Error::LoginFailed("No cookie found in response".to_string()))
    }

    async fn acquire_csrf_token(&mut self) -> Result<(), Error> {
        // We only need to acquire a token if we aren't already logged in, or we don't already have a token.
        if self.headers.contains_key("X-CSRF-Token") {
            return Ok(());
        }

        // UniFi OS has cross-site request forgery protection built into its web management UI.
        // We use this fact to fingerprint it by connecting directly to the supplied Protect controller address
        // and see if there's a CSRF token waiting for us.

        let response = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .get(&self.uri)
            .send()
            .await?;

        if response.status().is_success() {
            let csrf_token = response
                .headers()
                .get("X-CSRF-Token")
                .and_then(|value| value.to_str().ok());

            // We found a token.
            if let Some(token) = csrf_token {
                self.headers
                    .insert("X-CSRF-Token", token.parse()?);
                return Ok(());
            }
        }

        // Something went wrong, or no CSRF-Token is needed
        Ok(())
    }

    pub fn clear_login_credentials(&mut self) {
        self.headers.remove("Cookie");
    }
}
