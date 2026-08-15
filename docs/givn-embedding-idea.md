# Handover-Plan: `givn spec` für semantische Duplikatprüfung

## Ziel

`givn` soll Cucumber-/Gherkin-Features lokal indexieren und neue oder geänderte Szenarien vor dem Hinzufügen prüfen.

Die Prüfung soll:

- **exakte Duplikate sicher blockieren**,
- **wahrscheinliche semantische Duplikate erkennen**,
- fachlich ähnliche, aber bewusst unterschiedliche Szenarien als `related` markieren,
- Step Definitions und gebundene Parameter als Zusatzsignal berücksichtigen,
- vollständig lokal funktionieren — ohne Vector DB oder Netzwerk zur Laufzeit nach dem initialen Modell-Setup.

Die vorhandenen Specs enthalten technisch präzise Unterschiede wie `[DONE]` vs. EOF, Exit Status `0` vs. `3`, positive vs. negative Assertions und TTY vs. non-TTY. Diese Unterschiede dürfen nicht durch einen reinen Embedding-Score nivelliert werden.

---

# 1. Scope und Nicht-Ziele

## In Scope

- Rekursives Einlesen von `.feature`-Dateien.
- Extraktion von `Feature`, Tags, `Scenario` / `Scenario Outline` und Gherkin-Steps.
- Persistenter lokaler Embedding-Index.
- Lokale Embeddings mit `fastembed-rs`.
- Cosine-Similarity über den gesamten lokalen Bestand.
- Exakte Duplikaterkennung über normalisierte Scenario-Signaturen.
- Erkennung wesentlicher Assertions und Widersprüche.
- Optionales Auflösen von Step Definitions gegen eine Registry.
- CLI-Befehle `index`, `search`, `check`, `add`, `duplicates`.
- Menschlich lesbare und JSON-Ausgabe.
- CI-taugliche Exit Codes.

## Nicht im ersten MVP

- Vector Database, ANN/HNSW-Index oder Server-Prozess.
- Automatisches Verstehen des vollständigen Codes einer Step Definition.
- Automatische semantische Klassifikation aller fachlichen Konzepte.
- Automatisches Umschreiben oder Generieren von Feature-Dateien.
- Single-binary-Verpackung von Modellgewichten und ONNX Runtime im ersten Schritt.

---

# 2. Architektur

```text
.feature-Dateien
    │
    ▼
Gherkin Parser
    │
    ├── Scenario-Modell
    ├── Normalisierung
    ├── exakter Fingerprint
    ├── Assertions / Fakten extrahieren
    └── Step-Definition-Resolver
             │
             ▼
       Embedding-Dokument
             │
             ▼
     fastembed-rs / lokales Modell
             │
             ▼
    .givn/spec-index.<format>
             │
             ▼
givn spec search / check / add / duplicates
```

Der Index soll unterhalb des Projektverzeichnisses liegen:

```text
.givn/
├── config.toml
├── spec-index.json       # zunächst lesbar/debugbar
└── spec-index.bin        # optional später für Performance
```

Für den MVP reicht JSON. Die Menge an Szenarien ist klein, und ein lesbarer Index erleichtert Debugging erheblich.

---

# 3. CLI-Vertrag

## Befehle

```bash
givn spec index
givn spec reindex
givn spec search "stream ends without a completion marker"
givn spec search --file candidate.feature
givn spec check --file candidate.feature
givn spec check --stdin
givn spec check --git-diff
givn spec add --file candidate.feature
givn spec duplicates
givn spec status
```

## Bedeutung

| Befehl | Verhalten |
|---|---|
| `givn spec index` | Erstellt oder aktualisiert den Index inkrementell. |
| `givn spec reindex` | Löscht den bisherigen Index und berechnet alle Embeddings neu. |
| `givn spec search` | Zeigt die semantisch ähnlichsten bestehenden Szenarien. |
| `givn spec check` | Prüft Kandidaten, verändert aber keine Feature-Datei. |
| `givn spec add` | Prüft einen Kandidaten und übernimmt ihn nur, wenn die Policy dies erlaubt. |
| `givn spec duplicates` | Findet mögliche Duplikate innerhalb des bestehenden Bestands. |
| `givn spec status` | Zeigt Modell, Indexversion, Szenarioanzahl und Indexzustand. |

