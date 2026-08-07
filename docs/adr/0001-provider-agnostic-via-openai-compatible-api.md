# ADR-0001: Provider-agnostic via OpenAI-compatible API

- **Status:** accepted
- **Date:** 2024-12-01
- **Decision-makers:** architect

## Context and Problem Statement

The tool must support multiple LLM providers (OpenAI, Groq, Together AI, local
servers, etc.). Hardcoding each provider's API shape creates maintenance burden
and limits extension. How can we support the widest range of providers with the
least code?

## Decision Drivers

- Minimise per-provider implementation cost
- Support the "long tail" of OpenAI-compatible proxies, local servers, and
  self-hosted models
- Avoid third-party provider SDKs as dependencies

## Considered Options

- **OpenAI-compatible wire protocol** — implement `/v1/chat/completions` once;
  any provider that speaks this protocol works
- **Provider-specific adapters** — one adapter per provider (Anthropic, Google,
  etc.) with its own request/response shape
- **Generic HTTP + templates** — user provides a template for request/response

## Decision Outcome

Chosen: **OpenAI-compatible wire protocol**. Implement the SSE-based streaming
protocol once. Users configure an endpoint URL, API key, and model name.
Non-OpenAI providers (Anthropic, Google) are future work via adapter trait.

## Consequences

- Good: one implementation covers OpenAI, Groq, Together AI, llama.cpp, vLLM
- Bad: Anthropic, Google, and other non-compatible APIs require a separate adapter
- Neutral: the `Provider` trait exists for future adapters

## Confirmation

The Provider trait has exactly one implementation. Adding a second adapter
requires implementing the trait for a new type.
