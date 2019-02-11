use crate::config::EnvConfig;
use crate::error::Error;

#[test]
fn test_invalid_env_value() {
    let vars = vec![("GIT_BRWS_GHE_SSH_PORT".to_string(), "hello".to_string())];
    match EnvConfig::from_iter(vars.into_iter()).unwrap_err() {
        Error::EnvLoadError(e) => {
            let msg = format!("{}", e);
            assert!(msg.contains("invalid digit found in string"), "{}", msg);
        }
        err => assert!(false, "Unexpected error: {}", err),
    }
}
