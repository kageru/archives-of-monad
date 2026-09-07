# AGENTS.md

Guidance for an agent asked to fix this project after the `foundry` data has drifted out of sync
with our serde structs again. This happens periodically because `foundry` is the upstream
[foundry-vtt---pathfinder-2e](https://gitlab.com/hooking/foundry-vtt---pathfinder-2e) system repo,
cloned fresh (see `before_push.sh` and `.github/workflows/check.yml`), not a dependency we control.
Its authors change their JSON schema whenever it suits Foundry/PF2e development, without warning
and without versioning we can pin against.

Read this before touching any `src/data/*.rs` struct. It will save you from re-discovering things
the hard way.

## The core insight: most of this is mechanical, not creative

**serde ignores unknown fields by default.** Nothing in this codebase uses
`#[serde(deny_unknown_fields)]`. That means Foundry can add a hundred new keys to a JSON object and
none of it breaks us — only *renamed*, *restructured*, *removed*, or *retyped* fields that our
structs actually read will break deserialization. So the fix is never "model the whole new schema,"
it's "find the specific fields we read that moved, and move with them."

This means the single best strategy is **empirical, not analytical**: get the pipeline reading the
current file layout, run the actual binary against the real data, and let serde's error messages
point you at exactly what broke, one at a time. Don't try to diff the old schema against the new one
field-by-field from memory or by skimming — you will miss things and waste time on fields nobody
reads. Let the compiler and the deserializer do that work.

## The fix loop

1. **Make sure the file-discovery pipeline itself is current first** (see below) — otherwise
   every single type will fail with "no such file or directory" and hide the real errors.
2. `cargo build` (fixes compile errors from any struct edits already made).
3. Run the actual binary against the real checked-out `foundry` data:
   ```bash
   cd /path/to/repo && ./target/debug/archives-of-monad 2>&1 | grep -v "^Reading\|^Found bestiary"
   ```
   No `MEILI_KEY` needed — indexing is skipped and rendering proceeds regardless. This renders
   every category in `main.rs`'s `render_and_index!` calls (feats, spells, backgrounds, actions,
   conditions, deities, classfeatures, classes, equipment, ancestryfeatures, ancestries, heritages,
   creatures) end to end, in order, against the *entire* real dataset — tens of thousands of files.
   Each category is independent; a failure in one doesn't stop the others (except a raw `panic!`,
   which aborts everything after it — see "make deserialization panics into skips" below for why
   you want to avoid those while iterating).
4. Read the first error. It looks like:
   ```
   Error while rendering spell folder : invalid type: null, expected struct JsonSpellArea at line 7 column 20
   ```
   or, for a hard panic (usually inside `data/creature.rs`, which does manual
   `serde_json::from_value` on embedded item data instead of going through `#[serde(from = ...)]`):
   ```
   thread 'main' panicked at src/data/creature.rs:153:45:
   Could not deserialize item data for Painful Touch: Error("data did not match any variant of untagged enum JsonDamageRolls", ...)
   ```
   These messages name the Rust type that failed and the byte offset in *some* JSON file — not
   which file. Use `jq` (see below) to find a real example and inspect its actual shape.
5. Fix the one struct, rebuild, rerun. Repeat. This project fixed ~25 distinct breakages this way
   in one sitting — it converges fast because each fix is small and the error messages are precise.
6. Once the whole pipeline runs clean (exit code 0, no `Error while rendering X folder` lines), go
   back and check the *content* of what got rendered, not just that it didn't crash — serde will
   happily deserialize a field into a *type-compatible but semantically wrong* value (e.g. reading
   `.value` as a proficiency modifier is technically valid regardless of which JSON field it now
   corresponds to). Spot-check a handful of rendered pages under `output/` for a wildly-restructured
   type like creatures and spells. Also grep the run's stderr for `eprintln!` warnings and check
   their *volume*, not just that they exist: a warning that fires for 3 out of 50,000 files is a
   genuine rare-data anomaly worth leaving as a warning, but the same warning firing hundreds of
   times almost always means a whole category of data is being silently dropped and needs a real
   fix, not just a quieter log line (see the ritual/spellcasting-entry case below for an example).
7. Fix the tests (see "regenerating golden fixtures" below) and run `cargo test`, `cargo clippy
   --all-targets`, `cargo fmt --all -- --check` before calling it done.

