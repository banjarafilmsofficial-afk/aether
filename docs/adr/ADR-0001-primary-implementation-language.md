# ADR-0001: Primary Implementation Language

- **Status:** Accepted
- **Date:** 2026-09-05
- **Decision:** Rust is the primary implementation language for the Aether core; Python and TypeScript are first-class ecosystem languages; Go is permitted for selected integrations where justified.

## 1. Context

Aether is infrastructure and control-plane software, not merely an AI application. Its long-term responsibility is to provide a universal control plane for AI resources, economics, agents, and execution. That requires a systems layer capable of representing and controlling heterogeneous resources, enforcing policy and security boundaries, coordinating concurrent execution, integrating with local and remote infrastructure, and eventually supporting distributed execution.

The Aether core therefore needs stronger systems-level guarantees than would normally be required for an application-layer AI product. The implementation language must support reliable concurrency, predictable performance, secure resource handling, networking, local execution, cross-platform deployment, and long-term maintainability while still allowing Aether to integrate with the broader AI ecosystem.

The canonical AIResource representation is intentionally language-neutral through the existing JSON Schema. Language choice for the core must therefore establish a canonical implementation of Aether's semantics without making the resource model dependent on a single ecosystem language.

## 2. Decision

Aether adopts the following implementation-language architecture:

1. **Rust is the primary implementation language for the Aether core.**
2. **Python is a first-class ecosystem language**, including SDK, AI/ML, evaluation, research, optimization experimentation, and integrations where Python is the natural ecosystem boundary.
3. **TypeScript is a first-class ecosystem language**, including SDKs, web/control interfaces, Node/Bun/Deno tooling, MCP ecosystem integration, and developer-facing integrations.
4. **Go is an optional integration language**, permitted for selected infrastructure integrations or operational components when its characteristics provide a material advantage. Go does not become a second canonical Aether core.
5. **Secondary languages MUST NOT independently redefine Aether's core semantics.** They consume and expose the canonical Aether semantics through explicit APIs, protocols, schemas, and SDK boundaries.
6. **AIResource remains language-neutral.** The existing canonical JSON Schema remains the contract for AIResource representation.

## 3. Rationale

### 3.1 Memory safety

Aether is expected to operate close to infrastructure boundaries where memory safety and predictable resource behavior matter. Rust provides compile-time ownership and borrowing guarantees that eliminate broad classes of memory-safety defects without requiring a garbage-collected runtime. This is valuable for a long-lived control plane that may eventually manage local execution, resource isolation, credentials, and distributed compute.

### 3.2 Concurrency

Aether will coordinate concurrent resource discovery, execution, telemetry, policy checks, quota-aware operations, and eventually distributed workloads. Rust provides strong concurrency primitives and compile-time guarantees that make data races and unsafe shared-state behavior harder to introduce while retaining asynchronous execution capabilities.

### 3.3 Performance

The Aether core is intended to sit on critical execution and control paths. Rust provides native-code performance and predictable runtime characteristics without requiring a garbage collector. This gives the core headroom for high-throughput resource management, telemetry, scheduling, policy enforcement, and future execution workloads.

Performance alone is not the decision criterion; the objective is to combine performance with memory safety and systems-level correctness.

### 3.4 Networking

Aether's architecture depends on networking across providers, APIs, inference servers, agents, tools, MCP servers, A2A agents, local machines, cloud infrastructure, and eventually distributed resources. Rust has a mature asynchronous networking ecosystem suitable for implementing network-facing control-plane and execution components while retaining the core's safety properties.

### 3.5 Security

Security is a first-class Aether concern. The platform is expected to handle resource permissions, scoped credentials, policy enforcement, isolation, auditability, provider controls, and potentially sensitive local and remote execution. Rust's memory-safety guarantees and systems-level control are strong foundations for security-sensitive infrastructure. Rust does not make an application automatically secure; secure architecture, dependency management, threat modeling, testing, and operational controls remain necessary.

### 3.6 Local execution

Aether is intended to encompass local models, local inference servers, local machines, and other local compute resources in addition to hosted APIs. A systems language is appropriate for components that may need direct interaction with operating-system resources, processes, filesystems, networking, hardware, and execution isolation. Rust provides these capabilities while retaining portability and safety.

### 3.7 Distributed execution

The long-term Aether architecture includes distributed compute and a global AI Resource Network. Distributed execution increases the importance of predictable resource usage, concurrency, networking, failure handling, serialization boundaries, and security. Rust provides a strong foundation for building these systems without requiring the core semantics to be tied to a managed runtime.

This ADR does not authorize implementation of distributed execution; it only establishes the language direction for future work.

### 3.8 Cross-platform deployment

Aether must be able to operate across developer machines, servers, containers, and heterogeneous compute environments. Rust's compiled deployment model and broad platform support make it suitable for producing portable binaries and infrastructure components while keeping runtime requirements comparatively small.

### 3.9 Ecosystem integration

Aether must integrate with an ecosystem that is inherently polyglot. Python is central to AI/ML, evaluation, experimentation, and many agent ecosystems. TypeScript is central to web applications, Node-based tooling, and important developer ecosystems such as MCP. Go is widely used for infrastructure and operational tooling.

The decision therefore does not attempt to make Rust the only language used by Aether. Instead, Rust is the canonical systems core, while Python and TypeScript are deliberately first-class at the ecosystem boundary and Go remains available where justified. Standardized APIs, protocols, and the language-neutral AIResource schema prevent ecosystem languages from becoming competing implementations of Aether's core semantics.

### 3.10 Long-term maintainability

