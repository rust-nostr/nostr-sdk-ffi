// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::JsonUtil;
use nostr::nips::nip47;
use uniffi::{Enum, Object, Record};

use crate::error::Result;
use crate::protocol::key::{PublicKey, SecretKey};
use crate::protocol::types::{RelayUrl, Timestamp};
use crate::protocol::util::JsonValue;

/// NIP47 Response Error codes
#[derive(Enum)]
pub enum ErrorCode {
    ///  The client is sending commands too fast.
    RateLimited,
    /// The command is not known of is intentionally not implemented
    NotImplemented,
    /// The wallet does not have enough funds to cover a fee reserve or the payment amount
    InsufficientBalance,
    /// The payment failed. This may be due to a timeout, exhausting all routes, insufficient capacity or similar.
    PaymentFailed,
    /// The invoice could not be found by the given parameters.
    NotFound,
    /// The wallet has exceeded its spending quota
    QuotaExceeded,
    /// This public key is not allowed to do this operation
    Restricted,
    /// This public key has no wallet connected
    Unauthorized,
    /// An internal error
    Internal,
    /// Other error
    Other,
}

impl From<nip47::ErrorCode> for ErrorCode {
    fn from(value: nip47::ErrorCode) -> Self {
        match value {
            nip47::ErrorCode::RateLimited => Self::RateLimited,
            nip47::ErrorCode::NotImplemented => Self::NotImplemented,
            nip47::ErrorCode::InsufficientBalance => Self::InsufficientBalance,
            nip47::ErrorCode::PaymentFailed => Self::PaymentFailed,
            nip47::ErrorCode::NotFound => Self::NotFound,
            nip47::ErrorCode::QuotaExceeded => Self::QuotaExceeded,
            nip47::ErrorCode::Restricted => Self::Restricted,
            nip47::ErrorCode::Unauthorized => Self::Unauthorized,
            nip47::ErrorCode::Internal => Self::Internal,
            nip47::ErrorCode::Other => Self::Other,
        }
    }
}

impl From<ErrorCode> for nip47::ErrorCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::RateLimited => Self::RateLimited,
            ErrorCode::NotImplemented => Self::NotImplemented,
            ErrorCode::InsufficientBalance => Self::InsufficientBalance,
            ErrorCode::PaymentFailed => Self::PaymentFailed,
            ErrorCode::NotFound => Self::NotFound,
            ErrorCode::QuotaExceeded => Self::QuotaExceeded,
            ErrorCode::Restricted => Self::Restricted,
            ErrorCode::Unauthorized => Self::Unauthorized,
            ErrorCode::Internal => Self::Internal,
            ErrorCode::Other => Self::Other,
        }
    }
}

/// NIP47 Error message
#[derive(Record)]
pub struct NIP47Error {
    /// Error Code
    pub code: ErrorCode,
    /// Human Readable error message
    pub message: String,
}

impl From<nip47::NIP47Error> for NIP47Error {
    fn from(value: nip47::NIP47Error) -> Self {
        Self {
            code: value.code.into(),
            message: value.message,
        }
    }
}

impl From<NIP47Error> for nip47::NIP47Error {
    fn from(value: NIP47Error) -> Self {
        Self {
            code: value.code.into(),
            message: value.message,
        }
    }
}

/// Method
#[derive(Enum)]
pub enum Method {
    /// Pay Invoice
    PayInvoice,
    /// Multi Pay Invoice
    MultiPayInvoice,
    /// Pay Keysend
    PayKeysend,
    /// Multi Pay Keysend
    MultiPayKeysend,
    /// Make Invoice
    MakeInvoice,
    /// Lookup Invoice
    LookupInvoice,
    /// List transactions
    ListTransactions,
    /// Get Balance
    GetBalance,
    /// Get Info
    GetInfo,
    /// Make Hold Invoice
    MakeHoldInvoice,
    /// Cancel Hold Invoice
    CancelHoldInvoice,
    /// Settle Hold Invoice
    SettleHoldInvoice,
}

impl From<nip47::Method> for Method {
    fn from(value: nip47::Method) -> Self {
        match value {
            nip47::Method::PayInvoice => Self::PayInvoice,
            nip47::Method::MultiPayInvoice => Self::MultiPayInvoice,
            nip47::Method::PayKeysend => Self::PayKeysend,
            nip47::Method::MultiPayKeysend => Self::MultiPayKeysend,
            nip47::Method::MakeInvoice => Self::MakeInvoice,
            nip47::Method::LookupInvoice => Self::LookupInvoice,
            nip47::Method::ListTransactions => Self::ListTransactions,
            nip47::Method::GetBalance => Self::GetBalance,
            nip47::Method::GetInfo => Self::GetInfo,
            nip47::Method::MakeHoldInvoice => Self::MakeHoldInvoice,
            nip47::Method::CancelHoldInvoice => Self::CancelHoldInvoice,
            nip47::Method::SettleHoldInvoice => Self::SettleHoldInvoice,
        }
    }
}

