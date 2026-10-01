module Securedrop_protocol_minimal.Primitives
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

/// Fixed number of message ID entries to return in privacy-preserving fetch
/// This prevents traffic analysis by always returning the same number of entries,
/// regardless of how many actual messages exist.
let v_MESSAGE_ID_FETCH_SIZE: usize = mk_usize 10
