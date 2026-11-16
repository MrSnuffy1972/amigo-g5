# Game 1278 (dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)[non-playable]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01278). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (1 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 25 of 25 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 352 of 352 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1236 |
| data | 1801000 |
| padding | 4 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 1 of 2)[non-playable].adf:bootblock | 1 | 250 | 8 |
| Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf:bootblock | 1 | 102 | 2 |

## Cases handled

- extended reference run ignored: it does not re-execute 559 of the 586 PCs the canonical run logged (2139949 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2139238 of its 2139949 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- prefix run f1000 ignored: it logged 2139922 PCs the canonical run never executed (not a prefix of it)
- prefix run f2000 ignored: it logged 2139922 PCs the canonical run never executed (not a prefix of it)
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 1 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 1 of 2)[non-playable].adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.18)
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 2 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf: blocks 2-879 (bytes 1024-450560) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.20)
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 2 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.06)
- 1 packed track region(s), 449536 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 1 of 2)[non-playable].adf:bootblock

- `branch_into_routine`: Contains a single unconditional branch that jumps forward into the middle of another routine, so it performs no work of its own and never returns.
- `run_boot_setup_loop`: Repeatedly calls a routine pointed to by offset -198 of the structure in a6 until it returns zero, then calls the routine at offset -210.
- `store_header_and_copy_block`: Writes a fixed header into a structure at offsets 36, 40, 44, 28, and 48, then repeatedly calls a routine pointed to by offset 4 of a table until it returns zero.
- `relocate_hunk_data`: Clears a 26-word stack area, then reads a header whose first word is the executable hunk type code 1011; if it is not that code it invokes another routine and returns, otherwise it parses the hunk table, relocating the data and pointer hunks and building a jump table, then adjusts every relocated pointer.
- `call_pointer_offset`: Reads a pointer at offset 24 of the structure in the parameter register and jumps through it, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `jump_and_return`: Jumps through a pointer in a6 and then returns.
- `set_registers_and_branch`: Loads the values one, 255, eight, and seven into registers, pops one value off the stack into a register, and then branches to another location without returning.

### Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf:bootblock

- `initialize_and_process_strings`: Saves a global variable, stores a constant at offset 28 of a structure in a1, calls several routines through pointers derived from the saved value, fills a table with fixed 32-bit values, and then processes a null-terminated string by skipping over entries.
- `walk_null_terminated_string`: Calls a routine via a pointer in a6, then walks a null-terminated string pointed to by a0, advancing one byte at a time until the terminator is reached, and finally jumps through another pointer in a6.