impl From<Method> for nip47::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::PayInvoice => Self::PayInvoice,
            Method::MultiPayInvoice => Self::MultiPayInvoice,
            Method::PayKeysend => Self::PayKeysend,
            Method::MultiPayKeysend => Self::MultiPayKeysend,
            Method::MakeInvoice => Self::MakeInvoice,
            Method::LookupInvoice => Self::LookupInvoice,
            Method::ListTransactions => Self::ListTransactions,
            Method::GetBalance => Self::GetBalance,
            Method::GetInfo => Self::GetInfo,
            Method::MakeHoldInvoice => Self::MakeHoldInvoice,
            Method::CancelHoldInvoice => Self::CancelHoldInvoice,
            Method::SettleHoldInvoice => Self::SettleHoldInvoice,
        }
    }
}

/// Nostr Wallet Connect Request Params
#[derive(Enum)]
pub enum RequestParams {
    /// Pay Invoice
    PayInvoice { pay_invoice: PayInvoiceRequest },
    /// Multi Pay Invoice
    MultiPayInvoice {
        multi_pay_invoice: MultiPayInvoiceRequest,
    },
    /// Pay Keysend
    PayKeysend { pay_keysend: PayKeysendRequest },
    /// Multi Pay Keysend
    MultiPayKeysend {
        multi_pay_keysend: MultiPayKeysendRequest,
    },
    /// Make Invoice
    MakeInvoice { make_invoice: MakeInvoiceRequest },
    /// Lookup Invoice
    LookupInvoice {
        lookup_invoice: LookupInvoiceRequest,
    },
    /// List Transactions
    ListTransactions {
        list_transactions: ListTransactionsRequest,
    },
    /// Get Balance
    GetBalance,
    /// Get Info
    GetInfo,
    /// Make Hold Invoice
    MakeHoldInvoice { req: MakeHoldInvoiceRequest },
    /// Cancel Hold Invoice
    CancelHoldInvoice { req: CancelHoldInvoiceRequest },
    /// Settle Hold Invoice
    SettleHoldInvoice { req: SettleHoldInvoiceRequest },
}

impl From<nip47::RequestParams> for RequestParams {
    fn from(value: nip47::RequestParams) -> Self {
        match value {
            nip47::RequestParams::PayInvoice(pay_invoice) => Self::PayInvoice {
                pay_invoice: pay_invoice.into(),
            },
            nip47::RequestParams::MultiPayInvoice(multi_pay_invoice) => Self::MultiPayInvoice {
                multi_pay_invoice: multi_pay_invoice.into(),
            },
            nip47::RequestParams::PayKeysend(pay_keysend) => Self::PayKeysend {
                pay_keysend: pay_keysend.into(),
            },
            nip47::RequestParams::MultiPayKeysend(multi_pay_keysend) => Self::MultiPayKeysend {
                multi_pay_keysend: multi_pay_keysend.into(),
            },
            nip47::RequestParams::MakeInvoice(make_invoice) => Self::MakeInvoice {
                make_invoice: make_invoice.into(),
            },
            nip47::RequestParams::LookupInvoice(lookup_invoice) => Self::LookupInvoice {
                lookup_invoice: lookup_invoice.into(),
            },
            nip47::RequestParams::ListTransactions(list_transactions) => Self::ListTransactions {
                list_transactions: list_transactions.into(),
            },
            nip47::RequestParams::GetBalance => Self::GetBalance,
            nip47::RequestParams::GetInfo => Self::GetInfo,
            nip47::RequestParams::MakeHoldInvoice(req) => Self::MakeHoldInvoice { req: req.into() },
            nip47::RequestParams::CancelHoldInvoice(req) => {
                Self::CancelHoldInvoice { req: req.into() }
            }
            nip47::RequestParams::SettleHoldInvoice(req) => {
                Self::SettleHoldInvoice { req: req.into() }
            }
        }
    }
}

impl From<RequestParams> for nip47::RequestParams {
    fn from(value: RequestParams) -> Self {
        match value {
            RequestParams::PayInvoice { pay_invoice } => Self::PayInvoice(pay_invoice.into()),
            RequestParams::MultiPayInvoice { multi_pay_invoice } => {
                Self::MultiPayInvoice(multi_pay_invoice.into())
            }
            RequestParams::PayKeysend { pay_keysend } => Self::PayKeysend(pay_keysend.into()),
            RequestParams::MultiPayKeysend { multi_pay_keysend } => {
                Self::MultiPayKeysend(multi_pay_keysend.into())
            }
            RequestParams::MakeInvoice { make_invoice } => Self::MakeInvoice(make_invoice.into()),
            RequestParams::LookupInvoice { lookup_invoice } => {
                Self::LookupInvoice(lookup_invoice.into())
            }
            RequestParams::ListTransactions { list_transactions } => {
                Self::ListTransactions(list_transactions.into())
            }
            RequestParams::GetBalance => Self::GetBalance,
            RequestParams::GetInfo => Self::GetInfo,
            RequestParams::MakeHoldInvoice { req } => Self::MakeHoldInvoice(req.into()),
            RequestParams::CancelHoldInvoice { req } => Self::CancelHoldInvoice(req.into()),
            RequestParams::SettleHoldInvoice { req } => Self::SettleHoldInvoice(req.into()),
        }
    }
}