## Wichtige Optionen

```bash
--file <path>
--stdin
--top-k <n>
--format text|json
--allow-similar
--reason <text>
--force
--no-step-definitions
--threshold-warn <float>
--threshold-block <float>
```

`--allow-similar` darf nur semantische Blockierungen überschreiben. Exakte Duplikate bleiben standardmäßig blockiert; dafür wäre bewusst `--force` nötig.

---

# 4. Datenmodell

## Scenario

```rust
struct Scenario {
    id: String,
    feature_name: String,
    title: String,
    tags: Vec<String>,

    source_path: String,
    source_line: usize,

    steps: Vec<Step>,
    normalized_document: String,
    fingerprint: String,

    facts: ScenarioFacts,
    resolved_steps: Vec<ResolvedStep>,
}
```

```rust
struct Step {
    keyword: StepKeyword, // Given, When, Then, And, But
    text: String,
    line: usize,
}
```

`And` und `But` müssen beim Parsen auf den vorhergehenden semantischen Typ aufgelöst werden. Ein `And` nach `Then` ist also semantisch ebenfalls `Then`.

## Indexeintrag

```rust
struct IndexedScenario {
    scenario: Scenario,
    embedding: Vec<f32>,

    content_hash: String,
    model_id: String,
    normalization_version: u32,
    indexed_at: String,
}
```

Der `content_hash` wird aus Modell-ID, Normalisierungs-Version und normalisiertem Dokument erzeugt. Ändert sich einer dieser Werte, muss das Embedding neu berechnet werden.

---

# 5. Embedding-Repräsentation

Es wird **ein Szenario pro Dokument** eingebettet, nicht eine Datei und nicht einzelne Zeilen.

Beispiel:

```text
Feature: Incremental provider output
Tags: e2e
Scenario: EOF without DONE is a truncated stream

Given: a streaming provider flushes valid content "printf truncated"
Given: the provider closes cleanly without sending [DONE]
When: I run watn "detect a truncated stream"
Then: stdout should contain "printf truncated"
Then: stderr should contain "network error"
Then: stderr should not contain successful model metadata
Then: stderr should not contain "Execute now? [Y/n]"
Then: the exit status should be 3
```

## Normalisierung

Normalisieren:

- Whitespace vereinheitlichen.
- Gherkin-Keywords kanonisieren.
- Zufällige IDs, UUIDs, temporäre Pfade und Zeitstempel maskieren.
- Wiederholte Leerzeichen und Zeilenumbrüche bereinigen.

Nicht pauschal normalisieren:

- Exit Codes.
- CLI-Flags (`-x`, `-v`, `-2`).
- `[DONE]`.
- Modellnamen, wenn sie fachlich relevant sind.
- Erwartete stdout-/stderr-Literale.
- Werte wie `minimal`, `high`, `off`, `bogus`.
- Positive und negative Assertions.

---

# 6. Step Definitions als Zusatzsignal

Step Definitions werden **nicht als kompletter Code eingebettet**.

Stattdessen wird pro Step versucht, eine Definition aufzulösen und strukturiert zu speichern:

```rust
struct ResolvedStep {
    step_text: String,
    keyword: StepKeyword,

    definition_id: Option<String>,
    pattern: Option<String>,
    captures: Vec<String>,

    role: Option<StepRole>,
}
```

Beispiele:

```text
the exit status should be 3
→ assert_exit_status(expected_status = 3)

stderr should contain "network error"
→ assert_stderr_contains(expected_text = "network error")

stderr should not contain successful model metadata
→ assert_stderr_not_contains(expected_text = "successful model metadata")
```

## Step-Definition-Registry

