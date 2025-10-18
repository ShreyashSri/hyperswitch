use common_enums::Currency;
use masking::Secret;
use serde::{Deserialize, Serialize};

// PaymentsResponse
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PayloadPaymentStatus {
    Authorized,
    Declined,
    Processed,
    #[default]
    Processing,
    Rejected,
    Voided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayloadPaymentsResponse {
    PayloadCardsResponse(PayloadCardsResponseData),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReceipt {
    pub approved_amount: ApprovedAmount,
    pub processor_response_details: Option<ProcessorResponseDetails>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApprovedAmount {
    pub total: f64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AvsResponse {
    Unknown,
    NoMatch,
    Zip,
    Street,
    StreetAndZip,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayloadCardsResponseData {
    pub amount: f64,
    pub avs: Option<AvsResponse>,
    pub customer_id: Option<Secret<String>>,
    #[serde(rename = "id")]
    pub transaction_id: String,
    #[serde(rename = "payment_method_id")]
    pub connector_payment_method_id: Option<Secret<String>>,
    pub processing_id: Option<Secret<String>>,
    pub processing_method_id: Option<String>,
    pub ref_number: Option<String>,
    pub status: PayloadPaymentStatus,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    #[serde(rename = "type")]
    pub response_type: Option<String>,
}

// Type definition for Refund Response
// Added based on assumptions since this is not provided in the documentation
#[derive(Debug, Copy, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Declined,
    Processed,
    #[default]
    Processing,
    Rejected,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundsLedger {
    pub amount: f64,
    #[serde(rename = "assoc_transaction_id")]
    pub associated_transaction_id: String, // Connector transaction id
    #[serde(rename = "id")]
    pub ledger_id: Secret<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PayloadRefundResponse {
    pub amount: f64,
    #[serde(rename = "id")]
    pub transaction_id: String,
    pub ledger: Vec<RefundsLedger>,
    #[serde(rename = "payment_method_id")]
    pub connector_payment_method_id: Option<Secret<String>>,
    pub processing_id: Option<Secret<String>>,
    pub ref_number: Option<String>,
    pub status: RefundStatus,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub payment_receipt: PaymentReceipt,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PayloadErrorResponse {
    pub error: Option<Vec<PayloadErrorDetails>>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PayloadErrorDetails {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
    pub field: Option<String>,
    pub message: Option<String>,
    pub additional_info: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorResponseDetails {
    pub approval_status: Option<String>,
    pub approval_code: Option<String>,
    pub reference_number: Option<String>,
    pub processor: Option<String>,
    pub host: Option<String>,
    pub network_routed: Option<String>,
    pub network_international_id: Option<String>,
    pub response_code: Option<String>,
    pub response_message: Option<String>,
    pub host_response_code: Option<String>,
    pub host_response_message: Option<String>,
    pub additional_info: Option<Vec<AdditionalInfo>>,
    pub bank_association_details: Option<BankAssociationDetails>,
    pub response_indicators: Option<ResponseIndicators>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BankAssociationDetails {
    pub association_response_code: Option<String>,
    pub avs_security_code_response: Option<AvsSecurityCodeResponse>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvsSecurityCodeResponse {
    pub street_match: Option<String>,
    pub postal_code_match: Option<String>,
    pub security_code_match: Option<String>,
    pub association: Option<Association>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Association {
    pub avs_code: Option<String>,
    pub security_code_response: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseIndicators {
    pub alternate_route_debit_indicator: Option<bool>,
    pub signature_line_indicator: Option<bool>,
    pub signature_debit_route_indicator: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayloadWebhooksTrigger {
    Payment,
    Processed,
    Authorized,
    Credit,
    Refund,
    Reversal,
    Void,
    AutomaticPayment,
    Decline,
    Deposit,
    Reject,
    #[serde(rename = "payment_activation:status")]
    PaymentActivationStatus,
    #[serde(rename = "payment_link:status")]
    PaymentLinkStatus,
    ProcessingStatus,
    BankAccountReject,
    Chargeback,
    ChargebackReversal,
    #[serde(rename = "transaction:operation")]
    TransactionOperation,
    #[serde(rename = "transaction:operation:clear")]
    TransactionOperationClear,
}

// Webhook response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadWebhookEvent {
    pub object: String, // Added to match actual webhook structure
    pub trigger: PayloadWebhooksTrigger,
    pub webhook_id: String,
    pub triggered_at: String, // Added to match actual webhook structure
    // Webhooks Payload
    pub triggered_on: PayloadEventDetails,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadEventDetails {
    #[serde(rename = "id")]
    pub transaction_id: Option<String>,
    pub object: String,
    pub value: Option<serde_json::Value>, // Changed to handle any value type including null
}
