# Game 1407 (seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01407). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (924448 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 181 raw-word lines |
| Functions named and described | pass, 66 of 66 named (0 author symbols), 66 of 66 described (2 by rule, 64 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 736 of 736 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2185 of 2185 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11622 |
| code | 8302 |
| data | 900406 |
| padding | 22 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf:bootblock | 1 | 244 | 28 |
| logged disk load from Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf boot-block load | 1 | 72 | 3 |
| logged disk load from Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf boot-block load | 1 | 72 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 304 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1493 | 28 |

## Cases handled

- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: the boot block reads 5120 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: the boot block reads 5120 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 254 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 482 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: 2 logged loads cover 5120 bytes of the image
- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: blocks 1648-1650 (bytes 843776-845312) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.63)
- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: blocks 1652-1656 (bytes 845824-848384) outside the filesystem hold data, not code (opcode density 0.026, entropy 2.93)
- seek & destroy (1993)(mindscape)(disk 1 of 4)[cr dlm]/Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf: blocks 1663-1759 (bytes 851456-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.08)
- 1 packed track region(s), 837120 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `synchronize_hardware_and_delay` (bootblock): Configures CIA and interrupt enable registers before performing a series of calls to another routine to implement a timing delay. Kind: timing-based disk synchronization/track check. Pass path: the loop where sVar2 is decremented via u0__sub_02DC (the 'else if' branch) or the loop where sVar2 is decremented via u0__sub_02F8 (the 'if' branch), leading to the subsequent hardware synchronization calls..
- `flip_bit_in_global` (runtime segment an address): Sets or clears a specific bit in a global variable based on a value and an offset. Kind: bit-flip/checksum trap. Pass path: the branch where the bit is cleared/set without triggering a failure or a change in control flow (the path that avoids a jump or a decrement/subtraction that would lead to a failure state or a different routine execution).
- `iterate_and_set_global_bits` (runtime segment an address): Iterates through a loop that checks a bitmask in a global variable and uses a bit-manipulation routine to set or clear bits in a global variable based on a value and an offset. Kind: track/sector check. Pass path: return (DAT_00bfd100 bit 7 cleared or loop exit via _u3__d0_0218 == _u3__d0_0130).

## Functions


### Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf:bootblock

- `initialize_boot_environment`: Performs memory relocation and initialization before setting a global variable to zero and executing a software trap.
- `clear_global_variable`: Sets a global variable to zero.
- `relocate_and_initialize_boot_code`: Copies a block of code from its current location to a higher memory address and then performs a secondary memory copy to populate a specific range.
- `synchronize_hardware_and_delay`: Configures CIA and interrupt enable registers before performing a series of calls to another routine to implement a timing delay.
- `delay_and_wait_for_bit`: Calculates a difference between two values to determine a loop count, performs a delay, and then waits for a specific bit in a global variable to be set.
- `wait_and_return_global`: Performs a series of loops involving waiting for bits and copying memory before returning a value from a global variable.
- `return_global_value`: Returns the value stored in a global variable.
- `copy_memory_words`: Copies a sequence of longwords from a source address to a destination address.
- `copy_memory_words_2`: Copies a sequence of longwords from a source address to a destination address.
- `update_bitfield_and_store_word`: Modifies specific bits within a byte via a pointer based on a condition and then stores a 16-bit value into a global variable using two consecutive byte writes.
- `modify_byte_and_store_words`: Modifies a byte via a pointer by setting and clearing specific bits, then stores two consecutive bytes into a global variable.
- `initialize_hardware_and_wait_loop`: Sets several values in a data structure via a pointer and clears an interrupt register.
- `initialize_data_structure`: Populates a data structure with specific values and enters a loop that decrements a global variable while checking a bit in a memory location.
- `store_word_bytes`: Splits a 16-bit word into two bytes and stores them into a global variable.
- `synchronize_cia_interrupt_state`: Configures CIA control registers and clears specific bits in a global variable before entering a loop that waits for a bit to clear.
- `clear_bit_and_loop`: Clears a bit in a byte via a pointer and executes a loop that decrements a local variable until it reaches a specific value before clearing a global variable.
- `modify_bit_at_offset`: Sets or clears a specific bit in a byte using a value from a global variable as an offset.
- `wait_for_ciaa_bit`: Clears a bit in a byte and waits for a specific bit in a global variable to be cleared.
- `set_bits_and_wait_2`: Manipulates specific bits in a byte via a pointer and increments a global variable.
- `set_bits_and_wait`: Sets specific bits in a byte and waits for a bit in a global variable to become set.
- `modify_byte_and_decrement`: Modifies bits in a byte, decrements a global variable, and returns a value derived from a constant after potentially calling another routine.
- `modify_byte_and_decrement_2`: Clears the two least significant bits of a byte at a memory location and then sets the first bit.
- `set_bit_and_wait`: Clears the lowest bit and sets the lowest bit of a value pointed to by a register, then waits for a specific bit in a global variable to become set.
- `wait_for_global_bit`: Calls another routine and then waits for a specific bit in a global variable to be set.
- `wait_for_bit`: Enters a loop that waits for the lowest bit of a global variable to become set.
- `assign_constant_values`: Calls another routine and then assigns two constant values to a global variable.
- `perform_io_operation`: Allocates memory and performs an I/O operation using a pointer from a global variable.
- `copy_memory_block`: Copies a block of data from a source location to a destination location until a specific length is reached.

