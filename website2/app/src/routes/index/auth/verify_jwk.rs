use alcoholic_jwt::{validate, JWKS, ValidJWT};
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
    let res = reqwest::get(&config.jwk_url).await.expect("could not fetch JSON Web Keys from Firebase");
    res.json::<JWKS>().await.expect("Could not parse Firebase JWKS JSON")
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

/* use std::{collections::HashMap, error::Error, thread::__FastLocalKeyInner, time::Duration};

use reqwest::Response;
use serde::Deserialize;

// See https://medium.com/@maylukas/firebase-token-authentication-in-rust-a1885f0982df

const FIREBASE_KEY_URL: &'static str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

#[derive(Clone, Debug)]
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

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

pub struct JwkKeys {
    pub keys: Vec<JwkKey>,
    pub validity: Duration,
}

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

pub async fn fetch_keys_for_config(
    config: &JwkConfiguration,
) -> Result<JwkKeys, Box<dyn std::error::Error>> {
    let validity = get_max_age(&http_response).unwrap_or(FALLBACK_TIMEOUT);
    reqwest::get(&config.jwk_url)
        .await?
        .json::<KeyResponse>()
        .await?
        .map(|res| JwkKeys {
            keys: res.keys,
            validity,
        })
}

pub async fn fetch_keys() -> Result<JwkKeys, Box<dyn Error>> {
    return fetch_keys_for_config(&get_configuration()).await;
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    // The audience the token was issued for
    pub aud: String,
    // The expiry date -- as epoch seconds
    pub exp: i64,
    // The token issuer
    pub iss: String,
    // The subject the token refers to
    pub sub: String,
    // Issued at -- as epoch seconds
    pub iat: i64,
}

enum VerificationError {
    InvalidSignature,
    UnkownKeyAlgorithm,
}

pub enum MaxAgeParseError {
    NoMaxAgeSpecified,
    NoCacheControlHeader,
    MaxAgeValueEmpty,
    NonNumericMaxAge,
}

// Determines the max age of an HTTP response
pub fn get_max_age(response: &Response) -> Result<Duration, MaxAgeParseError> {
    let headers = response.headers();
    let header = headers.get("Cache-Control");

    match header {
        Some(header_value) => parse_cache_control_header(header_value),
        None => Err(MaxAgeParseError::NoCacheControlHeader),
    }
}

fn parse_max_age_value(cache_control_value: &str) -> Result<Duration, MaxAgeParseError> {
    let tokens: Vec<&str> = cache_control_value.split(",").collect();
    for token in tokens {
        let key_value: Vec<&str> = token.split("=").map(|s| s.trim()).collect();
        let key = key_value.first().unwrap();
        let val = key_value.get(1);

        if String::from("max-age").eq(&key.to_lowercase()) {
            match val {
                Some(value) => {
                    return Ok(Duration::from_secs(
                        value
                            .parse()
                            .map_err(|_| MaxAgeParseError::NonNumericMaxAge)?,
                    ))
                }
                None => return Err(MaxAgeParseError::MaxAgeValueEmpty),
            }
        }
    }
    return Err(MaxAgeParseError::NoMaxAgeSpecified);
}

fn parse_cache_control_header(
    header_value: &reqwest::header::HeaderValue,
) -> Result<Duration, MaxAgeParseError> {
    match header_value.to_str() {
        Ok(string_value) => parse_max_age_value(string_value),
        Err(_) => Err(MaxAgeParseError::NoCacheControlHeader),
    }
}

#[derive(Debug)]
pub struct JwkVerifier {
    keys: HashMap<String, JwkKey>,
    config: JwkConfiguration,
}

fn keys_to_map(keys: Vec<JwkKey>) -> HashMap<String, JwkKey> {
    let mut keys_as_map = HashMap::new();
    for key in keys {
        keys_as_map.insert(String::clone(&key.kid), key);
    }
    keys_as_map
}

impl JwkVerifier {
    pub fn new(keys: Vec<JwkKey>) -> JwkVerifier {
        JwkVerifier {
            keys: keys_to_map(keys),
            config: get_configuration(),
        }
    }

    pub fn verify(&self, token: &String) -> Option<TokenData<Claims>> {
        let token_kid = match decode_header(token).map(|header| header.kid) {
            Ok(Some(header)) => header,
            _ => return None,
        };

        let jwk_key = match self.get_key(token_kid) {
            Some(key) => key,
            _ => return None,
        };

        match self.decode_token_with_key(jwk_key, token) {
            Ok(token_data) => Some(token_data),
            _ => None,
        }
    }

    pub fn set_keys(&mut self, keys: Vec<JwkKey>) {
        self.keys = keys_to_map(keys);
    }

    fn get_key(&self, key_id: String) -> Option<&JwkKey> {
        self.keys.get(&key_id)
    }

    fn decode_token_with_key(
        &self,
        key: &JwkKey,
        token: &String,
    ) -> Result<TokenData<Claims>, VerificationError> {
        let algorithm = match Algorithm::from_str(&key.alg) {
            Ok(alg) => alg,
            Err(_error) => return Err(VerificationError::UnkownKeyAlgorithm),
        };

        let mut validation = Validation::new(algorithm);
        validation.set_audience(&[&self.config.audience]);
        validation.iss = Some(self.config.issuer.clone());
        let key = DecodingKey::from_rsa_components(&key.n, &key.e);
        return decode::<Claims>(token, &key, &validation)
            .map_err(|_| VerificationError::InvalidSignature);
    }
}

type CleanupFn = Box<dyn Fn() -> () + Send>;

pub struct JwkAuth {
    verifier: Arc<Mutex<JwkVerifier>>,
    cleanup: Mutex<CleanupFn>,
}

impl Drop for JwkAuth {
    fn drop(&mut self) {
        // Stop the update thread when the updater is destructed
        let cleanup_fn = self.cleanup.lock().unwrap();
        cleanup_fn();
    }
}

impl JwkAuth {
    pub fn new() -> JwkAuth {
        let jwk_key_result = jwk::fetch_keys();
        let jwk_keys: JwkKeys = match jwk_key_result {
            Ok(keys) => keys,
            Err(_) => {
                panic!("Unable to fetch jwk keys! Cannot verify user tokens! Shutting down...")
            }
        };
        let verifier = Arc::new(Mutex::new(JwkVerifier::new(jwk_keys.keys)));

        let mut instance = JwkAuth {
            verifier: verifier,
            cleanup: Mutex::new(Box::new(|| {})),
        };

        instance.start_key_update();
        instance
    }

    pub fn verify(&self, token: &String) -> Option<TokenData<Claims>> {
        let verifier = self.verifier.lock().unwrap();
        verifier.verify(token)
    }

    fn start_key_update(&mut self) {
        let verifier_ref = Arc::clone(&self.verifier);

        let stop = use_repeating_job(move || match fetch_keys() {
            Ok(jwk_keys) => {
                let mut verifier = verifier_ref.lock().unwrap();
                verifier.set_keys(jwk_keys.keys);
                println!(
                    "Updated JWK keys. Next refresh will be in {:?}",
                    jwk_keys.validity
                );
                jwk_keys.validity
            }
            Err(_) => Duration::from_secs(10),
        });

        let mut cleanup = self.cleanup.lock().unwrap();
        *cleanup = stop;
    }
}

type Delay = Duration;
type Cancel = Box<dyn Fn() -> () + Send>;

// Runs a given closure as a repeating job until the cancel callback is invoked.
// The jobs are run with a delay returned by the closure execution.
pub fn use_repeating_job<F>(job: F) -> Cancel
where
    F: Fn() -> Delay,
    F: Send + 'static,
{
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    thread::spawn(move || loop {
        let delay = job();
        thread::sleep(delay);

        if let Ok(_) | Err(TryRecvError::Disconnected) = shutdown_rx.try_recv() {
            break;
        }
    });

    Box::new(move || {
        println!("Stopping...");
        let _ = shutdown_tx.send("stop");
    })
}
 */
