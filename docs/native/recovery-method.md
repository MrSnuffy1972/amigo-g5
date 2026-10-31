# Code recovery method (step 1 of the native-port process)

Step 1 recovers 100% of a game's code into readable, reassemblable, named and explained source, and proves it
complete. It is not translation and not emulation. Game 296 was recovered first, by hand-tuned tools
(`/srv/fast/amigo/recovery/00296/tools`, kept as they are). This page describes the generalised, unattended pipeline
that now runs every game: `/srv/fast/amigo/recovery/tools` on Ryzen. Per game everything lives in
`/srv/fast/amigo/recovery/<id>/` and never leaves Ryzen; only no-bytes summaries go to this repository.

## The four proofs

| proof | what must hold |
|---|---|
| (a) classification | every byte of every disk image, file, unpacked stream and run-time segment is in exactly one class (code, data, asset, padding, unused), with no holes, no overlaps and nothing unknown |
| (b) reassembly | every recovered unit reassembles with vasm to the identical bytes: hunk executables per hunk (type, memory flags, size, every byte, the exact relocation set), boot blocks and run-time segments byte for byte |
| (c) functions | every routine has a name and a description that passed the quality check |
| (d) executed addresses | every PC the reference run executed in game code is an instruction start in both decoders, inside a described routine |

A game that cannot reach all four is marked **blocked** with the precise reason, and the queue moves on. No proof is
ever relaxed to make a game pass.

## Pipeline stages (`pipeline.py <id>`)

1. **prep** - copy the project's inputs (`projects/<id>/prekit/game`, following the archiver's links to
   `/srv/bulk/fast-archive`) and the canonical reference run (`prekit/out/port/ref`: executed PCs with their opcode
   words or first bytes, harness log). Re-run the project's own reference harness for the same number of frames; if it
   logs exactly the same executed PCs, keep its RAM at the end of the run (chip and slow RAM only, never ROM) as the
   source for run-time segments.
2. **recover** (`recover.py`, inside the existing `amigo-tools:4` image, capstone 4):
   - disks: our own AmigaDOS parser (OFS/FFS, INTL, DIRCACHE, DD/HD) gives every disk byte a region; DMS/ADZ are
     unpacked with temisu/ancient first; IPF is a blocker for now;
   - files: hunk executables are parsed (code, data, BSS, RELOC32 and RELOC32SHORT, symbols, debug, trailing bytes;
     overlays and 16/8-bit relocations are blockers); packed files are unpacked with ancient (PowerPacker, Imploder,
     CrunchMania, ByteKiller, RNC and the rest it knows), the packed stream is classified as packed data and the unpacked
     bytes are processed again (up to three levels), so packed executables are recovered from their unpacked form;
   - executed PCs are mapped to (file, hunk, offset) by byte matching, because hunk load addresses are not logged:
     candidate bases come from PCs whose opcode word is rare in a hunk and a base is accepted with at least 3 matching
     PCs and 90% of the PCs in its range matching (relocation positions masked);
   - PCs no file explains are code the game unpacked, copied or track-loaded. They are carved from the reference-run RAM
     as **run-time segments** (each carved PC must show the opcode word the reference run logged; code that changed after
     it ran is a blocker). A run-time segment is reassembled with `org` at its real address. Disk blocks outside the
     filesystem and blocks of custom-format disks are matched against the segments and classified through them, so
     track-loaded code is reassembled per loaded block. Unmatched blocks are decided on evidence: code-like bytes and
     packed blocks ancient cannot open are blockers; low-entropy blocks with no opcode signal are data;
   - recursive descent (own flow following, every relocation must match exactly one operand) from: the entry point,
     every executed PC, cross-hunk calls; then validated candidates (author symbols, pointers in data, addresses taken
     by lea/pea/move) that must decode with no problem; then whole gaps that decode cleanly and end in a flow terminator
     (unreached code);
   - every byte no instruction covers is typed automatically: relocated pointers, NUL-terminated text, zero padding or
     zero-filled tables, chip-memory assets, bulk data blocks, referenced tables or unreferenced bytes. This replaces the
     hand-written annotations of game 296;
   - source in vasm Motorola syntax: one section per hunk, labels for every routine, branch target and data reference,
     relocated operands as symbols, explicit sizes. Anything that cannot be formatted safely is emitted as raw words
     with the instruction in a comment (relocations still as symbols), so reassembly stays identical;
   - classification at hunk, file, unpacked-stream, run-time-segment and disk level, each checked for holes and overlaps;
   - routines (entry, call targets, address-taken code, handler pointers, accepted flows) with callers, callees, Amiga OS
     library calls (library bases tracked from OpenLibrary results; exec, dos, graphics and intuition vector names),
     custom-chip and CIA registers touched, and referenced text.
3. **ghidra** - Ghidra 12.1.4 headless (same image) imports one flat image with every unit at a fixed base (run-time
   segments at their real address), applies our labels, flow-disassembles and decompiles every routine.
4. **xcheck** - every capstone instruction against Ghidra's own listing (start and length).
5. **describe** (`describe.py`) - see below.
6. **emit + build** - re-emit with the final names, assemble every unit with vasm (`-Fhunkexe -kick1hunks -no-opt
   -nosym`, or `-Fbin` for boot blocks and run-time segments) and compare. Instructions whose bytes differ are re-emitted
   as raw words and rebuilt (at most four rounds), then the comparison must be identical.
7. **report** - the four proofs, `coverage.json/md`, `functions.md`, state.

## Naming and describing at scale (no paid API)

Each routine climbs the ladder only as far as it must:

1. reuse - a routine already accepted for the same key keeps its name and text;
2. deterministic rules on recovered facts - empty stubs, return-constant stubs, jump stubs, single library-call
   wrappers, standard boot blocks and the ProTracker replay entry points get exact names and texts;
3. the local model on Ryzen (Ollama, the pinned `muse-glimmer:30b`, thinking off, no `num_ctx`, structured JSON output)
   writes a name and a one-or-two-sentence description from the Ghidra pseudo-code plus the recovered facts. One game
   at a time holds the GPU, and the model must already be resident 100% in GPU memory.

Every text passes a quality check before it counts: not empty or too short, no generic wording ("performs
operations", "unknown", "helper function" and similar), no hexadecimal, no library call the routine does not make, no
"empty routine" claim for a routine with real instructions, not a copy of another routine's text, and for unnamed
routines a specific lower_snake_case name. A rejected answer is retried once with the reasons; if it fails again the
routine stays undescribed and the game is blocked on proof (c).

## Queue (`queue.py`)

A background process (setsid/nohup, not a unit, timer or cron job) works through the G5 games, then top-list titles
with a project at G3 or above (best project per title first), then the rest of the top list, then every other project
by highest gate reached. Stage A (prep to build) runs on several worker slots; stage B (naming, re-emit, build, report)
runs one game at a time. It skips finished games, resumes from each game's `state.json` after a restart, pauses when
`/srv/fast` falls below 85 GB free, writes `queue-status.json`, a one-line log per game and the dashboard page, and
commits these no-bytes summaries in batches.

## Known limits

- The reference run covers a few hundred frames, so proof (d) confirms only the code reached early; the rest is
  recovered statically.
- A run-time segment's extent is derived from the code reachable from the executed PCs it contains.
- IPF images, overlaid executables and 16/8-bit relocations are blockers until their stages exist.
