use anyhow::Result;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};

pub fn process_sign(
    alg: Algorithm,
    claims: &serde_json::Value,
    key: impl AsRef<[u8]>,
) -> Result<String> {
    let header = Header::new(alg);
    let encodingkey = match alg {
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            EncodingKey::from_secret(key.as_ref())
        }
        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => EncodingKey::from_rsa_der(key.as_ref()),
        Algorithm::ES256 | Algorithm::ES384 => EncodingKey::from_ec_der(key.as_ref()),
        Algorithm::EdDSA => EncodingKey::from_ed_der(key.as_ref()),
    };
    encode(&header, &claims, &encodingkey).map_err(|e| e.into())
}

pub fn process_verify(
    token: &str,
    key: impl AsRef<[u8]>,
    validation: Validation,
) -> Result<TokenData<serde_json::Value>> {
    let header = decode_header(token)?;
    let mut validation = validation;
    validation.algorithms = vec![header.alg];
    let decoding_key = match header.alg {
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            DecodingKey::from_secret(key.as_ref())
        }
        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => DecodingKey::from_rsa_der(key.as_ref()),
        Algorithm::ES256 | Algorithm::ES384 => DecodingKey::from_ec_der(key.as_ref()),
        Algorithm::EdDSA => DecodingKey::from_ed_der(key.as_ref()),
    };
    decode(token, &decoding_key, &validation).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const PKCS8_SK: &[u8] = include_bytes!("../../fixtures/ed25519_pkcs8.sk");
    const PKCS8_PK: &[u8] = include_bytes!("../../fixtures/ed25519_pkcs8.pk");
    #[test]
    fn test_jwt_eddsa() {
        let claims = json!(
            {
                "exp": 1000,
                "sub": "test_sub",
                "aud": "test_aud"
            }
        );
        let mut validation = Validation::default();
        validation.validate_exp = false;
        validation.set_audience(&["test_aud"]);

        let token = process_sign(Algorithm::EdDSA, &claims, PKCS8_SK).unwrap();
        let token_data = process_verify(&token, PKCS8_PK, validation).unwrap();
        assert_eq!(token_data.claims, claims);
    }
}
