use crate::error::{Error, Result};

pub enum Hash {
    Sha2_256,
    Sha2_384,
    Sha2_512,
}

impl Hash {
    /// Convert the identifier of an hash to an instance of `Self`
    fn from_alg(alg: &str) -> Result<Self> {
        let algorithm = match alg {
            "sha256" => Self::Sha2_256,
            "sha384" => Self::Sha2_384,
            "sha512" => Self::Sha2_512,
            _ => return Err(Error::UnknownHashAlgorithm),
        };

        Ok(algorithm)
    }

    /// Return the identifier of the current hash algorithm
    fn to_alg(&self) -> &str {
        match self {
            Self::Sha2_256 => "sha256",
            Self::Sha2_384 => "sha384",
            Self::Sha2_512 => "sha512",
        }
    }
}
