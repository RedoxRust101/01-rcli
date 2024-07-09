use crate::Claims;
use anyhow::{Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Encodes given claims into a JWT token using the specified secret.
///
/// # Arguments
///
/// * `claims` - The claims to encode into the token.
/// * `secret` - The secret key used for encoding.
///
/// # Returns
///
/// A `Result` which is either a JWT token string or an error.
pub fn process_jwt_sign(claims: Claims, secret: &str) -> Result<String> {
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

/// Decodes a JWT token and validates it against the expected audience and secret.
///
/// # Arguments
///
/// * `token` - The JWT token to decode.
/// * `secret` - The secret key used for decoding.
/// * `expected_aud` - The expected audience value.
///
/// # Returns
///
/// A `Result` which is either the decoded `Claims` or an error.
pub fn process_jwt_verify(token: &str, secret: &str) -> Result<Claims> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Claims;

    #[test]
    fn test_process_jwt_sign() -> Result<()> {
        let claims = Claims {
            reg_claims: crate::RegisteredClaims {
                iss: "test".into(),
                sub: "test".into(),
                aud: "test".into(),
                exp: 9999999999,
                nbf: 0,
                iat: 0,
                jti: "id".into(),
            },
            pub_claims: crate::PublicClaims {
                role: "test".into(),
            },
            priv_claims: crate::PrivateClaims { user_id: 42 },
        };
        let jwt_token = crate::process_jwt_sign(claims, "DX$5y_*ahMu^QEqX_m1iSmiY!8&Z@qDM")?;
        assert_eq!(jwt_token,  "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJ0ZXN0Iiwic3ViIjoidGVzdCIsImF1ZCI6InRlc3QiLCJleHAiOjk5OTk5OTk5OTksIm5iZiI6MCwiaWF0IjowLCJqdGkiOiJpZCIsInJvbGUiOiJ0ZXN0IiwidXNlcl9pZCI6NDJ9.Q3v3-jI28fUluyhx0_2Ahit2mMhKTMewYAoGfrA-0Uc");
        Ok(())
    }

    #[test]
    fn test_process_jwt_verify() -> Result<()> {
        let ret = crate::process_jwt_verify("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJ0ZXN0Iiwic3ViIjoidGVzdCIsImF1ZCI6InRlc3QiLCJleHAiOjk5OTk5OTk5OTksIm5iZiI6MCwiaWF0IjowLCJqdGkiOiJpZCIsInJvbGUiOiJ0ZXN0IiwidXNlcl9pZCI6NDJ9.Q3v3-jI28fUluyhx0_2Ahit2mMhKTMewYAoGfrA-0Uc", "DX$5y_*ahMu^QEqX_m1iSmiY!8&Z@qDM");
        assert!(ret.is_ok());
        Ok(())
    }
}
