// This file is @generated by wasmcloud/weld-codegen 0.5.0.
// It is not intended for manual editing.
// namespace: org.wasmcloud.examples.payments

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

/// Parameters sent for AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizePaymentRequest {
    /// Amount of transaction, in cents.
    #[serde(default)]
    pub amount: u32,
    /// The entity (customer) requesting this payment
    #[serde(rename = "paymentEntity")]
    #[serde(default)]
    pub payment_entity: String,
    /// Token of the payment method to be used
    #[serde(rename = "paymentMethod")]
    #[serde(default)]
    pub payment_method: String,
    /// Opaque Reference ID (e.g. order number)
    #[serde(rename = "referenceId")]
    #[serde(default)]
    pub reference_id: String,
    /// Amount of tax applied to this transaction, in cents
    #[serde(default)]
    pub tax: u32,
}

// Encode AuthorizePaymentRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_authorize_payment_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &AuthorizePaymentRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(5)?;
    e.str("amount")?;
    e.u32(val.amount)?;
    e.str("paymentEntity")?;
    e.str(&val.payment_entity)?;
    e.str("paymentMethod")?;
    e.str(&val.payment_method)?;
    e.str("referenceId")?;
    e.str(&val.reference_id)?;
    e.str("tax")?;
    e.u32(val.tax)?;
    Ok(())
}

