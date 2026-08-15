# Givn 0.3 Refactor: Analyse und Entscheidungsstand

Date: 2026-08-14

Status: Ergebnis einer Diskussion zwischen Autor und Modell. Implementiert
ist nichts; das Dokument hält Kontext, Befunde, Begründungen und offene
Fragen fest.

## 1. Ausgangslage

### 1.1 Das Problem

watn (der Referenz-Konsument von givn) hat über mehrere Wochen eine
Spec-Basis akkumuliert, die strukturell redundant ist:

| Oberfläche | Anzahl | Ausführungsstatus |
|---|---:|---|
| Aktive Feature-Dateien | 25 | ausgeführt |
| Aktive Szenarien | 223 | ausgeführt |
| Archivierte Feature-Dateien | 26 | nicht ausgeführt (Evidence) |
| Archivierte Szenarien | 216 | historisch |
| Step-Dateien | 30 (inkl. mod.rs) | 29 Module registriert |
| Binding-Attribut-Deklarationen | 858 | global registriert |

Der Report `feature-step-overlap-report.md` dokumentiert: zwei exakte
Titel-Duplikate (F1, F2), drei Subset-Szenarien (F3, F5, F6), wiederholte
Abdeckung derselben Invarianten über bis zu sieben Feature-Familien
(F8–F12), duplizierte Step-Mechanics über Module (S1–S5) und 23 lange
Szenarien. Die Suite war trotzdem grün — Grün bewies nur, dass globale
Bindings registrierbar sind, nicht dass die Szenarien unabhängig sind.

### 1.2 Prozess-Ursachen (aus dem Report)

1. **Change-lokale Planung.** Reviews erzwingen 1:1-Zuordnung zwischen
   Change-Inventar und eigenen Szenarien, aber keine Ownership-Prüfung
   gegen den gesamten permanenten Baum.
2. **Additives Archive.** Archive verschiebt Artefakte und hängt permanente
   Specs an; stärkere Szenarien wurden addiert, ältere Subsets blieben aktiv.
3. **Entry-Point-Slicing.** Szenarien wurden nach Kommandos organisiert
   (`watn setup`, `watn provider`, `watn models`), nicht nach Invarianten —
   dieselbe Invariante wurde pro Kommando neu eingeführt.
4. **Globale Cucumber-Registrierung.** Um Binding-Kollisionen zu vermeiden,
   erfanden spätere Capability-Module neue Prosa statt zu wiederverwenden —
   Alias- und Helper-Duplikation als Nebenwirkung.
5. **Superset-Additionen ohne Cleanup.** Stärkere Szenarien kamen später
   dazu; die schwächeren blieben, weil die Arbeit als additive Abdeckung
   galt.
6. **Befunde wurden nie zum Gate.** Frühere Reviews erkannten Duplikate
   (change-spezifisch) — daraus entstand kein repository-weiter Check.

### 1.3 Die drei Vorschläge und ihre Ziel-Ebenen

- **DeepSeek-Pro-Plan**: Enforcement. Es gab kein Gate, das Duplikate
  blockiert. F1/F2 hatten identische Titel — ein normalisierter
  Textvergleich hätte gereicht. Inkrementelle Phasen (Lint-Scope-Fix →
  Overlap-Lint → supersedes → Ledger → Skill-Rewrites).
- **Terra-Pro-Plan**: Transaktionssemantik. Der Runner führt alte und neue
  Szenarien nebeneinander aus, Archive ist additiv, Merge-Identität ist
  der Titel, Garantien in Skills überzeichnen das Binary. Antwort:
  V2-Kontrakt mit Ankern, Registry, CAS, Journal.
- **Embedding-Idee** (Autor): Retrieval. Der Autor sieht ähnliche
  Szenarien nicht und schreibt deshalb neue. Antwort: lokaler
  Embedding-Index mit exakter Duplikat-Sperre und semantischer
  Ähnlichkeitsprüfung.

Das Verdict (qwen3.8) bestätigte beide Pläne als faktisch solide, mit
einer Korrektur: DeepSeek ist der bessere Ausführungsplan, Terra die
bessere Systemanalyse. Die Diskussion endete nicht beim Verdict, sondern
hinterfragte dessen ungeprüfte Prämisse (dass semantische Suche der
Engpass sei) und rekonstruierte die Entscheidungen neu.

### 1.4 Verifizierte Code-Befunde (Grundlage)