## Investigating the real data with `jq`

`jq` is installed. This is far more reliable than grepping raw JSON. Useful patterns:

```bash
# Full shape of one representative file for a type
jq '.system' foundry/packs/pf2e/<pack>/<file>.json

# What values does an enum-like field actually take across the whole pack?
find foundry/packs/pf2e/<pack> -name "*.json" ! -name "_folders.json" \
  -exec jq -r '.system.category // empty' {} \; | sort -u

# Does a field even exist consistently, or is it sometimes missing/null?
find foundry/packs/pf2e/<pack> -name "*.json" ! -name "_folders.json" \
  -exec jq -e '.system | has("someField")' {} \; | sort | uniq -c
```

**Gotchas when scripting `jq` over many files:**
- Never pass a jq **filter string** through `find -exec ... {} \;` or `xargs -I{}` if the filter
  itself contains a literal `{}` (e.g. `.foo // {}`) — both `find -exec` and `xargs -I` do textual
  substitution of `{}` *anywhere* in the command line, including inside your quoted jq program, and
  will silently corrupt the filter. Use `keys?` / `// empty` / `// []` instead of `// {}`, or use a
  placeholder token other than `{}` for the filename (e.g. `xargs -I FILE jq '...' FILE`).
- `cwd` does not persist between separate Bash tool invocations in this harness — don't rely on a
  prior `cd` in one call affecting a later call. Use absolute paths or a single compound command.
- A `find | xargs -I{} sh -c '... "$1" ...' _ {}` construction is the safest way to run a jq filter
  containing `{}` per-file at scale.

## Places that are known trouble spots (check these first)

Roughly in order of how often they've broken:

- **`source` → `publication`.** Almost every type had a plain `source: ValueWrapper<String>` field
  that became `publication: {title, license, remaster}` (see `Publication` in `src/data/mod.rs`).
  If you see `missing field 'source'`, this is almost certainly it — grep for
  `ValueWrapper<String>` fields named `source` across `src/data/*.rs`.
- **`featType`/`feat_type` → `category`.** Feat, ClassFeature, AncestryFeature, and the
  boons-and-curses items all moved their type discriminator from a wrapped `featType` key to a bare
  `category` string. Check `src/data/feat_type.rs`'s `FeatType` enum for new category values too —
  new game content regularly adds ones we haven't seen (e.g. `Calling` was added this round).
- **Skill names**: stored as full lowercase words (`"acrobatics"`) not 3-letter codes (`"acr"`).
  `src/data/skills.rs`'s `Skill` enum uses `#[serde(rename_all = "lowercase")]` for this reason —
  don't reintroduce per-variant `#[serde(rename = "...")]` abbreviations.
- **Inline text formatting tags** (`src/parser.rs`) are Foundry's own evolving mini-syntax embedded
  in description HTML (`@Check[...]`, `@Template[...]`, `@Damage[...]`, `@UUID[...]`, `@Embed[...]`,
  etc.) and drift independently of the JSON schema, on their own schedule. When you see
  `eprintln!("Unknown @Formatting: ...")` warnings in the run output, or a panic inside
  `next_token`/`length_of_scope`, that's a syntax variant `parser.rs` doesn't handle yet. Grep the
  live corpus for the tag (e.g. `grep -rohE '@TagName\[[^]]*\]' foundry/packs/pf2e | sort -u`) to see
  every shape it's used in before writing the parse arm — these tags often support *both* an old and
  a new argument style simultaneously (e.g. `@Check[reflex]` bare-positional vs.
  `@Check[type:reflex]` explicit-key), and you need to handle both, not just whichever one you
  happened to see first.
- **`@UUID`/`@Compendium` category names**: the `match category.to_lowercase().as_str()` block in
  `parser.rs` is an explicit whitelist mapping compendium pack names to our own URL categories. New
  sourcebooks add new pack names constantly (new bestiaries, new `-effects` compendia, etc.). The
  fallback arm should **never** be a hard `unimplemented!()`/`panic!()` — it must degrade to
  rendering plain text with an `eprintln!` warning, because there will always be another new pack
  name next time. If you find a `panic!`/`unimplemented!` guarding an exhaustive-looking match on
  data-driven string values anywhere in this codebase, treat it as a latent crash waiting for the
  next content update and soften it, even if it's not the thing you were asked to fix.