Aether is intended to become a large infrastructure platform with stable domain semantics. A single canonical core reduces semantic drift, duplicated business logic, inconsistent security behavior, and cross-language disagreement over resource identity, quota, capacity, policy, economics, and execution semantics.

Rust's stricter compile-time model has a higher entry cost than some alternatives, but that cost is accepted in exchange for stronger correctness guarantees at the systems boundary and a clear ownership model for the core. Python and TypeScript remain available where their ecosystem advantages materially improve developer experience and integration velocity.

## 4. Alternatives Considered

### 4.1 Go

**Advantages:** strong concurrency model, straightforward deployment, mature networking and cloud-infrastructure ecosystem, fast compilation, and a relatively low operational footprint.

**Rejected as the primary core language:** Go is a strong fit for conventional cloud-native control planes, but Aether's long-term scope extends further into memory-safe local execution, resource isolation, high-performance execution, and distributed compute. Rust provides stronger compile-time guarantees for these systems-level responsibilities. Go remains permitted for selected integrations where it is the better tool.

### 4.2 TypeScript

**Advantages:** excellent developer experience, broad web and Node ecosystem, strong adoption for developer tooling, and natural alignment with web interfaces and MCP-related tooling.

**Rejected as the primary core language:** TypeScript's runtime and deployment model are less suitable for Aether's systems-level core, particularly where predictable resource behavior, low-level control, local execution, and isolation are important. TypeScript is therefore a first-class ecosystem language rather than the canonical systems language.

### 4.3 Python

**Advantages:** dominant AI/ML ecosystem, rapid experimentation, extensive evaluation and scientific tooling, and broad adoption among AI developers and agent frameworks.

**Rejected as the primary core language:** Python is highly valuable at the ecosystem and research boundary but is not the preferred foundation for a systems-oriented control plane requiring predictable performance, low-level resource control, and strong compile-time guarantees. Python is therefore a first-class ecosystem language.

### 4.4 Polyglot distributed core

A design in which Rust, Go, Python, and TypeScript each implement substantial portions of the Aether core was considered.

**Rejected:** a polyglot core would create duplicated domain semantics, serialization and RPC boundaries, distributed transaction complexity, duplicated security logic, more complicated testing and debugging, and greater risk of semantic drift. Polyglot integration is valuable; polyglot ownership of the canonical core is not.

## 5. Consequences

### Benefits

- Aether has a single canonical systems implementation.
- Memory safety and strong compile-time guarantees improve the foundation for security-sensitive infrastructure.
- Native performance and asynchronous concurrency support high-throughput control and execution paths.
- Rust is well suited to local execution, networking, resource isolation, and future distributed execution.
- Python and TypeScript preserve access to the largest relevant AI/ML and developer ecosystems.
- Go remains available where a specific infrastructure integration benefits materially from Go.
- The language-neutral AIResource schema prevents the canonical resource model from being coupled to Rust.
- A single core reduces long-term semantic drift and duplicated implementation effort.

### Costs and risks

- Rust has a higher learning curve and stricter compiler model than Go, TypeScript, or Python. This increases initial development friction and may reduce the pool of immediately productive contributors.
- Rust's type system and ownership model can require more design effort for application developers who are unfamiliar with the language.
- Cross-language SDK and integration boundaries introduce maintenance overhead even when the core semantics remain centralized.
- Maintaining high-quality Python and TypeScript SDKs requires explicit compatibility, release, and versioning discipline.
- Go integrations create another supported language surface and therefore must remain deliberately scoped.
- Rust does not remove the need for secure design, testing, observability, dependency governance, and operational controls.

These costs are accepted because the Aether core is infrastructure/control-plane software whose long-term requirements justify the stronger systems foundation.

## 6. Architectural Rule

**Aether MUST have one canonical systems core and one canonical resource model.**

Rust owns the canonical implementation of Aether's core semantics. Python and TypeScript are first-class ecosystem languages, and Go may be used for selected integrations where justified.

Secondary languages MUST integrate with the canonical core through explicit, versioned contracts such as APIs, protocols, schemas, and SDK interfaces. They MUST NOT independently redefine or fork core semantics.

The AIResource JSON Schema remains the language-neutral representation contract.

## 7. Scope Boundary

This ADR establishes implementation-language architecture only.

It does **not** authorize implementation of any future Aether subsystem, including but not limited to:

- Resource Graph
- provider or resource adapters
- routing
- optimization
- gateway behavior
- agents or agent control plane
- orchestration
- telemetry ingestion
- quota forecasting
- evaluation execution
- distributed compute
- Aether Network
- credential storage or credential brokering
- any other subsystem not explicitly authorized by the current implementation checkpoint

The current checkpoint remains **Phase 1 → Module 1 → AIResource v1**.

## 8. Relationship to AIResource v1

AIResource v1 is the current foundational implementation target. Its canonical representation remains the existing language-neutral JSON Schema.

After this ADR is accepted, **AIResource v1 will be implemented in Rust**. That implementation must conform to the existing AIResource v1 specification and JSON Schema. This ADR does not authorize changing either artifact as part of the language decision.

## 9. Decision Outcome

The language architecture is frozen as follows:

| Role | Language | Responsibility |
| --- | --- | --- |
| Primary core | **Rust** | Canonical Aether systems implementation and core semantics |
| First-class ecosystem | **Python** | AI/ML, evaluation, research, experimentation, SDKs, integrations |
| First-class ecosystem | **TypeScript** | Web, developer tooling, SDKs, Node/Bun/Deno, MCP ecosystem, integrations |
| Optional integration | **Go** | Selected infrastructure integrations where materially justified |

This decision remains the architectural baseline unless superseded by a formally recorded architecture decision or RFC.