- `merge.rs:42-65` validiert nur gegen die **eine** Merge-Zieldatei
  (`givn/specs/<cap>/<cap>.feature`, geroutet per Capability-Tag).
  F1/F2 (gleiche Titel in verschiedenen Dateien) sind unsichtbar.
- `delta.rs:4-12`: DeltaOp = Added/Modified/Removed; kein supersedes.
- `merge.rs:77-84`: Modified/Removed ersetzen/löschen per Titel-Lookup.
- `delta.rs:80-85`: Archive strippt alle `@givn.*`-Tags.
- `archive.rs:110-150`: merged **alle** Delta-Dateien eines Changes
  (Mehr-Dateien-Änderung ist möglich), mit Backup-Rollback über den
  ganzen Specs-Baum bei Verify-Fehler.
- `features_runner.rs:150-165`: watn's Runner führt `givn/specs` plus
  die rohen Deltas aller aktiven Changes aus (`GIVN_ARCHIVE_ONLY`
  als einzige Ausnahme beim Archive-Verify).
- `lint.rs` kennt nur ParseError/Wip; Default-Lint rekurriert in
  `givn/archive` (empirisch: 51 Dateien = 25 aktiv + 26 archiviert).
- `@e2e`-Regel existiert in drei Wortlauten; im Skill-Template
  (`assets/skills/givn-spec/SKILL.md.tmpl`) stehen zwei gleichzeitig
  (Zeilen 69 und 79).

## 2. Entscheidungen

### 2.1 Identität: baumweit eindeutige Titel

**Problem.** Der Merge-Engine identifiziert Szenarien über (Titel,
Datei). Zwei Szenarien mit gleichem Titel in verschiedenen Dateien sind
mehrdeutig (F1/F2); Renames sind Identitätswechsel; parallele Edits am
selben Szenario sind nicht auflösbar (Lost-Update).

**Optionen.** (a) Terra's stabile Anker + Registry-Revisionen + CAS;
(b) erzwungene baumweite Titel-Eindeutigkeit.

**Begründung für (b).** Titel-Eindeutigkeit löst die Mehrdeutigkeit —
das einzige Identitätsproblem, das in watn tatsächlich belegt ist (F1/F2).
Lost-Update ist hypothetisch: parallele Arbeit passiert in separaten
Worktrees/Branches, ein beobachteter Vorfall existiert nicht. Terra's
Maschinerie wäre allein größer als der Rest des Vorhabens und löst ein
Problem ohne belegte Instanz.

**Konsequenzen.** Ein Rename ist ein Identitätswechsel: Offene
Referenzen (`@givn.supersedes`, Modified-Targets) brechen laut am
Archive ("target not found") — nicht still. Der Autor korrigiert den Tag.
Das wird akzeptiert.

**Zusatzargument.** Die Autoren sind LLM-Agenten, die Referenzen lesbar
in Delta-Dateien schreiben müssen. `@givn.supersedes "Bash widget runs…"`
ist im Diff les- und tippbar; ein Anker-UUID wäre ein Lookup-Schritt bei
jedem Schreiben und eine Copy-Paste-Fehlerquelle. Prosa-Identität passt
zum KI-getriebenen Workflow; unsichtbare IDs nicht.

### 2.2 Supersession ohne neuen Merge-Op (Option C)

**Problem.** Stärkere Szenarien müssen schwächere ablösen können
(Report: "supersedes ist ein First-Class-Op, atomar"). Ohne das bleibt
der F4-Mechanismus bestehen: additiv statt ersetzend.

**Optionen.** (a) `@givn.supersedes` nur innerhalb derselben Datei;
(b) Cross-File-Supersession als Merge-Op mit baumweiter Vorvalidierung;
(c) kein neuer Op — `removed` + `added` über zwei Delta-Dateien, Gates
prüfen das Ergebnis.

**Begründung für (c).** Die Korrektur während der Analyse: Cross-File
ist heute **schon** ausdrückbar — `archive.rs` merged alle Delta-Dateien
eines Changes, jede in ihre Capability-Datei; Backup-Rollback existiert.
Der Unterschied der Optionen ist nicht "möglich/unmöglich", sondern wo
die Kausalbeziehung "A ersetzt B" festgehalten wird: (a) verliert sie
über Dateigrenzen — genau dort, wo die Duplikate entstanden (F1/F2/F3
sind Cross-File); (b) macht sie zum Maschinenbeleg, kostet baumweite
Vorvalidierung und setzt die Eindeutigkeits-Migration voraus (ein
`@givn.supersedes "Titel"` wäre in der heutigen Welt mehrdeutig); (c)
lässt sie als Prosa in review.md, prüft das Ergebnis deterministisch.

