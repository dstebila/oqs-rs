//! KEM API
//!
//! See [`Kem`] for the main functionality.
//! [`Algorithm`] lists the available algorithms.
use alloc::borrow;
use alloc::vec::Vec;

use core::ptr::NonNull;

#[cfg(feature = "no_std")]
use cstr_core::CStr;
#[cfg(not(feature = "no_std"))]
use std::ffi::CStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ffi::kem as ffi;
use crate::newtype_buffer;
use crate::*;

newtype_buffer!(PublicKey, PublicKeyRef);
newtype_buffer!(SecretKey, SecretKeyRef);
newtype_buffer!(Ciphertext, CiphertextRef);
newtype_buffer!(SharedSecret, SharedSecretRef);

macro_rules! implement_kems {
    { $( $kem: ident: $oqs_id: ident),* $(,)? } => (

        /// Supported algorithms by OQS
        ///
        /// Note that this doesn't mean that they'll be available.
        ///
        /// Optional support for `serde` if that feature is enabled.
        #[derive(Clone, Copy, Debug)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[allow(missing_docs)]
        pub enum Algorithm {
            $(
                $kem,
            )*
        }

        fn algorithm_to_id(algorithm: Algorithm) -> *const libc::c_char {
            let id: &[u8] = match algorithm {
                $(
                    Algorithm::$kem => &ffi::$oqs_id[..],
                )*
            };
            id as *const _ as *const i8
        }

        $(
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $kem {
                use super::*;
                #[test]
                fn test_encaps_decaps() -> Result<()> {
                    crate::init();

                    let alg = Algorithm::$kem;
                    let kem = Kem::new(alg)?;
                    let (pk, sk) = kem.keypair()?;
                    let (ct, ss1) = kem.encapsulate(&pk)?;
                    let ss2 = kem.decapsulate(&sk, &ct)?;
                    assert_eq!(ss1, ss2, "shared secret not equal!");
                    Ok(())
                }

                #[test]
                fn test_enabled() {
                    crate::init();

                    assert!(Algorithm::$kem.is_enabled());
                }
            }
        )*
    )
}

implement_kems! {
    Default: OQS_KEM_alg_default,
    BikeL1Cpa: OQS_KEM_alg_bike1_l1_cpa,
    BikeL3Cpa: OQS_KEM_alg_bike1_l3_cpa,
    BikeL1Fo: OQS_KEM_alg_bike1_l1_fo,
    BikeL3Fo: OQS_KEM_alg_bike1_l3_fo,
    ClassicMcEliece348864: OQS_KEM_alg_classic_mceliece_348864,
    ClassicMcEliece348864f: OQS_KEM_alg_classic_mceliece_348864f,
    ClassicMcEliece460896: OQS_KEM_alg_classic_mceliece_460896,
    ClassicMcEliece460896f: OQS_KEM_alg_classic_mceliece_460896f,
    ClassicMcEliece6688128: OQS_KEM_alg_classic_mceliece_6688128,
    ClassicMcEliece6688128f: OQS_KEM_alg_classic_mceliece_6688128f,
    ClassicMcEliece6960119: OQS_KEM_alg_classic_mceliece_6960119,
    ClassicMcEliece6960119f: OQS_KEM_alg_classic_mceliece_6960119f,
    ClassicMcEliece8192128: OQS_KEM_alg_classic_mceliece_8192128,
    ClassicMcEliece8192128f: OQS_KEM_alg_classic_mceliece_8192128f,
    Hqc128: OQS_KEM_alg_hqc_128,
    Hqc192: OQS_KEM_alg_hqc_192,
    Hqc256: OQS_KEM_alg_hqc_256,
    Kyber512: OQS_KEM_alg_kyber_512,
    Kyber768: OQS_KEM_alg_kyber_768,
    Kyber1024: OQS_KEM_alg_kyber_1024,
    Kyber512_90s: OQS_KEM_alg_kyber_512_90s,
    Kyber768_90s: OQS_KEM_alg_kyber_768_90s,
    Kyber1024_90s: OQS_KEM_alg_kyber_1024_90s,
    NtruHps2048509: OQS_KEM_alg_ntru_hps2048509,
    NtruHps2048677: OQS_KEM_alg_ntru_hps2048677,
    NtruHps4096821: OQS_KEM_alg_ntru_hps4096821,
    NtruHrss701: OQS_KEM_alg_ntru_hrss701,
    NtruPrimeNtrulpr653: OQS_KEM_alg_ntruprime_ntrulpr653,
    NtruPrimeNtrulpr761: OQS_KEM_alg_ntruprime_ntrulpr761,
    NtruPrimeNtrulpr857: OQS_KEM_alg_ntruprime_ntrulpr857,
    NtruPrimeSntrup653: OQS_KEM_alg_ntruprime_sntrup653,
    NtruPrimeSntrup761: OQS_KEM_alg_ntruprime_sntrup761,
    NtruPrimeSntrup857: OQS_KEM_alg_ntruprime_sntrup857,
    Lightsaber: OQS_KEM_alg_saber_lightsaber,
    Saber: OQS_KEM_alg_saber_saber,
    Firesaber: OQS_KEM_alg_saber_firesaber,
    FrodoKem640Aes: OQS_KEM_alg_frodokem_640_aes,
    FrodoKem640Shake: OQS_KEM_alg_frodokem_640_shake,
    FrodoKem976Aes: OQS_KEM_alg_frodokem_976_aes,
    FrodoKem976Shake: OQS_KEM_alg_frodokem_976_shake,
    FrodoKem1344Aes: OQS_KEM_alg_frodokem_1344_aes,
    FrodoKem1344Shake: OQS_KEM_alg_frodokem_1344_shake,
    SidhP434: OQS_KEM_alg_sidh_p434,
    SidhP503: OQS_KEM_alg_sidh_p503,
    SidhP610: OQS_KEM_alg_sidh_p610,
    SidhP751: OQS_KEM_alg_sidh_p751,
    SidhP434Compressed: OQS_KEM_alg_sidh_p434_compressed,
    SidhP503Compressed: OQS_KEM_alg_sidh_p503_compressed,
    SidhP610Compressed: OQS_KEM_alg_sidh_p610_compressed,
    SidhP751Compressed: OQS_KEM_alg_sidh_p751_compressed,
    SikeP434: OQS_KEM_alg_sike_p434,
    SikeP503: OQS_KEM_alg_sike_p503,
    SikeP610: OQS_KEM_alg_sike_p610,
    SikeP751: OQS_KEM_alg_sike_p751,
    SikeP434Compressed: OQS_KEM_alg_sike_p434_compressed,
    SikeP503Compressed: OQS_KEM_alg_sike_p503_compressed,
    SikeP610Compressed: OQS_KEM_alg_sike_p610_compressed,
    SikeP751Compressed: OQS_KEM_alg_sike_p751_compressed,
}

