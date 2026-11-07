# Game 2082 (killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02082). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908208 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 57 of 57 named (0 author symbols), 57 of 57 described (1 by rule, 39 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 619 of 619 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1167 of 1167 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 240590 |
| code | 4184 |
| data | 615711 |
| padding | 5227 |
| unused | 42496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf:bootblock | 1 | 63 | 5 |
| logged disk load from Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf boot-block load | 1 | 120 | 10 |
| logged disk load from Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf boot-block load | 1 | 155 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 829 | 40 |
| Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf bytes 4096-5632 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: custom-format (non-DOS) disk, 1660 non-empty blocks: matched against track-loaded segments
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: the boot block reads 236544 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 619 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: bytes 4096-5632 look like code no run loaded: recovered position-independently (load address unknown)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: 3 logged loads cover 243712 bytes of the image
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 484-698 (bytes 247808-357888) outside the filesystem hold data, not code (opcode density 0.032, entropy 5.16)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 701-806 (bytes 358912-413184) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.04)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 814-816 (bytes 416768-418304) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.97)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 818-996 (bytes 418816-510464) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.65)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 1012-1203 (bytes 518144-616448) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.09)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 1210-1293 (bytes 619520-662528) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.53)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 1296-1397 (bytes 663552-715776) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.72)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 1418-1422 (bytes 726016-728576) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.92)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 1430-1516 (bytes 732160-776704) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.47)
- killing game show, the (demo) (1990)(psygnosis)[h conqueror - zike]/Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf: blocks 1518-1644 (bytes 777216-842240) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.85)
- 1 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 49664 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf:bootblock

- `initialise_memory_descriptors`: Boot-block entry called by the startup strap.
- `copy_blocks_to_low_memory`: Clears two global words, copies a block of 257 longwords to a low-memory address, and copies 14 longwords into another region.
- `raise_trap_with_handler`: Stores a handler address in a global variable and then executes a trap instruction, passing control to that handler.
- `request_memory_and_invoke_block`: Fills a request record with fixed sizes and addresses, calls a library function with it, then calls a routine at a fixed address.
- `configure_request_and_call`: Clears one field of a request record and sets its size field to nine, then calls a library function.

### logged disk load from Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf boot-block load

- `set_disk_sync_registers`: Boot-block code that writes values to the disk-sync and disk-control hardware registers and then calls three unnamed routines.
- `loop_call_with_stride`: Counts a value down to minus one, calling an unnamed routine on each pass with an incrementing index and an address advanced by a fixed stride.
- `toggle_control_bit_and_delay`: Clears the high bit of a control byte, sets bit three and then clears it again, waits through a fixed-length countdown, and returns its input word.
- `set_control_bit_and_delay`: Sets the high bit of a control byte, sets bit three and then clears it again, waits through a fixed-length countdown, and returns its input word.
- `wait_while_status_bit_set`: Sets one bit in a control byte, then calls an unnamed routine repeatedly while a status bit remains set.
- `clear_global_word`: Clears a global 16-bit value and returns.
- `toggle_bit_zero_and_delay`: Sets and then clears bit zero of a control byte, waits through a fixed countdown, and returns its input word.
- `process_status_poll_and_transform`: Sets a hardware control bit, then repeatedly polls a status register until a flag clears or times out.
- `step_word_to_target`: Clears one bit of a control byte and compares the difference between an input word and half of a second input with a stored value.
- `scan_sync_marker_and_copy`: Scans memory word by word for a fixed sync marker word, then copies a fixed-length run of words from the data after it, calling an unnamed routine for each word.

### logged disk load from Killing Game Show, The (demo) (1990)(Psygnosis)[h Conqueror - Zike].adf boot-block load