- **Bestiary folder discovery** (`bestiary_folders()` in `main.rs`): it filters `packs/pf2e/`
  subfolder *names* by substring (`"bestiary"`, `"monster-core"`, `"npc-core"`, minus some
  exclusions). Foundry doesn't consistently suffix new monster sourcebooks with `-bestiary` — check
  `ls foundry/packs/pf2e/` for anything that looks like a creature compendium under a different
  naming scheme when a new monster book releases, and confirm it actually contains
  `"type": "npc"` items with `jq -r '.type' <file>` before adding it to the filter.
- **Untagged enums silently swallow real bugs.** Several structs use `#[serde(untagged)]` as a
  "try multiple possible shapes" mechanism (`WrappedOrNot<T>` in `equipment.rs`,
  `JsonDamageRolls` and `JsonNpc` in `creature.rs`, `JsonEquipmentDamageShape`). These are the right
  tool when a field genuinely has multiple legitimate shapes across different items, but the error
  message when *none* of the variants match (`"data did not match any variant of untagged enum X"`)
  never tells you which variant almost-matched or why. When you hit one, temporarily deserialize the
  raw `serde_json::Value` at that spot and print it, or `jq` a wider sample of real files for that
  field, rather than guessing.
- **A creature's rituals are spell items with no matching spellcasting entry — that's expected, not
  an error.** A creature's `items` array mixes normal spells (tied to a `spellcastingEntry` via
  `location`) with rituals it can perform, which have `location.value: null` and a populated
  `system.ritual` block instead, because rituals don't consume spell slots or have a DC/attack
  modifier the way prepared/spontaneous/innate spells do. `From<JsonCreature>` in `creature.rs`
  builds a `Spell` from every `type: "spell"` item unconditionally, then branches: if `location`
  matches a known `SpellCasting.id`, push into that casting entry as usual; else if
  `spell.category == SpellCategory::Ritual`, push into `Creature.rituals` instead; only fall through
  to an `eprintln!` warning if neither applies (rendered as a `<b>Rituals</b>` block on the creature
  page, right after normal spellcasting, via `render_rituals` in `html/creatures.rs`). If Foundry
  ever adds another spell-like item type that similarly doesn't tie to a spellcasting entry, check
  both paths the same way rather than just widening the "give up and warn" fallback — the fallback
  should stay reserved for genuinely anomalous data (single digits of occurrences across the whole
  corpus), not a whole category of legitimate content.
- **`JsonNpc` has a catch-all `Other(IgnoredAny)` variant.** The bestiary-tagged compendium folders
  ship non-creature documents alongside actual creatures (actions, effects, army units — whatever
  Foundry bundles that release). Don't remove this variant or make it stricter; it's load-bearing.
  If a *new* non-creature `"type"` starts showing up, it'll be silently absorbed by this variant
  already — no action needed unless you actually want to render that type.
  This "one compendium folder, several unrelated document shapes" pattern isn't unique to
  bestiaries: `boons-and-curses` also ships a handful of `"effect"` documents (the mechanical rule
  grants behind a boon/curse) with no `category` field, alongside the actual boon/curse feats. Same
  fix, same shape — `JsonBoonOrCurseDoc` in `boons_and_curses.rs` (try the real shape, fall back to
  `IgnoredAny`, map the fallback to a `"[Empty]"`-named placeholder that the existing
  `!e.name().starts_with("[Empty")` filter in `html::render` drops from the output). Expect this
  same pattern to keep recurring in other single-purpose-looking folders — check `jq -r '.type'`
  across a folder's files before assuming every file in it has the same shape.
- **Rank-1 (formerly "level 1") numeric fields that can be free text.** Several fields typed `i32`
  in the old schema can now legitimately be strings like `"up to 5"` or `"variable"`
  (`ritual.secondary.casters`, a spell's heightened `level.value` on certain creature-embedded
  spells). The existing `StringOrNum` untagged enum (`src/data/equipment.rs`) exists exactly for
  this — reach for it (or extend a field to `Option<...>`) rather than assuming a bare `i32` is safe
  just because most examples are numeric. Sample broadly with `jq` before trusting a numeric type.