impl core::default::Default for Algorithm {
    fn default() -> Self {
        Algorithm::Default
    }
}

impl Algorithm {
    /// Returns true if this algorithm is enabled in the linked version
    /// of liboqs
    pub fn is_enabled(self) -> bool {
        unsafe { ffi::OQS_KEM_alg_is_enabled(algorithm_to_id(self)) == 1 }
    }

    /// Provides a pointer to the id of the algorithm
    ///
    /// For use with the FFI api methods
    pub fn to_id(self) -> *const libc::c_char {
        algorithm_to_id(self)
    }
}

/// KEM algorithm
///
/// # Example
/// ```rust
/// use oqs;
/// oqs::init();
/// let kem = oqs::kem::Kem::default();
/// let (pk, sk) = kem.keypair().unwrap();
/// let (ct, ss) = kem.encapsulate(&pk).unwrap();
/// let ss2 = kem.decapsulate(&sk, &ct).unwrap();
/// assert_eq!(ss, ss2);
/// ```
pub struct Kem {
    kem: NonNull<ffi::OQS_KEM>,
}

unsafe impl Sync for Kem {}
unsafe impl Send for Kem {}

impl Drop for Kem {
    fn drop(&mut self) {
        unsafe { ffi::OQS_KEM_free(self.kem.as_ptr()) };
    }
}

impl core::default::Default for Kem {
    /// Get the default KEM algorithm in liboqs
    ///
    /// Panics if the default algorithm is not enabled in liboqs.
    fn default() -> Self {
        Kem::new(Algorithm::Default).expect("Expected default algorithm to be enabled")
    }
}

impl core::convert::TryFrom<Algorithm> for Kem {
    type Error = crate::Error;
    fn try_from(alg: Algorithm) -> Result<Kem> {
        Kem::new(alg)
    }
}

