# Game 675 (manchester united - europe (1991)(krisalis software)(m5)[cr afl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00675). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 26 of 26 named (0 author symbols), 26 of 26 described (7 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 13 of 13 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 28672 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 627 of 627 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 620572 |
| code | 2672 |
| data | 166468 |
| padding | 3888 |
| unused | 107520 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| AFLintro | 2 | 367 | 20 |
| wait | 2 | 117 | 2 |
| Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:game | 1 | 16 | 1 |
| Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:bootblock | 1 | 10 | 1 |
| Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:0016 (raw code file, load address unknown) | 1 | 0 | 0 |
| Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:AFL!0016 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- manchester united - europe (1991)(krisalis software)(m5)[cr afl]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:AFL!0479: high-entropy data file (7.71 bits/byte), classified as packed data
- manchester united - europe (1991)(krisalis software)(m5)[cr afl]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:0016: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- manchester united - europe (1991)(krisalis software)(m5)[cr afl]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:AFL!0016: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 28672 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- AFLintro

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_then_branch`: Copies the 16-bit value from d1 into d0 and then branches into the middle of another routine, so it never returns on its own.

### AFLintro

- `invoke_trap_vector`: Loads the number 14 into the trap vector register and invokes the trap, which calls the entry point of the library routine.
- `setup_bitmap_and_install_interrupt`: Clears two 6000-word memory regions and fills a bitmap buffer with a 7x48 pattern, then builds a palette table and installs a graphics.library interrupt handler that runs a custom routine on each frame.
- `handle_keyboard_and_timer`: Saves the processor registers on the stack, checks a keyboard status byte and returns to the keyboard handler when a key is pressed, then checks a global counter and either decrements it, resets it, or jumps to a key-processing routine when it underflows.
- `rotate_pointers_and_run_blitter`: Rotates four linked-list pointers left by one position while shifting a global counter left by one bit, then calls several helper routines and invokes a blitter sequence when the counter reaches a threshold or a flag is set.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `do_nothing_r2`: Consists of a single ori.b #0, d0 instruction that is a no-op, so the routine does nothing.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d6.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d7.
- `tiny_ori_4`: Small routine of 1 instruction that merges 54 into d6.
- `tiny_ori_5`: Small routine of 1 instruction that merges 54 into d7.
- `decrement_global_when_nonzero`: Tests a global word and returns immediately when it is zero, otherwise it decrements that word by one and then returns.
- `program_blitter_transfer`: Programs a blitter transfer by writing source and destination addresses, a modulo value, a size, and a command word to the blitter control registers, then polls a global counter until it reaches a threshold before resetting it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_list_and_check_marker`: Reads a byte from a linked-list pointer, computes an address, writes a series of blitter commands, then advances the pointer and branches to a caret handler when it finds a marker.
- `handle_caret_marker`: Checks the byte at a0 and, if it equals '^', stores the following two bytes into two global variables, copies them into two hardware registers, and adds 3 to a counter.
- `store_byte_register`: Writes the low byte of the data register d0 to the byte pointed to by the address register a0.
- `run_blitter_sequence_loop`: Waits for a hardware flag to clear, advances and wraps a counter, then loops writing blitter commands and reading a table until a second counter reaches 40.
- `setup_blitter_copy`: Waits for the vertical blank to start, then fills a table of 14 addresses with values computed from a counter and a base address, and finally programs the blitter to copy 10 blocks of data from that table into a screen buffer.
- `set_d0_to_zero`: Initializes the d0 register to zero and then does nothing else.
- `process_table_and_blitter_loop`: Advances and wraps a counter, then loops transforming a table of words, clamping values, and writing blitter commands until a counter reaches 180.

### wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_d1_to_d0`: Copies the value from register d1 into register d0 and then jumps to the routine that finds the maximum value and prepares the buffer.

### Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:game

- `initialize_system_and_jump`: Disables interrupts, clears DMA, sets up the stack and registers, copies saved registers into a low-memory area, and jumps there to begin execution.

### Manchester United - Europe (1991)(Krisalis Software)(M5)[cr AFL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