// Decode AuthorizePaymentRequest from cbor input stream
#[doc(hidden)]
pub fn decode_authorize_payment_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<AuthorizePaymentRequest, RpcError> {
    let __result = {
        let mut amount: Option<u32> = None;
        let mut payment_entity: Option<String> = None;
        let mut payment_method: Option<String> = None;
        let mut reference_id: Option<String> = None;
        let mut tax: Option<u32> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct AuthorizePaymentRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => amount = Some(d.u32()?),
                    1 => payment_entity = Some(d.str()?.to_string()),
                    2 => payment_method = Some(d.str()?.to_string()),
                    3 => reference_id = Some(d.str()?.to_string()),
                    4 => tax = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "amount" => amount = Some(d.u32()?),
                    "paymentEntity" => payment_entity = Some(d.str()?.to_string()),
                    "paymentMethod" => payment_method = Some(d.str()?.to_string()),
                    "referenceId" => reference_id = Some(d.str()?.to_string()),
                    "tax" => tax = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        }
        AuthorizePaymentRequest {
            amount: if let Some(__x) = amount {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizePaymentRequest.amount (#0)".to_string(),
                ));
            },

            payment_entity: if let Some(__x) = payment_entity {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizePaymentRequest.payment_entity (#1)".to_string(),
                ));
            },

            payment_method: if let Some(__x) = payment_method {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizePaymentRequest.payment_method (#2)".to_string(),
                ));
            },

            reference_id: if let Some(__x) = reference_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizePaymentRequest.reference_id (#3)".to_string(),
                ));
            },

            tax: if let Some(__x) = tax {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizePaymentRequest.tax (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Response to AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizePaymentResponse {
    /// Optional string containing the tx ID of auth
    #[serde(rename = "authCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Optional string w/rejection reason
    #[serde(rename = "failReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
    /// Indicates a successful authorization
    #[serde(default)]
    pub success: bool,
}

// Encode AuthorizePaymentResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_authorize_payment_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &AuthorizePaymentResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    if let Some(val) = val.auth_code.as_ref() {
        e.str("authCode")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.fail_reason.as_ref() {
        e.str("failReason")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("success")?;
    e.bool(val.success)?;
    Ok(())
}

// Decode AuthorizePaymentResponse from cbor input stream
#[doc(hidden)]
pub fn decode_authorize_payment_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<AuthorizePaymentResponse, RpcError> {
    let __result = {
        let mut auth_code: Option<Option<String>> = Some(None);
        let mut fail_reason: Option<Option<String>> = Some(None);
        let mut success: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct AuthorizePaymentResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        auth_code = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        fail_reason = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "authCode" => {
                        auth_code = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "failReason" => {
                        fail_reason = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "success" => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        AuthorizePaymentResponse {
            auth_code: auth_code.unwrap(),
            fail_reason: fail_reason.unwrap(),

            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AuthorizePaymentResponse.success (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Confirm the payment (e.g., cause the transaction amount
/// to be withdrawn from the payer's account)
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompletePaymentRequest {
    /// authorization code from the AuthorizePaymentResponse
    #[serde(rename = "authCode")]
    #[serde(default)]
    pub auth_code: String,
    /// An optional description field to be added to the payment summary
    /// (e.g., memo field of a credit card statement) |
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// Encode CompletePaymentRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_complete_payment_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CompletePaymentRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("authCode")?;
    e.str(&val.auth_code)?;
    if let Some(val) = val.description.as_ref() {
        e.str("description")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode CompletePaymentRequest from cbor input stream
#[doc(hidden)]
pub fn decode_complete_payment_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CompletePaymentRequest, RpcError> {
    let __result = {
        let mut auth_code: Option<String> = None;
        let mut description: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct CompletePaymentRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => auth_code = Some(d.str()?.to_string()),
                    1 => {
                        description = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "authCode" => auth_code = Some(d.str()?.to_string()),
                    "description" => {
                        description = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        CompletePaymentRequest {
            auth_code: if let Some(__x) = auth_code {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CompletePaymentRequest.auth_code (#0)".to_string(),
                ));
            },
            description: description.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompletePaymentResponse {
    /// True if the payment was successful
    #[serde(default)]
    pub success: bool,
    /// Timestamp (milliseconds since epoch, UTC)
    #[serde(default)]
    pub timestamp: u64,
    /// Transaction id issued by Payment provider
    #[serde(default)]
    pub txid: String,
}

// Encode CompletePaymentResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_complete_payment_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CompletePaymentResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    e.str("success")?;
    e.bool(val.success)?;
    e.str("timestamp")?;
    e.u64(val.timestamp)?;
    e.str("txid")?;
    e.str(&val.txid)?;
    Ok(())
}

// Decode CompletePaymentResponse from cbor input stream
#[doc(hidden)]
pub fn decode_complete_payment_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CompletePaymentResponse, RpcError> {
    let __result = {
        let mut success: Option<bool> = None;
        let mut timestamp: Option<u64> = None;
        let mut txid: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct CompletePaymentResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => success = Some(d.bool()?),
                    1 => timestamp = Some(d.u64()?),
                    2 => txid = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "success" => success = Some(d.bool()?),
                    "timestamp" => timestamp = Some(d.u64()?),
                    "txid" => txid = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        CompletePaymentResponse {
            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CompletePaymentResponse.success (#0)".to_string(),
                ));
            },

            timestamp: if let Some(__x) = timestamp {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CompletePaymentResponse.timestamp (#1)".to_string(),
                ));
            },

            txid: if let Some(__x) = txid {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CompletePaymentResponse.txid (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// A PaymentMethod contains a token string and a description
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

// Encode PaymentMethod as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_payment_method<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PaymentMethod,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    if let Some(val) = val.description.as_ref() {
        e.str("description")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.token.as_ref() {
        e.str("token")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode PaymentMethod from cbor input stream
#[doc(hidden)]
pub fn decode_payment_method(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<PaymentMethod, RpcError> {
    let __result = {
        let mut description: Option<Option<String>> = Some(None);
        let mut token: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct PaymentMethod, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        description = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        token = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "description" => {
                        description = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "token" => {
                        token = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        PaymentMethod {
            description: description.unwrap(),
            token: token.unwrap(),
        }
    };
    Ok(__result)
}
/// An ordered list of payment methods.
pub type PaymentMethods = Vec<PaymentMethod>;

// Encode PaymentMethods as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_payment_methods<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PaymentMethods,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_payment_method(e, item)?;
    }
    Ok(())
}

// Decode PaymentMethods from cbor input stream
#[doc(hidden)]
pub fn decode_payment_methods(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<PaymentMethods, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<PaymentMethod> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_payment_method(d).map_err(|e| {
                    format!(
                        "decoding 'org.wasmcloud.examples.payments#PaymentMethod': {}",
                        e
                    )
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<PaymentMethod> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_payment_method(d).map_err(|e| {
                        format!(
                            "decoding 'org.wasmcloud.examples.payments#PaymentMethod': {}",
                            e
                        )
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// wasmbus.contractId: wasmcloud:example:payments
/// wasmbus.providerReceive
#[async_trait]
pub trait Payments {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:payments"
    }
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Payments _cannot_ be completed without getting
    /// a validation code (in other words, all payments have to be pre-authorized).
    async fn authorize_payment(
        &self,
        ctx: &Context,
        arg: &AuthorizePaymentRequest,
    ) -> RpcResult<AuthorizePaymentResponse>;
    /// Completes a previously authorized payment.
    /// This operation requires the "authorization code" from a successful
    /// authorization operation.
    async fn complete_payment(
        &self,
        ctx: &Context,
        arg: &CompletePaymentRequest,
    ) -> RpcResult<CompletePaymentResponse>;
    /// `GetPaymentMethods` - Retrieves an _opaque_ list of payment methods,
    /// which is a list of customer-facing method names and the
    /// _[tokens](https://en.wikipedia.org/wiki/Tokenization_(data_security))_
    /// belonging to that payment method. You could think of this list as
    /// a previously saved list of payment methods stored in a "wallet".
    /// A payment method _token_ is required to authorize and subsequently
    /// complete a payment transaction. A customer could have previously
    /// supplied their credit card and user-friendly labels for those methods
    /// like "personal" and "work", etc.
    async fn get_payment_methods(&self, ctx: &Context) -> RpcResult<PaymentMethods>;
}

/// PaymentsReceiver receives messages defined in the Payments service trait
#[doc(hidden)]
#[async_trait]
pub trait PaymentsReceiver: MessageDispatch + Payments {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "AuthorizePayment" => {
                let value: AuthorizePaymentRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'AuthorizePaymentRequest': {}", e)))?;

                let resp = Payments::authorize_payment(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "CompletePayment" => {
                let value: CompletePaymentRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'CompletePaymentRequest': {}", e)))?;

                let resp = Payments::complete_payment(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "GetPaymentMethods" => {
                let resp = Payments::get_payment_methods(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Payments::{}",
                message.method
            ))),
        }
    }
}

/// PaymentsSender sends messages to a Payments service
/// client for sending Payments messages
#[derive(Debug)]
pub struct PaymentsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> PaymentsSender<T> {
    /// Constructs a PaymentsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl PaymentsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Payments provider
    /// implementing the 'wasmcloud:example:payments' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:payments",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Payments provider
    /// implementing the 'wasmcloud:example:payments' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:payments",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Payments for PaymentsSender<T> {
    #[allow(unused)]
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Payments _cannot_ be completed without getting
    /// a validation code (in other words, all payments have to be pre-authorized).
    async fn authorize_payment(
        &self,
        ctx: &Context,
        arg: &AuthorizePaymentRequest,
    ) -> RpcResult<AuthorizePaymentResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Payments.AuthorizePayment",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: AuthorizePaymentResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': AuthorizePaymentResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Completes a previously authorized payment.
    /// This operation requires the "authorization code" from a successful
    /// authorization operation.
    async fn complete_payment(
        &self,
        ctx: &Context,
        arg: &CompletePaymentRequest,
    ) -> RpcResult<CompletePaymentResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Payments.CompletePayment",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CompletePaymentResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CompletePaymentResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// `GetPaymentMethods` - Retrieves an _opaque_ list of payment methods,
    /// which is a list of customer-facing method names and the
    /// _[tokens](https://en.wikipedia.org/wiki/Tokenization_(data_security))_
    /// belonging to that payment method. You could think of this list as
    /// a previously saved list of payment methods stored in a "wallet".
    /// A payment method _token_ is required to authorize and subsequently
    /// complete a payment transaction. A customer could have previously
    /// supplied their credit card and user-friendly labels for those methods
    /// like "personal" and "work", etc.
    async fn get_payment_methods(&self, ctx: &Context) -> RpcResult<PaymentMethods> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Payments.GetPaymentMethods",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: PaymentMethods = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': PaymentMethods", e)))?;
        Ok(value)
    }
}