Für den MVP soll die Zuordnung konfigurierbar sein, nicht durch Parsing fremden Testcodes erfolgen.

Beispiel `.givn/config.toml`:

```toml
[[step_definition]]
id = "assert_exit_status"
pattern = "^the exit status should be (\\d+)$"
role = "outcome.exit_status"

[[step_definition]]
id = "assert_stdout_contains"
pattern = "^stdout should contain \"(.+)\"$"
role = "outcome.stdout.contains"

[[step_definition]]
id = "assert_stderr_contains"
pattern = "^stderr should contain \"(.+)\"$"
role = "outcome.stderr.contains"

[[step_definition]]
id = "assert_stderr_not_contains"
pattern = "^stderr should not contain (.+)$"
role = "outcome.stderr.not_contains"
```

Die Registry kann später automatisch aus dem verwendeten Cucumber-Framework abgeleitet werden. Das gehört nicht in den MVP.

---

# 7. Fakten- und Widerspruchserkennung

## Zu extrahierende Fakten

```rust
struct ScenarioFacts {
    expected_exit_codes: Vec<i32>,

    stdout_contains: Vec<String>,
    stdout_not_contains: Vec<String>,

    stderr_contains: Vec<String>,
    stderr_not_contains: Vec<String>,

    terminal_contains: Vec<String>,
    terminal_not_contains: Vec<String>,

    cli_flags: Vec<String>,
    command_names: Vec<String>,

    literals: Vec<String>,
    markers_present: Vec<String>,
    markers_absent: Vec<String>,

    environment: Vec<String>, // tty, non_tty, piped
}
```

Der MVP darf dazu erst Regex-/Pattern-basierte Heuristiken verwenden. Die Step-Definition-Registry soll diese im Laufe der Implementierung zunehmend ersetzen.

## Harte Widersprüche

Bei mindestens einem harten Widerspruch darf ein Treffer nicht automatisch als `probable_duplicate` blockiert werden.

| Widerspruch | Beispiel |
|---|---|
| Unterschiedlicher Exit Code | `0` vs. `3` |
| Positive vs. negative Assertion | `stdout contains X` vs. `stdout does not contain X` |
| Gegensätzliche Protokollbedingung | `[DONE]` vorhanden vs. `[DONE]` fehlt |
| Gegensätzlicher Ausgangszustand | Config existiert vs. keine Config vorhanden |
| Unterschiedliche Umgebung | TTY vs. non-TTY |
| Unterschiedliche Ausführungsart | `-x` vs. kein `-x` |
| Unterschiedliches Ergebnis | Erfolg vs. erwarteter Netzwerk-/Policy-/I/O-Fehler |

Beispiel: Ein erfolgreicher `[DONE]`-Stream und ein EOF ohne `[DONE]` bleiben semantisch nahe, müssen aber als `related_scenario`, nicht als Duplikat gelten.

---

# 8. Klassifikation und Policy

## Klassifikationen

| Status | Bedingung | Standardaktion |
|---|---|---|
| `exact_duplicate` | Identischer Fingerprint | Blockieren |
| `probable_duplicate` | Hohe semantische, strukturelle und Ergebnis-Ähnlichkeit; keine Widersprüche | Blockieren |
| `related_scenario` | Hohe semantische Nähe, aber relevante Unterschiede/Widersprüche | Warnen bzw. anzeigen |
| `distinct` | Kein relevanter ähnlicher Treffer | Zulassen |

## Bewertungslogik

Der Embedding-Score ist ein Retrieval-Signal: Er identifiziert Kandidaten für die eigentliche Prüfung.

```rust
struct Similarity {
    embedding: f32,
    step_definition_overlap: f32,
    argument_similarity: f32,
    outcome_similarity: f32,
    contradiction_count: usize,
}
```

Vorgeschlagene Entscheidungsregel:

```rust
let probable_duplicate =
    embedding >= config.thresholds.block_embedding
    && step_definition_overlap >= config.thresholds.block_step_overlap
    && outcome_similarity >= config.thresholds.block_outcome
    && contradiction_count == 0;
```

