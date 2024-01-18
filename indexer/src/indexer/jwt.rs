use {
    anyhow::{anyhow, Result},
    base64::{
        alphabet,
        engine::{self, general_purpose},
        Engine as _,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: Uuid,
    pub iat: usize,
    pub exp: usize,
}

pub fn get_id_from_jwt(token: &str) -> Result<Uuid> {
    let token_data = token.split('.').nth(1).ok_or(anyhow!("Invalid token"))?;

    let bytes = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::NO_PAD)
        .decode(token_data)?;

    let claims: TokenClaims = serde_json::from_slice(&bytes)?;

    Ok(claims.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjE0NjZjZjRmLTFkNzEtNDE1My1iOWFkLTRhOWMxYjQ4MTAxZSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxNTE2MjM5MDIyfQ.weKNyTDqRCHMnEmN1RNsKI5vD24w-qesqf9EMoqJz1M";

        let id = get_id_from_jwt(token).unwrap();
        assert_eq!(
            id,
            Uuid::parse_str("1466cf4f-1d71-4153-b9ad-4a9c1b48101e").unwrap()
        );
    }
}
