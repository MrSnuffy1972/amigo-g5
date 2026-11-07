# Game 1686 (switchblade ii (1991)(gremlin)[cr ep][t +5 ep]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01686). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911200 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 41 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (1 by rule, 26 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 387 of 387 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1416 of 1416 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28427 |
| code | 6390 |
| data | 872799 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf:bootblock | 1 | 50 | 3 |
| logged disk load from Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf boot-block load | 1 | 163 | 2 |
| logged disk load from Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf boot-block load | 1 | 85 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 635 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 365 | 22 |
| Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf bytes 7168-9216 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- switchblade ii (1991)(gremlin)[cr ep][t +5 ep]/Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- switchblade ii (1991)(gremlin)[cr ep][t +5 ep]/Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf: the boot block reads 20480 bytes from disk offset 610304 to an address (found statically); recovered as a unit
- switchblade ii (1991)(gremlin)[cr ep][t +5 ep]/Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf: the boot block reads 6144 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 142 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 37 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 171 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- switchblade ii (1991)(gremlin)[cr ep][t +5 ep]/Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf: bytes 7168-9216 look like code no run loaded: recovered position-independently (load address unknown)
- switchblade ii (1991)(gremlin)[cr ep][t +5 ep]/Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf: 3 logged loads cover 28672 bytes of the image
- 1 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 867840 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_disk_dma_transfer` (bootblock): Writes the disk sync word, the disk transfer length, the disk control word and the DMA control register, which sets up a disk DMA transfer. Kind: disk hardware/sync validation. Pass path: The branch taken immediately following the call (typically the one that proceeds to the next stage of the loader, whereas the failure branch leads to a hang, reset, or error loop)..
- `copy_memory_block` (runtime segment an address): Calculates a source and destination, then copies a block of bytes from a source table to a destination buffer using a countdown loop. Kind: disk/track/size check. Pass path: the loop continues or returns via the iVar1 == 0 check, specifically avoiding the exit condition that would trigger a failure or a different routine (like a checksum or invalid track error).
- `sync_global_variable` (runtime segment an address): Increments or decrements a global variable until it matches a value derived from a hardware register, then sets or clears a bit in a byte at a pointer. Kind: track/sync check. Pass path: the loop terminates when uVar1 == u7__d0_02B8, proceeding to modify pbVar2 and return.

## Functions


### Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf:bootblock

- `perform_exec_io_request`: Makes one exec library DoIO call, which runs an I/O request through the operating system, and calls one other unnamed routine.
- `call_library_function_at_offset`: Calls a library routine through the vector offset -456 and calls three other unnamed routines.
- `configure_disk_dma_transfer`: Writes the disk sync word, the disk transfer length, the disk control word and the DMA control register, which sets up a disk DMA transfer.

### logged disk load from Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf boot-block load

- `pack_byte_and_shift_longword`: Reads the longword just before its input pointer, stores its low byte combined with a high byte into a memory word, and returns that longword shifted right by one with the top bit set.
- `shift_bits_with_refill`: Shifts a bit register right one position per pass and calls an unnamed helper to reload it whenever it empties.

### logged disk load from Switchblade II (1991)(Gremlin)[cr EP][t +5 EP].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_list_program`: A large routine of 456 instructions that writes the copper list pointer and the copper jump strobe, enables DMA, and reads the fire-button port.
- `restart_copper_with_helpers`: Restarts the copper by writing the copper list pointer and jump strobe and enables DMA, with three helper calls.
- `update_text_entry_buffer`: Reads a key code from a global and clears it when it is below a threshold.
- `call_fixed_address_routine`: Makes one indirect call to a fixed code address and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_copper_wait_state`: Calls another routine and then writes a specific word and byte to memory locations relative to a pointer held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: Enters an infinite loop that prevents further execution of the current thread.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_nibble_threshold_counts`: Loops over a table of packed words, splits each word into three four-bit values, and compares each with a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_variable`: Checks a bit in a global variable and, if set, writes a constant value to that variable before calling another routine and setting it to a new value.
- `update_dma_and_color`: This routine modifies a color register and updates the DMA controller configuration, including enabling a DMA channel.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_table`: Writes specific values to a table and custom hardware registers, then modifies a byte at an offset and clears a global variable.
- `update_byte_bit`: Sets a bit in a byte at a specific offset, performs a countdown loop, and then clears a bit in a byte at a different offset.
- `clear_byte_bit`: Sets a byte at an offset from a pointer to zero and clears a specific bit in a byte at another offset from a pointer.
- `modify_byte_at_offset`: Sets or clears specific bits in a byte at an offset from a pointer and performs a countdown loop.
- `copy_memory_block`: Calculates a source and destination, then copies a block of bytes from a source table to a destination buffer using a countdown loop.
- `interleave_memory_data`: Uses a table-based search to find a source address and copies a block of bytes to a destination, while updating hardware registers and a global variable.
- `set_ram_byte`: Writes a certain byte value to a specific RAM address.
- `search_and_interleave_bits`: Searches a table for a specific short value and performs bitwise masking and shifting on longwords.
- `process_bit_interleave`: Performs bitwise operations and masking on a table of longwords while checking a counter.
- `merge_odd_even_bits`: Combines two longwords by masking one with a fixed bit pattern and shifting the other left by one bit.
- `return_empty_stack`: The routine is empty and returns a value loaded from the stack.
- `sync_global_variable`: Increments or decrements a global variable until it matches a value derived from a hardware register, then sets or clears a bit in a byte at a pointer.
- `sync_global_and_set_bit`: Increments or decrements a global variable until it matches a value derived from a register, then sets or clears a bit in a byte.
- `compare_and_set_bit`: Compares a value from a routine against a global variable while decrementing or incrementing that global variable, then sets a bit in a byte.
- `set_and_mask_byte`: Sets a bit in a byte at a pointer and may clear a bit in that same byte based on a bit test.
- `set_bits_in_byte`: Sets and clears specific bits in a byte at a pointer followed by a countdown loop.
- `clear_and_latch_drive_bit`: Clears the second control bit and the first, then sets the first bit again in the byte at the target address, followed by a fixed countdown loop.
- `clear_custom_register`: Clears a value in a custom hardware register and performs no other operations before returning.
- `update_hardware_and_counters`: Updates hardware registers and counters, and potentially increments a counter or resets a value if a certain condition is met.
- `update_frame_dependent_data`: Calculates an offset into a table based on a timer-like value and updates several memory locations with values from that table.
- `update_timer_value`: Checks a byte at an offset from a pointer and, depending on its value, either decrements or increments a short at another offset, while clamping the result to a certain range and storing it in a third location.
- `mask_and_shift_global`: Masks a global variable with a bitmask and then shifts the result right by seven bits.