(c) gewählt: keine Engine-Änderung, die Gates entstehen ohnehin, nichts
verfestigt sich — ein späterer supersedes-Tag bliebe als reine
Engine-Validierung offen.

**Akzeptierte Schwäche.** Der Net-Delta-Receipt meldet
`removed: 1, added: 1`, kann aber nicht beweisen, dass das Paar
zusammengehört. Der schwächste Durchsetzungspunkt im ganzen Konstrukt;
kompensiert durch die Dispositionstabelle mit gepinnten Suchtreffern
(2.5).

### 2.3 Dreistufige deterministische Prüfung

**Der prägende Einblick.** Wenn zwei Szenarien sehr ähnlich *aussehen*,
gibt es zwei Fälle: (1) sie decken dasselbe Verhalten ab → eines wird
gelöscht (welches, hängt von E2E-Coverage und Varianten ab); (2) sie
sind verschieden, aber schlecht benannt → jeder menschliche Reviewer
hätte dasselbe Problem → die Benamung ist der Fehler, die Titel müssen
so unterschiedlich werden, dass sie nicht mehr ähnlich klingen. Der Gate
muss nicht entscheiden, welcher Fall vorliegt — er meldet Ähnlichkeit,
der Autor klassifiziert, der Gate prüft das **Ergebnis** der
Klassifikation: Fall 1 durch Verschwinden, Fall 2 durch unterscheidbare
Titel.

| Tier | Befund | Aktion |
|---|---|---|
| 1 | Identischer Titel baumweit | Hard Error |
| 2 | Sehr ähnlicher Titel + Fingerprint-Match, keine Widersprüche | Umbenennen oder Löschen erzwingen |
| 3 | Fingerprint-Match, Titel klar verschieden | Warnung + Disposition in review.md |

Zusätzlich: Shape-Match mit hartem Widerspruch → `related_scenario`
(Info, kein Block). Subset (Then-Folge B ⊂ Then-Folge A) → Warnung
"Teilmenge — löschen oder Boundary im Titel".

Hard-Fail nur auf Sicherheit: identischer Titel und identischer Shape
sind deterministisch und unbestreitbar; alles Weichere warnt und
verlangt Disposition.

### 2.4 Fingerprint-Algorithmus (zustandslos)

Kein Index, keine Persistenz, deterministisch auf jeder Maschine,
CI-fähig ohne Setup.

**Schritt 1 — Normalisierung.** `And`/`But` auf den semantischen Typ
des Vorgängers auflösen; quoted Strings → `<v>`; Zahlen → `<n>`; Tokens
aus einer pro Projekt konfigurierbaren Enum-Liste maskieren (Shells,
Modell-/Provider-Namen, reasoning-Werte); Polarität neutralisieren
(`should not contain X` → `should contain X` plus Fact `not_contains: X`),
damit positive/negative Paare im Shape matchen und die Unterscheidung
in die Facts wandert; Whitespace kollabieren.

**Schritt 2 — Shape-Vergleich.** Then-Sequenzen separat per LCS
(Assertions sind das primäre Signal, Given/When schwächer gewichten);
Subset-Erkennung über die Then-Folgen; Scenario Outline pro
Examples-Zeile abflachen; Background-Steps dem Kontrakt voranstellen
(vererbter Kontext gehört zum Kontrakt).

**Schritt 3 — Facts und Widersprüche.** Exit-Codes, stdout/stderr
contains/not-contains, `[DONE]` vorhanden/fehlt, TTY/non-TTY,
CLI-Flags, Config-Präsenz, Kommandoname. Harte Widersprüche:
unterschiedliche Exit-Codes, contains vs. not-contains mit gleichem
Literal, `[DONE]` vs. ohne `[DONE]`, TTY vs. non-TTY, Erfolg vs.
erwarteter Fehler. Bei Widerspruch nie Duplikat-Klassifikation — egal
wie ähnlich der Shape ist.

**Schritt 4 — Titel-Ähnlichkeit.** Normalisierte Titel, Token-LCS/Jaccard,
als Proxy für Namensqualität (Tier 2).

