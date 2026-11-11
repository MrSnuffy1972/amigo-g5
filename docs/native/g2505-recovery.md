# Game 2505 (deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02505). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902544 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (0 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 431 of 431 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1069 of 1117 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6256 |
| code | 2750 |
| data | 893538 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf:bootblock | 1 | 64 | 2 |
| logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load | 1 | 252 | 20 |
| logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load | 1 | 278 | 10 |
| logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load | 1 | 163 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 360 | 19 |

## Cases handled

- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: the boot block reads 1536 bytes from disk offset 2560 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: the boot block reads 4096 bytes from disk offset 890880 to an address (found statically); recovered as a unit
- runtime segment an address: 259 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: 3 logged loads cover 8704 bytes of the image
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: blocks 1748-1759 (bytes 894976-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- 1 packed track region(s), 885248 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_system_state` (runtime segment an address): Updates several global variables and pointers, modifies a bitmask in a global byte, and calls another routine through a pointer. Kind: checksum trap / integrity check. Pass path: the instruction following the call (bsr.w an address), specifically the execution of 'rte' which implies the routine is called from an interrupt or exception handler context, or a return to the main loop..

## Functions


### Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf:bootblock

- `validate_memory_and_relocate_boot_data`: Traverses a memory list to verify available RAM bounds before calling a function to allocate a buffer.
- `initialize_system_and_copy`: Performs I/O operations, traverses a linked list to validate memory, and copies a block of data to a new location.

### logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load

- `initialize_engine_data_structures`: Initializes a series of memory blocks by copying data from fixed locations and writing specific instruction sequences into a target memory area.
- `relocate_relative_pointers`: Iterates through a list of offsets to update a table of pointers by adding a base address to each value.
- `concatenate_registers`: Combines two 32-bit registers into a single 64-bit return value after calling another routine twice.
- `decompress_bitstream_data`: Decompresses a custom bitstream-encoded data block into a destination memory buffer.
- `write_dma_control_constants`: Writes a sequence of four constant values to a global variable and modifies the DMA control register.
- `execute_bit_manipulation_sequence`: Performs a sequence of bitwise operations and status updates while waiting for specific bits to change in memory.
- `toggle_bit_and_wait_for_flag`: Toggles the third bit of a byte at a specific offset within a memory block based on a register bit, clears two other bytes, and then waits for a bit to be set in a different memory location before clearing a final bit.
- `update_status_flags_2`: Sets bits in a memory location, writes zero or specific values to several offsets of a pointer, and waits for a bit to be set in a memory location.
- `update_status_and_poll`: Performs several subroutine calls, modifies bytes at specific offsets within a memory block, and waits for a bit to be set in a byte.
- `increment_and_toggle_bits`: Increments a global variable with a wrap-around check and toggles bits in a memory location while waiting for a bit to be set.
- `wait_for_bit_and_clear`: Writes values to offsets of a pointer, waits for a specific bit in a memory location to become set, and then clears that bit.
- `set_bits_and_wait`: Sets specific bits in a memory location, performs a subroutine call, clears a bit, and then waits for a bit in another location to be set.
- `set_variable_bit_and_wait`: Clears a bit, sets a bit at a position determined by a value, waits for a bit in another location to be set, and then clears that bit.
- `set_multiple_bits_and_wait`: Sets several bits in a memory location, performs multiple subroutine calls to manipulate bits, and waits for a bit in another location to be set.
- `clear_bit_and_wait`: Clears a bit in a memory location, performs a subroutine call, sets a bit, and then waits for a bit in another location to be set.
- `wait_for_bit_and_return`: Sets a global variable to zero and waits for a specific bit in a memory location to be cleared before returning a value.
- `wait_for_bit`: Waits until a specific bit in a memory location is cleared.
- `set_specific_bit_and_wait`: Sets a specific bit in a memory location and then waits for a bit in another location to be set.
- `update_status_flags`: Sets bits in a memory location, writes zero or specific values to several offsets of a pointer, waits for a bit to be set in a memory location, and then clears a bit.
- `poll_status_bit_loop`: Calls another routine and loops while a specific bit in a memory location remains set.

### logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load

- `return_empty_stack_3`: This routine performs no operations and returns a value from the stack.
- `verify_checksum_and_populate_table`: Scans memory for a specific marker to validate data integrity using bitwise XOR checksums.
- `verify_and_interleave_bitstream`: Validates a series of data blocks using bitwise XOR checksums and reconstructs original byte values by interleaving bits from multiple source locations.
- `validate_and_map_checksum_blocks`: Searches for specific data markers to identify valid blocks, performs bitwise XOR checksum verification using bit-interleaving patterns, and populates a table with pointers to the validated data structures.
- `clear_multiple_memory_locations`: Sets several word and long values to zero at various offsets relative to a global variable and a pointer.
- `validate_and_reconstruct_bitstream`: Iterates through a data structure to verify bitwise XOR checksums and reconstructs original data by interleaving bits from multiple sources.
- `return_empty_stack_2`: This routine performs no operations and immediately returns to the caller.
- `verify_data_integrity_and_reconstruct`: Iterates through a data structure to perform bitwise XOR checksum calculations and compares the results against expected values.
- `verify_checksum_and_data`: Iterates through a data structure to perform bitwise XOR checksum calculations and compares the results against expected values stored in a table.
- `return_empty_stack`: This routine is empty and performs no operations before returning.

### logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load

- `decompress_bitstream_data_2`: Copies a block of memory from one location to another and then performs a bitstream decompression loop that reads values via perform_recursive_bit_shift and shift_and_store_word to write bytes into a destination buffer.
- `shift_and_store_word`: Shifts a value right by one bit, sets the highest bit, and stores the result through a pointer.
- `perform_recursive_bit_shift`: Performs a recursive bit-shifting operation on a value until it reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_libraries`: Performs a system trap and uses pointers from a global variable to call several functions and modify memory through an offset.
- `call_subroutines_repeatedly`: Calls another routine five times in succession after clearing a global variable.
- `initialize_data_tables`: Performs a loop that writes specific constant values into two separate memory tables.
- `populate_incrementing_tables`: Fills two memory tables with values that increment by a fixed amount during each iteration of a loop.
- `update_system_state`: Updates several global variables and pointers, modifies a bitmask in a global byte, and calls another routine through a pointer.
- `xor_obfuscated_data_loop`: XORs a byte from a global variable with a value from a register and stores the result into a memory location in a loop.
- `write_obfuscated_data`: Performs a loop that XORs a byte from a global variable with a value from a register and stores the result into a memory location.
- `clear_memory_block_small`: Fills a specific range of memory with zeros.
- `clear_memory_block_large`: Fills a large range of memory with zeros using a loop.
- `load_character_attributes_2`: Loads character attributes by using a byte from a pointer to index into a table and copying data into a destination structure.
- `toggle_global_byte`: Toggles a global byte between zero and all bits set, then enters a loop that repeatedly calls itself.
- `toggle_global_byte_and_loop`: Inverts the bits of a global byte between zero and all bits set, then enters a recursive loop that continues until a specific value is returned.
- `toggle_byte_and_loop`: Toggles the value of a global byte between zero and all bits set, then enters a loop that repeatedly calls itself until a specific value is returned in a register.
- `invoke_character_attribute_loading`: Calls a routine to load character attributes by using a byte from a pointer to index into a table and copying data into a destination structure.
- `load_character_attributes`: Uses a byte from a pointer to index into a table and copies several bytes from that table into a destination structure.
- `increment_multiple_tables`: Runs two loops that increment bytes at regular intervals across three different memory locations.
- `copy_memory_block`: Copies a block of memory from one location to another using a loop.
- `write_constant_and_call`: Writes a specific constant value to a global variable multiple times and then calls a function through a pointer.
- `execute_system_calls_and_updates`: Updates several memory locations, performs a system trap, and executes multiple subroutines via pointers.