impl Kem {
    /// Construct a new algorithm
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        let kem = unsafe { ffi::OQS_KEM_new(algorithm_to_id(algorithm)) };
        NonNull::new(kem).map_or_else(|| Err(Error::AlgorithmDisabled), |kem| Ok(Self { kem }))
    }

    /// Get the name of the algorithm
    pub fn name(&self) -> borrow::Cow<str> {
        let kem = unsafe { self.kem.as_ref() };
        let cstr = unsafe { CStr::from_ptr(kem.method_name) };
        cstr.to_string_lossy()
    }

    /// Get the version of the implementation
    pub fn version(&self) -> borrow::Cow<str> {
        let kem = unsafe { self.kem.as_ref() };
        let cstr = unsafe { CStr::from_ptr(kem.method_name) };
        cstr.to_string_lossy()
    }

    /// Get the claimed nist level
    pub fn claimed_nist_level(&self) -> u8 {
        let kem = unsafe { self.kem.as_ref() };
        kem.claimed_nist_level
    }

    /// Is the algorithm ind_cca secure
    pub fn is_ind_cca(&self) -> bool {
        let kem = unsafe { self.kem.as_ref() };
        kem.ind_cca
    }

    /// Get the length of the public key
    pub fn length_public_key(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_public_key
    }

    /// Get the length of the secret key
    pub fn length_secret_key(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_secret_key
    }

    /// Get the length of the ciphertext
    pub fn length_ciphertext(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_ciphertext
    }

    /// Get the length of a shared secret
    pub fn length_shared_secret(&self) -> usize {
        let kem = unsafe { self.kem.as_ref() };
        kem.length_shared_secret
    }

    /// Obtain a secret key objects from bytes
    ///
    /// Returns None if the secret key is not the correct length.
    pub fn secret_key_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<SecretKeyRef<'a>> {
        if self.length_secret_key() != buf.len() {
            None
        } else {
            Some(SecretKeyRef::new(buf))
        }
    }

    /// Obtain a public key from bytes
    ///
    /// Returns None if the public key is not the correct length.
    pub fn public_key_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<PublicKeyRef<'a>> {
        if self.length_public_key() != buf.len() {
            None
        } else {
            Some(PublicKeyRef::new(buf))
        }
    }

    /// Obtain a ciphertext from bytes
    ///
    /// Returns None if the ciphertext is not the correct length.
    pub fn ciphertext_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<CiphertextRef<'a>> {
        if self.length_ciphertext() != buf.len() {
            None
        } else {
            Some(CiphertextRef::new(buf))
        }
    }

    /// Obtain a secret key from bytes
    ///
    /// Returns None if the shared secret is not the correct length.
    pub fn shared_secret_from_bytes<'a>(&self, buf: &'a [u8]) -> Option<SharedSecretRef<'a>> {
        if self.length_shared_secret() != buf.len() {
            None
        } else {
            Some(SharedSecretRef::new(buf))
        }
    }

    /// Generate a new keypair
    pub fn keypair(&self) -> Result<(PublicKey, SecretKey)> {
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.keypair.unwrap();
        let mut pk = PublicKey {
            bytes: Vec::with_capacity(kem.length_public_key),
        };
        let mut sk = SecretKey {
            bytes: Vec::with_capacity(kem.length_secret_key),
        };
        let status = unsafe { func(pk.bytes.as_mut_ptr(), sk.bytes.as_mut_ptr()) };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            pk.bytes.set_len(kem.length_public_key);
            sk.bytes.set_len(kem.length_secret_key);
        }
        Ok((pk, sk))
    }

    /// Encapsulate to the provided public key
    pub fn encapsulate<'a, P: Into<PublicKeyRef<'a>>>(
        &self,
        pk: P,
    ) -> Result<(Ciphertext, SharedSecret)> {
        let pk = pk.into();
        if pk.bytes.len() != self.length_public_key() {
            return Err(Error::InvalidLength);
        }
        let kem = unsafe { self.kem.as_ref() };
        let func = kem.encaps.unwrap();
        let mut ct = Ciphertext {
            bytes: Vec::with_capacity(kem.length_ciphertext),
        };
        let mut ss = SharedSecret {
            bytes: Vec::with_capacity(kem.length_shared_secret),
        };
        // call encapsulate
        let status = unsafe {
            func(
                ct.bytes.as_mut_ptr(),
                ss.bytes.as_mut_ptr(),
                pk.bytes.as_ptr(),
            )
        };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe {
            ct.bytes.set_len(kem.length_ciphertext);
            ss.bytes.set_len(kem.length_shared_secret);
        }
        Ok((ct, ss))
    }

    /// Decapsulate the provided ciphertext
    pub fn decapsulate<'a, 'b, S: Into<SecretKeyRef<'a>>, C: Into<CiphertextRef<'b>>>(
        &self,
        sk: S,
        ct: C,
    ) -> Result<SharedSecret> {
        let kem = unsafe { self.kem.as_ref() };
        let sk = sk.into();
        let ct = ct.into();
        if sk.bytes.len() != self.length_secret_key() || ct.bytes.len() != self.length_ciphertext()
        {
            return Err(Error::InvalidLength);
        }
        let mut ss = SharedSecret {
            bytes: Vec::with_capacity(kem.length_shared_secret),
        };
        let func = kem.decaps.unwrap();
        // Call decapsulate
        let status = unsafe { func(ss.bytes.as_mut_ptr(), ct.bytes.as_ptr(), sk.bytes.as_ptr()) };
        status_to_result(status)?;
        // update the lengths of the vecs
        // this is safe to do, as we have initialised them now.
        unsafe { ss.bytes.set_len(kem.length_shared_secret) };
        Ok(ss)
    }
}
