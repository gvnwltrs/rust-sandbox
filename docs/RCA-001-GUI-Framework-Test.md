# RCA Experiment Findings

Experiment ID: RCA-EXP-001
Date: 2026-03-09
Domain: Desktop GUI Application (Rust + egui/eframe)
RCA Variant: Variant 1 — Sequential Engine

---

# 1. Objective

Evaluate the behavior and limits of **RCA Variant 1** when applied to a GUI-based application domain.

Prior experiments primarily involved:

* terminal programs
* embedded-style programs
* low-level runtime experiments

This experiment introduces a **GUI framework dependency**, which is expected to stress:

* loop control
* threading assumptions
* boundary ownership
* state mutation policies

The goal is to observe how RCA behaves when interacting with a framework that already contains:

* an event loop
* rendering cycles
* internal threading assumptions.

---

# 2. Experimental Setup

Implementation domain:

Rust desktop GUI application using:

* `eframe`
* `egui`

Application constructed:

Minimal **Notepad-style GUI text editor** supporting:

* text input
* display updates
* menu interaction
* state updates
* status display

RCA architecture components used:

Data → States → Threads → Tasks → Engine

Key RCA assumptions under test:

* engine loop controlled by architecture
* deterministic execution order
* task collections representing logical threads
* mutation of apex state only within engine execution

---

# 3. Observed Architectural Pressure

The following pressures emerged during implementation.

## 3.1 Framework-Owned Loop

The `eframe` framework contains an internal runtime loop that repeatedly invokes:

update()

This conflicts with the RCA expectation that:

```
main() → engine loop → thread execution → task execution
```

Instead, the framework initially forced the structure:

```
eframe loop → update() → RCA tasks
```

This inversion of control violated the intended precedence rules of RCA.

---

## 3.2 Framework-Owned Runtime Environment

GUI frameworks impose strong assumptions regarding:

* thread ownership
* event dispatch
* rendering cycles
* UI state updates

This creates a situation where the architecture cannot fully control the runtime environment.

Instead, the architecture must **adapt to a host runtime**.

---

## 3.3 UI State Ownership

Initial attempts caused UI behavior problems because GUI widgets locally mutate their own state.

This conflicts with RCA’s intended rule:

Only the engine thread mutates apex system data.

Resolution required introducing a **message-based exchange model** between GUI and engine.

---

# 4. Architectural Adaptation

The architecture was adapted as follows.

## 4.1 Thread Separation

Two OS-level threads were introduced:

Engine Thread
Runs RCA engine loop and executes logical threads.

GUI Thread
Runs `eframe` event loop.

Architecture becomes:

```
GUI Thread (framework runtime)
        ↓
Message boundary
        ↓
RCA Engine Thread
        ↓
Logical threads (task collections)
```

---

## 4.2 Bidirectional Message Boundary

Communication between GUI and engine implemented using channels.

Two directional flows were created.

### GUI → RCA

User input events:

* text input
* menu actions
* clear/save/open commands

Represented as:

GuiInput events.

---

### RCA → GUI

Display state projection.

Engine publishes a snapshot representation:

DisplayModel

The GUI renders this model without owning it.

---

## 4.3 Snapshot Rendering Model

The GUI does not mutate authoritative system state.

Instead:

1. GUI sends input events
2. Engine updates system state
3. Engine publishes new display snapshot
4. GUI renders snapshot

This preserves RCA's mutation policy.

---

# 5. Validated RCA Invariants

The following architectural principles remained valid.

### Deterministic Execution

Task execution remained fully deterministic.

### Engine-Owned State Mutation

All system state mutation occurred inside the RCA engine.

### Logical Thread Model

Task collections representing logical threads remain valid.

### Clear Data Flow

Data → State → Tasks → Output flow remained understandable.

### Architecture-Driven System Behavior

Despite framework pressure, RCA remained the primary decision authority over system behavior.

---

# 6. Stressed or Violated Assumptions

The following assumptions require reconsideration.

### Engine Loop Ownership

In GUI domains, the architecture may not control the outermost loop.

Frameworks often require ownership of runtime iteration.

---

### Pure Logical Thread Model

Some domains may require OS-level threads at runtime boundaries.

However this does not invalidate the logical thread abstraction inside the architecture.

---

### Direct Execution of External Operations

External frameworks may require adaptation layers rather than direct execution from engine tasks.

---

# 7. Architectural Discoveries

Several new insights emerged.

## Frameworks as Boundary Environments

Frameworks should be treated as **boundary runtime environments**, not core architecture components.

Architecture should interact through adapters.

---

## Display as Projection

GUI rendering should operate on **projected display models**, not direct system state.

This maintains architectural authority over data.

---

## Input as Events

GUI actions should be represented as **events** sent to the architecture.

This prevents UI code from directly mutating system state.

---

## Engine Authority Preservation

Even when frameworks own the outer loop, architecture authority can be preserved through:

* thread isolation
* message boundaries
* projection models

---

# 8. Open Questions

The experiment revealed areas for future exploration.

### Logical Thread Implementation

Can GUI integration be achieved using **RCA logical threads alone** without OS-level thread spawning?

---

### Framework Adapter Layer

Should RCA explicitly define a **Framework Adapter pattern**?

---

### Event Model Standardization

Should RCA define a standard **event protocol** for boundary systems?

---

### Snapshot Model Formalization

Should display state always be expressed as immutable projection models?

---

# 9. Outcome Assessment

This experiment is considered **successful**.

RCA Variant 1 was able to adapt to a complex GUI runtime environment while preserving its core architectural principles.

The experiment revealed important architectural boundaries and clarified how RCA should interact with framework-owned systems.

The results inform the next iteration of RCA refinement.

---

# 10. Next Experiment Domains

Future experiments should stress different system properties.

Recommended domains:

1. Network service (HTTP server)
2. File processing pipeline
3. Simulation engine
4. Actor/message system
5. Embedded hardware simulation

Each domain stresses different architectural assumptions.

---

# Summary

RCA Variant 1 demonstrated resilience when applied to a GUI framework domain.

While framework-owned loops and runtime constraints required adaptation, the core principles of RCA — deterministic execution, engine-owned state mutation, and logical task sequencing — remained valid.

The experiment confirmed that RCA can operate effectively as a **core architecture beneath external runtime frameworks**, provided that clear message boundaries and projection models are used.