pub(crate) mod dh_akem;
pub(crate) mod mlkem;
pub(crate) mod provider;
pub mod ristretto255;
pub(crate) mod xwing;

/// Fixed number of message ID entries to return in privacy-preserving fetch
///
/// This prevents traffic analysis by always returning the same number of entries,
/// regardless of how many actual messages exist.
pub const MESSAGE_ID_FETCH_SIZE: usize = 10;
