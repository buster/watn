# Arc42 Assessment: fix-review-response-fallback

| # | Chapter | Affected | Reason |
|---:|---|---|---|
| 1 | Introduction and goals | No | No new goal; the review experience keeps its comprehension goal. |
| 2 | Architecture constraints | No | Existing constraints (no alternate screen, stdout separation, no invented purposes) are unchanged. |
| 3 | Context and scope | No | No new external interface or consumer. |
| 4 | Solution strategy | No | The response contract stays versioned and structured. |
| 5 | Building block view | No | No new component; a recovery function is added inside the existing review response building block. |
| 6 | Runtime view | Yes | The review flow gains the payload recovery branch and the no-command `Unavailable` outcome. |
| 7 | Deployment view | No | No production service, binary, or deployment topology changes. |
| 8 | Cross-cutting concepts | Yes | Terminal text sanitization flattens line breaks and tabs so rendered values stay on one inline row; the provider prompt gains a no-fence instruction. |
| 9 | Architecture decisions | No | No durable decision; recovery stays inside the reviewed capability. No ADR is created or amended. |
| 10 | Quality requirements | Yes | QS-072 gains fenced, prose-wrapped, and command-recovery response cases. |
| 11 | Risks and technical debt | Yes | R-072 mitigation gains payload-shape tolerance and command recovery. |
| 12 | Glossary | No | Candidate, Stage purpose, and purpose-unavailable keep their recorded meanings. |

## Status

STATUS: DONE