**Maskierungs-Kontradiktion zwischen den Vorarbeiten.** DeepSeek will
Zahlen maskieren — das würde Exit-Codes mitmaskieren, und dann matchen
sich Szenarien, die Exit 0 vs. Exit 3 fordern (falsch). Die
Embedding-Idee des Autors ist hier korrekt: Exit-Codes, CLI-Flags,
`[DONE]`, erwartete Literale und Polarität tragen den
Verhaltensunterschied und gehören in die Facts, nicht in die Maskierung.
Diese Architektur (Shape getrennt von Facts) übernimmt der
Fingerprint.

**Bekannte Lücken.** (1) Paraphrasen — gleiche Bedeutung, andere
Wortwahl ("exit status should be 0" vs. "the program exits
successfully"); (2) Schrittzerlegung (durch Authoring-Regeln gegen
Composite-Steps begrenzt); (3) Werte außerhalb der Enum-Liste;
(4) Given-Reihenfolge. Lücken 2–4 akzeptiert; Lücke 1 ist der einzige
Zweck der Embedding-Schicht.

### 2.5 Embeddings: nur Retrieval

**Der entscheidende Reframe.** Lücke 1 zerfällt in zwei Teilprobleme:
**Finden** (ein semantischer Nachbar muss auftauchen) und **Beurteilen**
(ist er Duplikat, Variante oder eigene Boundary?). Das Beurteilen ist
bereits gelöst — gratis: Der Autor ist ein LLM-Agent, der eine Handvoll
Kandidaten selbst klassifizieren kann; der Reviewer verifiziert in der
Disposition. Der mehrstufige Klassifikator der Embedding-Idee
(Embedding ≥ 0.91 UND Step-Overlap UND Outcome UND keine Widersprüche)
ist Überbau auf der falschen Hälfte. Übrig bleibt nur das Finden, und
Finden braucht Recall, keine Präzision: Ein Klassifikator darf Fehler
machen, wenn er nur findet; er darf keine machen, wenn er blockt.

**Konsequenz.** `givn spec index` + `givn spec search`. Kein `check`,
kein Block, keine Policy-Maschinerie. Die Gates bleiben deterministisch
und zustandslos; der Index wird dort nie gebraucht — die teure Folge
"frischer Index als Gate-Voraussetzung" entfällt. Ein vergessener
Index-Lauf ist ein False-Negative-Risiko (ein frisch archiviertes
Szenario fehlt im Suchresultat), kein False-Positive.

**Suchzeitpunkte.**
1. Verhaltens-Query vor dem Schreiben (givn-spec, Schritt 0) — drei
   Queries: User-Action, Kern-Assertion, Boundary. Verhindert, dass das
   Szenario in die falsche Richtung entsteht (F8-Muster: sieben
   Feature-Familien für eine Tier-Auswahl). Propose/explore können die
   Suche optional früher nutzen.
2. Fertiger Gherkin nach dem Schreiben — gegen die permanenten Specs,
   nicht gegen den eigenen Delta (Selbst-Match ausschließen). Höchste
   Recall, weil Szenario-gegen-Szenario-Matching die stärkste
   Embedding-Form ist. Gilt auch für `@givn.modified`-Szenarien: der
   F2-Duplikat entstand laut Historie über einen modified-Delta
   (eb328dd).
3. Reviewer-Wiederholung im givn-review — gleiche Query, Vergleich gegen
   die aufgezeichnete Disposition. Die billigste Betrugsprüfung: Der
   Autor kann die Suche nicht "vergessen", ohne dass der zweite Lauf es
   zeigt.

**Belegpflicht.** Treffer werden als Beleg in die Dispositionstabelle
in review.md gepinnt (Datei, Titel, Score) — damit wird die Disposition
auditierbar statt nur behauptet.

**Validierungsmetrik.** Recall@k (k=5) auf den bekannten Duplikat- und
Boundary-Paaren des Reports — nicht Schwellwert-Trennung, weil auf dem
Score niemand blockt.

**Index.** Inkrementell per content_hash, deckt den aktiven Baum ab
(permanente Specs + aktive Deltas), lokal, Modell-Download beim ersten
Lauf (kein Single-Binary im MVP).

### 2.6 Längengates: Hinweis, kein Hard-Fail

Der Report hat eine Skala (10–14 inspizieren, 15–19
Boundary-Entscheidung, 20+ Rechtfertigung); DeepSeek macht >19 zum
Error außer mit `@long.rationale`. Verworfen aus zwei Gründen: Der
Report warnt selbst doppelt — Split erzwungene Transaktionen nicht nur
wegen des Step-Counts, und verstecke Workflows nicht in Composite-Steps.
Ein hartes Gate setzt den Anreiz genau in die verbotene Richtung: Der
Agent kürzt sichtbare Steps, indem er Seitenabläufe in einen
undurchsichtigen Step "complete setup" verpackt. Und die 23 langen
Szenarien in watn sind dort lang, wo sie es sein müssen
(PTY-Setup-Präambeln, echte Transaktionen).

