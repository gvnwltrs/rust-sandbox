
# Purpose

A **standard experiment template** will do two important things for you:

1. **Force clarity** about what you're actually testing.
2. Allow multiple experiments to **accumulate into architectural evidence** instead of anecdotes.

The template below is designed specifically around how you’ve been thinking about RCA (Data → States → Threads → Tasks → Engine) while still being lightweight enough to use quickly.

Think of it as an **engineering lab notebook format for architecture experiments**.

---

```
# RCA Experiment Template

Experiment ID:
Date:
Domain:
RCA Variant:
Language / Environment:

---

# 1. Objective

What architectural question is this experiment trying to answer?

This should be one or two sentences describing **what pressure or domain is being tested**.

Example:

> Evaluate how RCA Variant 1 behaves when interacting with a GUI framework that owns its own runtime loop.

---

# 2. Hypothesis

State the expectation **before running the experiment**.

This helps prevent hindsight bias.

Example:

> RCA should be able to maintain control of system state mutation even when a framework owns the runtime loop.

RCA Variant Rationale

Selected Variant:
Why selected:
Expected fit:
Observed fit:
Alternative variant(s) worth testing:

---

# 3. Domain Characteristics

Describe the important traits of the domain being tested.

Examples:

* event-driven
* hardware interaction
* GUI rendering
* networking
* long-running processes
* blocking IO
* real-time constraints

This helps identify **what architectural pressure the domain introduces**.

---

# 4. Experimental Setup

Describe the environment and structure.

Language / tools used
Libraries / frameworks used
Architecture variant used

Example:

Rust application using:

* `eframe`
* `egui`

Minimal GUI notepad implemented using RCA Variant 1.

---

# 5. RCA Configuration

Document how RCA was applied in this experiment.

### Data Layer

What data structures represented apex data?

---

### State Model

What states were used?

Example:

```text
Init
Running
Failure
Degraded
Shutdown
```
```

---

### Threads

Logical threads used:

Example:

```text
Main logical thread
GUI runtime thread
```
```

---

### Tasks

Major tasks implemented.

Example:

* display update
* performance monitoring
* text input handling

---

### Engine

Describe how the loop executed.

Example:

Single RCA engine loop executing task collections.

---

# 6. Architectural Pressure Encountered

Document where the domain pushed against the architecture.

Examples:

* framework loop ownership
* thread ownership
* state mutation conflicts
* blocking IO
* asynchronous events

Each pressure should be described clearly.

---

# 7. Adaptations Applied

Describe what architectural changes or workarounds were implemented.

Examples:

* message channels between threads
* display snapshot projection
* thread boundary introduced
* adapter layer added

This section records **how the architecture responded**.

---

# 8. Observed Behavior

What actually happened during execution?

Examples:

* architecture behaved as expected
* unexpected behavior occurred
* framework constraints forced adaptation
* deterministic behavior preserved

This should be **observational**, not interpretive.

---

# 9. Validated RCA Principles

Document which architectural assumptions held true.

Examples:

* deterministic execution maintained
* engine-owned state mutation preserved
* logical thread model still valid
* task sequencing clear

---

# 10. Stressed RCA Principles

Document which assumptions were challenged.

Examples:

* outer loop ownership
* single-thread assumptions
* framework compatibility

---

# 11. Architectural Discoveries

This is the **most important section**.

Record new insights that emerged.

Examples:

* frameworks should be treated as runtime boundaries
* GUI should render projection models instead of core state
* input should be treated as events

---

# 12. Open Questions

What remains unresolved?

Examples:

* can logical threads replace OS threads in this domain?
* should RCA include a formal adapter layer?
* should RCA define a message/event standard?

---

# 13. Outcome Assessment

Classify the experiment result.

Example categories:

Success
Partial Success
Failure
Inconclusive

Explain briefly why.

---

# 14. Architectural Implications

What should be considered for RCA evolution?

Examples:

* add boundary adapter pattern
* define event protocol
* formalize snapshot model

---

# 15. Next Experiment Candidates

List domains that would stress RCA differently.

Examples:

* HTTP server
* streaming pipeline
* actor system
* embedded device driver
* distributed service

---

# Optional Section: Metrics (if relevant)

If performance or behavior was measured.

Examples:

* loop throughput
* task latency
* message round-trip time
* thread switching overhead
```

---

# Why this template works well

This format separates **four critical things** that often get mixed together:

1. What you **expected**
2. What **actually happened**
3. What **held up**
4. What **changed**

That separation is extremely valuable when evolving an architecture.

---

# Useage 

Numbering experiments like this:

```text
RCA-EXP-001  GUI Framework Test
RCA-EXP-002  HTTP Server Test
RCA-EXP-003  File Pipeline Test
RCA-EXP-004  Simulation Engine
RCA-EXP-005  Actor System
```

Over time it'll build something like:

```text
RCA Evidence Log
```

which becomes **very powerful documentation**.

---
