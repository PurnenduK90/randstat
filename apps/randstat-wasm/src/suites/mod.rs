//! Feature-gated suite modules for `randstat-wasm`.

#[cfg(feature = "ais31")]
pub mod ais31;
#[cfg(feature = "dieharder")]
pub mod dieharder;
#[cfg(feature = "ent")]
pub mod ent;
#[cfg(feature = "full")]
pub mod full;
#[cfg(feature = "gjrand")]
pub mod gjrand;
#[cfg(feature = "nist")]
pub mod nist;
#[cfg(feature = "practrand")]
pub mod practrand;
#[cfg(feature = "sp800-90b")]
pub mod sp80090b;
#[cfg(feature = "testu01")]
pub mod testu01;