### logged disk load from Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf boot-block load

- `validate_and_shuffle_data_blocks`: Validates a sequence of data blocks by checking for a specific marker and verifying a checksum, then recursively processes and shuffles the resulting data structures using a selection-based sorting algorithm.
- `calculate_bitmask_expression_2`: Combines a bitwise AND of two memory operands with a bitwise AND of a third value and a fourth memory operand that has been shifted left by one.
- `calculate_weighted_bitwise_result`: Performs a bitwise AND between two values, then adds that result to the product of another bitwise AND operation and two.

### logged disk load from Seek & Destroy (1993)(Mindscape)(Disk 1 of 4)[cr DLM].adf boot-block load

- `validate_and_shuffle_data_blocks`: Validates a sequence of data blocks by checking for a specific marker and verifying a checksum, then recursively processes and shuffles the resulting data structures using a selection-based sorting algorithm.
- `calculate_bitmask_expression_2`: Combines a bitwise AND of two memory operands with a bitwise AND of a third value and a fourth memory operand that has been shifted left by one.
- `calculate_weighted_bitwise_result`: Performs a bitwise AND between two values, then adds that result to the product of another bitwise AND operation and two.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `flip_bit_in_global`: Sets or clears a specific bit in a global variable based on a value and an offset.
- `iterate_and_set_global_bits`: Iterates through a loop that checks a bitmask in a global variable and uses a bit-manipulation routine to set or clear bits in a global variable based on a value and an offset.
- `clear_stored_block_or_cycle`: If a stored pointer and size allow, it zeroes a block of memory and advances the pointer; otherwise, it writes a masked value into a global in a loop.
- `initialize_hardware_and_memory_tables`: Detects AGA, extra memory, and 68020 hardware, then initializes several memory-clearing loops and populates a large table with incrementing pointer values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d2, then returns.
- `update_global_state_and_branch`: Updates a global variable using fixed arithmetic and calls helper routines based on the resulting value, finally storing the value sixteen in a global variable.
- `update_display_state_and_dma_parameters`: Manages a state machine that updates a set of three global values based on a counter and performs conditional resets of memory locations.
- `configure_dma_transfer_2`: Performs a coordinate transformation, swaps global pointers, and sets up hardware registers for a DMA transfer while waiting for a status flag.
- `transform_coordinate_table`: Iterates through a table of coordinates and applies a linear transformation using scaling and rotation factors derived from a data structure.
- `calculate_area_product`: Calculates the product of two differences derived from coordinates stored in a table.
- `update_bounding_box`: Calculates a bounding box based on two input points and updates a structure with the resulting dimensions and pointers.
- `initialize_structure_values`: Sets several constant word values within a specific data structure.
- `reset_memory_blocks`: Sets a series of memory locations to specific constant values.
- `swap_and_store_pointer`: Swaps a global pointer and stores the old pointer or a modified version into several global variables.
- `update_pointer_table`: Calculates three different offsets from a base value and stores the resulting longwords into a series of global variables.
- `configure_dma_transfer`: Sets up parameters for a DMA transfer by writing to a structure and checking a status flag.
- `zero_memory_regions`: Clears two separate memory regions by writing zeros to them in a loop.
- `reset_memory_blocks_and_state`: Clears memory regions, updates pointer tables, and resets certain registers or state variables while waiting for a hardware status flag.
- `copy_data_blocks`: Iterates through a source buffer and copies eight-byte chunks from a table to a destination based on values found in the buffer.
- `update_state_and_bounds`: Updates a global variable and a secondary state variable based on a range check against a pointer-derived or register-derived value.
- `update_audio_frequency_and_display_state`: Calculates a new audio frequency or channel assignment based on the value of a global variable and updates hardware registers.
- `copy_table_and_reset_registers`: Iterates through a pointer-based table to copy a sequence of words to a fixed memory location, then updates hardware registers including audio or DMA-related or interrupt-related registers and certain memory-mapped or global values.
- `expand_nibble_fields_into_tables`: Copies a table of words and then splits each word's nibbles into three separate longword tables containing scaled values and calculated differences.
- `update_table_with_checksums`: updates a table of values by performing an addition-based checksum or error-detection calculation and storing the result.
- `update_address_table`: updates a table of pointers by adding an offset to existing values and masking the high-order bits.
- `update_table_with_offset_logic`: Updates a memory table by copying or calculating new values based on a conditional check of a global counter.
- `update_table_with_offset_clamping`: This routine updates a table of values by adding an offset to existing entries, while clamping the high-order bits of the resulting or modified values to a maximum limit.
- `update_address_table_2`: updates a table of pointers by adding an offset or a constant value to existing entries.
- `update_instruction_table`: Updates a table of instructions by adding or modifying certain values or offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 1 into d2.
- `update_address_table_with_clamping`: Updates a table of pointers by adding an offset to existing values and clamps the high-order bits of the resulting or modified values to a maximum range.
- `copy_and_clear_word_table`: Copies thirty-two words from a source table into a target table, zeroing each source word as it goes, then stores a fixed address and a zero word into a record and loops until a global becomes zero.
