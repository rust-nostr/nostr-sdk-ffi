// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;

use uniffi::Object;

mod builder;

use crate::client::Client;
use crate::error::Result;
use crate::protocol::nips::nip47::{
    GetBalanceResponse, GetInfoResponse, ListTransactionsRequest, LookupInvoiceRequest,
    LookupInvoiceResponse, MakeInvoiceRequest, MakeInvoiceResponse, NostrWalletConnectUri,
    PayInvoiceRequest, PayInvoiceResponse, PayKeysendRequest, PayKeysendResponse,
};

/// Nostr Wallet Connect client
#[derive(Object)]
pub struct NostrWalletConnect {
    inner: nwc::NostrWalletConnect,
}

impl Deref for NostrWalletConnect {
    type Target = nwc::NostrWalletConnect;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nwc::NostrWalletConnect> for NostrWalletConnect {
    fn from(inner: nwc::NostrWalletConnect) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl NostrWalletConnect {
    /// Construct a new client
    ///
    /// Use the [`NostrWalletConnectBuilder`] to configure the client.
    #[uniffi::constructor]
    pub fn new(uri: &NostrWalletConnectUri) -> Self {
        Self {
            inner: nwc::NostrWalletConnect::new(uri.deref().clone()),
        }
    }

    /// Get client
    #[inline]
    pub fn client(&self) -> Client {
        self.inner.client().clone().into()
    }

    /// Pay invoice
    pub async fn pay_invoice(&self, params: PayInvoiceRequest) -> Result<PayInvoiceResponse> {
        Ok(self.inner.pay_invoice(params.into()).await?.into())
    }

    /// Pay keysend
    pub async fn pay_keysend(&self, params: PayKeysendRequest) -> Result<PayKeysendResponse> {
        Ok(self.inner.pay_keysend(params.into()).await?.into())
    }

    /// Create invoice
    pub async fn make_invoice(&self, params: MakeInvoiceRequest) -> Result<MakeInvoiceResponse> {
        Ok(self.inner.make_invoice(params.into()).await?.into())
    }

    /// Lookup invoice
    pub async fn lookup_invoice(
        &self,
        params: LookupInvoiceRequest,
    ) -> Result<LookupInvoiceResponse> {
        Ok(self.inner.lookup_invoice(params.into()).await?.into())
    }

    /// List transactions
    pub async fn list_transactions(
        &self,
        params: ListTransactionsRequest,
    ) -> Result<Vec<LookupInvoiceResponse>> {
        let list = self.inner.list_transactions(params.into()).await?;
        Ok(list.into_iter().map(|l| l.into()).collect())
    }

    /// Get balance
    pub async fn get_balance(&self) -> Result<GetBalanceResponse> {
        Ok(self.inner.get_balance().await?.into())
    }

    /// Get info
    pub async fn get_info(&self) -> Result<GetInfoResponse> {
        Ok(self.inner.get_info().await?.into())
    }
}
