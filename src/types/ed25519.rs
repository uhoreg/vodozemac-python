use crate::error::*;
use pyo3::{prelude::*, types::PyType};

#[pyclass]
pub struct Ed25519PublicKey {
    pub(crate) inner: vodozemac::Ed25519PublicKey,
}

#[pymethods]
impl Ed25519PublicKey {
    #[classmethod]
    pub fn from_base64(_cls: &Bound<'_, PyType>, key: &str) -> Result<Self, KeyError> {
        Ok(Self {
            inner: vodozemac::Ed25519PublicKey::from_base64(key)?,
        })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }

    pub fn verify_signature(
        &self,
        message: &str,
        signature: &Ed25519Signature,
    ) -> Result<(), SignatureError> {
        self.inner.verify(message.as_bytes(), &signature.inner)?;

        Ok(())
    }

    #[classattr]
    const __hash__: Option<PyObject> = None;

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl From<vodozemac::Ed25519PublicKey> for Ed25519PublicKey {
    fn from(value: vodozemac::Ed25519PublicKey) -> Self {
        Self { inner: value }
    }
}

#[pyclass]
pub struct Ed25519Signature {
    inner: vodozemac::Ed25519Signature,
}

#[pymethods]
impl Ed25519Signature {
    #[classmethod]
    pub fn from_base64(
        _cls: &Bound<'_, PyType>,
        session_key: &str,
    ) -> Result<Self, SignatureError> {
        Ok(Self {
            inner: vodozemac::Ed25519Signature::from_base64(session_key)?,
        })
    }

    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}

impl From<vodozemac::Ed25519Signature> for Ed25519Signature {
    fn from(value: vodozemac::Ed25519Signature) -> Self {
        Self { inner: value }
    }
}
