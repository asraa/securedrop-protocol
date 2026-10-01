#![no_std]
// Deny direct access to system randomness except in tests.
// See clippy.toml
// hax breaks on these clippy statements
#![cfg_attr(not(hax), deny(clippy::disallowed_types, clippy::disallowed_methods))]
#![cfg_attr(all(not(hax), test), allow(clippy::disallowed_types))]
#![cfg_attr(all(not(hax), test), allow(clippy::disallowed_methods))]
extern crate alloc;

pub mod crypto;
pub mod primitives;
pub mod protocol;
pub mod server;

// Re-exported at their pre-reorg paths so downstream imports are unchanged.
pub use protocol::{api, encrypt_decrypt, journalist, keys, source, wire};
use protocol::{ciphertext, traits};
pub use server::{setup, storage};

pub use ciphertext::{Envelope, FetchResponse, Plaintext};

pub use keys::{
    DhFetchKeyPair, Enrollment, KeyBundlePublic, KeyPair, SessionStorage, SignedKeyBundlePublic,
    SignedLongtermPubKeyBytes, SigningKeyPair,
};
pub use primitives::dh_akem::DH_AKEM_PUBLIC_KEY_LEN;
pub use primitives::mlkem::{MLKEM768_PRIVATE_KEY_LEN, MLKEM768_PUBLIC_KEY_LEN};
pub use primitives::ristretto255::DH_PUBLIC_KEY_LEN;
pub use primitives::xwing::XWING_PUBLIC_KEY_LEN;

pub use traits::{Enrollable, JournalistPublic, UserPublic, UserSecret};

pub use journalist::{
    EphemeralBundleBytes, Journalist, JournalistLongTermBytes, JournalistPublicView,
};
pub use source::{Source, SourcePublicView};

pub(crate) use keys::MessageKeyBundle;

// Primitives for signing
pub use crypto::sign;
pub use sign::{
    DomainTag, FpfOnNewsroom, JournalistEphemeralKey, JournalistLongTermKey, NewsroomOnJournalist,
    Signature, SigningKey, VerifyingKey,
};

pub use crypto::{message, metadata};

// Do not make this module public or re-export it anywhere!
/// It uses the [sealed trait pattern](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed)
/// to gate features that downstream crates should not implement.
mod sealed;
