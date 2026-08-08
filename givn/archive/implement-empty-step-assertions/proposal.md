# Proposal: implement-empty-step-assertions

## Problem / Opportunity

Four step definitions in the test suite are empty — they accept parameters and
do nothing with them. The scenarios pass because the test framework only fails
on panics; no actual verification occurs. A reader who inspects the feature
file will be misled into believing the system is tested for these behaviours
when it is not:

- Whether a request is sent to the correct provider endpoint
- Whether the model list endpoint is queried at all
- Whether the request includes the expected Authorization header

The test infrastructure already provides everything needed to verify these
conditions (a mock HTTP server that records which requests it receives). The
step definitions simply need to check the recorded requests.

## Proposed Solution

The four empty steps are filled in to verify that the mock server received the
expected HTTP requests. The verification checks that requests reached the
expected endpoints (path + method), and where applicable that the expected
Authorization header was present.

No observable behaviour of the `watn` binary itself changes. The user-visible
change is that a failing scenario will now actually fail instead of silently
passing.

## Out of Scope

- Adding new scenarios or changing existing scenario text.
- Adding request logging to the `watn` binary.
- Changing how the test mock server is started or configured beyond what is
  needed to store mock handles for assertion.

## Open Questions

None.
