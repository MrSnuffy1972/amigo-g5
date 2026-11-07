# Game 2525 (bloodwych (1989)(imageworks)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02525). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 464 of 464 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 391504 |
| code | 1652 |
| data | 55424 |
| padding | 956 |
| unused | 451584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| TEST | 1 | 166 | 9 |
| BLOODWYCH | 1 | 288 | 11 |
| Bloodwych (1989)(ImageWorks)[a].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### TEST

- `run_startup_and_delay`: Calls several initialisation other routines, runs a nested countdown loop, and then writes a fixed word to a global cell.
- `decode_rle_stream_into_rows`: Doubles sixteen consecutive words in a global table, then decodes a run-length compressed stream into a buffer in rows of forty bytes.
- `loop_mask_color_words`: Repeats a sixteen-pass loop that calls a other routine sixty-four times and then masks sixteen words of a global table to their low twelve bits.
- `copy_to_color_registers`: Copies sixteen words from a global table into the hardware colour registers, then writes zero to the following sixteen register words.
- `busy_wait_fixed_count`: Runs a full 16-bit countdown loop doing no other work, then returns.
- `increment_nibble_byte`: Selects the high or low nibble of a table byte by the parity of an index, then compares the two nibbles of that byte.
- `loop_mask_color_words_2`: Loops sixteen times, each time calling a routine sixty-four times, then masks sixteen words from a global variable to 4095 and calls two other routines.
- `decrement_nibble_byte`: Selects the high or low nibble of a table byte by the parity of an index and, when that nibble is nonzero, subtracts a fixed amount from the byte.
- `build_display_setup_block`: Writes a block of display-setup constants into a register image, then fills bitplane pointer slots in a structure, stepping each pointer by a fixed row distance.

### BLOODWYCH

- `access_interrupt_control_registers`: The decompiler timed out on this ten-instruction program entry.
- `read_mouse_port_state`: The decompiler timed out on this sixty-two-instruction routine.
- `absolute_difference_bytes`: Subtracts one byte from another and returns the absolute value of the difference.
- `update_clamped_position_state`: When a mode flag is clear it stores values derived from a returned word into several cells.
- `write_bytes_to_buffer`: Derives four byte values from the 32-bit input and stores each at a fixed offset in the buffer the address register points to.
- `adjust_position_with_flags`: Reads a 32-bit position from a structure, nudges its low and high words according to flag bits in the input byte and the structure's boundary bytes, and wraps the high word within a fixed range.
- `compute_bitmask_from_input`: Derives several intermediate bits from a 16-bit input by shifting and XOR-ing it, then combines the bits, with carry results, into a single byte.
- `propagate_input_bytes`: Reads two port-derived values through other routine calls and combines them with a global cell into a 32-bit value.
- `update_color_register_state`: Advances a per-object counter stored in a structure when the global flag is clear, reloads it when it expires, and selects a value from a table indexed by the stored state and parity bits.
- `toggle_globals_and_decrement`: Returns early when one flag bit is set.
- `toggle_and_write_short_table`: Toggles a global between two fixed values, then writes four table entries.

### Bloodwych (1989)(ImageWorks)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
