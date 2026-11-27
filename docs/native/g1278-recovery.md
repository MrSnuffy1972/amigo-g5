# Game 1278 (dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)[non-playable]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01278). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802928 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (1 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 451 of 452 (99.78%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 659 of 659 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 2434 |
| data | 1800436 |
| padding | 58 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 1 of 2)[non-playable].adf:bootblock | 1 | 276 | 9 |
| Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf:bootblock | 1 | 102 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 281 | 3 |

## Cases handled

- extended reference run ignored: it does not re-execute 559 of the 586 PCs the canonical run logged (2139949 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2139238 of its 2139949 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 269 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 1 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 1 of 2)[non-playable].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 1 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 1 of 2)[non-playable].adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.18)
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 2 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf: blocks 2-879 (bytes 1024-450560) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.20)
- dragon's lair - escape from singe's castle (demo) (1989)(readysoft)(ntsc)(disk 2 of 2)[non-playable]/Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.06)
- 1 packed track region(s), 449024 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

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
- `process_rotated_bit_table`: Rotates a 32-bit value from d0 by the amount in d7, then if a bit is set it decrements the pointer in a0, clears a 16-bit word and a following word, stores the rotated value at two 16-bit offsets, writes several fixed 32-bit constants, and finally walks a NUL-terminated table calling a handler routine at a6-240 for each entry.

### Dragon's Lair - Escape From Singe's Castle (demo) (1989)(ReadySoft)(NTSC)(Disk 2 of 2)[non-playable].adf:bootblock

- `initialize_and_process_strings`: Saves a global variable, stores a constant at offset 28 of a structure in a1, calls several routines through pointers derived from the saved value, fills a table with fixed 32-bit values, and then processes a null-terminated string by skipping over entries.
- `walk_null_terminated_string`: Calls a routine via a pointer in a6, then walks a null-terminated string pointed to by a0, advancing one byte at a time until the terminator is reached, and finally jumps through another pointer in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_addresses_branch`: Compares the address in a1 with the address in a2 and branches if a1 is less than a2, otherwise it returns.
- `rotate_and_split_bytes`: Rotates the 32-bit value in d0 right by the amount in d7 and splits the result into a byte pair, then repeatedly rotates and shifts those bytes right by the same amount while reading successive bytes from the buffer pointed to by a0 and writing them through the pointer in a2 until the high bits are exhausted.
- `rotate_and_copy_bytes`: Rotates the byte in d0 right by the amount in d5 and shifts the bits of the buffer pointed to by a0 into uVar6, then copies two bytes from a table indexed by uVar8 in a6 to offsets -2 and -1.
