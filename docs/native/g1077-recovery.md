# Game 1077 (ruff 'n' tumble (1994)(renegade)(disk 1 of 2)[cr psg][o]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01077). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912464 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (1 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 286 of 286 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7220 |
| code | 882 |
| data | 900778 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf:bootblock | 1 | 32 | 1 |
| logged disk load from Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf boot-block load | 1 | 124 | 2 |
| logged disk load from Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf boot-block load | 1 | 126 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |

## Cases handled

- ruff 'n' tumble (1994)(renegade)(disk 1 of 2)[cr psg][o]/Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf: custom-format (non-DOS) disk, 1773 non-empty blocks: matched against track-loaded segments
- ruff 'n' tumble (1994)(renegade)(disk 1 of 2)[cr psg][o]/Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf: the boot block reads 2560 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- ruff 'n' tumble (1994)(renegade)(disk 1 of 2)[cr psg][o]/Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf: the boot block reads 5632 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ruff 'n' tumble (1994)(renegade)(disk 1 of 2)[cr psg][o]/Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf: 2 logged loads cover 8192 bytes of the image
- ruff 'n' tumble (1994)(renegade)(disk 1 of 2)[cr psg][o]/Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf: blocks 22-23 (bytes 11264-12288) outside the filesystem hold data, not code (opcode density 0.039, entropy 5.91)
- 3 packed track region(s), 898560 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf:bootblock

- `allocate_and_execute_boot_code`: Performs an I/O operation using a provided structure, allocates a memory block of 5888 bytes, and enters a loop that performs further I/O operations until a specific byte in the allocated memory is not zero, finally jumping to an address stored in the allocated block.

### logged disk load from Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf boot-block load

- `select_half_or_input_word`: Returns half of a word from a register if a flag in d7 is one, otherwise it returns the input word in d0 unchanged.
- `apply_bitmask_to_word`: Calculates an index based on d1, fetches a word from a table at an offset determined by that index, and returns the bitwise AND of that word with the value in d6.

### logged disk load from Ruff 'N' Tumble (1994)(Renegade)(Disk 1 of 2)[cr PSG][o].adf boot-block load

- `initialize_hardware_and_copy_data`: Sets specific values in hardware registers for color and DMA, then copies a block of data from one memory location to another using a loop.
- `setup_tables_and_copy_data`: Populates a table with memory addresses using a loop, calls another routine, and then performs a second loop to copy fifteen words from a source to a destination before writing three values to a structure in a6.
- `copy_signed_bytes_loop`: Contains a loop that reads a byte, extends it to a word, and if the value is not negative, copies bytes from a source to a destination until a counter reaches zero.
- `copy_bytes_with_decrement`: Contains a loop that reads a byte from a source, decrements it, and copies bytes from a second source to a destination until the byte reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_byte_with_carry`: Adds a byte from d0 to d2 with carry and then returns.