Startwerte — erst nach Kalibrierung verbindlich machen:

```toml
[spec.thresholds]
warn_embedding = 0.80
block_embedding = 0.91
block_step_overlap = 0.80
block_outcome = 0.85
```

Diese Werte müssen gegen reale Beispiele aus dem Repository kalibriert werden; sie sind nicht modellübergreifend gültig.

---

# 9. Ausgabe und Exit Codes

## Textausgabe

```text
BLOCKED  Scenario: EOF without DONE reports a network error

Best match: EOF without DONE is a truncated stream
Location: features/incremental-sse-rendering.feature:93

Embedding similarity:      0.96
Step-definition overlap:   0.92
Outcome similarity:        1.00

Reasons:
  ✓ same stream-ending condition
  ✓ same missing [DONE] condition
  ✓ same expected output/error behavior
  ✓ same exit status: 3

Use --allow-similar --reason "<reason>" to accept intentionally.
```

## JSON-Ausgabe

```json
{
  "status": "probable_duplicate",
  "exit_code": 2,
  "scenarios": [
    {
      "title": "EOF without DONE reports a network error",
      "matches": [
        {
          "classification": "probable_duplicate",
          "path": "features/incremental-sse-rendering.feature",
          "line": 93,
          "title": "EOF without DONE is a truncated stream",
          "scores": {
            "embedding": 0.96,
            "step_definition_overlap": 0.92,
            "outcome_similarity": 1.0
          },
          "contradictions": []
        }
      ]
    }
  ]
}
```

## Exit Codes

| Code | Bedeutung |
|---:|---|
| `0` | Akzeptiert; keine blockierende Ähnlichkeit |
| `1` | Technischer Fehler |
| `2` | Exaktes oder wahrscheinliches Duplikat; blockiert |
| `3` | Ähnliches Szenario gefunden; Policy verlangt Bestätigung |
| `4` | Ungültiges oder nicht unterstütztes Gherkin |

---

# 10. Implementierungsphasen

## Phase 0 — Repository-Analyse

**Ziel:** Bestehende Architektur und Test-Framework verstehen.

Aufgaben:

- CLI-Struktur, Argumentparser und Konfigurationskonzept prüfen.
- Herausfinden, welches Cucumber-/Gherkin-Framework und welche Sprache für Step Definitions verwendet werden.
- Speicherorte der `.feature`-Dateien und Step Definitions identifizieren.
- Bestehende Error-/Exit-Code-Konventionen übernehmen.
- Prüfen, ob es bereits einen Projekt-Cache-Ordner gibt.
- Ein kurzes Architecture Decision Record schreiben: Parser-Choice, Indexformat, Modell und Modell-Distribution.

**Ergebnis:** Ein umsetzbarer Architekturentscheid ohne Annahmen über bestehende `givn`-Strukturen.

---

## Phase 1 — Feature-Discovery und Scenario-Parser

**Ziel:** Alle Szenarien zuverlässig in ein internes Modell überführen.

Aufgaben:

- `.feature`-Dateien rekursiv finden.
- Gherkin parsen; möglichst einen etablierten Rust-Gherkin-Parser einsetzen.
- Feature-Tags und Scenario-Tags zusammenführen.
- `Scenario` und `Scenario Outline` unterstützen.
- `And` / `But` auf ihren kontextuellen Step-Typ auflösen.
- Stabilen Identifier aus relativem Pfad, Szenariotitel und Startzeile erzeugen.
- Parserfehler mit Datei und Zeile ausgeben.

**Akzeptanzkriterien:**

- Alle bereitgestellten Feature-Beispiele werden ohne Informationsverlust eingelesen.
- Das Szenario *EOF without DONE is a truncated stream* enthält Exit Code `3`, `[DONE]` und alle stdout/stderr-Assertions.
- `givn spec index --dry-run` gibt Szenarioanzahl und Quellen aus.

---

