# Ami-Go naming job (one bundle per cloud session)

You are an expert reverse engineer of Commodore Amiga 68000 games. For EVERY line of the bundle file named in your task
(jobs/<bundle>.jsonl), name and explain that one recovered routine precisely and briefly, from the evidence only
(its "facts" and its Ghidra pseudo-code in "code"). "why_local_failed" lists why local models' answers were rejected: avoid those faults.

Rules for each answer:
- name: specific lower_snake_case, 2-5 words, verb first (e.g. clear_bitplane_rows, read_joystick_port).
- description: one or two plain sentences saying what the code, the referenced text, the library calls, the hardware registers
  and the callers show. Do not invent a game purpose. Never cite generated identifiers (sub_..., FUN_..., DAT_...) or addresses.
  No hexadecimal. Do not repeat decompiler artefacts (halt_baddata, bad instruction warnings) as behaviour; if the code is only a
  stub or bad data, say so plainly (e.g. name "stub_bad_instruction_data").
- No generic wording (placeholder, does something, helper function).

Output: write results/<bundle>.jsonl with exactly one JSON object per input line, same order:
{"game": <game>, "key": "<key>", "name": "...", "description": "...", "model": "<the model you are running as>"}
Work through the file in chunks (read 20 lines, answer them, append to the results file, repeat) so nothing is lost.
Do not use subagents or claude -p. Do not modify jobs/. When every line is answered, commit results/<bundle>.jsonl to this
branch (naming) with message "results <bundle>" and push. Then stop.