Stattdessen: Lint warnt bei >14 Steps; die Dispositionstabelle bekommt
eine Spalte "split-or-keep" (echte Transaktionsgrenze oder Begründung
für Zusammenhalt); maschinell prüfbar bleibt nur: langes Szenario ohne
Eintrag → Review blockiert. Der Rest ist Reviewer-Urteil.

### 2.7 Test-Policy: Black-Box first

**Prinzip.** Möglichst immer die echten Schnittstellen testen.
Interne Logik nur im Ausnahmefall — sie kann sich ändern, ohne dass
sich beobachtbares Verhalten ändert. E2E ist teuer (Laufzeit), daher
sind schnellere Unit-Tests für Edge-Cases angebracht. Hexagonal:
Domainkern mit Fake-Adaptern, Happy Path und produktive Schnittstellen
per E2E (Beispiel: InMemory-Repository für alle Fälle,
Postgres-Testcontainer-Repository für den Happy Path).

**Regel.** Ein interner Test darf nur Fälle abdecken, die das E2E nicht
abdeckt — nie denselben Fall. Review-Frage: "Welchen Fall deckt dieses
Szenario, den das E2E nicht deckt?" Ohne Antwort wird gelöscht.

**Konsequenz für den Report.** F1's nicht-E2E-Szenario wird gelöscht,
nicht umbenannt; Report-Empfehlungen wie "rename it to a distinct
lower-level search-state contract" (F6) werden zu Löschungen — die
Konsolidierung wird aggressiver als der Report plant. Die Policy
gehört in die Skills (givn-design, givn-steps), nicht in Gates.

### 2.8 Step-Seite

**Entscheidungen.** Kein Source-Parsing im Binary (ein generisches CLI
soll keinen fremden Testcode interpretieren — `givn steps report` mit
sprachabhängigen Regex verworfen). Kein projektseitiger Binding-Index
(zu komplex).

**Zweiteilung der Duplikation.** (1) Szenario-getriebene Aliase (S3, S5)
lösen sich durch die Szenario-Konsolidierung — entfallene Szenarien
entziehen den Alias-Bindings den einzigen Aufrufer. (2)
Mechanics-Duplikation (S1/S2: Page-Poller, PTY-Treiber) überleben die
Konsolidierung, weil ihre Szenarien gelayered sind und bleiben — sie
sind Prompt/Regel-Thema plus manuelles Helper-Refactor nach dem
Delegationsmodell (S6). Die Hypothese (Szenario-Arbeit mindert
Step-Duplikation) ist mit vorhandenen Daten testbar: pro Binding
zählen, wie viele aktive Szenarien es referenzieren, gegen die
Konsolidierungskandidaten schneiden.

### 2.9 Prosa: eine normative Quelle

`assets/instructions/*.md` ist die einzige Quelle für Regeltext. Skills
und Slash-Commands werden dünne Choreografie, die `givn instructions`
ausführt. Drift ist belegt (drei `@e2e`-Wortlaute, zwei davon im selben
Template); jede der Regeln aus dieser Diskussion müsste sonst dreifach
gepflegt werden.

**Normative `@e2e`-Regel:** eine E2E-Evidenzzelle pro realer
User-Action, repository-weit. Ein zweites E2E-Szenario für dieselbe
Action braucht eine neue Schnittstellen-Rechtfertigung. Die
feature-lokale Formulierung ("pro User Interaction Inventory entry")
ist der Mechanismus, der in watn die Setup-, Modellwahl- und
Filter-Flows pro Capability dupliziert hat (F8/F10) — sie ist nicht nur
Drift, sie ist die Ursache eines Befunds.

## 3. Analytischer Befund: F1 präzise geklärt

