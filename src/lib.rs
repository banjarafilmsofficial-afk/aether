//! Canonical Aether AIResource v1 domain representation.
//!
//! This module mirrors the frozen `AIRESOURCE_SPEC.md` and
//! `schemas/ai-resource.schema.json` contract without implementing future
//! resource-management behavior.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// A non-negative JSON number used by capacity and cost measurements.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct NonNegative(pub f64);

impl TryFrom<f64> for NonNegative {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && value >= 0.0 {
            Ok(Self(value))
        } else {
            Err("value must be finite and non-negative")
        }
    }
}

impl From<NonNegative> for f64 {
    fn from(value: NonNegative) -> Self {
        value.0
    }
}

/// A confidence value constrained to the schema's inclusive 0..=1 range.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Confidence(pub f64);

impl TryFrom<f64> for Confidence {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err("confidence must be finite and between 0 and 1")
        }
    }
}

impl From<Confidence> for f64 {
    fn from(value: Confidence) -> Self {
        value.0
    }
}

/// Resource sharing state from AIResource v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharingState {
    #[serde(rename = "INDEPENDENT")]
    Independent,
    #[serde(rename = "SHARED")]
    Shared,
    #[serde(rename = "PARTIALLY_SHARED")]
    PartiallyShared,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

/// Security classification from AIResource v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityClassification {
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "TRUSTED")]
    Trusted,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "SENSITIVE")]
    Sensitive,
}

/// Verification lifecycle state from AIResource v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationState {
    #[serde(rename = "DISCOVERED")]
    Discovered,
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "CERTIFIED")]
    Certified,
    #[serde(rename = "DEGRADED")]
    Degraded,
    #[serde(rename = "QUARANTINED")]
    Quarantined,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "REJECTED")]
    Rejected,
}

/// Ownership and scope metadata. It contains identifiers only, never secrets.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ownership {
    pub account: Option<String>,
    pub project: Option<String>,
    pub organization: Option<String>,
    pub quota_domain: Option<String>,
}

/// A native capacity measurement and its levels of knowledge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapacityMeasure {
    pub unit: String,
    pub advertised: Option<NonNegative>,
    pub verified: Option<NonNegative>,
    pub effective: Option<NonNegative>,
    pub remaining: Option<NonNegative>,
    pub reset_at: Option<String>,
    pub reset_period: Option<String>,
}

impl CapacityMeasure {
    /// Create an empty measurement for a native capacity unit.
    pub fn new(unit: impl Into<String>) -> Self {
        Self {
            unit: unit.into(),
            advertised: None,
            verified: None,
            effective: None,
            remaining: None,
            reset_at: None,
            reset_period: None,
        }
    }
}

/// Native capacity dimensions. Different dimensions are never implicitly converted.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capacity {
    pub tokens: Option<CapacityMeasure>,
    pub requests: Option<CapacityMeasure>,
    #[serde(rename = "TPM")]
    pub tpm: Option<CapacityMeasure>,
    #[serde(rename = "RPM")]
    pub rpm: Option<CapacityMeasure>,
    #[serde(rename = "TPD")]
    pub tpd: Option<CapacityMeasure>,
    #[serde(rename = "RPD")]
    pub rpd: Option<CapacityMeasure>,
    pub concurrency: Option<CapacityMeasure>,
    pub credits: Option<CapacityMeasure>,
    pub compute: Option<CapacityMeasure>,
}

/// A capability value whose type remains explicit rather than treating unknown as false.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CapabilityValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Object(serde_json::Map<String, Value>),
    Null,
}

/// Capability metadata supported by AIResource v1.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    pub reasoning: Option<CapabilityValue>,
    pub coding: Option<CapabilityValue>,
    pub vision: Option<CapabilityValue>,
    pub audio: Option<CapabilityValue>,
    pub tools: Option<CapabilityValue>,
    pub structured_output: Option<CapabilityValue>,
    pub context: Option<CapabilityValue>,
}

/// A time-stamped operational metric.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metric {
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub observed_at: Option<String>,
}

/// Operational observations; these are not resource identity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operations {
    pub latency: Option<Metric>,
    pub throughput: Option<Metric>,
    pub errors: Option<Metric>,
    pub reliability: Option<Metric>,
    pub cooldown: Option<Metric>,
}

