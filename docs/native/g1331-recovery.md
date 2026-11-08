# Game 1331 (rick dangerous (1989)(micro style)[cr oracle][t +4 oracle]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01331). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 115 of 115 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 246 of 246 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16 |
| code | 648 |
| data | 189614 |
| padding | 1722 |
| unused | 709120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| Rick Dangerous (1989)(Micro Style)[cr Oracle][t +4 Oracle].adf:RICK | 2 | 119 | 3 |
| Rick Dangerous (1989)(Micro Style)[cr Oracle][t +4 Oracle].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0_and_branch`: Copies the 16-bit value from register d1 into register d0, then jumps unconditionally into the middle of another routine.

### Rick Dangerous (1989)(Micro Style)[cr Oracle][t +4 Oracle].adf:RICK

- `process_buffer_with_bit_rotation`: Initializes pointers from a global table and loads a longword to process, then repeatedly calls helper routines with different arguments and branches based on their results.
- `fetch_shifted_bit_word`: Loads a longword from the address in a0 into register d0, sets the condition code register, and rotates that longword right by one bit.
- `shift_and_rotate_until_counter_zero`: Decrements a 16-bit counter in register d1, clears register d2, then shifts register d0 right by one bit and returns if the result is zero.

### Rick Dangerous (1989)(Micro Style)[cr Oracle][t +4 Oracle].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
