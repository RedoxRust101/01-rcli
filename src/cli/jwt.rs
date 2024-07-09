use crate::{convert_duration_to_seconds, load_config_from_file, new_uuid, CmdExecutor};
use anyhow::{Ok, Result};
use chrono::{Duration, Utc};
use clap::{command, Parser};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(about = "Sign a message with a private key")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a message with a public key")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long)]
    sub: String,
    #[arg(long)]
    aud: String,
    #[arg(long, default_value = "1h")]
    exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    token: String,
}

// Define a struct for the registered claims (standard fields)
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredClaims {
    // Examples of registered claims
    pub iss: String, // Issuer
    pub sub: String, // Subject
    pub aud: String, // Audience
    pub exp: usize,  // Expiration Time
    pub nbf: usize,  // Not Before
    pub iat: usize,  // Issued At
    pub jti: String, // JWT ID
}

// Define a struct for the public claims (custom fields that are not private)
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicClaims {
    pub role: String,
}

// Define a struct for the private claims (custom fields that are private)
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateClaims {
    pub user_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    iss: String,
    secret: String,
}

// Define a struct that combines all the claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Registered claims
    #[serde(flatten)]
    pub reg_claims: RegisteredClaims,
    // Public claims
    #[serde(flatten)]
    pub pub_claims: PublicClaims,
    // Private claims
    #[serde(flatten)]
    pub priv_claims: PrivateClaims,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        let config = load_config_from_file("config.json")?;

        let duration_in_seconds =
            convert_duration_to_seconds(&self.exp).expect("Failed to parse duration");

        let now = Utc::now();
        let now_usize = now.timestamp() as usize;
        let expiration_time = now + Duration::seconds(duration_in_seconds as i64);

        let claims = Claims {
            reg_claims: RegisteredClaims {
                iss: config.iss,
                sub: self.sub,
                aud: self.aud,
                exp: expiration_time.timestamp() as usize,
                nbf: now_usize,
                iat: now_usize,
                jti: new_uuid(),
            },
            pub_claims: PublicClaims {
                role: "test".into(),
            },
            priv_claims: PrivateClaims { user_id: 42 },
        };
        let jwt_token = crate::process_jwt_sign(claims, &config.secret)?;
        println!("{}", jwt_token);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        let config = load_config_from_file("config.json")?;
        let claims = crate::process_jwt_verify(&self.token, &config.secret)?;
        println!("{:?}", claims);
        Ok(())
    }
}
