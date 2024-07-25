// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

mod handlers;
mod server;

pub use crate::server::MockServer;
use criticaltrust::keys::{EphemeralKeyPair, PublicKey};
use criticaltrust::manifests::ReleaseManifest;
use criticaltrust::revocation_info::RevocationInfo;
use criticaltrust::signatures::SignedPayload;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use time::macros::datetime;

#[derive(Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AuthenticationToken {
    pub name: Cow<'static, str>,
    pub organization_name: Cow<'static, str>,
    pub expires_at: Option<Cow<'static, str>>,
}

pub struct Data {
    pub tokens: HashMap<String, AuthenticationToken>,
    pub keys: Vec<SignedPayload<PublicKey>>,
    pub revoked_signatures: SignedPayload<RevocationInfo>,
    pub release_manifests: HashMap<(String, String), ReleaseManifest>,
}

pub fn new() -> Builder {
    Builder {
        data: Data {
            tokens: HashMap::new(),
            keys: Vec::new(),
            revoked_signatures: SignedPayload::new(&RevocationInfo::new(
                Vec::new(),
                datetime!(2025-01-01 0:00 UTC),
            ))
            .unwrap(),
            release_manifests: HashMap::new(),
        },
    }
}

pub struct Builder {
    data: Data,
}

impl Builder {
    pub fn add_token(mut self, token: &str, info: AuthenticationToken) -> Self {
        self.data.tokens.insert(token.into(), info);
        self
    }

    pub fn add_key(mut self, key: SignedPayload<PublicKey>) -> Self {
        self.data.keys.push(key);
        self
    }

    pub fn add_revocation_info(mut self, revocation_key: &EphemeralKeyPair) -> Self {
        self.data
            .revoked_signatures
            .add_signature(revocation_key)
            .unwrap();
        self
    }

    pub fn add_release_manifest(
        mut self,
        product: String,
        release: String,
        manifest: ReleaseManifest,
    ) -> Self {
        self.data
            .release_manifests
            .insert((product, release), manifest);
        self
    }

    pub fn start(self) -> MockServer {
        MockServer::spawn(self.data)
    }
}
