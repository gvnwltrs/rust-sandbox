

## RCA Family Overview

```mermaid
flowchart LR 
    RCA[Regulated Cell Architecture<br/>RCA Family]

    RCA --> CORE[Shared RCA Core]
    RCA --> S[RCA-S<br/>Sequential]
    RCA --> E[RCA-E<br/>Event-Driven]
    RCA --> A[RCA-A<br/>Async / Concurrency]

    CORE --> C1[Apex Data]
    CORE --> C2[State Model]
    CORE --> C3[Cells]
    CORE --> C4[Tasks]
    CORE --> C5[Engine Mutation Authority]
    CORE --> C6[Thread / Execution Lane Concept]

    S --> S1[Deterministic ordered execution]
    S --> S2[Single loop or staggered logical lanes]
    S --> S3[Best fit: embedded, low-level, simulation]

    E --> E1[Reactive to external events]
    E --> E2[Event intake drives execution]
    E --> E3[Best fit: GUI, requests, interrupts]

    A --> A1[Multiple active execution contexts]
    A --> A2[Message passing / channel boundaries]
    A --> A3[Best fit: GUI/runtime split, services, workers]
```


## Shared RCA Core 

```mermaid
flowchart LR
    D[Data<br/>Apex State]
    ST[States]
    TH[Threads / Execution Lanes]
    CE[Cells]
    TA[Tasks]
    EN[Engine]
    MU[Mutate Apex State]

    D --> ST
    ST --> TH
    TH --> CE
    CE --> TA
    TA --> EN
    EN --> MU
    MU --> D
```

## RCA Iteration Model

```mermaid
flowchart TD
    LOOP[Loop Iteration]
    STEP[Step Current Thread]
    EXEC[Execute Current Cell]
    TASK[Run Task / Activation Function]
    OUT[Produce CellData + TaskOutput]
    MUT[Mutate Apex Data or Handoff]
    NEXT[Advance to Next Iteration]

    LOOP --> STEP
    STEP --> EXEC
    EXEC --> TASK
    TASK --> OUT
    OUT --> MUT
    MUT --> NEXT
    NEXT --> LOOP
```

## RCA-S Variant

This shows the sequential variant as the most deterministic and explicit form.

```mermaid
flowchart TD
    MAIN[Main Loop]
    T1[Logical Thread 1]
    T2[Logical Thread 2]
    T3[Logical Thread N]

    MAIN --> T1
    MAIN --> T2
    MAIN --> T3

    T1 --> C11[Cell]
    C11 --> C12[Cell]
    C12 --> C13[Cell]

    T2 --> C21[Cell]
    C21 --> C22[Cell]

    T3 --> C31[Cell]
```

```mermaid
sequenceDiagram
    participant L as Main Loop
    participant T1 as Logical Thread 1
    participant T2 as Logical Thread 2

    L->>T1: step()
    T1->>T1: execute current cell
    T1->>T1: mutate / handoff
    L->>T2: step()
    T2->>T2: execute current cell
    T2->>T2: mutate / handoff
    L->>T1: next step()
```

## RCA-A Variant 

This shows the async/concurrency-oriented variant, where multiple active contexts exist but mutation authority still remains regulated.

```mermaid
flowchart TD
    IN1[Input Source A]
    IN2[Input Source B]
    IN3[Input Source C]

    A1[Active Context / Worker A]
    A2[Active Context / Worker B]
    A3[Active Context / Worker C]

    BUS[Message / Channel Boundary]
    ENG[RCA Engine Authority]
    MUT[Mutate Apex Data]
    PROJ[Projection / Output]

    IN1 --> A1
    IN2 --> A2
    IN3 --> A3

    A1 --> BUS
    A2 --> BUS
    A3 --> BUS

    BUS --> ENG
    ENG --> MUT
    MUT --> PROJ
```



## RCA-E Variant

This shows the event-driven variant, where events become the intake that trigger regulated execution.

```mermaid
flowchart TD
    EV[External Event]
    Q[Event Intake / Queue]
    SEL[Select Event]
    TH[Thread / Lane Selection]
    CE[Execute Cell]
    TA[Run Task]
    MU[Mutate Apex Data]
    OUT[Produce Updated Output / Projection]

    EV --> Q
    Q --> SEL
    SEL --> TH
    TH --> CE
    CE --> TA
    TA --> MU
    MU --> OUT
```

## Variant Selection Workflow

This documents how to think about choosing a variant by domain pressure.

```mermaid
flowchart TD
    START[New Experiment Domain]

    START --> Q1{Naturally ordered<br/>and deterministic?}
    Q1 -->|Yes| S[RCA-S]
    Q1 -->|No| Q2{Primarily reactive<br/>to external events?}

    Q2 -->|Yes| E[RCA-E]
    Q2 -->|No| Q3{Multiple active contexts<br/>or runtime separation required?}

    Q3 -->|Yes| A[RCA-A]
    Q3 -->|No| Q4{Can domain be simplified<br/>to ordered execution first?}

    Q4 -->|Yes| S
    Q4 -->|No| E
```

## GUI Experiment Interpretation

This is my current interpretation of what happened in the notepad experiment.

```mermaid
flowchart LR
    GUI[eframe / egui Runtime]
    IN[GUI Input Events]
    SNAP[Display Snapshot]
    RCA[RCA Engine Thread]
    DATA[Apex Data / State]
    TASKS[Cells / Tasks]
    MUT[Mutation Authority]

    GUI --> IN
    IN --> RCA
    RCA --> TASKS
    TASKS --> MUT
    MUT --> DATA
    DATA --> SNAP
    SNAP --> GUI
```

This is the key finding from the experiment:

* GUI runtime can remain a boundary

* RCA remains authoritative

* projected display models bridge the two

## Workflow Between Variants

This one shows the broader lifecycle I'm aiming for: choose variant, run experiment, extract findings, refine family.

```mermaid
flowchart TD
    D[Select Domain]
    V[Select RCA Variant]
    B[Build Minimal Experiment]
    R[Run Stress Test]
    F[Capture Findings]
    A[Assess Variant Fit]
    U[Update RCA Family Understanding]
    N[Choose Next Domain]

    D --> V
    V --> B
    B --> R
    R --> F
    F --> A
    A --> U
    U --> N
```

## Practical Domain-to-Variant Map

This is a quick documentation artifact for where the variants currently appear to fit best.

```mermaid
flowchart LR 
    S[RCA-S]
    E[RCA-E]
    A[RCA-A]

    S --> S1[Embedded Superloop]
    S --> S2[Drivers / Low-Level Utilities]
    S --> S3[Simulation Engine]
    S --> S4[Deterministic Pipelines]

    E --> E1[GUI]
    E --> E2[HTTP / Request Handling]
    E --> E3[Interrupt-Like Systems]
    E --> E4[Actor / Event Systems]

    A --> A1[GUI Runtime Split]
    A --> A2[Concurrent Services]
    A --> A3[Worker Pipelines]
    A --> A4[Boundary-Separated Systems]
```