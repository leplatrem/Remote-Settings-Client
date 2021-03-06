use {
    super::{Collection, SignatureError, Verification},
    log::debug,
};

pub struct DefaultVerifier {}

impl Verification for DefaultVerifier {
    fn verify(&self, _collection: &Collection) -> Result<(), SignatureError> {
        debug!("default verifier implementation");
        Ok(())
    }
}