- **Game-design removals, not just renames.** The 2023 remaster removed whole *mechanics*, not just
  reshuffled JSON keys: schools of magic, spell components (verbal/somatic/material), and creature
  alignment are all just gone from the data. If a field you're trying to fix has no replacement
  anywhere in the current schema (check with a broad `jq 'keys'` sample across many files, not just
  one), the right fix is usually to delete the field/enum from our struct entirely (and any
  now-dead rendering code) rather than inventing a fallback value. Don't leave fields silently
  defaulting to a fixed value forever — that's a correctness bug wearing a compile-success costume.
- **`JsonTraits` (`src/data/traits.rs`)** is the one shared struct used by nearly every type
  (`value: Vec<String>`, `rarity: Option<Rarity>`). It's `#[serde(default)]` on `value` because some
  newer items omit an empty trait list key entirely rather than sending `"value": []`. If you add a
  field here, make sure it's meaningful for *every* type that embeds `JsonTraits` (spells, equipment,
  creatures, feats, etc. all reuse it) — type-specific extras (like spell `traditions`) belong in a
  wrapper struct with `#[serde(flatten)] base: JsonTraits` next to the extra field instead (see
  `JsonSpellTraits` in `spells.rs` for the pattern), not bolted onto the shared struct.

## Long-form lore has moved into `journals/*.json` — watch for content that just vanished

Foundry increasingly stores an item's full descriptive lore in a separate journal entry page
instead of the item's own `description` field. What's left on the item itself is often just a short
blurb plus `@UUID[Compendium.pf2e.journals.JournalEntry.<entryId>.JournalEntryPage.<pageId>]{Label}`
— and our old `@UUID` handling only ever rendered `Label` as plain text, so this reads as "the item's
description mysteriously got much shorter," not as an obvious schema error (nothing fails to
deserialize; the text is just gone). This has hit ancestries (all 50), feats (305 references),
classes (29), and to a lesser extent equipment, spells, and class-features — check
`grep -rlE '@UUID\[Compendium\.pf2e\.journals\.' foundry/packs/pf2e --include="*.json" | sed -E 's#.*/pf2e/([^/]+)/.*#\1#' | sort | uniq -c`
any time you're chasing this class of bug again, since the exact set of affected packs will change
with each Foundry release.

**How to notice this at all**: byte-count each `tests/html/*.html` fixture against its `git show
HEAD:<path>` counterpart. A handful of fixtures shrinking to ~85–95% of their old size from wording
changes is normal churn; anything dropping to single-digit percentages (spooder.html/Anadi went to
10%) is content silently routed somewhere else, not a rewrite.
```bash
for f in tests/html/*.html; do
  old=$(git show HEAD:"$f" 2>/dev/null | wc -c); new=$(wc -c < "$f")
  [ "$old" -gt 0 ] && echo "$((100 * new / old))% ($old -> $new) $f"
done | sort -n
```

**The fix** (already implemented, but worth understanding if it needs extending): `src/data/journal_pages.rs` reads every `foundry/packs/pf2e/journals/*.json` file up front into a
`HashMap<page_id, content>` (`JsonJournal { pages: Vec<JsonJournalPage> }`, keyed by each page's
`_id`), exposed as a `JOURNAL_PAGES` static in `main.rs` the same way `TRANSLATIONS` is. In
`parser.rs`, a dedicated `Token::CompendiumReference` match arm intercepts `category == "journals"`
*before* the generic category-to-URL mapping, looks up `key.rsplit('.').next()` (the trailing page
id — robust regardless of how many `JournalEntry.<id>.JournalEntryPage.` segments precede it) in
`JOURNAL_PAGES`, and if found, recursively `text_cleanup`s the resolved content in place of the
plain-text fallback (journal pages have their own `@`-tags that need the same cleanup).

Two non-obvious pitfalls this ran into, in case the pattern needs extending to another
`@Embed`/`@UUID` variant later:

