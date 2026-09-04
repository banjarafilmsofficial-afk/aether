use aether::{AIResource, SecurityClassification, SharingState, VerificationState};
use serde_json::Value;

#[test]
fn canonical_minimal_json_contains_only_schema_fields() {
    let resource = AIResource::new("resource-1", "provider-1");
    let value: Value = serde_json::from_str(&resource.to_json().unwrap()).unwrap();

    let object = value.as_object().unwrap();
    assert!(object.contains_key("resource_id"));
    assert!(object.contains_key("provider"));
    assert!(object.contains_key("sharing_state"));
    assert!(object.contains_key("security_classification"));
    assert!(object.contains_key("verification_state"));
    assert!(!object.contains_key("credentials"));
    assert!(!object.contains_key("router"));
}

#[test]
fn contract_enums_round_trip_through_ai_resource() {
    let states = [
        SharingState::Independent,
        SharingState::Shared,
        SharingState::PartiallyShared,
        SharingState::Unknown,
    ];
    for sharing_state in states {
        let mut resource = AIResource::new("resource-1", "provider-1");
        resource.sharing_state = sharing_state;
        resource.security_classification = SecurityClassification::Sensitive;
        resource.verification_state = VerificationState::Certified;

        let decoded = AIResource::from_json(&resource.to_json().unwrap()).unwrap();
        assert_eq!(decoded.sharing_state, sharing_state);
        assert_eq!(decoded.security_classification, SecurityClassification::Sensitive);
        assert_eq!(decoded.verification_state, VerificationState::Certified);
    }
}

#[test]
fn invalid_contract_enum_is_rejected_at_deserialization_boundary() {
    let invalid = r#"{
        "resource_id":"resource-1",
        "provider":"provider-1",
        "sharing_state":"INVALID",
        "security_classification":"UNKNOWN",
        "verification_state":"DISCOVERED"
    }"#;

    assert!(AIResource::from_json(invalid).is_err());
}
