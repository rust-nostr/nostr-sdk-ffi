// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;

use nostr::nips::nip96;
use nostr::{JsonUtil, Url};
use uniffi::Object;

use crate::error::Result;
use crate::protocol::signer::NostrSigner;

/// Get the NIP96 server config URL for a given server
/// Returns the URL that should be fetched for configuration of the server
pub fn nip96_get_server_config_url(server_url: &str) -> Result<String> {
    let url: Url = Url::parse(server_url)?;
    Ok(nip96::get_server_config_url(&url).map(|url| url.to_string())?)
}

/// NIP-96 server config
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Nip96ServerConfig {
    inner: nip96::ServerConfig,
}

impl Deref for Nip96ServerConfig {
    type Target = nip96::ServerConfig;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nip96::ServerConfig> for Nip96ServerConfig {
    fn from(inner: nip96::ServerConfig) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Nip96ServerConfig {
    /// Parse NIP-96 server config from JSON
    #[uniffi::constructor]
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(Self {
            inner: nip96::ServerConfig::from_json(json)?,
        })
    }

    /// Serialize as JSON
    pub fn as_json(&self) -> Result<String> {
        Ok(self.inner.try_as_json()?)
    }

    /// API URL
    pub fn api_url(&self) -> String {
        self.inner.api_url.to_string()
    }

    /// Download URL
    pub fn download_url(&self) -> String {
        self.inner.download_url.to_string()
    }

    /// Delegated URL
    pub fn delegated_to_url(&self) -> Option<String> {
        self.inner
            .delegated_to_url
            .as_ref()
            .map(|url| url.to_string())
    }

    /// Allowed content types
    pub fn content_types(&self) -> Option<Vec<String>> {
        self.inner.content_types.clone()
    }
}

/// NIP96 upload request information
/// Contains all data needed to make a file upload request
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Nip96UploadRequest {
    inner: nip96::UploadRequest,
}

#[uniffi::export]
impl Nip96UploadRequest {
    /// Prepare upload request data
    ///
    /// This function prepares the authorization header and returns all the data
    /// needed to make an upload request with the HTTP client.
    ///
    /// Note: please create the multipart form data yourself using your
    /// preferred HTTP client's multipart impl.
    #[uniffi::constructor]
    pub async fn create(
        signer: &NostrSigner,
        config: &Nip96ServerConfig,
        file_data: &[u8],
    ) -> Result<Self> {
        Ok(Self {
            inner: nip96::UploadRequest::new(signer.deref(), config.deref(), file_data).await?,
        })
    }

    /// Get the URL to POST to
    pub fn url(&self) -> String {
        self.inner.url.to_string()
    }

    /// Get the Authorization header value
    pub fn authorization(&self) -> String {
        self.inner.authorization.to_string()
    }
}

/// NIP-96 upload response
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Nip96UploadResponse {
    inner: nip96::UploadResponse,
}

#[uniffi::export]
impl Nip96UploadResponse {
    /// Parse NIP-96 upload response from JSON
    #[uniffi::constructor]
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(Self {
            inner: nip96::UploadResponse::from_json(json)?,
        })
    }

    /// Serialize as JSON
    pub fn as_json(&self) -> Result<String> {
        Ok(self.inner.try_as_json()?)
    }

    /// Check if success
    pub fn is_success(&self) -> bool {
        self.inner.status.is_success()
    }

    /// Free text success, failure or info message
    pub fn message(&self) -> String {
        self.inner.message.to_string()
    }

    /// Extract the download URL from the upload response
    ///
    /// Returns an error if the upload was unsuccessful or if the URL cannot be found
    pub fn download_url(&self) -> Result<String> {
        Ok(self.inner.download_url().map(|url| url.to_string())?)
    }
}