## Phase 2 — Normalisierung, Fingerprints und Fakten

**Ziel:** Exakte Duplikate und wichtige Ergebnisunterschiede deterministisch erkennen.

Aufgaben:

- Kanonisches Embedding-Dokument erzeugen.
- Normalisierung mit versionierter Implementierung einführen.
- SHA-256-Fingerprint des normalisierten Scenario-Dokuments erzeugen.
- Heuristiken für Exit Status, stdout/stderr, Terminal, CLI-Flags, `[DONE]`, TTY/non-TTY implementieren.
- Widerspruchserkennung implementieren.

**Akzeptanzkriterien:**

- Identische Szenarien mit anderem Whitespace erzeugen denselben Fingerprint.
- Unterschied `exit status 0` vs. `exit status 3` wird als Widerspruch erkannt.
- `stderr should contain X` vs. `stderr should not contain X` wird als Widerspruch erkannt.
- `[DONE]` vs. `without [DONE]` wird als Widerspruch erkannt.

---

## Phase 3 — Lokale Embeddings und Index

**Ziel:** Szenarien persistent lokal einbetten und durchsuchen.

Aufgaben:

- `fastembed-rs` integrieren.
- Modell über Konfiguration auswählbar machen.
- Für gemischte Deutsch-/Englisch-Spezifikationen ein mehrsprachiges Modell verwenden.
- Modellcache konfigurierbar machen.
- Cosine Similarity implementieren.
- JSON-Index lesen/schreiben.
- Inkrementelles Indexing anhand von `content_hash` implementieren.
- Veraltete Einträge gelöschter Szenarien entfernen.
- `givn spec index`, `reindex`, `status` implementieren.

**Akzeptanzkriterien:**

- Ein zweiter `givn spec index` ohne Änderungen berechnet keine Embeddings neu.
- Eine geänderte Spec berechnet nur das betroffene Szenario neu.
- `givn spec search` liefert semantisch passende Treffer aus dem Bestand.
- Der Index funktioniert ohne Vector DB.

---

## Phase 4 — `search`, `check` und Klassifikation

**Ziel:** Kandidaten gegen den Bestand prüfen und nachvollziehbar klassifizieren.

Aufgaben:

- Top-$k$ Ähnlichkeitssuche implementieren.
- Ergebnis-Score und Klassifikation berechnen.
- Harten Widerspruch als Sperre gegen automatische Duplikatklassifikation umsetzen.
- Text- und JSON-Renderer implementieren.
- CI-Exit-Codes implementieren.
- `givn spec search` und `givn spec check` implementieren.

**Akzeptanzkriterien:**

- Ein absichtlich identischer Kandidat wird als `exact_duplicate` blockiert.
- Eine Paraphrase mit identischen Ergebnissen wird als `probable_duplicate` blockiert oder gewarnt.
- Erfolgsfall mit `[DONE]` und Fehlerfall ohne `[DONE]` werden als `related_scenario` ausgegeben.
- `--format json` ist stabil und für Automatisierung geeignet.

---

## Phase 5 — Step-Definition-Registry

**Ziel:** Ähnlichkeit anhand der tatsächlichen Testsemantik präzisieren.

Aufgaben:

- Konfigurierbare Pattern-Registry laden.
- Step-Texte gegen Pattern auflösen.
- Captures speichern und typisieren, soweit möglich (`int`, String, Flag).
- Rollen wie `outcome.exit_status` oder `outcome.stderr.contains` auswerten.
- Step-Definition-Overlap und Argument-Ähnlichkeit in den Score aufnehmen.
- Bei unbekannten Steps robust degradieren: keine Fehlermeldung, nur fehlendes Zusatzsignal.

**Akzeptanzkriterien:**

- `the exit status should be 0` und `the exit status should be 3` lösen dieselbe Definition mit unterschiedlichen Argumenten auf.
- Gleiche Step Definition mit unterschiedlichen zentralen Argumenten wird nicht als identisches Ergebnis bewertet.
- Fehlende Registry-Einträge verhindern weder `index` noch `check`.