Die erste Charakterisierung in der Diskussion ("legitime Schichtung,
umbenennen, nicht blockieren") war falsch; die Prüfung des Codes ergab:

- Szenario A (`credential-sources.feature:17-26`, `@e2e`): startet das
  echte Binary, prüft Exit-Status und den API-Key im echten
  HTTP-Request.
- Szenario B (`provider-setup.feature:187-195`): `provider_setup_steps.rs:
  185-199` ruft direkt `config::get_provider_api_key` auf und legt das
  Ergebnis in `world.pending_config["resolved_key"]` ab. Der Then-Step
  (`provider_setup_steps.rs:1183-1210`) degradiert bei vorhandenem
  `resolved_key` zu einem Rückgabewert-Vergleich — die Prosa "API
  request should use API key" überzeichnet, was tatsächlich geprüft
  wird. Kein Binary, kein Request, kein HTTP.

B ist eine strikte Teilmenge von A mit überzeichneter Assertion; A deckt
den Resolver-Pfad über das echte Binary mit ab. Richtige Disposition:
Löschung. Lehre für das Design: F1 hätte auch ein deterministischer
Check gefangen (identischer Titel) — für die Gate-Rechtfertigung der
Embeddings ist F1 kein Beleg; der Beleg wäre nur ein Fall mit
verschiedenen Titeln, den der Fingerprint verpasst. Genau das misst das
geparkte Experiment.

## 4. Verworfen (mit Grund)

- **Terra's V2-Kontrakt** (Anker, Registry, CAS, Journal,
  Verlustfrei-Garantie): löst Mutation/Lost-Update, das als hypothetisch
  eingestuft ist; allein größer als der Rest des Vorhabens zusammen.
  Die billigen Einzelbefunde (GIVN_FEATURES, Doc-Drift, Integritäts-
  Gate) bleiben unter "Verschoben".
- **Embedding-Blocking** (`probable_duplicate` → Block, Exit-Code 2):
  macht die Gates zustandsbehaftet (Indexfrische als Gate-Voraussetzung,
  Modell-Footprint inkl. CI-Agents, Kalibrierung pro Modell und
  Projekt). Ein Auto-Block hätte F1 zwar korrekt getroffen, aber auf
  einem Score, auf dem niemand blocken darf.
- **Binding-Index und Source-Parsing**: ein generisches CLI soll keinen
  fremden Testcode interpretieren; projektseitiger Index zu komplex.
- **Längen-Hard-Fail**: setzt Anreiz zu Composite-Steps (2.6).
- **Dispositions-Tags (`@boundary.*`) und Mini-Ledger**: Der Default ist
  Löschung (2.7); für die seltenen Ausnahmen reicht Prosa in review.md
  mit gepinnten Suchtreffern. Der Ledger wäre der kleinste Verwandte der
  abgelehnten Registry.

## 5. Bewusst verschoben

- **Runner-Doppelausführung.** `features_runner.rs:150-165` führt
  permanente Specs plus rohe Deltas aus: modified-Szenarien laufen
  neben ihrer veralteten Version, removed-Platzhalter bleiben
  ausführbar, bis archiviert ist. Konsequenz: Die Suite kann während
  eines Changes altes und neues Verhalten gleichzeitig verlangen;
  gelöschte Bindings müssen bis zum Archive am Leben bleiben. Hat nie
  bewusst gebissen — behandeln, wenn es als Problem erkennbar wird.
- **`GIVN_FEATURES`-Dead-Contract.** givn setzt die Variable, watn
  liest sie nie.
- **Doc-Drift `config.yaml` vs. `commands.yaml`.** Mit der
  Prosa-Konsolidierung mitbehandeln.

## 6. Offen

- **Embedding-Validierungsexperiment** (läuft parallel beim Autor):
  Recall@k (k=5) der bekannten Duplikat-/Boundary-Paare aus dem Report
  gegen den watn-Bestand — existieren überhaupt Paraphrasen-Paare mit
  verschiedenen Titeln, die der Fingerprint verpasst? Das Ergebnis
  entscheidet, ob die Retrieval-Schicht gebaut wird.

## 7. Nächste Schritte (Skizze, nicht entschieden)

- Deterministische Schicht zuerst: Lint-Scope-Fix (Archive aus dem
  Default-Lint), Tier-1–3-Gates, Split-or-keep-Spalte,
  Net-Delta-Receipt.
- Prosa-Konsolidierung auf eine Quelle mit der neuen `@e2e`-Norm.
- Skill-Änderungen: givn-spec Suchschritte (Query + Gherkin), givn-review
  Dispositionstabelle mit gepinnten Treffern, givn-design/givn-steps
  Black-Box-Policy.
- watn-Konsolidierung als Dogfooding (Report's Recommended Consolidation
  Order, korrigiert um die Black-Box-First-Policy: Löschungen statt
  Umbenennungen).
