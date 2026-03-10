# RCA Experiment Findings

## RCA-EXP-002 — HTTP Server (RCA-E Variant)

**Author:** Gavin Walters
**Date:** 2026-03-10
**Variant Tested:** RCA-E (Event-Driven)
**Domain:** Network / HTTP Request–Response System

---

# 1. Experiment Objective

Evaluate how the **Regulated Cell Architecture – Event Variant (RCA-E)** performs when applied to a simple HTTP request–response system using a real TCP socket interface.

This experiment was designed to test whether RCA-E naturally aligns with externally triggered systems where data enters the system as events and must be interpreted and transformed into responses.

Key questions:

* Does RCA-E map cleanly to network request/response flows?
* Can the architecture process real event input without structural friction?
* Does branching response logic remain clear within the task chain?
* How much boundary adaptation is required between the OS/network layer and the RCA system?

---

# 2. System Overview

The experimental HTTP server was implemented using Rust’s standard library (`TcpListener`) with RCA-E responsible for processing request events.

The system was structured as follows:

### Boundary Layer (main.rs)

Responsible for interacting with the external operating system:

* Accept TCP connection
* Read raw HTTP request bytes
* Invoke RCA-E runtime flow
* Serialize and return HTTP response

```
Socket Accept
   ↓
Raw HTTP Request
   ↓
run_rca_event_flow()
   ↓
HTTP Response Model
   ↓
Socket Write
```

### RCA-E Core

RCA-E was responsible for interpreting and processing request events.

Task sequence used:

```
ParseRequest
   ↓
BuildResponse
```

Data flow:

```
CellData::RawRequest
      ↓
CellData::RequestModel
      ↓
CellData::ResponseModel
```

---

# 3. Architecture Layout

RCA-E source structure after refactoring:

```
rca_e/
  mod.rs
  data.rs
  state.rs
  threads.rs
  cell.rs
  engine.rs
```

Responsibilities:

| File         | Responsibility                                        |
| ------------ | ----------------------------------------------------- |
| `data.rs`    | Apex data model and domain-specific models            |
| `state.rs`   | System lifecycle states                               |
| `threads.rs` | Logical thread execution and stepping                 |
| `cell.rs`    | Cell structure, task definitions, and task behavior   |
| `engine.rs`  | Runtime engine configuration and execution procedures |

Notable architectural refinement:

* `tasks.rs` was merged into `cell.rs`
* `engine.rs` introduced as the runtime orchestration layer

This separation significantly improved clarity and reduced unnecessary fragmentation.

---

# 4. Implementation Summary

### Task Chain

```
Cell 0 → ParseRequest
Cell 1 → BuildResponse
```

### ParseRequest

Transforms raw HTTP text into structured request data.

```
RawRequest → RequestModel
```

Example parsed fields:

```
Method: GET
Path: /
Host: 127.0.0.1:7878
```

### BuildResponse

Produces response artifacts based on request interpretation.

Response policy implemented:

```
GET /       → 200 OK
Other paths → 404 Not Found
```

Example output:

```
Method: GET
Path: /
Host: 127.0.0.1:7878
```

or

```
Not Found

Method: GET
Path: /test
Host: 127.0.0.1:7878
```

---

# 5. Test Results

Test commands:

```
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878/test
curl -X POST http://127.0.0.1:7878/
```

Observed behavior:

| Request     | Result                           |
| ----------- | -------------------------------- |
| `GET /`     | Correct parsing and 200 response |
| `GET /test` | Correct parsing and 404 response |
| `POST /`    | Method parsed correctly          |

All request variations were successfully processed through the RCA-E task chain.

---

# 6. Observations

### 6.1 Event Alignment

RCA-E aligns naturally with request-driven systems.

HTTP requests behave exactly like events:

```
External Event → System Reaction → Response
```

RCA-E mirrors this structure directly.

---

### 6.2 Natural Data Transformation

The data pipeline felt intuitive:

```
RawRequest
   ↓
RequestModel
   ↓
ResponseModel
```

Each transformation maps cleanly to a task cell.

No structural workarounds were required.

---

### 6.3 Boundary Separation

Keeping the socket interface outside RCA-E proved to be a clean design choice.

Boundary layer responsibilities:

* TCP socket lifecycle
* byte-level IO

RCA-E responsibilities:

* event interpretation
* system behavior
* response policy

This separation prevented unnecessary coupling between system IO and architectural logic.

---

### 6.4 Branching Logic

Introducing response policy branching (`200` vs `404`) did not complicate the architecture.

Branching logic fits naturally inside task execution.

Example:

```
ParseRequest
   ↓
BuildResponse
   ├── GET /      → 200
   └── otherwise  → 404
```

This confirmed that RCA-E supports conditional system behavior without disrupting the cell execution model.

---

### 6.5 Runtime Control

The RCA engine maintained full control of:

* task sequencing
* state mutation
* data propagation

The architecture remained deterministic and transparent.

---

# 7. Architectural Insights

This experiment revealed several useful design insights.

### 7.1 Event Systems Are a Strong Fit

RCA-E appears particularly suitable for systems where:

* external inputs trigger system execution
* events must be interpreted
* responses must be generated

Examples include:

* HTTP servers
* network services
* message brokers
* API handlers
* distributed systems

---

### 7.2 Boundary Isolation Is Important

Attempting to move OS-level IO directly into RCA-E would likely introduce unnecessary complexity.

Keeping boundary adapters outside the architecture allows RCA to focus on system logic rather than infrastructure mechanics.

---

### 7.3 Cell/Task Consolidation Improved Clarity

Merging `tasks.rs` into `cell.rs` simplified the architecture significantly.

The relationship between cells and tasks is inherently tight, so separating them introduced unnecessary indirection.

This refactor improved maintainability and reasoning.

---

### 7.4 Engine Layer Emerged Naturally

The introduction of `engine.rs` reflects a consistent pattern:

Each experiment or system requires a small runtime configuration layer responsible for:

* loading tasks
* running execution flows
* integrating with system boundaries

This suggests the RCA architecture benefits from a dedicated engine configuration layer.

---

# 8. Comparison to Previous Experiment

| Experiment | Variant | Domain          | Result                                                      |
| ---------- | ------- | --------------- | ----------------------------------------------------------- |
| EXP-001    | RCA-S   | GUI application | Architectural friction due to framework loop ownership      |
| EXP-002    | RCA-E   | HTTP server     | Strong alignment with event-driven request/response systems |

The HTTP experiment required significantly less architectural negotiation than the GUI experiment.

---

# 9. Conclusion

The RCA-E architecture proved to be a strong fit for HTTP request–response systems.

The architecture cleanly handled:

* real external events
* structured request parsing
* deterministic task sequencing
* conditional response logic
* response artifact generation

Minimal adaptation was required to integrate the architecture with the network boundary.

This experiment suggests that **RCA-E may be particularly well suited for event-driven and network-based systems**.

---
