# Game 1042 (universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01042). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1005744 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 26109 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 26151 of 26153 (99.99%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 26349 of 26349 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29567 |
| code | 105108 |
| data | 833578 |
| padding | 627 |
| unused | 36864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf:bootblock | 1 | 65 | 2 |
| logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf boot-block load | 1 | 116 | 2 |
| logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26168 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |

## Cases handled

- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: custom-format (non-DOS) disk, 1686 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: the boot block reads 4096 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)[a]/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6)[a].adf: the boot block reads 26112 bytes from disk offset 843264 to an address (found statically); recovered as a unit
- runtime segment an address: 26152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
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

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `fill_a3_negative_offsets_with_a0`: Stores the value of a0 into 59 consecutive longword slots at negative offsets from a3, then enters a loop comparing a pointer against a limit near a fixed address.
