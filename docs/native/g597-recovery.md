# Game 597 (Carrier Command): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00597). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (1 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 81 of 81 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 260 of 260 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 908 |
| data | 163174 |
| padding | 1062 |
| unused | 735744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Carrier Command.adf:load | 1 | 250 | 16 |
| Carrier Command.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- Carrier Command/Carrier Command.adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Carrier Command.adf:load

- `disable_dma_and_interrupts`: The routine disables DMA channels and interrupts by writing to DMACON and INTENA, then calls another routine, saves the current supervisor state with exec.SuperState, calls another routine, restores the supervisor state, and calls a third routine.
- `decode_bitfield_to_ram`: The routine repeatedly reads a 32-bit value from another routine and expands its bits into a RAM buffer, writing bytes derived from the bit stream into a table that is moved down by a small offset each iteration.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.
- `reset_graphics_dma_state`: The routine copies a global variable table to a fixed memory area, saves and temporarily replaces a pointer in a library structure, disables DMA by writing to DMACON, clears a memory block, calls library routines, restores the saved pointer and global variable, and loops calling another routine for each entry in a name table.
- `clear_memory_table`: The routine clears a block of memory by writing zeros to successive words, then calls three other routines via indirect jumps, sets a global variable to two different base addresses in turn, and iterates over a table of 32-byte entries terminated by a zero byte, invoking another routine twice for each entry.
- `call_two_function_pointers`: The routine calls two functions through pointers stored in a table referenced by a register.
- `poll_input_wait_loop`: The routine runs a loop that repeatedly calls another routine three times per iteration, resetting a counter when it reaches zero, and continues while a flag bit in a global variable is set.
- `wait_for_null_terminated_string`: The routine repeatedly assigns a global variable address to an uninitialized pointer and tests the pointed byte for zero, then calls another routine twice.
- `call_indirect_table_entry`: The routine loads an address from a global variable and calls the routine stored at an offset from that base.
- `wait_for_global_flag`: The routine contains no instructions and therefore performs no operation.
- `write_table_entry_for_pointer`: The routine copies a 16-bit value from a register into a 16-bit location in a table referenced by a pointer.
- `expand_message_table`: The routine copies bytes from a table at a fixed address into a destination buffer pointed to by a global pointer, inserting a sentinel between entries, and expands repeated byte runs by decoding a count and a fill byte.
- `decode_message_bytes`: The routine walks through a source buffer that contains length-prefixed bytes and a sentinel marker, copies the payload bytes into a destination buffer while decrementing a length counter, and skips over the sentinel and length fields in the source.
- `shift_and_copy_bytes`: The routine processes a source buffer through a destination pointer, shifting each non-terminated block one byte forward and then copying the remainder of the block up to a sentinel byte.
- `copy_string_with_terminator`: The routine copies bytes from a source pointer through a destination pointer, handling a count byte and a terminator byte, and writes a terminator to the destination.

### Carrier Command.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
