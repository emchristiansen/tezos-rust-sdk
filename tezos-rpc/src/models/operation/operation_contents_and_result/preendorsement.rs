use tezos_core::types::encoded::{BlockPayloadHash, ImplicitAddress};

use {
    crate::models::balance_update::BalanceUpdate,
    crate::models::operation::kind::OperationKind,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Preendorsement {
    /// [OperationKind::Preendorsement]
    pub kind: OperationKind,
    /// integer ∈ [-2^31-1, 2^31]
    pub level: i32,
    /// integer ∈ [0, 2^16-1]
    pub slot: u16,
    /// integer ∈ [-2^31-1, 2^31]
    pub round: i32,
    /// Hash of a consensus value (Base58Check-encoded)
    pub block_payload_hash: BlockPayloadHash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PreendorsementMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PreendorsementMetadata {
    /// Public key hash (Base58Check-encoded)
    pub delegate: ImplicitAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    /// integer ∈ [-2^30, 2^30]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endorsement_power: Option<i32>,
}
