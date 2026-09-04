# AIResource Specification v1

**Status:** FROZEN
**Version:** 1.0.0
**Phase:** 1 — Aether Foundation
**Module:** 1 — AIResource

## 1. Purpose

`AIResource` is Aether's foundational representation of an AI resource. It provides a common identity, ownership, capacity, capability, operational, economic, policy, and provenance model without collapsing materially different resources into a single synthetic unit.

An AIResource may represent a model, API, provider endpoint, agent, tool, MCP server, A2A agent, memory system, GPU/compute resource, local execution environment, inference server, quota-backed account/project, or another resource exposed through an Aether adapter.

## 2. Design Principles

1. Aether models resources; it does not require every resource to be an LLM.
2. Resource identity is distinct from access path, credential, router, and quota domain.
3. Native quota semantics MUST be preserved.
4. Shared capacity MUST NOT be double-counted.
5. Advertised, verified, and effective capacity are distinct states of knowledge.
6. Unknown information remains unknown; implementations MUST NOT manufacture precision.
7. Credentials are never part of the AIResource intelligence representation.
8. Policy and security constraints are first-class resource attributes.
9. Provenance and verification evidence accompany externally asserted facts.
10. This specification defines the canonical v1 resource object; later capabilities build on it.

## 3. Resource Identity

Required identity fields:

- `resource_id`: stable Aether resource identifier.
- `provider`: provider or owning system name.
- `model`: model/resource name when applicable.
- `version`: version when applicable.
- `endpoint`: logical endpoint/access location when applicable.

The endpoint is an access path, not proof of an independent resource or quota. Multiple endpoints may resolve to one underlying resource or quota domain.

## 4. Ownership and Scope

Ownership/scope MAY include:

- account
- project
- organization
- quota domain

These fields describe resource/account boundaries and MUST NOT contain secrets.

## 5. Resource Sharing

`sharing_state` MUST be one of:

- `INDEPENDENT`
- `SHARED`
- `PARTIALLY_SHARED`
- `UNKNOWN`

`quota_domain_id` identifies the domain in which capacity is actually constrained. Resource identity and quota-domain identity MUST remain separate so that shared capacity can be represented without duplication.

## 6. Capacity

Capacity is represented as native measurements. Supported v1 dimensions include:

- tokens
- requests
- TPM
- RPM
- TPD
- RPD
- concurrency
- credits
- compute

Each capacity dimension MAY contain advertised, verified, and effective values plus native unit and reset information.

Aether MUST NOT convert fundamentally different resources into token-equivalent capacity. Examples include GPU-hours, VRAM, Neurons/day, credits, requests/day, and tokens/minute.

Capacity knowledge levels:

1. **Advertised Capacity** — externally stated or configured capacity.
2. **Verified Capacity** — capacity supported by successful verification/testing/evidence.
3. **Effective Capacity** — capacity Aether currently considers usable after policy, reliability, utilization, cooldown, sharing, and other constraints.

## 7. Capabilities

Capabilities MAY include:

- reasoning
- coding
- vision
- audio
- tools
- structured output
- context

Capability values may be boolean, numeric, or structured metadata where appropriate. Unknown capability information MUST remain distinguishable from false.

## 8. Operations

Operational intelligence MAY include:

- latency
- throughput
- errors
- reliability
- cooldown

Operational observations are time-varying telemetry and are not immutable resource identity.

## 9. Economics

Economic metadata MAY include:

- cost
- free
- recurring
- promotional
- paid

Cost representation MUST preserve the native billing unit and pricing basis. A resource being free or promotional does not imply unlimited capacity.

## 10. Policy and Security

Policy metadata MAY include:

- privacy
- region
- ToS status
- permitted workload
- security classification

Security classification values are:

- `LOCAL`
- `TRUSTED`
- `STANDARD`
- `UNKNOWN`
- `SENSITIVE`

Aether MUST reject or quarantine resources that violate configured policy. The resource object contains metadata and references, never raw credentials.

## 11. Provenance and Verification

Externally asserted resource facts MUST be traceable to provenance where available:

- source
- evidence
- timestamp
- verification
- confidence

Verification lifecycle states are:

- `DISCOVERED`
- `UNVERIFIED`
- `CERTIFIED`
- `DEGRADED`
- `QUARANTINED`
- `EXPIRED`
- `REJECTED`

The verification state describes Aether's confidence in the resource's current validity; it does not itself assert provider ownership or legal entitlement.

## 12. Resource Lifecycle

A resource can progress through discovery and verification independently of its operational availability. Verification MUST use the sequence:

`DISCOVER → IDENTIFY → CONNECT → HEALTH TEST → CAPABILITY TEST → QUOTA TEST → POLICY CHECK → CERTIFY`

Failure or degradation MUST be represented explicitly rather than silently removing historical identity.

## 13. Resource Graph Compatibility

AIResource is a node-level object. Future graph relationships MUST be able to express at minimum:

`Application → Workflow → Agent → Model/Tool → Provider → Endpoint → Quota Domain → Actual Capacity`

The v1 object therefore exposes stable identifiers for resource and quota domain without embedding graph traversal logic.

## 14. Non-Goals for v1

AIResource v1 does NOT implement:

- routing or optimization algorithms
- credential storage or secret encryption
- telemetry ingestion
- quota forecasting
- billing calculations
- evaluation execution
- agent orchestration
- MCP/A2A protocol execution
- gateway behavior
- resource discovery adapters

Those capabilities consume the canonical AIResource abstraction in later modules.

## 15. Canonical JSON Representation

The canonical machine-readable contract is `schemas/ai-resource.schema.json`.

The schema and this specification MUST remain consistent. Changes to the canonical v1 contract require an explicit version change or RFC under Aether governance.

## 16. Freeze Criteria

AIResource v1 is frozen when:

- the specification is committed;
- the JSON Schema is committed;
- the two are reviewed for semantic consistency;
- required identity semantics are unambiguous;
- native capacity semantics are preserved;
- sharing/deduplication semantics are explicit;
- credential separation is explicit;
- no implementation-specific routing behavior is embedded in the contract.

**AIResource v1 status: FROZEN after specification/schema consistency review.**
