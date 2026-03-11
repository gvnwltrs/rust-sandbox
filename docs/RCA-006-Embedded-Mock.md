Good. That’s enough to close Experiment 6.

# Experiment 6 Findings Report

## RCA Experiment 6 — Embedded Hardware Mock (RCA-S)

Author: Gavin Walters
Date: 2026-03-11
Architecture Variant: **RCA-S**

## Objective

The goal of Experiment 6 was to test RCA in a small embedded-style control domain using mocked hardware components.

This experiment introduced pressure from:

* periodic control execution
* hardware-like state
* sampled inputs
* threshold-based decision logic
* device-style output effects

The key architectural question was:

> Does RCA naturally support embedded-style control systems where the engine owns the loop and commits hardware-facing state?

## Domain

A minimal mocked hardware system was implemented with:

* `Timer`
* `Adc`
* `Gpio`
* `Uart`

Behavior per tick:

1. sample ADC
2. evaluate against threshold
3. determine GPIO state
4. generate UART-style status output

Deterministic ADC input sequence:

```text
Tick 1 -> 200
Tick 2 -> 450
Tick 3 -> 800
Tick 4 -> 300
Tick 5 -> 900
```

Threshold:

```text
700
```

Expected behavior:

* low
* low
* high
* low
* high

## RCA structure

The experiment used the RCA-S structure:

```text
Data → Constraints → Cells → Threads → Engine
```

Where:

* `Data` remained the frozen apex context
* mocked hardware lived in support state (`SystemData`)
* cells performed staged control behavior
* one logical thread executed per tick
* the engine owned tick progression and committed hardware-facing state

## Cell pipeline

The thread used three cells:

### `ReadAdc`

Accepted the current ADC sample into the pipeline.

### `EvaluateThreshold`

Determined whether the ADC sample exceeded threshold.

### `RenderStatus`

Produced the final human-readable UART/status artifact.

Per-tick flow:

```text
ADC sample → threshold decision → rendered status → engine commit
```

## Runtime result

The experiment executed successfully and produced:

```text
ADC=200 | GPIO=LOW  | UART="NORMAL adc below threshold"
ADC=450 | GPIO=LOW  | UART="NORMAL adc below threshold"
ADC=800 | GPIO=HIGH | UART="ALERT threshold exceeded"
ADC=300 | GPIO=LOW  | UART="NORMAL adc below threshold"
ADC=900 | GPIO=HIGH | UART="ALERT threshold exceeded"
```

## Key findings

### 1. RCA-S fits embedded-style periodic control very well

This domain is a strong fit for RCA-S.

The architecture handled:

* periodic execution
* deterministic sequencing
* sampled input evaluation
* output commitment

without feeling forced.

### 2. Cells map naturally to firmware/control stages

The cell abstraction felt natural in this domain.

The stages:

* read input
* evaluate condition
* render/report output

are all recognizable embedded/control phases.

This suggests RCA cells are well suited for small control-loop stages and mock driver-like behavior.

### 3. Engine-owned commitment of hardware state feels correct

This experiment strongly supports the rule you refined earlier:

> the engine is the sole authority for meaningful state change.

The cells computed intermediate results, but the engine committed:

* GPIO state
* UART output
* tick progression
* durable support-state updates

That felt natural and aligned with the architecture.

### 4. Support-state hardware models worked cleanly

Mocked hardware components living in support state (`SystemData`) fit the architecture well.

There was no need to expand the frozen apex `Data` structure to represent hardware-specific models.

That again validates the split between:

* apex system context
* support/domain state

### 5. The fit was stronger than the actor/message system

Compared with Experiment 5, this domain felt substantially more natural.

Why:

* execution remained cyclical and centralized
* update stages were explicit
* state commitment belonged naturally to the engine
* no artificial scheduler pressure emerged

This makes embedded-style control a much stronger match for RCA than decentralized actor/message flow.

## Strengths revealed

* strong fit for deterministic control loops
* strong fit for driver/service-style staged behavior
* engine-owned hardware state commitment feels clean
* support-state hardware modeling fits without expanding apex `Data`
* thread abstraction works well as ordered control pipeline

## Pressure points revealed

* hardware output commitment still concentrates responsibility in the engine
* if hardware models become much richer, support-state retention may become more elaborate
* future interrupt-style behavior may pressure the current sequential model differently

## Conclusion

Experiment 6 showed that RCA-S is a strong fit for small embedded-style control systems.

The architecture handled:

* periodic execution
* explicit control stages
* deterministic update order
* engine-owned commitment of meaningful hardware state

with very little friction.

This experiment, together with the simulation engine experiment, suggests that RCA-S is especially well aligned with domains where:

* loop ownership is centralized
* behavior proceeds through explicit ordered stages
* final state commitment belongs to a regulating runtime authority

## Current fit map after Experiments 1–6

### Strong fits

* HTTP request handling (RCA-E)
* log/event processing (RCA-E)
* deterministic simulation loops (RCA-S)
* embedded hardware/control mock (RCA-S)

### Weaker / more strained fit

* GUI frameworks that own the main loop
* actor/message systems with stronger decentralization expectations

## Overall takeaway

At this stage, RCA appears strongest when the domain has these properties:

* explicit stage-based behavior
* regulated outputs
* centralized runtime ownership
* deterministic or bounded event flow
* clear distinction between computed artifacts and committed state

That is the strongest pattern across the experiments.

---
