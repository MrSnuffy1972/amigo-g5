# Game 2505 (deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02505). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902544 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 63 of 63 named (0 author symbols), 63 of 63 described (2 by rule, 60 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 431 of 431 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1161 of 1161 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6162 |
| code | 3374 |
| data | 893008 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf:bootblock | 1 | 64 | 2 |
| logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load | 1 | 409 | 31 |
| logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load | 1 | 135 | 2 |
| logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load | 1 | 163 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 390 | 25 |

## Cases handled

- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: the boot block reads 1536 bytes from disk offset 2560 to an address (found statically); recovered as a unit
- deliverance (1992)(21st century)(disk 1 of 3)[cr scx][t +10 atx]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf: the boot block reads 4096 bytes from disk offset 890880 to an address (found statically); recovered as a unit
- runtime segment an address: 259 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
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
- `check_and_set_dma_flags`: Initializes several hardware registers with specific values, copies a long value from one location to another, checks for certain conditions in memory, and returns early if those conditions are met.
- `initialize_dma_control`: Sets the DMA control register to 16384 and returns a value of -1.
- `validate_and_process_data`: Validates data by comparing checksums and performing bit operations on byte pairs from a buffer, storing processed results into another memory location.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
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
- `save_registers_and_call`: Saves all general-purpose and address registers onto the stack, calls three other routines, restores the registers from the stack, and then returns.
- `initialize_graphics_and_dma`: Opens a library, clears a large memory block, initializes several hardware registers with specific values, sets up jump table entries, copies data from one location to another, and performs conditional operations based on byte values.
- `copy_and_increment`: Copies data from one memory location to another, increments a pointer, and performs a loop that copies bytes while incrementing both pointers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `test_joystick_bit`: Tests bit 6 of a hardware register and branches if the bit is set, otherwise clears a memory location to zero and continues execution.
- `check_potgor_and_set_flag`: Tests a specific bit in a hardware register, branches if the bit is set, sets a memory location to 1, and then returns.
- `setup_interrupt_flags`: Writes 49152 to a memory location, modifies a word value from another location by setting bit 15, writes the modified value twice to different locations, copies a long value from one structure to another, and then returns.

### logged disk load from Deliverance (1992)(21st Century)(Disk 1 of 3)[cr SCX][t +10 ATX].adf boot-block load

- `copy_memory_and_setup`: Copies data between memory locations, sets up various hardware registers with specific values, writes jump table entries, and performs several conditional checks before returning.
- `return_empty_stack_3`: This routine performs no operations and returns a value from the stack.

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
- `compare_version_string`: Loads addresses into multiple registers, compares a long word value with a fixed address, branches if not equal, and then returns immediately.
- `increment_digit_buffer`: Compares a byte value with 57, branches if not equal, increments another byte, sets the first byte to 48, then loops back to continue processing.
- `increment_bytes_and_pointer`: Increments bytes at two memory locations, increments a pointer, and branches back to an earlier instruction in the same routine.
- `compare_and_load_strings`: Loads addresses into multiple registers, compares a long word value with a fixed address, branches if not equal, and then returns immediately.
- `compare_and_modify_byte`: Compares a byte value with 48, branches if not equal, decrements another byte, sets the first byte to 57, and then branches back to an earlier instruction.
- `decrement_and_load_attributes`: Decrements bytes at two memory locations, increments a pointer, and repeatedly calls another routine until a condition is met.
- `invoke_character_attribute_loading`: Calls a routine to load character attributes by using a byte from a pointer to index into a table and copying data into a destination structure.
- `load_character_attributes`: Uses a byte from a pointer to index into a table and copies several bytes from that table into a destination structure.
- `increment_multiple_tables`: Runs two loops that increment bytes at regular intervals across three different memory locations.
- `copy_memory_block`: Copies a block of memory from one location to another using a loop.
- `write_constant_and_call`: Writes a specific constant value to a global variable multiple times and then calls a function through a pointer.
- `execute_system_calls_and_updates`: Updates several memory locations, performs a system trap, and executes multiple subroutines via pointers.
