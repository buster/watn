# ADR-0002: Streaming-first response

- **Status:** accepted
- **Date:** 2024-12-01
- **Decision-makers:** architect

## Context and Problem Statement

LLM responses can take seconds to generate. Users should see output
progressively. How should the tool handle response streaming?

## Decision Drivers

- Interactive users expect immediate feedback
- Scripts may want the complete response at once
- SSE streaming is the OpenAI-standard mechanism

## Considered Options

- **Streaming by default** — always request SSE, render tokens as they arrive
- **Buffered by default** — wait for complete response, then print
- **Configurable** — user chooses streaming vs. buffered

## Decision Outcome

Chosen: **Streaming by default**, with a config toggle for non-streaming.

## Consequences

- Good: immediate user feedback
- Good: works with pipes (output appears as tokens arrive)
- Bad: requires SSE parsing on the HTTP response stream
- Neutral: non-streaming mode is a single flag in the SSE parser

## Confirmation

E2E scenarios verify incremental output and metadata after completion.