- `store_low_byte_return_shifted`: Reads a longword just before the input pointer, stores its low byte into an output location, and returns the value shifted right by one with the top bit set.
- `shift_bits_refill_on_zero`: Shifts a bit word right once per pass for a count taken from the input.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sync_sequence`: Calls several subroutines, sets a global variable, and executes a software trap.
- `execute_trap_sequence`: Checks a global variable, updates it with a constant, and executes a software trap.
- `trigger_disk_sync_2`: Sets a global variable and executes a software trap.
- `search_and_clear_table`: Searches through a data stream for a specific byte value and, upon finding it, clears a series of memory locations based on a calculated offset and a table entry.
- `process_audio_sequence`: Manages a counter in a global variable, performs delays, and copies a series of values into a memory range.
- `update_audio_parameters`: Updates audio control structures and hardware registers based on a command stream and a global variable.
- `process_audio_command_stream`: Parses a command structure to update audio control parameters and hardware-related data.
- `update_audio_control`: Calculates a value based on a global variable and a memory offset, then writes it to a structure while interacting with audio and DMA hardware registers.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_control_structure`: Modifies a control structure based on a four-way command mode determined by a masked nibble.
- `tiny_ori`: Small routine of 1 instruction that merges 112 into d3.
- `execute_repeated_calls`: Executes two specific routines repeatedly in a loop for a fixed number of iterations determined by a local counter.
- `execute_repeated_subroutines`: Executes two subroutines twice in a loop while decrementing a local counter.
- `execute_nested_loops`: Executes a series of nested loops that repeatedly call itself a fixed number of times.
- `trigger_overflow_wait`: Calls another routine multiple times, sets a global variable, and enters a loop that waits for an integer overflow to occur when adding a local value to that global variable.
- `execute_nested_loops_3`: Performs nested loops that call another routine while decrementing two local counters.
- `execute_recursive_overflow_wait`: Calls itself multiple times and sets a global variable before entering a loop that waits for an addition to trigger an integer overflow.
- `run_nested_loops_with_init`: Initializes a global variable to a maximum value before executing nested loops that decrement local counters.
- `wait_for_overflow_short`: Waits for an integer overflow to occur by adding a local value to a global variable.
- `wait_for_overflow_long`: Waits for an integer overflow to occur by adding a local value to a global variable after several preceding calls.
- `wait_for_overflow_extended`: Waits for an integer overflow to occur by adding a local value to a global variable after multiple preceding calls.
- `parse_and_repack_data`: Searches for specific tags in a memory block, repacks three bytes into a single word, and then iterates through a data block.
- `copy_memory_block`: Copies a block of data from a source to a destination, handling signed byte lengths and potentially performing bitwise operations during the transfer.
- `wait_for_overflow`: Enters a loop that waits for a mathematical calculation involving a global variable and a register value to result in an integer overflow.
- `wait_for_scaled_overflow`: Enters a loop that waits for a bit-shifted value, scaled by a factor, to cause an integer overflow when added to a global variable.
- `iterate_null_terminated_strings`: Iterates through a list of null-terminated strings and calls another routine for each string found.
- `apply_mask_to_strings`: Iterates through a list of null-terminated strings and applies a bitwise OR operation to memory using a table.
- `apply_bitwise_mask`: Performs a bitwise OR operation on blocks of memory using values retrieved from a table indexed by a byte value.
- `dispatch_interrupt_handler`: Checks specific bits in a global variable to determine which subroutine to call.
- `update_interrupt_request`: Checks specific bits in a global variable and updates that variable with a single bit value based on the highest bit found.
- `set_interrupt_request`: Sets a bit in a global variable that corresponds to a hardware interrupt request register.
- `set_interrupt_request_high`: Sets a different value in a global variable that corresponds to a hardware interrupt request register.
- `set_trap_variable`: Sets a global variable to a constant value and executes a software trap.
- `increment_global_counter`: Increments a global variable and conditionally increments it again based on its value and bitwise properties.
- `trigger_disk_sync`: Sets a global variable to a specific value and executes a system trap.
- `poll_hardware_status`: Polls a global variable in a loop, checking specific bits and value ranges, likely monitoring hardware status via the CIAA register.
- `clear_memory_range`: Fills a large block of memory at a fixed address with zeros.
- `execute_nested_loops_2`: Executes nested loops that perform overflow waits and nibble decrements.
- `update_nibble_bits`: Compares the nibbles of two values and sets specific bits in a memory location if they do not match.
- `decrement_nibble_values`: Decrements each non-zero nibble within a word stored at a memory location pointed to by a register.
