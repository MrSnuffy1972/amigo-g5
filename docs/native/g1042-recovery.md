# Game 1042 (universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01042). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901588 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 11 of 11 named (0 author symbols), 11 of 11 described (1 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 32 of 32 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 239 of 239 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29567 |
| code | 766 |
| data | 833764 |
| padding | 627 |
| unused | 36864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf:bootblock | 1 | 65 | 2 |
| logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf boot-block load | 1 | 116 | 2 |
| logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: custom-format (non-DOS) disk, 1686 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: the boot block reads 4096 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: the boot block reads 26112 bytes from disk offset 843264 to an address (found statically); recovered as a unit
- 32 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 5 first-run image unit(s) in 1 version group(s); 334 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: 2 logged loads cover 30208 bytes of the image
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: blocks 11-12 (bytes 5632-6656) outside the filesystem hold data, not code (opcode density 0.031, entropy 2.96)
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: blocks 22-1646 (bytes 11264-843264) hold a RNC1: Rob Northen RNC1 Compressor stream (832000 -> 40064 bytes)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf:bootblock

- `setup_and_jump_to_entry`: Sets up the environment by calling a routine at offset -456, opening the trackdisk device, initializing a structure at offset 28 with a long at offsets 36, 40 and 44, and performing various library calls.
- `init_struct_and_call_lib`: Initializes a structure pointed to by a1 by storing a word at offset 28 and a long at offsets 36, 40 and 44, then jumps to a routine at offset -456 of the exec library in a6.

### logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf boot-block load

- `select_half_or_input_word`: Returns half of a word in d6 when the flag in d7 equals one, otherwise it returns the input word in d0 unchanged.
- `masked_table_lookup`: Loads a word from a table indexed by d1 at offset a fixed address, masks it against the value in d6, and returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_infinitely`: Performs an infinite loop by branching to itself.
- `fill_memory_on_interrupt`: Checks bit 3 of the interrupt request register and, if set, stores the value from a register into thirteen consecutive 4-byte locations starting at an offset from a pointer in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_store_constant`: Increments a global counter, calls another routine, and stores the constant 16 into a global variable before returning.
- `check_vposr_and_store_interrupt`: Checks a byte from the video pointer register and, depending on its value, calls another routine before storing the constant 4 into the interrupt request register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_if_zero`: Tests a byte in a global variable and branches to another routine if the byte is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_if_non_zero`: Tests a byte in a global variable and branches to another routine if it is zero; otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