- **Journal pages reference other journal pages, sometimes directly self-referentially** (e.g. an
  archetype's own journal page linking back to `JournalEntryPage.<its-own-id>`), and inlining one
  runs it back through `text_cleanup`, which can try to inline it again. Left unguarded, this is a
  real stack overflow in production data (not a hypothetical) — it happened on the very first full
  corpus run after adding journal resolution, `panic`-free but a hard `SIGABRT` from Rust's stack
  guard page, well before any `Result`-based error handling gets a chance to run. The fix is a
  `thread_local! { Cell<u32> }` depth counter around the recursive `text_cleanup` call (see
  `JOURNAL_RESOLUTION_DEPTH`/`MAX_JOURNAL_RESOLUTION_DEPTH` in `parser.rs`), capped low (4): beyond
  that depth, fall back to the plain-text label instead of resolving further. Any *other* place that
  makes `text_cleanup` recursively call itself based on data-driven content (not just this one) needs
  the same kind of bound — the existing `@Localize` recursion happens to be safe only because
  translation strings don't reference each other, not because of any structural guarantee.
- **A "which specific item is failing" debugging trap**: when hunting the overflow above, piping the
  binary's output through `| grep ... | tail -N` produced *nothing* for several minutes and looked
  identical to a hang, because `tail` buffers until EOF and the process was still very much alive
  (just processing tens of thousands of files) — not stuck, not crashed. Redirect straight to a file
  with `> /tmp/x.log 2>&1 &` and `tail`/`grep` the file separately instead of piping live; that also
  survives you needing to inspect progress with the process still running, and separates "still
  working" from "genuinely stuck" (check `pgrep -f target/debug/archives-of-monad` for the former).
- **`get_data_path()` (and anything built on it, like `TRANSLATIONS`/`JOURNAL_PAGES`) resolves
  differently under `cargo test`.** `DATA_PATH` reads `std::env::args().nth(1)`; under a normal run
  that's `None` (falls back to `"foundry"`), but under `cargo test` argv[1] is a test filter string
  or a flag like `--exact`, not a path. `TRANSLATIONS`'s own loader hard-`panic!`s on a bad path, so
  this landmine was never tripped — no test happens to exercise the *production* `crate::TRANSLATIONS`
  through `text_cleanup`'s `@Localize` handling (tests that check translations import the separate,
  hardcoded-path `crate::tests::TRANSLATIONS` directly instead). `read_journal_pages` degrades
  *silently* on a bad path (logs and returns an empty map) rather than panicking, so this same
  landmine showed up as "journal resolution silently does nothing under `cargo test`" instead of a
  crash — much harder to notice. Fixed once, at the source, in `DATA_PATH` itself
  (`if cfg!(test) { "foundry" } else { ...argv... }`), rather than special-casing every static built
  on top of it. If you add another `get_data_path()`-dependent static, you don't need to repeat this
  fix — but if you ever see a test pass with suspiciously *empty*/*default* content instead of a
  hard failure, this is the first thing to check.

## Regenerating golden HTML fixtures (`tests/html/*.html`)

Many `html::*::tests` compare real rendered output against a checked-in golden `.html` file via
`assert_eq_ignore_linebreaks(actual, include_str!("../../tests/html/NAME.html"))`. When the
underlying game content changes (not just the schema — errata changes actual numbers/wording too),
these fixtures go stale and need regenerating from the *current* real data, not hand-edited.

The fastest way to do this at scale (used to fix ~20 fixtures at once this round):

1. Temporarily rewrite each call site from
   `assert_eq_ignore_linebreaks(EXPR, include_str!("../../tests/html/NAME.html"));` to
   `{ let __actual: String = (EXPR).to_string(); std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/html/NAME.html"), __actual).unwrap(); }`
   — a small Python script with a regex over `src/**/*.rs` matching
   `assert_eq_ignore_linebreaks\(\s*(.*?),\s*include_str!\("\.\./\.\./tests/html/(\w+)\.html"\)\s*,?\s*\)`
   (`re.DOTALL`, non-greedy) does this in one pass. Watch for two call-site formatting variants:
   with and without a trailing comma before the closing paren — match both.
2. If a fixture file is being renamed/added and doesn't exist yet, `touch` an empty placeholder
   first so `include_str!` still compiles (content doesn't matter, it's about to be overwritten).
3. `cargo test` — every converted test now *writes* the real current output instead of asserting.
4. Run the inverse regex to put the real `assert_eq_ignore_linebreaks(...)` calls back.
5. `cargo test` again — should be green now, comparing against the freshly-written fixtures.
6. `cargo fmt --all` to clean up formatting (the substitution isn't fmt-clean on its own — watch
   for stray double-semicolons `);;` left over from the reverse substitution and strip them, `cargo
   fmt` won't do it for you).
7. Diff the fixture changes by eye for anything that looks like a bug rather than a legitimate
   content update (e.g. a field silently rendering empty because a struct fix was subtly wrong).
8. Fixture files backing tests you had to point at a *different* source JSON file (because the
   original named entity — e.g. a specific spell, heritage, or feat — no longer exists under that
   name) will need a new filename; delete the old, now-orphaned fixture rather than leaving it
   around (check `grep -rl "old_name.html" src/` returns nothing first).

Also make sure every `read_test_file("...")` path embedded in test bodies still points at a real
file — `read_test_file` resolves relative to `foundry/packs/pf2e/`. A renamed/moved/deleted example
file (very common after a remaster-style content pass — see e.g. half-elf → aiuvarin, aasimar
heritage → aasimar ancestry) means picking a *different*, currently-real example and updating the
test's expected values to match that file's actual current content — don't try to preserve the old
expected values against a new file.

## Content changes worth knowing about (2023 "remaster")

If the `foundry` checkout has moved past where this file was written, some of this may already be
stale — but as of the last pass:

- Ability score *minimums* on feat prerequisites became ability score *boosts* (e.g. "Strength 14"
  → "Strength +2").
- Half-Elf/Half-Orc heritages were renamed to Aiuvarin/Dromaar; Aasimar and similar "versatile
  heritages" (celestial/fiend-blooded humans) were consolidated into ancestry-independent
  "versatile heritages" like Nephilim, and some became their own full ancestries.
- Alignment (LG/CE/etc.) was removed from deities and creatures game-wide, replaced by a
  holy/unholy "sanctification" concept for deities; alignment-flavored creature traits like
  "chaotic"/"evil" still appear, just as ordinary trait tags, not a separate field.
- Schools of magic and spell components were removed from spells entirely.
- Class DC proficiency is no longer stored per-class in the data (it's always Trained at level 1 by
  rule) — don't expect a `classDC` key to reappear; if it does, that'd itself be a schema reversion
  worth double-checking against the rules before trusting it.

## Files you'll be editing

- `src/data/mod.rs` — shared primitives (`ValueWrapper<T>`, `Publication`, `HasName`, `HasLevel`,
  the `ord_by_*!`/`has_*!` macros). Changes here ripple everywhere; check all callers.
- `src/data/*.rs` — one file per compendium type, each with a public "clean" struct
  (`#[serde(from = "JsonX")]`) and a private `JsonX`/`JsonXData` struct pair mirroring the raw
  Foundry shape, plus a `#[cfg(test)] mod test`/`tests` with at least one real-file deserialization
  test.
- `src/data/creature.rs` — by far the largest and most fragile file; NPCs/hazards/vehicles/
  characters share one `items: Vec<JsonCreatureItem>` array with a `system: serde_json::Value` field
  deserialized manually per `item_type` inside `From<JsonCreature>` (not through the normal
  `#[serde(from = ...)]` path), so its errors show up as runtime panics rather than the graceful
  `Error while rendering X folder` messages the other types get. Consider whether a given
  `.expect()`/`.unwrap()` in that `match` block should instead degrade gracefully (`eprintln!` +
  skip) if the failure mode is "some creatures have this, some don't" rather than "this is always
  malformed and worth stopping for."
- `src/html/*.rs` — one file per rendered page type, mirroring `src/data/*.rs` names; mostly
  untouched by schema changes except where a field was removed/renamed that rendering code reads
  directly (e.g. dropping spell components meant deleting a line in `html/spells.rs`, not just a
  struct field), and their `#[cfg(test)]` blocks holding the golden-fixture tests.
- `src/main.rs` — `bestiary_folders()`, `render_and_index!` folder name list (must match current
  `packs/pf2e/<name>/` directory names exactly, singular/plural and hyphenation included), and
  `read_test_file`'s base path.
- `src/html/mod.rs` — `read_data`/`collect_json_paths`: the recursive directory walk. Shouldn't need
  to change again unless Foundry changes the base `packs/pf2e/` layout itself (as opposed to what's
  inside it).