/// Pay Invoice Request
#[derive(Record)]
pub struct PayInvoiceRequest {
    /// Optional id
    pub id: Option<String>,
    /// Request invoice
    pub invoice: String,
    /// Optional amount in millisatoshis
    pub amount: Option<u64>,
}

impl From<nip47::PayInvoiceRequest> for PayInvoiceRequest {
    fn from(value: nip47::PayInvoiceRequest) -> Self {
        Self {
            id: value.id,
            invoice: value.invoice,
            amount: value.amount,
        }
    }
}

impl From<PayInvoiceRequest> for nip47::PayInvoiceRequest {
    fn from(value: PayInvoiceRequest) -> Self {
        Self {
            id: value.id,
            invoice: value.invoice,
            amount: value.amount,
        }
    }
}

/// Multi Pay Invoice Request Params
#[derive(Record)]
pub struct MultiPayInvoiceRequest {
    /// Invoices to pay
    pub invoices: Vec<PayInvoiceRequest>,
}

impl From<nip47::MultiPayInvoiceRequest> for MultiPayInvoiceRequest {
    fn from(value: nip47::MultiPayInvoiceRequest) -> Self {
        Self {
            invoices: value.invoices.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<MultiPayInvoiceRequest> for nip47::MultiPayInvoiceRequest {
    fn from(value: MultiPayInvoiceRequest) -> Self {
        Self {
            invoices: value.invoices.into_iter().map(|i| i.into()).collect(),
        }
    }
}

/// TLVs to be added to the keysend payment
#[derive(Record)]
pub struct KeysendTLVRecord {
    /// TLV type
    pub tlv_type: u64,
    /// TLV value
    pub value: String,
}

impl From<nip47::KeysendTLVRecord> for KeysendTLVRecord {
    fn from(value: nip47::KeysendTLVRecord) -> Self {
        Self {
            tlv_type: value.tlv_type,
            value: value.value,
        }
    }
}

impl From<KeysendTLVRecord> for nip47::KeysendTLVRecord {
    fn from(value: KeysendTLVRecord) -> Self {
        Self {
            tlv_type: value.tlv_type,
            value: value.value,
        }
    }
}

/// Pay Invoice Request
#[derive(Record)]
pub struct PayKeysendRequest {
    /// Optional id
    pub id: Option<String>,
    /// Amount in millisatoshis
    pub amount: u64,
    /// Receiver's node id
    pub pubkey: String,
    /// Optional preimage
    pub preimage: Option<String>,
    /// Optional TLVs to be added to the keysend payment
    pub tlv_records: Vec<KeysendTLVRecord>,
}

impl From<nip47::PayKeysendRequest> for PayKeysendRequest {
    fn from(value: nip47::PayKeysendRequest) -> Self {
        Self {
            id: value.id,
            amount: value.amount,
            pubkey: value.pubkey,
            preimage: value.preimage,
            tlv_records: value.tlv_records.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl From<PayKeysendRequest> for nip47::PayKeysendRequest {
    fn from(value: PayKeysendRequest) -> Self {
        Self {
            id: value.id,
            amount: value.amount,
            pubkey: value.pubkey,
            preimage: value.preimage,
            tlv_records: value.tlv_records.into_iter().map(|t| t.into()).collect(),
        }
    }
}

/// Multi Pay Keysend Request
#[derive(Record)]
pub struct MultiPayKeysendRequest {
    /// Keysends
    pub keysends: Vec<PayKeysendRequest>,
}

impl From<nip47::MultiPayKeysendRequest> for MultiPayKeysendRequest {
    fn from(value: nip47::MultiPayKeysendRequest) -> Self {
        Self {
            keysends: value.keysends.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<MultiPayKeysendRequest> for nip47::MultiPayKeysendRequest {
    fn from(value: MultiPayKeysendRequest) -> Self {
        Self {
            keysends: value.keysends.into_iter().map(|i| i.into()).collect(),
        }
    }
}

/// Make Hold Invoice Request
#[derive(Record)]
pub struct MakeHoldInvoiceRequest {
    /// Amount in millisatoshis
    pub amount: u64,
    /// Invoice description
    pub description: Option<String>,
    /// Invoice description hash
    pub description_hash: Option<String>,
    /// Invoice expiry in seconds
    pub expiry: Option<u64>,
    /// payment_hash
    pub payment_hash: String,
    /// The minimum CLTV delta to use for the final hop
    pub cltv_expiry_delta: Option<u32>,
}

impl From<nip47::MakeHoldInvoiceRequest> for MakeHoldInvoiceRequest {
    fn from(value: nip47::MakeHoldInvoiceRequest) -> Self {
        Self {
            amount: value.amount,
            description: value.description,
            description_hash: value.description_hash,
            expiry: value.expiry,
            payment_hash: value.payment_hash,
            cltv_expiry_delta: value.cltv_expiry_delta,
        }
    }
}

impl From<MakeHoldInvoiceRequest> for nip47::MakeHoldInvoiceRequest {
    fn from(value: MakeHoldInvoiceRequest) -> Self {
        Self {
            amount: value.amount,
            description: value.description,
            description_hash: value.description_hash,
            expiry: value.expiry,
            payment_hash: value.payment_hash,
            cltv_expiry_delta: value.cltv_expiry_delta,
        }
    }
}

/// Cancel Hold Invoice Request
#[derive(Record)]
pub struct CancelHoldInvoiceRequest {
    /// payment_hash
    pub payment_hash: String,
}

impl From<nip47::CancelHoldInvoiceRequest> for CancelHoldInvoiceRequest {
    fn from(value: nip47::CancelHoldInvoiceRequest) -> Self {
        Self {
            payment_hash: value.payment_hash,
        }
    }
}

impl From<CancelHoldInvoiceRequest> for nip47::CancelHoldInvoiceRequest {
    fn from(value: CancelHoldInvoiceRequest) -> Self {
        Self {
            payment_hash: value.payment_hash,
        }
    }
}

/// Settle Hold Invoice Request
#[derive(Record)]
pub struct SettleHoldInvoiceRequest {
    /// preimage
    pub preimage: String,
}

impl From<nip47::SettleHoldInvoiceRequest> for SettleHoldInvoiceRequest {
    fn from(value: nip47::SettleHoldInvoiceRequest) -> Self {
        Self {
            preimage: value.preimage,
        }
    }
}

impl From<SettleHoldInvoiceRequest> for nip47::SettleHoldInvoiceRequest {
    fn from(value: SettleHoldInvoiceRequest) -> Self {
        Self {
            preimage: value.preimage,
        }
    }
}

/// Transaction Type
#[derive(Enum)]
pub enum TransactionType {
    /// Incoming payments
    Incoming,
    /// Outgoing payments
    Outgoing,
}

impl From<TransactionType> for nip47::TransactionType {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Incoming => Self::Incoming,
            TransactionType::Outgoing => Self::Outgoing,
        }
    }
}

impl From<nip47::TransactionType> for TransactionType {
    fn from(value: nip47::TransactionType) -> Self {
        match value {
            nip47::TransactionType::Incoming => Self::Incoming,
            nip47::TransactionType::Outgoing => Self::Outgoing,
        }
    }
}

/// Transaction State
#[derive(Enum)]
pub enum TransactionState {
    /// Pending
    Pending,
    /// Settled
    Settled,
    /// Expired (for invoices)
    Expired,
    /// Failed (for payments)
    Failed,
}

impl From<nip47::TransactionState> for TransactionState {
    fn from(value: nip47::TransactionState) -> Self {
        match value {
            nip47::TransactionState::Pending => Self::Pending,
            nip47::TransactionState::Settled => Self::Settled,
            nip47::TransactionState::Expired => Self::Expired,
            nip47::TransactionState::Failed => Self::Failed,
        }
    }
}

impl From<TransactionState> for nip47::TransactionState {
    fn from(value: TransactionState) -> Self {
        match value {
            TransactionState::Pending => Self::Pending,
            TransactionState::Settled => Self::Settled,
            TransactionState::Expired => Self::Expired,
            TransactionState::Failed => Self::Failed,
        }
    }
}

/// Make Invoice Request
#[derive(Record)]
pub struct MakeInvoiceRequest {
    /// Amount in millisatoshis
    pub amount: u64,
    /// Invoice description
    pub description: Option<String>,
    /// Invoice description hash
    pub description_hash: Option<String>,
    /// Invoice expiry in seconds
    pub expiry: Option<u64>,
}

impl From<nip47::MakeInvoiceRequest> for MakeInvoiceRequest {
    fn from(value: nip47::MakeInvoiceRequest) -> Self {
        Self {
            amount: value.amount,
            description: value.description,
            description_hash: value.description_hash,
            expiry: value.expiry,
        }
    }
}

impl From<MakeInvoiceRequest> for nip47::MakeInvoiceRequest {
    fn from(value: MakeInvoiceRequest) -> Self {
        Self {
            amount: value.amount,
            description: value.description,
            description_hash: value.description_hash,
            expiry: value.expiry,
        }
    }
}

/// Lookup Invoice Request
#[derive(Record)]
pub struct LookupInvoiceRequest {
    /// Payment hash of invoice
    pub payment_hash: Option<String>,
    /// Bolt11 invoice
    pub invoice: Option<String>,
}

impl From<nip47::LookupInvoiceRequest> for LookupInvoiceRequest {
    fn from(value: nip47::LookupInvoiceRequest) -> Self {
        Self {
            payment_hash: value.payment_hash,
            invoice: value.invoice,
        }
    }
}

impl From<LookupInvoiceRequest> for nip47::LookupInvoiceRequest {
    fn from(value: LookupInvoiceRequest) -> Self {
        Self {
            payment_hash: value.payment_hash,
            invoice: value.invoice,
        }
    }
}

/// List Invoice Request
#[derive(Record)]
pub struct ListTransactionsRequest {
    /// Starting timestamp in seconds since epoch
    pub from: Option<Arc<Timestamp>>,
    /// Ending timestamp in seconds since epoch
    pub until: Option<Arc<Timestamp>>,
    /// Number of invoices to return
    pub limit: Option<u64>,
    /// Offset of the first invoice to return
    pub offset: Option<u64>,
    /// If true, include unpaid invoices
    pub unpaid: Option<bool>,
    /// [`TransactionType::Incoming`] for invoices, [`TransactionType::Outgoing`] for payments, [`None`] for both
    pub transaction_type: Option<TransactionType>,
}

impl From<nip47::ListTransactionsRequest> for ListTransactionsRequest {
    fn from(value: nip47::ListTransactionsRequest) -> Self {
        Self {
            from: value.from.map(|t| Arc::new(t.into())),
            until: value.until.map(|t| Arc::new(t.into())),
            limit: value.limit,
            offset: value.offset,
            unpaid: value.unpaid,
            transaction_type: value.transaction_type.map(|t| t.into()),
        }
    }
}

impl From<ListTransactionsRequest> for nip47::ListTransactionsRequest {
    fn from(value: ListTransactionsRequest) -> Self {
        Self {
            from: value.from.map(|t| **t),
            until: value.until.map(|t| **t),
            limit: value.limit,
            offset: value.offset,
            unpaid: value.unpaid,
            transaction_type: value.transaction_type.map(|t| t.into()),
        }
    }
}

/// NIP47 Request
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Request {
    inner: nip47::Request,
}

impl From<nip47::Request> for Request {
    fn from(inner: nip47::Request) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Request {
    #[uniffi::constructor]
    pub fn new(method: Method, params: RequestParams) -> Self {
        Self {
            inner: nip47::Request {
                method: method.into(),
                params: params.into(),
            },
        }
    }

    #[uniffi::constructor]
    pub fn parse(json: String) -> Result<Self> {
        Ok(nip47::Request::from_json(json)?.into())
    }

    pub fn method(&self) -> Method {
        self.inner.method.into()
    }

    pub fn params(&self) -> RequestParams {
        self.inner.params.clone().into()
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct PayInvoiceResponse {
    /// Response preimage
    pub preimage: String,
    /// Fees paid
    pub fees_paid: Option<u64>,
}

impl From<nip47::PayInvoiceResponse> for PayInvoiceResponse {
    fn from(value: nip47::PayInvoiceResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

impl From<PayInvoiceResponse> for nip47::PayInvoiceResponse {
    fn from(value: PayInvoiceResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct PayKeysendResponse {
    /// Response preimage
    pub preimage: String,
    /// Fees paid
    pub fees_paid: Option<u64>,
}

impl From<nip47::PayKeysendResponse> for PayKeysendResponse {
    fn from(value: nip47::PayKeysendResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

impl From<PayKeysendResponse> for nip47::PayKeysendResponse {
    fn from(value: PayKeysendResponse) -> Self {
        Self {
            preimage: value.preimage,
            fees_paid: value.fees_paid,
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct MakeInvoiceResponse {
    /// Bolt 11 invoice
    pub invoice: String,
    /// Invoice's payment hash
    pub payment_hash: Option<String>,
    /// Invoice's description
    pub description: Option<String>,
    /// Invoice's description hash
    pub description_hash: Option<String>,
    /// Payment preimage
    pub preimage: Option<String>,
    /// Amount in msats.
    pub amount: Option<u64>,
    /// Creation timestamp in seconds since epoch
    pub created_at: Option<Arc<Timestamp>>,
    /// Expiration timestamp in seconds since epoch
    pub expires_at: Option<Arc<Timestamp>>,
}

impl From<nip47::MakeInvoiceResponse> for MakeInvoiceResponse {
    fn from(value: nip47::MakeInvoiceResponse) -> Self {
        Self {
            invoice: value.invoice,
            payment_hash: value.payment_hash,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            amount: value.amount,
            created_at: value.created_at.map(|t| Arc::new(t.into())),
            expires_at: value.expires_at.map(|t| Arc::new(t.into())),
        }
    }
}

impl From<MakeInvoiceResponse> for nip47::MakeInvoiceResponse {
    fn from(value: MakeInvoiceResponse) -> Self {
        Self {
            invoice: value.invoice,
            payment_hash: value.payment_hash,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            amount: value.amount,
            created_at: value.created_at.map(|t| **t),
            expires_at: value.expires_at.map(|t| **t),
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct LookupInvoiceResponse {
    /// Transaction type
    pub transaction_type: Option<TransactionType>,
    /// Transaction state.
    pub state: Option<TransactionState>,
    /// Bolt11 invoice
    pub invoice: Option<String>,
    /// Invoice's description
    pub description: Option<String>,
    /// Invoice's description hash
    pub description_hash: Option<String>,
    /// Payment preimage
    pub preimage: Option<String>,
    /// Payment hash
    pub payment_hash: String,
    /// Amount in millisatoshis
    pub amount: u64,
    /// Fees paid in millisatoshis
    pub fees_paid: u64,
    /// Creation timestamp in seconds since epoch
    pub created_at: Arc<Timestamp>,
    /// Expiration timestamp in seconds since epoch
    pub expires_at: Option<Arc<Timestamp>>,
    /// Settled timestamp in seconds since epoch
    pub settled_at: Option<Arc<Timestamp>>,
    /// Optional metadata about the payment
    pub metadata: Option<JsonValue>,
}

impl From<nip47::LookupInvoiceResponse> for LookupInvoiceResponse {
    fn from(value: nip47::LookupInvoiceResponse) -> Self {
        Self {
            transaction_type: value.transaction_type.map(|t| t.into()),
            state: value.state.map(|t| t.into()),
            invoice: value.invoice,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            payment_hash: value.payment_hash,
            amount: value.amount,
            fees_paid: value.fees_paid,
            created_at: Arc::new(value.created_at.into()),
            expires_at: value.expires_at.map(|t| Arc::new(t.into())),
            settled_at: value.settled_at.map(|t| Arc::new(t.into())),
            metadata: value.metadata.and_then(|m| m.try_into().ok()),
        }
    }
}

impl From<LookupInvoiceResponse> for nip47::LookupInvoiceResponse {
    fn from(value: LookupInvoiceResponse) -> Self {
        Self {
            transaction_type: value.transaction_type.map(|t| t.into()),
            state: value.state.map(|t| t.into()),
            invoice: value.invoice,
            description: value.description,
            description_hash: value.description_hash,
            preimage: value.preimage,
            payment_hash: value.payment_hash,
            amount: value.amount,
            fees_paid: value.fees_paid,
            created_at: **value.created_at,
            expires_at: value.expires_at.map(|t| **t),
            settled_at: value.settled_at.map(|t| **t),
            metadata: value.metadata.and_then(|m| m.try_into().ok()),
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct GetBalanceResponse {
    /// Balance amount in msats
    pub balance: u64,
}

impl From<nip47::GetBalanceResponse> for GetBalanceResponse {
    fn from(value: nip47::GetBalanceResponse) -> Self {
        Self {
            balance: value.balance,
        }
    }
}

impl From<GetBalanceResponse> for nip47::GetBalanceResponse {
    fn from(value: GetBalanceResponse) -> Self {
        Self {
            balance: value.balance,
        }
    }
}

/// NIP47 Response Result
#[derive(Record)]
pub struct GetInfoResponse {
    /// The alias of the lightning node
    pub alias: Option<String>,
    /// The color of the current node in hex code format
    pub color: Option<String>,
    /// Lightning Node's public key
    pub pubkey: Option<String>,
    /// Active network
    pub network: Option<String>,
    /// Current block height
    pub block_height: Option<u32>,
    /// Most Recent Block Hash
    pub block_hash: Option<String>,
    /// Available methods for this connection
    pub methods: Vec<Method>,
    /// List of supported notifications for this connection (optional)
    pub notifications: Vec<String>,
}

impl From<nip47::GetInfoResponse> for GetInfoResponse {
    fn from(value: nip47::GetInfoResponse) -> Self {
        Self {
            alias: value.alias,
            color: value.color,
            pubkey: value.pubkey.map(|p| p.to_string()),
            network: value.network,
            block_height: value.block_height,
            block_hash: value.block_hash,
            methods: value.methods.into_iter().map(|m| m.into()).collect(),
            notifications: value.notifications,
        }
    }
}

impl From<GetInfoResponse> for nip47::GetInfoResponse {
    fn from(value: GetInfoResponse) -> Self {
        Self {
            alias: value.alias,
            color: value.color,
            pubkey: value.pubkey.and_then(|p| p.parse().ok()),
            network: value.network,
            block_height: value.block_height,
            block_hash: value.block_hash,
            methods: value.methods.into_iter().map(|m| m.into()).collect(),
            notifications: value.notifications,
        }
    }
}

/// Make Hold Invoice Response
#[derive(Record)]
pub struct MakeHoldInvoiceResponse {
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Bolt11 invoice
    pub invoice: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Description hash
    pub description_hash: Option<String>,
    /// Payment hash
    pub payment_hash: String,
    /// Amount in millisatoshis
    pub amount: u64,
    /// Creation timestamp
    pub created_at: Arc<Timestamp>,
    /// Expiration timestamp
    pub expires_at: Arc<Timestamp>,
    /// Metadata
    pub metadata: Option<JsonValue>,
}

impl From<nip47::MakeHoldInvoiceResponse> for MakeHoldInvoiceResponse {
    fn from(value: nip47::MakeHoldInvoiceResponse) -> Self {
        Self {
            transaction_type: value.transaction_type.into(),
            invoice: value.invoice,
            description: value.description,
            description_hash: value.description_hash,
            payment_hash: value.payment_hash,
            amount: value.amount,
            created_at: Arc::new(value.created_at.into()),
            expires_at: Arc::new(value.expires_at.into()),
            metadata: match value.metadata {
                Some(metadata) => metadata.try_into().ok(),
                None => None,
            },
        }
    }
}

impl From<MakeHoldInvoiceResponse> for nip47::MakeHoldInvoiceResponse {
    fn from(value: MakeHoldInvoiceResponse) -> Self {
        Self {
            transaction_type: value.transaction_type.into(),
            invoice: value.invoice,
            description: value.description,
            description_hash: value.description_hash,
            payment_hash: value.payment_hash,
            amount: value.amount,
            created_at: **value.created_at,
            expires_at: **value.expires_at,
            metadata: match value.metadata {
                Some(metadata) => metadata.try_into().ok(),
                None => None,
            },
        }
    }
}

/// Cancel Hold Invoice Response
#[derive(Record)]
pub struct CancelHoldInvoiceResponse {}

impl From<nip47::CancelHoldInvoiceResponse> for CancelHoldInvoiceResponse {
    fn from(_value: nip47::CancelHoldInvoiceResponse) -> Self {
        Self {}
    }
}

impl From<CancelHoldInvoiceResponse> for nip47::CancelHoldInvoiceResponse {
    fn from(_value: CancelHoldInvoiceResponse) -> Self {
        Self {}
    }
}

/// Settle Hold Invoice Response
#[derive(Record)]
pub struct SettleHoldInvoiceResponse {}

impl From<nip47::SettleHoldInvoiceResponse> for SettleHoldInvoiceResponse {
    fn from(_value: nip47::SettleHoldInvoiceResponse) -> Self {
        Self {}
    }
}

impl From<SettleHoldInvoiceResponse> for nip47::SettleHoldInvoiceResponse {
    fn from(_value: SettleHoldInvoiceResponse) -> Self {
        Self {}
    }
}

/// NIP47 Response Result
#[derive(Enum)]
pub enum ResponseResult {
    /// Pay Invoice
    PayInvoice { pay_invoice: PayInvoiceResponse },
    /// Multi Pay Invoice
    MultiPayInvoice { pay_invoice: PayInvoiceResponse },
    /// Pay Keysend
    PayKeysend { pay_keysend: PayKeysendResponse },
    /// Multi Pay Keysend
    MultiPayKeysend { pay_keysend: PayKeysendResponse },
    /// Make Invoice
    MakeInvoice { make_invoice: MakeInvoiceResponse },
    /// Lookup Invoice
    LookupInvoice {
        lookup_invoice: LookupInvoiceResponse,
    },
    /// List Transactions
    ListTransactions {
        list_transactions: Vec<LookupInvoiceResponse>,
    },
    /// Get Balance
    GetBalance { get_balance: GetBalanceResponse },
    /// Get Info
    GetInfo { get_info: GetInfoResponse },
    /// Make Hold Invoice
    MakeHoldInvoice { res: MakeHoldInvoiceResponse },
    /// Cancel Hold Invoice
    CancelHoldInvoice { res: CancelHoldInvoiceResponse },
    /// Settle Hold Invoice
    SettleHoldInvoice { res: SettleHoldInvoiceResponse },
}

impl From<nip47::ResponseResult> for ResponseResult {
    fn from(value: nip47::ResponseResult) -> Self {
        match value {
            nip47::ResponseResult::PayInvoice(pay_invoice) => Self::PayInvoice {
                pay_invoice: pay_invoice.into(),
            },
            nip47::ResponseResult::MultiPayInvoice(multi_pay_invoice) => Self::MultiPayInvoice {
                pay_invoice: multi_pay_invoice.into(),
            },
            nip47::ResponseResult::PayKeysend(pay_keysend) => Self::PayKeysend {
                pay_keysend: pay_keysend.into(),
            },
            nip47::ResponseResult::MultiPayKeysend(multi_pay_keysend) => Self::MultiPayKeysend {
                pay_keysend: multi_pay_keysend.into(),
            },
            nip47::ResponseResult::MakeInvoice(make_invoice) => Self::MakeInvoice {
                make_invoice: make_invoice.into(),
            },
            nip47::ResponseResult::LookupInvoice(lookup_invoice) => Self::LookupInvoice {
                lookup_invoice: lookup_invoice.into(),
            },
            nip47::ResponseResult::ListTransactions(list_transactions) => Self::ListTransactions {
                list_transactions: list_transactions.into_iter().map(|i| i.into()).collect(),
            },
            nip47::ResponseResult::GetBalance(get_balance) => Self::GetBalance {
                get_balance: get_balance.into(),
            },
            nip47::ResponseResult::GetInfo(get_info) => Self::GetInfo {
                get_info: get_info.into(),
            },
            nip47::ResponseResult::MakeHoldInvoice(res) => {
                Self::MakeHoldInvoice { res: res.into() }
            }
            nip47::ResponseResult::CancelHoldInvoice(res) => {
                Self::CancelHoldInvoice { res: res.into() }
            }
            nip47::ResponseResult::SettleHoldInvoice(res) => {
                Self::SettleHoldInvoice { res: res.into() }
            }
        }
    }
}

impl From<ResponseResult> for nip47::ResponseResult {
    fn from(value: ResponseResult) -> Self {
        match value {
            ResponseResult::PayInvoice { pay_invoice } => Self::PayInvoice(pay_invoice.into()),
            ResponseResult::MultiPayInvoice { pay_invoice } => {
                Self::MultiPayInvoice(pay_invoice.into())
            }
            ResponseResult::PayKeysend { pay_keysend } => Self::PayKeysend(pay_keysend.into()),
            ResponseResult::MultiPayKeysend { pay_keysend } => {
                Self::MultiPayKeysend(pay_keysend.into())
            }
            ResponseResult::MakeInvoice { make_invoice } => Self::MakeInvoice(make_invoice.into()),
            ResponseResult::LookupInvoice { lookup_invoice } => {
                Self::LookupInvoice(lookup_invoice.into())
            }
            ResponseResult::ListTransactions { list_transactions } => {
                Self::ListTransactions(list_transactions.into_iter().map(|i| i.into()).collect())
            }
            ResponseResult::GetBalance { get_balance } => Self::GetBalance(get_balance.into()),
            ResponseResult::GetInfo { get_info } => Self::GetInfo(get_info.into()),
            ResponseResult::MakeHoldInvoice { res } => Self::MakeHoldInvoice(res.into()),
            ResponseResult::CancelHoldInvoice { res } => Self::CancelHoldInvoice(res.into()),
            ResponseResult::SettleHoldInvoice { res } => Self::SettleHoldInvoice(res.into()),
        }
    }
}

/// NIP47 Response
#[derive(Debug, PartialEq, Eq, Object)]
#[uniffi::export(Debug, Eq)]
pub struct Response {
    inner: nip47::Response,
}

impl From<nip47::Response> for Response {
    fn from(inner: nip47::Response) -> Self {
        Self { inner }
    }
}

impl Response {
    #[uniffi::constructor]
    pub fn new(
        result_type: Method,
        result: Option<ResponseResult>,
        error: Option<NIP47Error>,
    ) -> Self {
        Self {
            inner: nip47::Response {
                result_type: result_type.into(),
                error: error.map(|e| e.into()),
                result: result.map(|r| r.into()),
            },
        }
    }

    /// Deserialize from JSON string
    #[uniffi::constructor]
    pub fn parse(json: String) -> Result<Self> {
        Ok(nip47::Response::from_json(json)?.into())
    }

    pub fn result_type(&self) -> Method {
        self.inner.result_type.into()
    }

    pub fn result(&self) -> Option<ResponseResult> {
        self.inner.result.clone().map(|i| i.into())
    }

    pub fn error(&self) -> Option<NIP47Error> {
        self.inner.error.clone().map(|i| i.into())
    }
}

/// Nostr Connect URI
#[derive(Debug, PartialEq, Eq, Object)]
#[uniffi::export(Debug, Display, Eq)]
pub struct NostrWalletConnectURI {
    inner: nip47::NostrWalletConnectURI,
}

impl fmt::Display for NostrWalletConnectURI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for NostrWalletConnectURI {
    type Target = nip47::NostrWalletConnectURI;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nip47::NostrWalletConnectURI> for NostrWalletConnectURI {
    fn from(inner: nip47::NostrWalletConnectURI) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl NostrWalletConnectURI {
    /// Create new Nostr Wallet Connect URI
    #[uniffi::constructor]
    pub fn new(
        public_key: &PublicKey,
        relays: Vec<Arc<RelayUrl>>,
        random_secret_key: &SecretKey,
        lud16: Option<String>,
    ) -> Result<Self> {
        Ok(nip47::NostrWalletConnectURI::new(
            **public_key,
            relays
                .into_iter()
                .map(|u| u.as_ref().deref().clone())
                .collect(),
            random_secret_key.deref().clone(),
            lud16,
        )
        .into())
    }

    #[uniffi::constructor]
    pub fn parse(uri: String) -> Result<Self> {
        Ok(nip47::NostrWalletConnectURI::from_str(&uri)?.into())
    }

    /// App Pubkey
    pub fn public_key(&self) -> Arc<PublicKey> {
        Arc::new(self.inner.public_key.into())
    }

    /// URLs of the relays of choice where the `App` is connected and the `Signer` must send and listen for messages.
    pub fn relays(&self) -> Vec<Arc<RelayUrl>> {
        self.inner
            .relays
            .iter()
            .cloned()
            .map(|u| Arc::new(u.into()))
            .collect()
    }

    /// 32-byte randomly generated hex encoded string
    pub fn secret(&self) -> Arc<SecretKey> {
        Arc::new(self.inner.secret.clone().into())
    }

    /// A lightning address that clients can use to automatically setup the lud16 field on the user's profile if they have none configured.
    pub fn lud16(&self) -> Option<String> {
        self.inner.lud16.clone()
    }
}
