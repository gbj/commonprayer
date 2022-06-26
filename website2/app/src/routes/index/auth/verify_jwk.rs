use alcoholic_jwt::{validate, ValidJWT, JWKS};
use anyhow::Result;
use cached::proc_macro::cached;

const FIREBASE_KEY_URL: &'static str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct JwkConfiguration {
    pub jwk_url: String,
    pub audience: String,
    pub issuer: String,
}

pub fn get_configuration() -> JwkConfiguration {
    let app_id = std::env::var("FIREBASE_APP_ID").expect("FIREBASE_APP_ID env var not set");
    JwkConfiguration {
        jwk_url: FIREBASE_KEY_URL.to_string(),
        issuer: format!("https://securetoken.google.com/{}", &app_id),
        audience: app_id,
    }
}

// Per Firebase: keys *must not* be cached for > 6 hours
// https://firebase.google.com/docs/reference/appcheck/rest/v1beta/jwks
// We'll cache for five hours
#[cached(time = 18000)]
pub async fn fetch_keys() -> JWKS {
    let config = get_configuration();
    let res = reqwest::get(&config.jwk_url)
        .await
        .expect("could not fetch JSON Web Keys from Firebase");
    res.json::<JWKS>()
        .await
        .expect("Could not parse Firebase JWKS JSON")
}

pub async fn validate_token(token: &str) -> Result<ValidJWT> {
    let jwks: JWKS = fetch_keys().await;

    // Several types of built-in validations are provided:
    let config = get_configuration();
    let validations = vec![alcoholic_jwt::Validation::Issuer(config.issuer)];

    // If a JWKS contains multiple keys, the correct KID first
    // needs to be fetched from the token headers.
    let kid = alcoholic_jwt::token_kid(&token)
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    Ok(validate(token, jwk, validations)?)
}
