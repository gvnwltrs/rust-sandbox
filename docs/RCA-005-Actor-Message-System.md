Good enough. The only issue is formatting: you’re printing the `String` with debug formatting somewhere, so the quotes and `\n` escapes are showing literally.

Use normal display printing, not debug printing.

If your output line is something like:

```rust
println!("{output:?}\n");
```

change it to:

```rust
println!("{output}\n");
```

or if you’re printing from `display.body` directly:

```rust
println!("{}", display.body);
println!();
```

Your actual actor flow is working.

# Experiment 5 findings report

## RCA Experiment 5 — Actor / Message System

Author: Gavin Walters
Date: 2026-03-11
Architecture Variant: **RCA-E**

## Objective

The goal of Experiment 5 was to test whether RCA can represent **distributed logical execution units** using an actor/message style domain.

This experiment introduced pressure from:

* logical actor isolation
* message passing
* decentralized behavior
* queued event dispatch
* scheduler-like engine behavior

The key architectural question was:

> Can RCA represent actor-like systems without collapsing entirely into an unnatural centralized execution model?

## Domain

A minimal actor/message pipeline was implemented with three actors:

* Actor A
* Actor B
* Actor C

And three message kinds:

* `Start`
* `Ping`
* `Done`

Message flow:

```text
Actor A receives Start
Actor A sends Ping to Actor B
Actor B sends Ping to Actor C
Actor C sends Done to Actor A
Actor A handles Done
System complete
```

## RCA structure

The experiment used the RCA-E structure:

```text
Data → Constraints → Cells → Threads → Engine
```

Where:

* `Data` remained the apex context
* actor/message models lived in support state (`SystemData`)
* cells processed one message per cycle
* a single logical thread handled per-message execution
* the engine owned queue progression, actor updates, and completion logic

## Cell pipeline

The message-processing thread used three cells:

### `LoadNextMessage`

Accepted the current message for processing.

### `DispatchActor`

Matched the target actor and message kind, and produced the next textual event result.

### `RenderEvent`

Converted the event result into a committed output artifact.

This yielded a per-cycle flow like:

```text
Message → Dispatch → Render → Engine regulation
```

## Runtime result

The experiment executed successfully, producing the expected actor/message sequence:

```text
Actor A handled Start
Actor A sent Ping to Actor B

Actor B handled Ping
Actor B sent Ping to Actor C

Actor C handled Ping
Actor C sent Done to Actor A

Actor A handled Done
System complete
```

## Key findings

### 1. RCA-E can represent actor-style message flow at small scale

RCA-E handled discrete message processing cleanly when each dispatch cycle was treated as a bounded event.

This means RCA-E can model actor/message behavior at least in a **logical** actor sense, even without real concurrency.

### 2. The architecture remained understandable by treating actors as support-state units

Actors did not need to become real runtime owners. Instead, they could be represented as support models in `SystemData`, while the engine managed queue progression.

This kept the experiment small and made the architecture easier to reason about.

### 3. The engine became more scheduler-like than in previous experiments

This is the most important pressure point revealed.

Compared with HTTP or log processing, the engine now had to do more than regulate outputs. It also had to:

* pop the next message
* route progression through the queue
* update actor statistics
* enqueue the next message
* determine completion

This pushed the engine closer to a **scheduler/dispatcher authority**.

### 4. Actor independence was only logical, not structural

This is the main limitation revealed by the experiment.

Although the system modeled actors conceptually, execution still passed through a single centralized RCA thread:

```text
LoadNextMessage -> DispatchActor -> RenderEvent
```

So the architecture supported **logical decentralization**, but not true structural decentralization.

That is an important distinction.

### 5. RCA-E still worked, but the fit was weaker than log processing

Compared with Experiment 3, this domain felt more strained.

Why:

* actors suggest local ownership of behavior
* RCA still centralized most behavior through one dispatch funnel
* the engine absorbed more orchestration responsibility

So the architecture still functioned, but the fit felt less natural than pure event-transformation pipelines.

## Strengths revealed

* RCA-E can process discrete queued messages cleanly
* actor/message systems can be approximated using support-state actors plus centralized dispatch
* cell structure still works for per-message transformation stages
* engine-regulated output remains consistent

## Pressure points revealed

* engine becomes more scheduler-heavy
* actor autonomy is more simulated than real
* decentralized domains push RCA toward centralized orchestration
* the thread abstraction can start to feel like a forced per-message funnel rather than a naturally decentralized runtime

## Conclusion

Experiment 5 showed that RCA-E can represent **small actor/message systems**, but primarily as a **centralized event-dispatch architecture that simulates actor-like behavior**, rather than as a truly distributed actor model.

This means RCA can model actor/message flow at a logical level, but the experiment also exposed a likely architectural boundary:

> RCA appears stronger when processing discrete events through regulated transformation pipelines than when trying to represent many independently behaving execution units.

That does not mean RCA fails in actor-style systems, but it suggests that this is a more strained fit than HTTP, log processing, or deterministic simulation.

## Current fit map update

### Stronger fits so far

* HTTP request handling
* log/event processing
* deterministic simulation loops

### More strained fit so far

* actor/message systems with decentralized behavior expectations

---