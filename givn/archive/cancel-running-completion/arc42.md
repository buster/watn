# Arc42 Assessment: cancel-running-completion

## 12-row assessment

| # | Chapter | Impact | Note |
|---|---|---|---|
| 1 | 01 introduction-and-goals / 10 quality-requirements | No | Cancellation is an existing UX expectation, not a new goal or quality scenario; goals and quality requirements unchanged |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organisational constraints |
| 3 | 03 context-and-scope | No | No new external system or interface; Ctrl+C is a behaviour of the existing CLI conversation, not a new boundary |
| 4 | 04 solution-strategy | No | Strategy (blocking provider, synchronous callback) is preserved; the worker-thread watchdog is a tactical mechanism, recorded as ADR-0019 rather than a strategy change |
| 5 | 05 building-block-view | Yes | `OpenAICompatibleProvider` gains a shared interrupt flag and `Interrupted` abort; `src/main.rs` gains a worker-thread watchdog around the streaming call |
| 6 | 06 runtime-view | Yes | New scenario "Cancelling a running completion" with a worker/main-watchdog sequence |
| 7 | 07 deployment-view | No | No deployment/topology change |
| 8 | 08 crosscutting-concepts | Yes | Error handling: new `Interrupted` variant, exit table already listed 130; cancellation semantics and 500 ms grace documented |
| 9 | 09 architecture-decisions | Yes | New ADR-0019: worker thread plus bounded grace chosen because reqwest blocking cannot split connect and read timeouts |
| 10 | 10 quality-requirements | No | Cancellation bounded by grace is captured in the runtime and cross-cutting docs, not a new quality scenario |
| 11 | 11 risks-and-technical-debt | Yes | New R-056 (grace may cut final buffered bytes), R-057 (brief detached worker), TD-009 (async migration if the heuristic is unacceptable), and ADR-0019 consequence coverage |
| 12 | 12 glossary | No | No new domain terms |

## Status

STATUS: DONE