---

## Phase 6 — `add`, Overrides und Auditierbarkeit

**Ziel:** Policy-gesteuertes Hinzufügen implementieren.

Aufgaben:

- `givn spec add --file …` implementieren.
- Vor dem Schreiben immer `check` ausführen.
- Bei mehreren Szenarien transaktionales Verhalten: Standardmäßig keine Teilübernahme.
- `--partial` optional ergänzen.
- `--allow-similar --reason` für bewusste Übernahmen.
- Override-Grund protokollieren, z. B. `.givn/overrides.jsonl`.
- Exakte Duplikate standardmäßig auch mit `--allow-similar` blockieren.

**Akzeptanzkriterien:**

- Blockierte Szenarien verändern den Zielbestand nicht.
- Ein Override wird mit Zeitpunkt, Grund, Kandidat und ähnlichstem Treffer gespeichert.
- Ein erfolgreicher Add-Vorgang aktualisiert den lokalen Index.

---

# 11. Teststrategie

## Unit-Tests

- Normalisierung.
- Fingerprint-Bildung.
- Faktenextraktion.
- Widerspruchserkennung.
- Cosine Similarity.
- Pattern-Auflösung und Capture-Extraktion.
- Klassifikationslogik.

## Fixture-Tests

Die vorhandenen Features als feste Fixtures verwenden, insbesondere:

- `auto-init-config.feature`
- `incremental-sse-rendering.feature`
- `reasoning-policy.feature`

Mindestfälle:

| Fall | Erwartung |
|---|---|
| Identisches Scenario mit anderem Whitespace | `exact_duplicate` |
| `[DONE]`-Erfolg vs. EOF ohne `[DONE]` | `related_scenario` |
| Mid-stream failure vs. EOF without DONE | wahrscheinlich `related_scenario`; nicht zwangsläufig Duplikat |
| Gleiches Scenario mit nur umformuliertem Titel | `probable_duplicate` |
| `exit 0` vs. `exit 3` | keine automatische Duplikatblockierung |
| `minimal` vs. `bogus` in Reasoning-Szenarien | relevante Parameterdifferenz erhalten |

## Golden-Tests

Für `--format json` Golden Files verwenden. Dadurch wird das CLI-Ausgabeformat für CI und andere Tools stabil gehalten.

---

# 12. Konfiguration: initialer Vorschlag

```toml
[spec]
features_dir = "features"
index_path = ".givn/spec-index.json"
top_k = 5
embedding_model = "multilingual-e5-small"
normalization_version = 1

[spec.thresholds]
warn_embedding = 0.80
block_embedding = 0.91
block_step_overlap = 0.80
block_outcome = 0.85

[spec.policy]
block_exact_duplicates = true
require_override_reason = true
compare_step_definitions = true
compare_tags = true

[spec.normalization]
mask_uuids = true
mask_timestamps = true
mask_temp_paths = true
preserve_numbers = true
preserve_quoted_literals = true
```

Die Schwellenwerte müssen nach Einführung mit einem kleinen gelabelten Set aus echten Duplikaten, ähnlichen Gegenbeispielen und unabhängigen Szenarien kalibriert werden.

---

# 13. Definition of Done

Die Erweiterung ist fertig, wenn:

- `givn spec index`, `search`, `check`, `add`, `duplicates` und `status` verfügbar sind.
- Der Index lokal und inkrementell arbeitet.
- Exakte Duplikate zuverlässig blockiert werden.
- Embeddings semantische Nachbarn finden.
- Widersprüchliche Outcomes automatische Duplikatblocks verhindern.
- Step Definitions per Registry als strukturierte Zusatzinformation einfließen.
- Text- und JSON-Ausgabe inklusive stabiler Exit Codes existieren.
- Die bereitgestellten Streaming-, Konfigurations- und Reasoning-Features als Fixtures getestet werden.
- CI kann mit `givn spec check --git-diff --format json` laufen.