/// Native billing information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cost {
    pub amount: Option<NonNegative>,
    pub currency: Option<String>,
    pub unit: Option<String>,
    pub basis: Option<String>,
}

/// Economic metadata.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Economics {
    pub cost: Option<Cost>,
    pub free: Option<bool>,
    pub recurring: Option<bool>,
    pub promotional: Option<bool>,
    pub paid: Option<bool>,
}

/// Policy metadata. Security classification is intentionally top-level per the frozen schema.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub privacy: Option<String>,
    pub region: Option<String>,
    pub tos: Option<String>,
    pub permitted_workload: Option<String>,
}

/// Provenance for externally asserted resource facts.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source: Option<String>,
    pub evidence: Option<String>,
    pub timestamp: Option<String>,
    pub verification: Option<String>,
    pub confidence: Option<Confidence>,
}

/// Canonical AIResource v1 representation.
///
/// `endpoint` is an access path, not a resource or quota identity. Credentials
/// are intentionally absent from this intelligence representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AIResource {
    pub resource_id: String,
    pub provider: String,
    pub model: Option<String>,
    pub version: Option<String>,
    pub endpoint: Option<String>,
    pub ownership: Option<Ownership>,
    pub quota_domain_id: Option<String>,
    pub sharing_state: SharingState,
    pub capacity: Option<Capacity>,
    pub capabilities: Option<Capabilities>,
    pub operations: Option<Operations>,
    pub economics: Option<Economics>,
    pub policy: Option<Policy>,
    pub security_classification: SecurityClassification,
    pub provenance: Option<Provenance>,
    pub verification_state: VerificationState,
}

impl AIResource {
    /// Construct the minimal valid v1 resource representation.
    pub fn new(resource_id: impl Into<String>, provider: impl Into<String>) -> Self {
        Self {
            resource_id: resource_id.into(),
            provider: provider.into(),
            model: None,
            version: None,
            endpoint: None,
            ownership: None,
            quota_domain_id: None,
            sharing_state: SharingState::Unknown,
            capacity: None,
            capabilities: None,
            operations: None,
            economics: None,
            policy: None,
            security_classification: SecurityClassification::Unknown,
            provenance: None,
            verification_state: VerificationState::Discovered,
        }
    }

    /// Serialize this resource using the canonical JSON representation.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize a resource from the canonical JSON representation.
    pub fn from_json(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }
}

