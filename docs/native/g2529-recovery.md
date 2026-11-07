# Game 2529 (Bloodwych - The Extended Levels): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02529). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901520 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 306 of 306 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 52423 |
| code | 1142 |
| data | 234393 |
| padding | 1722 |
| unused | 611840 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| TEST | 1 | 166 | 9 |
| BLOODWYCH | 1 | 46 | 3 |
| Bloodwych - The Extended Levels.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 84 | 3 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Bloodwych - The Extended Levels/Bloodwych - The Extended Levels.adf:C/BLOODWYCH: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### TEST

- `initialize_chipset_and_delay`: Program entry point that calls several unnamed initialization routines, runs a nested countdown delay loop of one hundred outer passes, calls another routine, and stores the largest positive sixteen-bit value in a shared variable.
- `expand_run_length_bytes`: Doubles sixteen shared words, then decodes a run-length byte stream into a block of memory: a negative count repeats the following byte, a positive count copies that many literal bytes, and output advances in rows of forty bytes.
- `loop_mask_twelve_bit_words`: Repeats sixteen times: calls an unnamed routine sixty-four times, masks sixteen shared words to twelve bits, and calls two more unnamed routines.
- `copy_to_color_registers`: Copies sixteen words from a shared variable into the hardware color registers starting at the first one, then zeroes the next sixteen color register words.
- `decrement_word_until_wrap`: Decrements a 16-bit counter through one full cycle of its range, returning when it wraps back to minus one.
- `check_nibble_of_table_byte`: Reads a byte from a shared table at an index taken from a register and extracts a high or low nibble depending on the register's parity.
- `loop_mask_color_words`: Loops sixteen times, each time invoking another routine sixty-four times, then masks sixteen consecutive words from a global variable to 4095 and calls two other routines once each.
- `decrement_nibble_byte`: Reads a byte from a shared table at an index taken from a register, selects its high or low nibble by the register's parity, and when that nibble is nonzero subtracts one step from the byte.
- `configure_custom_chipset`: Writes display window, bitplane modulo, copper and DMA control values into shared variables, loops over four pointer pairs computed from a base address, and rebuilds pointers from eight table words.

### BLOODWYCH

- `disable_interrupts_then_run`: Program entry point that disables every interrupt and DMA channel by writing the clear-everything mask to the interrupt and DMA control registers, copies a block of words into a fixed low memory area, and calls a routine stored there.
- `copy_block_then_call_routine`: Copies a block of words into a fixed low memory area, then calls a routine stored there.
- `decode_marked_run_stream`: Decodes a byte stream into memory: a special marker byte introduces a run whose count and value follow, other bytes are copied through, and the loop runs until the input pointer reaches a fixed upper address.

### Bloodwych - The Extended Levels.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_timed_table_value`: Advances a per-object timer stored in a record: when the timer runs out it sets a reload value, otherwise it decrements the timer.
- `toggle_parity_and_step_counters`: Checks an interrupt-request status bit and returns early when it is set.
- `invoke_parity_toggle`: Invokes the caller routine, which toggles a parity flag and advances two counters, and then returns.
