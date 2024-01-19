// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Miscellaneous utility helpers.

mod account_id;
pub mod bits;
mod era;
mod multi_address;
mod multi_signature;
mod static_type;
mod unchecked_extrinsic;
mod wrapper_opaque;

use crate::error::RpcError;
use crate::Error;
use codec::{Compact, Decode, Encode};
use derivative::Derivative;
use url::Url;

pub use account_id::AccountId32;
pub use era::Era;
pub use multi_address::MultiAddress;
pub use multi_signature::MultiSignature;
pub use static_type::Static;
pub use unchecked_extrinsic::UncheckedExtrinsic;
pub use wrapper_opaque::WrapperKeepOpaque;

// Used in codegen
#[doc(hidden)]
pub use primitive_types::{H160, H256, H512};

/// Wraps an already encoded byte vector, prevents being encoded as a raw byte vector as part of
/// the transaction payload
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encoded(pub Vec<u8>);

impl codec::Encode for Encoded {
    fn encode(&self) -> Vec<u8> {
        self.0.to_owned()
    }
}

/// Decodes a compact encoded value from the beginning of the provided bytes,
/// returning the value and any remaining bytes.
pub(crate) fn strip_compact_prefix(bytes: &[u8]) -> Result<(u64, &[u8]), codec::Error> {
    let cursor = &mut &*bytes;
    let val = <Compact<u64>>::decode(cursor)?;
    Ok((val.0, *cursor))
}

/// A URL is considered secure if it uses a secure scheme ("https" or "wss") or is referring to localhost.
///
/// Returns an error if the the string could not be parsed into a URL.
pub fn url_is_secure(url: &str) -> Result<bool, Error> {
    let url = Url::parse(url).map_err(|e| Error::Rpc(RpcError::ClientError(Box::new(e))))?;

    let secure_scheme = url.scheme() == "https" || url.scheme() == "wss";
    let is_localhost = url.host().is_some_and(|e| match e {
        url::Host::Domain(e) => e == "localhost",
        url::Host::Ipv4(e) => e.is_loopback(),
        url::Host::Ipv6(e) => e.is_loopback(),
    });

    Ok(secure_scheme || is_localhost)
}

/// Validates, that the given Url is secure ("https" or "wss" scheme) or is referring to localhost.
pub fn validate_url_is_secure(url: &str) -> Result<(), Error> {
    if !url_is_secure(url)? {
        Err(Error::Rpc(crate::error::RpcError::InsecureUrl(url.into())))
    } else {
        Ok(())
    }
}

/// A version of [`std::marker::PhantomData`] that is also Send and Sync (which is fine
/// because regardless of the generic param, it is always possible to Send + Sync this
/// 0 size type).
#[derive(Derivative, Encode, Decode, scale_info::TypeInfo)]
#[derivative(
    Clone(bound = ""),
    PartialEq(bound = ""),
    Debug(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Hash(bound = "")
)]
#[scale_info(skip_type_params(T))]
#[doc(hidden)]
pub struct PhantomDataSendSync<T>(core::marker::PhantomData<T>);

impl<T> PhantomDataSendSync<T> {
    pub(crate) fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

unsafe impl<T> Send for PhantomDataSendSync<T> {}
unsafe impl<T> Sync for PhantomDataSendSync<T> {}

/// This represents a key-value collection and is SCALE compatible
/// with collections like BTreeMap. This has the same type params
/// as `BTreeMap` which allows us to easily swap the two during codegen.
pub type KeyedVec<K, V> = Vec<(K, V)>;