impl fmt::Display for SharingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Independent => "INDEPENDENT",
            Self::Shared => "SHARED",
            Self::PartiallyShared => "PARTIALLY_SHARED",
            Self::Unknown => "UNKNOWN",
        };
        f.write_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measure(unit: &str) -> CapacityMeasure {
        CapacityMeasure {
            unit: unit.to_owned(),
            advertised: Some(NonNegative(100.0)),
            verified: Some(NonNegative(90.0)),
            effective: Some(NonNegative(80.0)),
            remaining: Some(NonNegative(70.0)),
            reset_at: Some("2026-09-05T12:00:00Z".to_owned()),
            reset_period: Some("1m".to_owned()),
        }
    }

    fn fully_populated() -> AIResource {
        AIResource {
            resource_id: "resource-1".into(),
            provider: "provider-1".into(),
            model: Some("model-1".into()),
            version: Some("v1".into()),
            endpoint: Some("https://example.invalid/v1".into()),
            ownership: Some(Ownership {
                account: Some("account-1".into()),
                project: Some("project-1".into()),
                organization: Some("org-1".into()),
                quota_domain: Some("quota-1".into()),
            }),
            quota_domain_id: Some("quota-domain-1".into()),
            sharing_state: SharingState::PartiallyShared,
            capacity: Some(Capacity {
                tokens: Some(measure("tokens")),
                requests: Some(measure("requests")),
                tpm: Some(measure("tokens/minute")),
                rpm: Some(measure("requests/minute")),
                tpd: Some(measure("tokens/day")),
                rpd: Some(measure("requests/day")),
                concurrency: Some(measure("requests")),
                credits: Some(measure("credits")),
                compute: Some(measure("GPU-hours")),
            }),
            capabilities: Some(Capabilities {
                reasoning: Some(CapabilityValue::Boolean(true)),
                coding: Some(CapabilityValue::Number(1.0)),
                vision: Some(CapabilityValue::String("image".into())),
                audio: Some(CapabilityValue::Null),
                tools: Some(CapabilityValue::Object(serde_json::Map::from_iter([(
                    "native".into(),
                    Value::Bool(true),
                )]))),
                structured_output: Some(CapabilityValue::Boolean(true)),
                context: Some(CapabilityValue::Number(128_000.0)),
            }),
            operations: Some(Operations {
                latency: Some(Metric {
                    value: Some(120.0),
                    unit: Some("ms".into()),
                    observed_at: Some("2026-09-05T11:00:00Z".into()),
                }),
                throughput: Some(Metric {
                    value: Some(100.0),
                    unit: Some("tokens/s".into()),
                    observed_at: Some("2026-09-05T11:00:00Z".into()),
                }),
                errors: Some(Metric {
                    value: Some(0.01),
                    unit: Some("ratio".into()),
                    observed_at: Some("2026-09-05T11:00:00Z".into()),
                }),
                reliability: Some(Metric {
                    value: Some(0.99),
                    unit: Some("ratio".into()),
                    observed_at: Some("2026-09-05T11:00:00Z".into()),
                }),
                cooldown: Some(Metric {
                    value: Some(10.0),
                    unit: Some("s".into()),
                    observed_at: Some("2026-09-05T11:00:00Z".into()),
                }),
            }),
            economics: Some(Economics {
                cost: Some(Cost {
                    amount: Some(NonNegative(0.25)),
                    currency: Some("USD".into()),
                    unit: Some("request".into()),
                    basis: Some("per request".into()),
                }),
                free: Some(false),
                recurring: Some(false),
                promotional: Some(true),
                paid: Some(true),
            }),
            policy: Some(Policy {
                privacy: Some("provider-policy".into()),
                region: Some("global".into()),
                tos: Some("permitted".into()),
                permitted_workload: Some("inference".into()),
            }),
            security_classification: SecurityClassification::Trusted,
            provenance: Some(Provenance {
                source: Some("provider-docs".into()),
                evidence: Some("quota-test-1".into()),
                timestamp: Some("2026-09-05T11:00:00Z".into()),
                verification: Some("quota-test".into()),
                confidence: Some(Confidence(0.95)),
            }),
            verification_state: VerificationState::Certified,
        }
    }

    #[test]
    fn minimal_resource_round_trips() {
        let resource = AIResource::new("r1", "provider");
        let json = resource.to_json().unwrap();
        let decoded = AIResource::from_json(&json).unwrap();
        assert_eq!(decoded, resource);
    }

    #[test]
    fn fully_populated_resource_round_trips() {
        let resource = fully_populated();
        let json = resource.to_json().unwrap();
        let decoded = AIResource::from_json(&json).unwrap();
        assert_eq!(decoded, resource);
        assert!(json.contains("\"TPM\""));
        assert!(json.contains("\"GPU-hours\""));
    }

    #[test]
    fn all_sharing_states_serialize_with_contract_names() {
        let states = [
            (SharingState::Independent, "INDEPENDENT"),
            (SharingState::Shared, "SHARED"),
            (SharingState::PartiallyShared, "PARTIALLY_SHARED"),
            (SharingState::Unknown, "UNKNOWN"),
        ];
        for (state, expected) in states {
            assert_eq!(serde_json::to_string(&state).unwrap(), format!("\"{expected}\""));
        }
    }

    #[test]
    fn all_verification_states_are_supported() {
        let states = [
            (VerificationState::Discovered, "DISCOVERED"),
            (VerificationState::Unverified, "UNVERIFIED"),
            (VerificationState::Certified, "CERTIFIED"),
            (VerificationState::Degraded, "DEGRADED"),
            (VerificationState::Quarantined, "QUARANTINED"),
            (VerificationState::Expired, "EXPIRED"),
            (VerificationState::Rejected, "REJECTED"),
        ];
        for (state, expected) in states {
            assert_eq!(serde_json::to_string(&state).unwrap(), format!("\"{expected}\""));
        }
    }

    #[test]
    fn all_security_classifications_are_supported() {
        let values = [
            (SecurityClassification::Local, "LOCAL"),
            (SecurityClassification::Trusted, "TRUSTED"),
            (SecurityClassification::Standard, "STANDARD"),
            (SecurityClassification::Unknown, "UNKNOWN"),
            (SecurityClassification::Sensitive, "SENSITIVE"),
        ];
        for (value, expected) in values {
            assert_eq!(serde_json::to_string(&value).unwrap(), format!("\"{expected}\""));
        }
    }

    #[test]
    fn native_capacity_dimensions_remain_distinct() {
        let resource = fully_populated();
        let capacity = resource.capacity.unwrap();
        assert_eq!(capacity.tpm.unwrap().unit, "tokens/minute");
        assert_eq!(capacity.rpm.unwrap().unit, "requests/minute");
        assert_eq!(capacity.compute.unwrap().unit, "GPU-hours");
    }

    #[test]
    fn capacity_knowledge_levels_and_reset_information_remain_distinct() {
        let value = measure("requests/day");
        assert_eq!(value.advertised, Some(NonNegative(100.0)));
        assert_eq!(value.verified, Some(NonNegative(90.0)));
        assert_eq!(value.effective, Some(NonNegative(80.0)));
        assert_eq!(value.remaining, Some(NonNegative(70.0)));
        assert_eq!(value.reset_at.as_deref(), Some("2026-09-05T12:00:00Z"));
        assert_eq!(value.reset_period.as_deref(), Some("1m"));
    }

    #[test]
    fn identity_endpoint_and_quota_domain_are_separate_fields() {
        let resource = fully_populated();
        assert_eq!(resource.resource_id, "resource-1");
        assert_eq!(resource.endpoint.as_deref(), Some("https://example.invalid/v1"));
        assert_eq!(resource.quota_domain_id.as_deref(), Some("quota-domain-1"));
    }

    #[test]
    fn credentials_are_not_part_of_the_representation() {
        let resource = fully_populated();
        let json = resource.to_json().unwrap();
        assert!(!json.contains("credential"));
        assert!(!json.contains("api_key"));
        assert!(!json.contains("password"));
        assert!(!json.contains("secret"));
    }

    #[test]
    fn unknown_optional_information_is_preserved_as_null() {
        let resource = AIResource::new("r1", "provider");
        let value: Value = serde_json::from_str(&resource.to_json().unwrap()).unwrap();
        assert_eq!(value["model"], Value::Null);
        assert_eq!(value["endpoint"], Value::Null);
        assert_eq!(value["sharing_state"], "UNKNOWN");
        assert_eq!(value["security_classification"], "UNKNOWN");
    }

    #[test]
    fn provenance_and_policy_round_trip() {
        let resource = fully_populated();
        let decoded = AIResource::from_json(&resource.to_json().unwrap()).unwrap();
        assert_eq!(decoded.provenance.unwrap().confidence, Some(Confidence(0.95)));
        assert_eq!(decoded.policy.unwrap().tos.as_deref(), Some("permitted"));
    }

    #[test]
    fn invalid_enum_values_are_rejected() {
        let invalid_sharing = r#""NOT_A_SHARING_STATE""#;
        assert!(serde_json::from_str::<SharingState>(invalid_sharing).is_err());

        let invalid_security = r#""NOT_A_SECURITY_CLASS""#;
        assert!(serde_json::from_str::<SecurityClassification>(invalid_security).is_err());

        let invalid_verification = r#""NOT_A_VERIFICATION_STATE""#;
        assert!(serde_json::from_str::<VerificationState>(invalid_verification).is_err());
    }

    #[test]
    fn numeric_constraints_are_rejected() {
        assert!(serde_json::from_str::<NonNegative>("-1").is_err());
        assert!(serde_json::from_str::<Confidence>("1.1").is_err());
    }

    #[test]
    fn unknown_object_fields_are_rejected() {
        let invalid = r#"{
            "resource_id":"r1",
            "provider":"p1",
            "sharing_state":"UNKNOWN",
            "security_classification":"UNKNOWN",
            "verification_state":"DISCOVERED",
            "credentials":"must-not-exist"
        }"#;
        assert!(AIResource::from_json(invalid).is_err());
    }
}
