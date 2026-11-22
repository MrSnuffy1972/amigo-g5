# Game 376 (R-Type): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00376). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921840 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 25 raw-word lines |
| Functions named and described | pass, 394 of 394 named (0 author symbols), 394 of 394 described (74 by rule, 307 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2089 of 2089 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2560 bytes, 11 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6597 of 6597 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4731 |
| code | 40784 |
| data | 802597 |
| unused | 73728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| R-Type.adf:bootblock | 1 | 36 | 4 |
| logged disk load from R-Type.adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3962 | 204 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1026 | 89 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 236 | 6 |
| R-Type.adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 73 | 2 |
| R-Type.adf bytes 143872-144384 (code never loaded by a reference run) | 1 | 127 | 3 |
| R-Type.adf bytes 156672-157696 (code never loaded by a reference run) | 1 | 107 | 3 |
| R-Type.adf blocks 1759-1759 (bytes 900608-901120) (code never loaded by a reference run, load address unknown) | 1 | 116 | 3 |
| R-Type.adf:blocks 66-275 (bytes 33792-141312)#unpacked | 1 | 847 | 78 |

## Cases handled

- R-Type/R-Type.adf: custom-format (non-DOS) disk, 1610 non-empty blocks: matched against track-loaded segments
- 1752 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- R-Type/R-Type.adf: the boot block reads 5120 bytes from disk offset 895488 to an address (found statically); recovered as a unit
- runtime segment an address: 1752 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 144 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 193 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- R-Type/R-Type.adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- R-Type/R-Type.adf: bytes 143872-144384 look like code no run loaded: recovered position-independently (load address unknown)
- R-Type/R-Type.adf: bytes 156672-157696 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 7 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 5 address, 2 branch displacement
- R-Type/R-Type.adf: 4 logged loads cover 7168 bytes of the image
- R-Type/R-Type.adf: 39 track-loaded blocks found in recovered runtime segments and classified through them
- R-Type/R-Type.adf: blocks 4-62 (bytes 2048-32256) hold 3 packed streams (BK: ByteKiller) found by scanning, 96% of the bytes
- R-Type/R-Type.adf: blocks 66-275 (bytes 33792-141312) hold a BK: ByteKiller stream (107520 -> 128816 bytes)
- R-Type/R-Type.adf: blocks 277-277 (bytes 141824-142336) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.04)
- R-Type/R-Type.adf: blocks 309-320 (bytes 158208-164352) outside the filesystem hold data, not code (opcode density 0.013, entropy 3.99)
- R-Type/R-Type.adf: blocks 330-509 (bytes 168960-261120) hold a BK: ByteKiller stream (92160 -> 161136 bytes)
- R-Type/R-Type.adf: blocks 517-571 (bytes 264704-292864) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.48)
- R-Type/R-Type.adf: blocks 583-585 (bytes 298496-300032) outside the filesystem hold data, not code (opcode density 0.029, entropy 4.78)
- R-Type/R-Type.adf: blocks 592-598 (bytes 303104-306688) outside the filesystem hold data, not code (opcode density 0.015, entropy 5.11)
- R-Type/R-Type.adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.036, entropy 5.52)
- R-Type/R-Type.adf: blocks 946-962 (bytes 484352-493056) outside the filesystem hold data, not code (opcode density 0.026, entropy 5.56)
- R-Type/R-Type.adf: blocks 968-1097 (bytes 495616-562176) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.18)
- R-Type/R-Type.adf: blocks 1100-1113 (bytes 563200-570368) outside the filesystem hold data, not code (opcode density 0.037, entropy 5.29)
- R-Type/R-Type.adf: blocks 1243-1252 (bytes 636416-641536) outside the filesystem hold data, not code (opcode density 0.018, entropy 5.10)
- R-Type/R-Type.adf: blocks 1254-1346 (bytes 642048-689664) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.16)
- R-Type/R-Type.adf: blocks 1353-1359 (bytes 692736-696320) outside the filesystem hold data, not code (opcode density 0.017, entropy 4.58)
- R-Type/R-Type.adf: blocks 1364-1422 (bytes 698368-728576) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.16)
- R-Type/R-Type.adf: blocks 1430-1445 (bytes 732160-740352) outside the filesystem hold data, not code (opcode density 0.030, entropy 5.26)
- R-Type/R-Type.adf: blocks 1452-1573 (bytes 743424-805888) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.18)
- R-Type/R-Type.adf: blocks 1584-1590 (bytes 811008-814592) outside the filesystem hold data, not code (opcode density 0.028, entropy 5.49)
- R-Type.adf blocks 1759-1759 (bytes 900608-901120) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 268 of 512 bytes decode)
- R-Type/R-Type.adf:blocks 66-275 (bytes 33792-141312)#unpacked: hunk executable inside unpacked track data, recovered as unit u9 after the main pass
- 4 packed track region(s), 280576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_ciab_register` (runtime segment an address): Clears bit 7 of the CIAB_PRB register, sets bits 3, 4, and 5, then clears a bit in CIAB_PRB at an offset determined by a value from a memory location, and calls two other routines. Kind: hardware register/status bit manipulation for disk timing or sync validation. Pass path: the code continues to sub_3E2A and sub_3D68 after the bitwise masking of the CIAB_PRB register.
- `handle_ciab_and_delay` (runtime segment an address): Saves registers, checks a global variable, sets bit 2 of a register, and then enters a loop that calls several routines based on a comparison between two values derived from a global variable and d0. Kind: disk hardware/timing check. Pass path: the branch where uVar4 == uVar3 >> 1 (skipping the do-while loops).
- `toggle_ciab_bits` (runtime segment an address): Sets bit 1 of the CIAB_PRB register, clears bit 0 of the same register, and then sets bit 0 of the register. Kind: hardware state manipulation/timing check. Pass path: sub_3D74.
- `clear_ciab_bits` (runtime segment an address): Clears bit 1 of the CIAB_PRB register, clears bit 0 of the register, and then sets bit 0 of the register. Kind: bit-masking/state-flagging. Pass path: the execution continues to the next instruction (bsr.b an address) assuming the bit manipulation satisfies a subsequent hardware state check.
- `process_bit_stream` (R-Type.adf bytes 156672-157696 (code never loaded by a reference run)): Calls two routines to read bits from a buffer and then returns a byte with bits 1 and 7 set. Kind: sabotage path / integrity trap. Pass path: not determined.

## Functions


### R-Type.adf:bootblock

- `boot_entry_adjust_interrupts`: Boot block entry that calls one unnamed routine and a system library function, and writes the interrupt enable register; the decompiler timed out, so only the register touch and call count are recovered.
- `stub_pcrel_pointer_target`: A two-instruction routine whose address is taken by a pc-relative load and that has one unnamed caller; no calls, registers or text are recorded for it.
- `load_payload_via_doio`: Fills an exec I/O request with read command 2, a length of five kilobytes, a destination address and a disk offset, sends it through the DoIO call, then copies a 67-byte block of its own code to low memory.
- `execute_trap_zero`: Invokes trap zero and returns the combined 64-bit value of the D0 and D1 registers.

### logged disk load from R-Type.adf boot-block load

- `select_half_or_input_word_2`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `lookup_masked_table_word`: Reads a word from a table indexed by twice the D1 word, ANDs it with the D6 word, and returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `jump_to_initialize_and_copy_data`: Jump stub: passes control straight on to initialize_and_copy_data.
- `jump_to_calculate_scaled_index`: Jump stub: passes control straight on to calculate_scaled_index.
- `jump_to_subtract_and_branch`: Jump stub: passes control straight on to subtract_and_branch.
- `jump_to_adjust_bcd_values`: Jump stub: passes control straight on to adjust_bcd_values.
- `initialize_system_state`: Branches to another routine, clears many global words and bytes, sets values in a structure, calls several routines to initialize tables and lists, and finally jumps to an address in a0 if a specific global byte is zero.
- `frag_bsr`: Short code fragment that calls another routine.
- `frag_btst_beq_move`: Short code fragment that tests bit 6 of a fixed memory address, then branches if the last result was zero, then copies 868 to a fixed memory address.
- `execute_system_tasks`: Tests bytes at two global addresses and calls routines accordingly, sets several longwords in a structure, calls a routine via a pointer from a global address, and performs several other routine calls and bit manipulations before entering a loop.
- `update_and_copy_data_buffers`: Checks a bit in a hardware register and, if set, calls two routines to process data, updates a byte in a structure by adding 49 to a value from a global variable, and calls two more routines to copy memory blocks before incrementing the global variable and branching.
- `initialize_game_state_structures`: Checks two global status bytes and, if zero, initializes several global variables and calls a routine to clear status data.
- `clear_status_data`: Calls two subroutines, clears a byte at two global addresses, clears a byte at a third global address, clears a word at a fourth global address, and stores a long at a fifth global address before returning.
- `reset_system_state`: Compares a value from a pointer in a global variable against another global variable, then if the comparison passes, it updates several global pointers, clears multiple global bytes and words, sets a global function pointer to a specific routine, calls a routine via a pointer, and clears several more global bytes and a word.
- `process_data_table`: Checks if a global pointer is non-zero, then loops through a list of three-word structures, using the second word as a multiplier and the first word as an index to write the third word into a destination table.
- `update_status_byte`: Decrements a byte in a global variable and, if it reaches zero, sets it to two, loads a pointer from a global address into a0, calls another routine, decrements another global byte, and if that reaches zero, sets a bit in a global variable, stores a value from a register, and updates a pointer in a global variable.
- `reset_and_decrement_counters`: If a global byte is non-zero, it clears three specific bytes, sets a value at an offset of 150 in a6, calls three subroutines, clears two global bytes, modifies several fields in a structure pointed to by a0, and updates several global variables.
- `process_nibble_table`: Tests a byte at a global address; if it is zero, it calls a subroutine, clears a byte at a global address, sets a byte at a global address, and decrements a byte at a global address in a loop, calling another routine and clearing a bit in a hardware register when the loop finishes; otherwise, it enters a loop that iterates through 15 words in a table, checking nibbles and subtracting values from the table entries.
- `process_and_clear_data`: Calculates an offset using values from a global table, calls several subroutines to copy or clear memory, and uses a loop to process data based on a byte from a global variable.
- `clear_memory_block`: Clears 15 words starting at a specific memory location using a loop.
- `update_nibble_values`: Loops through a table of 15 words, and for each word, performs a nested loop that compares the lower 4 bits of the table word with the lower 4 bits of a word from a provided source, adding a value to the table word if the condition is met.
- `subtract_nibble_flags`: Loops through 15 words in a table, checks if any of the four nibbles in each word are non-zero, and if so, subtracts a calculated value from that word.
- `fill_memory_with_spaces`: Loads a long value from a memory location into d0, compares it to a long value at a pointer in a1, and if the value in d0 is less than the value at the pointer, branches to another routine; otherwise it increments the pointer by 2 and branches to another routine.
- `convert_nibble_to_char`: Loads a 16-bit word from a pointer in a1, sets a byte at a global offset to 18, and if a comparison between the word and itself fails, it masks a value from a global pointer, adds 48, and stores it at the end of a buffer pointed to by a2.
- `convert_to_string_and_clear`: Calls another routine to get a value, adds 48 to it, and stores the character in a buffer pointed to by a2.
- `adjust_bcd_values`: If a specific global byte is zero, it takes a 32-bit value from d0, splits it into bytes, and adds those bytes to several global locations using BCD adjustment routines.
- `manage_structure_list`: Compares two global longwords, updates one if they differ, and if a third global longword is less than four, increments it and enters a loop that clears several structures in a table and then copies data from another table into them.
- `copy_bit_expanded_data`: Performs bitwise operations on a 32-bit value to generate a sequence of bytes, then uses those bytes to either clear or copy data from a source table into a destination table at specific offsets.
- `initialize_and_copy_structures`: Clears a series of structures in a memory block, and if a global word is non-negative, copies data from a source table into the cleared structures at specific offsets.
- `initialize_linked_lists_and_tables`: Clears a global long, builds a linked list of 40 entries in a table, clears 13 words in another table, and clears 15 words in a third table at 14-byte intervals.
- `fill_pointer_table`: Loops 25 times to copy a pointer from a global variable into a table, incrementing the pointer by a value stored in a global variable during each iteration.
- `initialize_hardware_and_tables`: Configures CIA and custom hardware registers, copies sixteen words from a table to a color register area, and populates two separate tables with incrementing values using loops.
- `clear_large_memory_block`: Clears a memory block starting at a global address until a specific limit is reached.
- `populate_offset_table`: Stores several derived values into a table at offset 4d1a, calls another routine, and then enters a loop that stores the negated result of that call into a destination buffer using offsets from a table.
- `write_offset_values`: Iterates through a list of offsets in a table and uses them to store the value in a register into a destination buffer.
- `configure_timer_interrupts`: Sets a 16-bit value at offset 154 of a structure in a6, configures CIAA control and timer registers, calls another routine twice with different CIAA control values, and returns a 16-bit value formed from two bytes in CIAA registers.
- `check_status_byte`: Compares a byte at an offset of a structure with itself and returns if a specific bit in another offset of the same structure is set.
- `handle_keyboard_interrupt`: Checks a bit in a CIA register and, if set, processes a byte from the keyboard serial register to update a global variable and then clears an interrupt request.
- `calculate_wrapped_value`: Performs arithmetic on a global variable, XORs it with a value from a hardware register, and returns the result of a division.
- `decrement_counter_on_condition`: Tests a word at offset 50 of a structure in a0; if positive, it increments a word at offset 4 of the same structure, compares it to 220, and if not higher, decrements a word at offset 50 of the structure before returning.
- `update_indexed_buffers`: Checks if a global variable is non-zero, then calls another routine twice to add returned words to two different global variables, decrements a global variable, and if that variable is negative, calls another routine twice and updates a global variable from a table.
- `decrement_and_shift_buffer`: Decrements a byte at a pointer, and if the result is negative, shifts the contents of the buffer forward and returns a signed word from the third byte.
- `fetch_indexed_bytes`: Reads a byte from a table using an index, and if the index is negative, reads a second byte from a different offset in the same table using the absolute value of that index.
- `subtract_table_values`: If a byte at a specific address is non-zero, it subtracts values read from two different global locations from two other values and returns the result.
- `execute_nested_loops`: Sets several values in a structure pointed to by a6, performs bitwise operations on a word from a global address to calculate and store new values in a structure, loads a pointer from a global address, and enters nested loops that call a routine via a pointer from a structure and update values in a6 and a1.
- `execute_callback_from_list`: Increments a global counter and, if a condition is met, calls a routine via a pointer found within a structure at a specific offset.
- `clear_structure_fields_2`: Calls another routine and then clears several 16-bit and 8-bit fields at specific offsets within a structure pointed to by a0, before storing d2, d0, and d1 into offsets 36, 10, and 12 of that structure.
- `copy_structure_fields_and_offset`: Calls another routine, copies several longwords and a word from a structure pointed to by a2 into a structure pointed to by a0 at offsets 18, 6, 42, and 46, then if the word at offset 46 is not zero, calls another routine and adds 75 to the word at offset 46 before copying it to offset 48.
- `iterate_pointer_list`: Loads a long from a global address into d7, stores that address in another global address, and if d7 is non-zero, enters a loop that checks a word at offset 6 of the structure in a0, calls a subroutine at that address if it is non-zero, calls three other subroutines, and updates the global address with the value from offset 0 of the structure before looping.
- `reinitialize_entity_from_table`: Checks a byte at offset 5 of the structure in a0; if it is positive, it clears several fields including a 16-bit value at offset 30, a byte at offset 32, and a word at offset 46, then fetches new data from a table based on a value at offset 33 and updates the structure's pointer and state values.
- `update_entity_state_and_position`: Decrements a byte at offset 32 of a structure, and if it reaches zero, updates several 16-bit values from a table and calculates a new position.
- `transform_word_bits`: Rotates a 16-bit value in d0 left by 4, masks d4 with the result, masks d0 with 4095, shifts d0 left by 1, and returns.
- `frag_btst_bne_rts`: Short code fragment that tests bit 6 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `dereference_global_pointer`: If a global variable at offset 4910 is not zero, it replaces that global variable with the value stored at the address it contains, then returns the original value.
- `update_pointer_structure`: Clears a word at the offset of a0, copies a longword from a0 to a destination pointed to by a global variable, moves a longword from a global variable to a0, and then stores a0 into that same global variable.
- `insert_into_sorted_list`: Uses a pointer from a global variable to traverse a linked list, inserting a new node at the correct position based on a byte comparison or replacing an existing node.
- `initialize_and_copy_data`: Decrements a word at offset 48 of a0 and, if the result is non-negative, enters a loop that checks if a global variable is zero; if so, it populates a structure at a1 with values from d0 and d1, calls another routine, and then copies two words from a table at a2 into the structure at a1.
- `process_object_table`: Iterates through a table of 16 entries, performing various checks on 16-bit and 8-bit values at offsets of the current table entry, and either updates the entry with values from other offsets or calls routines to modify memory based on specific range constraints.
- `calculate_scaled_index`: Calculates a value based on the difference between d1 and d3 and d0 and d2, performs a division, and uses the result to select a byte from a table at offset 4a1c.
- `subtract_and_branch`: Subtracts a word in d1 from a word in d0, then branches to different locations based on the sign of the result and a comparison of d0 to a constant.
- `increment_d1_conditionally`: Compares a word in d0 to 8, and if it is not higher, increments d1 and branches to another instruction.
- `clamp_d1_value`: Decrements d1 by one and, if the result is negative, sets d1 to 15 before returning.
- `clamp_byte_value`: Compares d1 to 15; if d1 is 15 or greater, it clears d1 before returning, otherwise it returns.
- `decrement_and_update_structure_state`: Decrements a byte at offset 40 of the structure in a0 and sets bit 0 of a byte at offset 35 if the new value is odd, otherwise it performs a series of conditional calls to other routines based on a 16-bit value at offset 10.
- `process_list_elements`: Iterates through a list of 14 elements, performing bounds checks and updating structure values based on specific conditions for different index values.
- `check_status_bits_2`: Checks bit 0 of a byte at a global location and, if set, calls another routine; then checks bit 1 of that same byte and, if set, calls another routine before returning.
- `update_object_status_and_score`: Increments two 16-bit values in a structure, checks a status byte and a bounds value, and either clears a word at the structure pointer or updates a multi-byte BCD score using values from a structure.
- `check_bounds_range`: Checks if the absolute difference between d0 and d3 is less than or equal to a3, and if the absolute difference between d1 and d4 is less than or equal to a4, returning 1 if both are true and 0 otherwise.
- `update_structure_values_4`: Adds a word from d0 to a word at offset 38 of a structure in a0, then checks a byte at offset 35 and a word at offset 36; if certain conditions are met, it calls another routine, modifies offset 35 and 40, and branches to another routine, otherwise it clears offset 35, sets offset 5 to 1, clears offset 6, and performs further operations involving a pointer in a1 and branches to another routine.
- `update_system_state`: Performs calculations using global values, calls other routines to update a structure in a0, and modifies various global bytes and pointers.
- `increment_list_values`: Loops through a list of 8 elements, adding values to structure members if a global byte is non-zero and a bounds check passes, then calls another routine.
- `check_and_update_global`: Checks if a byte at a global address is non-zero; if so, it calls another routine, and if the result is not negative, it stores 255 in the global address and returns, otherwise it returns.
- `update_structure_from_registers`: Subtracts 32 from d0 and 3 from d1, stores these values into a structure at a0, decrements a word at offset 50 of the structure, and jumps to another routine if d0 is not positive.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_input_and_update_table`: Checks a global variable and a bit in the CIAA Port A register, then either increments a value in a table and updates associated global memory or clears several global variables and calls another routine.
- `clear_and_increment_global_counters`: Clears two longwords and two words in global memory, then enters a loop that increments a byte in a global variable and calls another routine if that byte reaches 6.
- `copy_data_from_table`: Iterates up to four times, comparing a value from a table to d0, and if the condition is met, copies multiple words and longwords from the table into a structure and global variables before calling another routine.
- `process_data_list`: Sets two words at offsets 96 and 102 of a6, then loops up to 9 times through a list starting at a pointer in a0, calling another routine for each entry, updating bits in a pointer in a4, and modifying values in the list based on data from a table at a3.
- `update_list_elements`: Compares d5 to 368, d6 to 16, and d6 to 240, and if certain conditions are met, clears d5, stores d6 at offset 28 of a0, stores d5 at offset 0 of a0, decrements the pointer in a0 by 2, and loops using d7; otherwise it returns.
- `store_calculated_data`: Adds values from two global variables to d0 and d3, calls two routines, and then stores several values including d3, d4, d1, and a1 into a structure at an offset from a6.
- `update_object_attributes_and_increment`: Checks a global index and modifies a byte at a specific offset within a structure pointed to by a4, either clearing a word or setting bit 5 based on bit tests.
- `update_object_state_and_attributes`: Checks a value in a pointer passed in a0 and, if non-zero, modifies its attributes by either bitwise inverting a word at offset 54 or setting bit 5 and clearing bit 4 of a byte at a pointer in a4, before updating the value in a0 with a calculated result.
- `validate_coordinate_range`: Checks if d0 is between 38 and 367 and d1 is between 16 and 223; if so, it returns d0, otherwise it decrements a global variable at offset 4b5f and returns 0.
- `spawn_object_entities`: Checks a global flag and, depending on its value, initializes a series of object structures in a memory pool by populating their properties from a table and registers, then calls other routines to handle the new entities.
- `recursively_initialize_objects`: Checks if a pointer in a0 is zero; if it is, it initializes a structure with specific values and returns, otherwise it checks bits in a global variable and recursively calls itself to process elements from two different tables.
- `update_object_properties`: Iterates through a list of pointers starting at a0; if a zero is found, it populates that structure with values from d0 through d5, otherwise it continues the loop.
- `handle_interrupt_requests`: Checks the interrupt request register for specific bits and, if bit 4 is set, clears that bit and executes a series of subroutines; if bit 5 is set, it increments a global variable, conditionally calls a routine via a pointer, and performs several state updates to global variables before calling other subroutines and setting the interrupt request register to 96.
- `update_joystick_movement_bounds`: Reads the joystick data register and modifies two global variables based on the input state and current values.
- `update_control_byte`: Compares a byte at a global address to eight, and based on the result, sets a byte at a global address to one, negative one, or zero.
- `update_table_values`: Calculates a value by adding a byte from a global address to another, and if the result is greater than 16, it updates a global byte and a table with new values derived from a table at offset 5108; otherwise it sets a global longword using a calculated index.
- `decrement_and_branch_status`: Decrements a global byte and, if it reaches zero, sets it to four and performs an indirect jump.
- `initialize_data_tables`: Sets several global variables to specific values and then uses a loop to initialize two entries in a table starting at offset 49342 with specific 16-bit values.
- `process_table_updates`: Shifts a byte from a global address right by one, and if the result is non-zero, performs two separate sequences where it loads addresses into a0 and a1, calls another routine, and stores two modified word values into a table at specific offsets.
- `update_list_elements_2`: Calls another routine using a word from a pointer in a1, stores the result back, updates two values in a table at a0 using offsets derived from a global word, increments the pointer in a1, and performs further updates and potential pointer resets.
- `process_object_data`: If d0 is not less than 3, it clears d0 and returns; otherwise it sets a hardware register, calls several routines to copy data from various tables to specific destinations, and finally returns from an interrupt.
- `pack_and_store_word`: Combines bits from d0, d1, and the sum of d1 and d2 into a 32-bit value, stores it at the address in a0, and then stores the same value with bit 7 set at an offset in a0 determined by d3.
- `copy_nested_blocks`: Performs a nested loop to copy data from a source pointed to by a1 to a destination pointed to by a0, where the outer loop runs twice and the inner loop runs a number of times specified by d3.
- `decrement_and_reset_counters`: Decrements a word at a global variable and, if it is negative, calls another routine and then sets a global variable to either 1 or 2 based on a byte at another global variable.
- `update_table_data`: Copies a longword from a memory location to another, toggles a byte at a global variable, and then enters a loop to write four longwords to a table starting at address 21234 based on a calculated value.
- `write_longword_array`: Writes a longword to a table at the address in a0 by splitting it into two words, then adds d2 to the longword and increments the pointer by 4, repeating this for the number of iterations in d1.
- `update_object_state_and_dispatch`: Loads object parameters from tables using an index, updates a global pointer, and calls a function via a pointer in a table.
- `modify_status_bits`: Tests bit 7 of a pointer in a3; if set, it sets bit 5, then tests bit 4, and if that is also set, it clears the word at the address in a3 before returning.
- `reset_status_parameters`: If a byte at a global address is non-zero, it clears a byte at another global address, stores a long value at a third global address, sets a byte at a fourth global address, clears three more bytes at global addresses, compares a byte at a global address to two, and if it is two, decrements that byte and clears four more bytes at global addresses before storing two words and a long at global addresses and returning.
- `execute_conditional_branch`: Calls another routine, and if the result is not zero, it stores a long value into a global address and branches to another routine.
- `increment_and_check_global`: Calls another routine, increments a long value at a global address by 2, and if that value is not less than 192, sets a byte at a global address to 1 and stores a new address in another global variable before returning.
- `process_global_value`: Calls another routine, compares a word from a global variable to 270, and if it is less than 270, checks a byte at a global variable; if that byte is 1, it clears it, otherwise it sets it to 1 and calls another routine.
- `reset_global_counters`: Checks a byte at a global variable, and if it is not negative, clears it, sets d0 to 242, calls another routine, stores the byte back, and clears two other bytes at global locations.
- `decrement_and_wrap_value`: Subtracts a value at a global variable from another global variable, and if the result is less than or equal to 4, it decrements a byte at a global variable and potentially wraps it to 16 or 0.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies 16 to a fixed memory address, then returns.
- `adjust_global_counter`: Subtracts a value at a global variable from d0, and if the result is less than 4, it adds 1 to the global variable and increments a7 by 4, otherwise it negates d6, d1, and d0.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `update_global_counter_2`: Calls another routine, then calculates a value by adding 4 to a 16-bit value from a global pointer and subtracting a 16-bit value from another global pointer; if the result is negative, it negates several registers and checks if the negated result is less than 3, otherwise it adds a negated value to the second global pointer.
- `check_bounds_and_update`: Loads two 16-bit values from a global table, adds offsets to them, and calls another routine.
- `increment_global_byte`: Increments a global byte if it equals one, then stores a constant into a global long word and calls another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_counters`: Adds two words from global locations to a word in d0 and stores the result in another global location, repeats this for a second pair of global words, calls another routine, and if the result is non-zero, it negates a value, calls another routine, and updates a global pointer and a byte at a global location.
- `update_global_counter`: Loads a word from a global variable, negates it, and if a byte at another global variable is zero, adds the negated value to a word at offset 24 of a global variable and updates a pointer at offset 20 of a global variable if the result is between 134 and 422.
- `frag_moveq_bra`: Short code fragment that loads 0 into d7, then jumps to another point in the code.
- `adjust_global_counter_short`: Subtracts a value at a global variable from d0, and if the result is less than 2, it adds 1 to the global variable, otherwise it negates d1, d7, and d0.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d7, then returns.
- `check_status_bits`: Sets a word at offset 52 of a6, then tests bit 14 of d0 and a byte at a global variable, clearing a byte at a global variable if both conditions are met.
- `check_bits_and_set_byte`: Tests bit 14 of d0, and if it is set, checks two global variables at 16908 and 16903, then sets a byte at 20812 to 1 if both are zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_entity_status_and_table`: Reads a byte from offset 50 and two words from offset 10 of a structure in a0, calls another routine, sets bits 2 and 3 of a byte at offset 35, updates offset 31 with the byte from offset 50, clears offset 34, calls another routine, and updates a table entry based on a global variable comparison.
- `decrement_and_jump`: Tests a byte at a global variable, and if it is zero, branches; otherwise it decrements a word at offset 2 of the structure in a0 and jumps to another routine if the result is not 16.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_structure_and_jump`: Calls another routine with two words from a structure in a1, sets bit 1 of offset 35 in a0, and populates various fields in a0 with bytes and longwords before jumping to another routine.
- `validate_structure_offsets`: Compares a word at offset 12 of the structure in a0 against 16 and 224, and compares a word at offset 10 of the structure in a0 against 16 and 274, jumping to another routine if the values are within specific ranges.
- `frag_movea_jmp`: Short code fragment that copies the memory at an offset from a0 to a2, then copies the memory at an offset from a0 to a1, then jumps to the address in a1.
- `update_object_offset`: Loads a long from a global address into d0 and adds it to a4, then uses a byte from a structure in a0 to index into a table at a1 to add a value to d1, increments a byte at offset 52 of the structure, and if a byte at offset 51 is negative, calls two routines and stores a long at offset 54 of the structure if the second routine returns a specific value.
- `branch_on_bit_test`: Loads a long value from a global address into d0, adds it to a4, tests bit 7 of the memory at a4, and if bit 7 is clear, branches to an earlier instruction; otherwise, it stores the value 4608 at offset 54 of a structure and branches to another routine.
- `call_multiple_subroutines`: Saves d0 and d1 from a structure in a0, calls another routine, adds 23 to d1, calls another routine, and if the result is zero, stores 4608 at offset 54 of a0, calls another routine, and branches to another routine.
- `frag_addq_subq_rts`: Short code fragment that adds 2 to the memory at an offset from a0, then subtracts 1 from the memory at an offset from a0, then returns.
- `update_object_state_and_pointer`: Modifies a byte at offset 51 of a structure by negating it, updates a function pointer at offset 54, and sets several fields including a pointer at offset 42, a byte at offset 52, a word at offset 30, and a byte at offset 32 before calling another routine.
- `modify_memory_via_pointer`: Loads a longword from a global variable into d0, adds d0 to a4, tests bit 7 of the memory at a4, and if the bit is set, checks a byte at offset 51 of a0 and, if it is negative, adds -2 to a word at offset 10 of a0.
- `frag_addq_rts`: Short code fragment that adds 2 to the memory at an offset from a0, then returns.
- `add_negative_offset`: Tests a byte at an offset of 51 from a0, and if it is negative, adds 65532 to d0 before returning.
- `tiny_add_rts`: Small routine of 2 instructions that adds 29 to d0, then returns.
- `modify_structure_fields`: Modifies a word at offset 12 of a structure in a0 based on its bit 3, clears bytes at offsets 30 and 32, and then stores a longword at offset 42 based on a signed byte at offset 51.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies a1 to the memory at an offset from a0, then returns.
- `clear_global_blocks`: Clears several bytes and words at specific global memory locations including 21924, 21930, 21928, 21868, and 21896.
- `test_and_decrement_global`: Tests a byte at a global address and branches if it is non-zero; otherwise, it tests a word at another global address and, if it is zero, branches; if not, it decrements a word at that global address, loads a pointer into a0, calls another routine, and loads another pointer into a0.
- `update_structure_and_manage_linked_list`: Updates a structure by calling a function pointer at its start, modifying its internal word values, and performing bounds checks against a table.
- `modify_structure_values`: Extends a byte from offset 25 of a structure in a0 to a word, adds it to a word at offset 6 of the structure, decrements a byte at offset 24, and if the result is positive, sets a long value at offset 48 and a byte at offset 24, otherwise it returns.
- `modify_structure_and_branch`: Increments a 16-bit value at an offset of 4 from a pointer in a0, decrements a byte at offset 24 of a0, and branches if the result is positive; otherwise it moves a long value into a0 and returns.
- `search_linked_list`: Traverses a linked list to find an entry matching specific criteria, updates a global word with a calculated value, and modifies a structure in a0.
- `apply_table_offsets`: Loads a pointer from offset 8 of a structure in a0, checks a word at offset 10 of that pointer and a byte at offset 5; if conditions are met, it decrements a byte at offset 24 of a0, calls two other routines, and uses a value from a table to add two words to offsets 4 and 6 of a0.
- `call_subroutine_twice`: Checks a byte at offset 69 of a global variable; if it is non-zero, it calls another routine twice using two different addresses loaded into a1.
- `clear_structure_and_decrement`: If a pointer in a1 is non-null, it adds offsets to d0 and d1 from the structure, calls another routine, clears the memory at a1, and decrements a global byte.
- `initialize_and_jump`: Stores the stack pointer in a global variable, clears two bytes at offsets 56fe and 56ff, and calls two subroutines.
- `update_sequence_state`: Calls a routine, sets a word at offset 56f6 to 4, and decrements a byte at offset 56f3.
- `update_sequence_parameters`: Calls three different routines, sets a word at offset 56f6 to 4, decrements a byte at offset 56f3, and if the result is non-negative, branches to another routine; otherwise it sets a word at offset 56f6 to 250, a byte at offset 56f3 to 15, a long value at offset 56e6 to 13340, and branches to another routine.
- `initialize_sequence_parameters`: Calls another routine, sets a word at offset 56f6 to 4, decrements a byte at offset 56f3, and if the result is non-negative, branches to another instruction; otherwise it calls two other routines, loads a pointer to offset 56aa into a1, calls another routine, and stores a long value at offset 56e6.
- `execute_conditional_callback`: Calls another routine, checks a byte at offset 56f2, and if it is zero, sets a long value at offset 56e6 to 13340 and branches to another routine; otherwise it stores a byte at offset 513a and branches to another routine.
- `initialize_structure_loop`: Compares a longword at a global variable to a constant, calls another routine, and then enters a loop that runs 7 times, performing various assignments to a structure in a0 and calling another routine within the loop.
- `execute_callback_and_set_fields`: Calls another routine twice with different parameters, sets three fields in a structure in a6, and then checks a global variable to call a routine via a pointer if it is not zero.
- `initialize_structure_member`: Clears a 32-bit value at offset 68 of a structure in a6, calculates a new address by adding a multiple of 208 to a pointer from a global variable and stores it at offset 84 of the structure, clears 16-bit values at offsets 102, 116, and 66, sets offset 64 to 496, moves d1 to offset 88, and branches to another routine if bit 6 of offset 2 in the structure is set; otherwise it returns.
- `copy_word_sequence`: Copies 14 words from a source pointed to by a1 to a destination starting at a fixed address, incrementing the destination pointer by 2 for each word.
- `clear_memory_range`: Clears a 32-bit word at a fixed address and continues clearing words in a loop until the pointer reaches a specific limit.
- `calculate_pointer_offset`: Loads a longword from a global table into a0, compares a value in d0 to a value at offset 8 of the table, and uses a loop to decrement a0 before returning.
- `load_structure_pointers`: Loads a longword from offset 4 of a structure into d2 and branches to another location.
- `process_and_encode_data`: Copies 16-byte blocks between two structures in a loop, fills a destination buffer with spaces, and then performs a nested loop to encode data into a buffer using a lookup table and bit manipulation.
- `copy_data_tables`: Enters a nested loop structure where it copies several 32-bit words from a source table to two destination tables at specific offsets, then calls another routine twice using a pointer from a global table and a counter.
- `process_data_stream`: Saves a pointer, calls a routine, sets a byte at a destination, calls another routine, and then enters a loop that checks a hardware register and a global value to decide whether to call another routine or copy a byte from a table to a destination.
- `copy_and_branch`: If d0 is not 2, it returns; otherwise it decrements d7, and if d7 is not negative, it copies a byte from a source at a2 plus offset d7 to a destination at a3, calls another routine, and branches back to the start of the routine.
- `compare_and_decrement_pointer`: Compares d0 to 3, then compares d7 to 44; if d7 equals 44 and d0 does not equal the longword at offset 4 of a4, it stores the byte 32 at the address in a3 and decrements a3 before branching to another routine.
- `compare_and_move_byte_2`: Compares a byte in d7 to 43; if it is not equal, it branches to another routine, otherwise it stores the byte 32 at the address in a3 and increments a3, stores 32 in d6, and branches to another location.
- `compare_and_move_byte`: Compares a byte in d7 to 46, and if it is not equal, branches to another routine; otherwise, it stores the byte 32 at the address in a3 and returns.
- `transfer_and_compare_byte`: Moves a byte from a location in a2 indexed by d7 to a location in a3, moves another byte from the same source to d6, subtracts a long value at offset 4 of a4 from the value in d0, and if the result is less than or equal to 15, branches to another routine; otherwise it returns.
- `push_and_branch`: Moves a byte from an offset in a2 indexed by d7 to a3, saves registers a0 and a1 to the stack, calls another routine, restores the registers, and branches to another location.
- `check_disk_status`: Tests bit 7 of the CIAA_PRA register and enters a loop that decrements d1 from 1024 to 0 if the bit is set, otherwise it returns with d0 set to 3.
- `modify_d0_value`: Clears a 16-bit value at offset 54 of a6, moves a 16-bit value from offset 12 of a6 to d0, and if d0 is not zero, masks it with 255; if the result is zero, it sets d0 to 1, otherwise it returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `process_list_loop`: Enters a loop that loads a 32-bit value from a pointer in a4, and if the value is not negative, sets a1 to that value, loads the next 32-bit value from a4 into a0, and calls another routine before repeating.
- `loop_and_copy_bytes`: Checks bit 7 of a hardware register and a byte at offset 56fe; if both conditions are met, it enters a loop that calls another routine and copies bytes from a pointer in a0 to a destination in a1.
- `search_and_copy_attributes`: Searches for a byte in a string, calculates an offset based on the string position, and then enters a loop to copy bytes and specific offsets from a table into two different structures.
- `execute_conditional_function`: Loads a byte from a global variable, sets that variable to 2, checks bit 1 of the loaded byte, and calls a function from a pointer at a fixed address if the bit is set; otherwise it calls a different function at a fixed address, and if the result is zero, clears a byte at a global variable before returning.
- `initialize_object_transfer_parameters`: Sets a destination pointer, a count of 47, and a type value of 5 in registers, then calls a routine to clear a memory range, updates a table entry, and calls a routine to copy a block of data.
- `copy_memory_block_and_call_subroutine`: Copies a block of memory from a location stored in a global variable to a destination buffer using a loop, then calls a subroutine with two parameters before jumping to another routine.
- `update_signed_byte_and_call`: Checks a byte in a global variable; if it is positive, it decrements the value, sets a specific byte to 255, and calls a function pointer from a table; if it is negative, it negates the value and calls the same function pointer.
- `update_table_entry`: Checks if a byte in a global variable is zero; if not, it calculates an index using d0, fetches a value from a table, compares it against bytes at a second table location, and potentially writes d0 and the fetched value into a structure.
- `update_audio_dma_parameters`: Loads a pointer into a register, loads the address of the audio hardware registers into another register, calls update_interrupt_and_dma, and then increments both registers by their respective offsets.
- `update_interrupt_and_dma`: Checks a byte at offset 2 of a structure in a6, and if it is positive and the structure in a6 is not zero, it updates DMA and interrupt registers, or calculates an index to copy data from a table into a structure and updates interrupt and DMA registers.
- `manage_interrupt_requests`: Reads the custom interrupt request register, masks it, and if certain bits are set, calls another routine using a pointer from a structure before writing the result back to the custom interrupt request register and returning via an interrupt return.
- `reset_dma_and_interrupts`: If a byte at offset 3 of a structure in a6 is non-zero, it writes two words from the structure to the DMACON and INTENA hardware registers, then clears the first two bytes of the structure; otherwise, it sets the byte at offset 3 to 1.
- `search_and_update_object_table`: Searches a table for an entry matching two input values; if found, it copies a block of data from a source pointer to a destination pointer, otherwise it searches for an empty entry, calculates a new offset using a multiplication and bit shift, updates the table entry, and performs a block copy.
- `process_loop_and_hardware`: Calls two routines, moves d1 to d3, sets a1 to point to d2, stores d2 in a global variable, saves registers to the stack, calls two routines, restores registers, increments d2 by 5696, loads a word from offset 5bb2 into d0, increments d0, calls another routine, decrements d1, and if d1 is not zero, branches to another routine; otherwise it branches to another routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `perform_double_hardware_update`: Loads a word from offset 5bb2 into d0, adds 2 to d0, calls another routine, loads the same word from offset 5bb2 into d0, subtracts 2 from d0, calls another routine, calls another routine, and branches to another routine.
- `configure_hardware_parameters`: Extracts the high word of a long value from a global variable and stores it at offset 126 of a structure in a6, then writes several constant values to offsets 32, 36, 96, and 158 of the structure, finally checking a bit in offset 30 to conditionally write to offset 156 and offset 36.
- `modify_ciab_register`: Clears bit 7 of the CIAB_PRB register, sets bits 3, 4, and 5, then clears a bit in CIAB_PRB at an offset determined by a value from a memory location, and calls two other routines.
- `frag_btst_bne_rts_2`: Short code fragment that tests bit 5 of a fixed memory address, then branches if the last result was not zero, then returns.
- `handle_ciab_and_delay`: Saves registers, checks a global variable, sets bit 2 of a register, and then enters a loop that calls several routines based on a comparison between two values derived from a global variable and d0.
- `loop_on_ciaa_bit`: Enters a loop that repeatedly calls two other routines as long as bit 4 of the CIAA_PRA register is set, then clears a word in a global variable and returns.
- `toggle_ciab_bits`: Sets bit 1 of the CIAB_PRB register, clears bit 0 of the same register, and then sets bit 0 of the register.
- `clear_ciab_bits`: Clears bit 1 of the CIAB_PRB register, clears bit 0 of the register, and then sets bit 0 of the register.
- `delay_loop`: Decrements a counter from 16384 down to 0 using a loop and returns.
- `decompress_bitstream`: Performs bitwise operations on a stream of data, using a loop to shift bits and call a refill routine when a register becomes zero, and writes decoded values into a destination buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bit_shift_and_load`: Performs a loop based on a value in d1, shifting d0 right and loading new values from a pointer in a0 into d0 whenever d0 becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_table_2`: Iterates through a table pointed to by a structure in a1, calling another routine and copying bytes from the table into a structure in a0 at offsets 50 and 47, then calls another routine again.
- `update_index_and_call_handler`: Decrements a byte at offset 50 of a structure in a0 if it is non-zero; otherwise, it adds a value from a table to a field at offset 12, increments an index, and if the index reaches 35, it clears it and enters a loop that calls a function pointer from a global variable.
- `loop_and_populate_structure`: Runs a loop based on a word at offset 10 of a structure in a1, calling another routine to populate fields in a structure in a0 from a1 and an incrementing byte value, then calling another routine again.
- `process_indexed_data`: Increments a byte at an offset in a structure if it is non-zero, otherwise it fetches a byte from a table using an index, calls a routine three times to update various offsets in the structure, and potentially jumps to a function pointer.
- `fetch_indexed_byte`: Loads a pointer from a list into a register, then fetches a byte from that pointer using an index in d0 and stores it in d2.
- `insert_into_linked_list`: Calls another routine, copies two words from a structure in a2 to a structure in a0, and then calls another routine before jumping to another location.
- `frag_cmpi_bhi_jmp`: Short code fragment that compares the memory at an offset from a0 with 16, then branches if the comparison was higher (unsigned), then jumps to another point in the code.
- `decrement_structure_fields_2`: Tests a byte at offset 54 of a structure in a0 and branches if it is zero or one; otherwise, it decrements a word at offset 52 and a byte at offset 53, calls another routine if the word result is not negative, and returns.
- `modify_d0_and_d1`: Calls another routine, saves two words from a structure in a0 into d0 and d1, increments d1 by 11, and then modifies d0 based on a signed word at offset 50 of the structure in a0.
- `add_offset_to_d0`: Adds 33 to d0, calls a routine via a jump to an address in d6, and if the result is non-zero, adds a word from a structure at offset 50 of a0 to d0 before returning.
- `decrement_structure_fields`: Decrements a word at offset 52 of a structure in a0, decrements a word at offset 52 of the same structure, and if the result is not negative, branches to another routine; otherwise it returns.
- `update_structure_values_2`: Writes a word to offset 34 of a structure in a0, decrements a byte at offset 37, and if the result is not negative, calls another routine.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `initialize_structure_fields`: Sets a byte at offset 54 to 2, a byte at offset 55 to 20, clears a byte at offset 34, and sets a word at offset 30 to 3.
- `process_structure_state`: Calls another routine, and if the result in d0 is between 55 and 85, calls another routine, sets a byte at offset 54 to 1, copies a byte from offset 59 to 55, copies a word from offset 56 to 52, stores a value at offset 34, writes a long constant to offset 42, writes a word to offset 30, clears a byte at offset 32, calls another routine, and finally stores d2 into offset 60.
- `calculate_offset_difference`: Subtracts a word from a global variable and a word from a structure at offset 10 in a0, then returns the absolute value or a modified result based on the sign.
- `calculate_adjusted_difference`: Subtracts 31 from a global variable and a word from a structure at offset 12 in a0, then returns the absolute value of the result.
- `update_structure_state_2`: Calls a routine and compares the result to 60 and 80; depending on the result, it clears a byte at offset 54 of a structure and writes either 2, 65534, or 70 minus the result into a word at offset 50, while also updating a word at offset 50 based on the sign of d2.
- `update_structure_state`: Sets a word at offset 50 to 2 or 65534 based on d2, sets a byte at offset 54 to 0, stores a 32-bit address at offset 42, and sets a byte at offset 34 to 255.
- `calculate_structure_offsets`: Calls two routines to calculate differences, writes the results to offsets 50 and 52 of a structure, and then calls a third routine after setting several fields in the structure.
- `process_indexed_values`: Fetches bytes from tables using values from a structure, increments two 16-bit words at offsets 50 and 52, and performs arithmetic on offsets 10 and 12 based on the sign of a register and table values before jumping to a function pointer.
- `process_indexed_value`: Fetches a byte from a table using a2 and d0, adds 9 if d2 is negative, stores the result in a structure at offset 46, and calls another routine.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a4, then jumps to another point in the code.
- `calculate_table_index`: Sets d7 to 19 and subtracts d2 from it, loads a word from offset ee, subtracts 48 from it, shifts it right by 4, subtracts that from d7, and if d7 is not negative, subtracts 3 from d7 and branches to another routine; otherwise it adds 8 to d6 and branches to another routine.
- `execute_table_lookup`: Increments d6 by 8 and d7 by 3, and if d7 is negative, increments d6 by 8 again.
- `execute_sequence_of_calls`: Clears two words in a structure, performs arithmetic on d0 and d1 involving shifts and additions, calls three different routines, and sets specific bytes in a structure.
- `modify_structure_bits`: Checks bits in d1 and, if specific conditions are met, performs a bitwise OR on a 16-bit word at offset -2 of a structure and subtracts 16 from it.
- `frag_subi_rts`: Short code fragment that subtracts 256 from the memory at an offset from a0, then returns.
- `modify_and_call_routine`: Performs bitwise operations on d1 to create a word, stores it at offset -2 of a structure, calculates two new values using shifts and additions, and calls another routine.
- `initialize_structure_values_2`: Saves d0-d1 from offsets 2 and 3 of a1, moves a word from offset 8 of a2 to d2 and sets d2 to 30, calls another routine, clears a byte at offset 34 of a structure in a0, sets a byte at offset 33 to 2, clears a word at offset 58, sets a word at offset 60 to 125, calls another routine, and jumps to another routine.
- `decrement_and_check_byte`: Tests a byte at offset 57 of a structure in a0; if it is not zero and not equal to 1, it decrements the byte at offset 58 and returns if the result is negative, otherwise it returns.
- `compare_and_add_values`: Compares a 16-bit word at offset 12 with d1 after checking the sign of a 16-bit word at offset 52, and if the comparison is not greater, adds values from offsets 50 and 52 to offsets 10 and 12 respectively.
- `frag_cmp_blt_bra`: Short code fragment that compares d1 with the memory at an offset from a0, then branches if the comparison was less, then jumps to another point in the code.
- `compare_and_add_offset`: Compares a 16-bit word at offset 10 with d1 after checking the sign of a 16-bit word at offset 50, and if the comparison is not greater, adds a 16-bit word from offset 50 to the 16-bit word at offset 10.
- `frag_cmp_blt_bra_2`: Short code fragment that compares d1 with the memory at an offset from a0, then branches if the comparison was less, then jumps to another point in the code.
- `fetch_and_store_byte`: Sets a byte at offset 57 to 2, calls another routine, stores the returned byte at offset 58, and clears a word at offset 30.
- `update_object_timer_and_state`: Decrements a byte at offset 56 of the structure in a0 and, if it reaches zero, calls another routine to update a value from offset 56, then modifies several fields in the structure including offsets 10, 18, 30, 35, 42, 50, 52, and 54 before calling two other routines.
- `update_timer_and_offset`: Loads a word from offset f0, subtracts 15, and compares it to a word at offset 12 of a structure in a0; if the value is within a specific range, it clears offset 52 of the structure in a0, calls a routine via a pointer in d2, and if a byte at offset 59 of the structure in a0 is not negative, adds d2 to a word at offset 10 of the structure in a0 and compares the result to 290.
- `calculate_offset_adjustment`: Calls another routine, and if d0 is non-negative, negates d2, adds a word from offset 10 of a0 to d2, stores the result in offset 36 of a0, sets a byte at offset 39 to 1, and branches to another location.
- `update_timer_and_status`: Clears a byte at offset 57 of a0, loads a word from a pointer at offset f0, subtracts 30, and calls a routine via that pointer.
- `update_status_from_call`: Calls another routine, stores the result in a 16-bit word at offset 50 of a structure, and sets a 16-bit word at offset 30 to either 1 or 2 depending on the sign of the returned value.
- `scale_returned_value`: Calls another routine, then performs a series of arithmetic shifts and additions on the result in d0 before returning.
- `decrement_and_branch_on_sign`: Decrements a 16-bit word at offset 60 of the structure in a0, then branches based on the sign bit, stores a byte at offset 59 of the structure, and either branches or jumps to a fixed address.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_and_test_sign`: Decrements a 16-bit word at offset 10 of the structure in a0, tests the result, and either branches or jumps to a fixed address.
- `dispatch_and_initialize`: Loads words from offsets 2 and 4 of a1 and a word from offset 8 of a2, then calls a routine via a pointer in a2.
- `update_entity_state_and_counters`: Validates a 16-bit value at offset 10 and a 16-bit value at offset 12 of a structure in a0, otherwise it calls another routine via a pointer in d6.
- `initialize_structure_fields_2`: Sets several fields in a structure, including a byte at offset 50, a word at offset 54, and a word at offset 56, then conditionally updates offsets 10 and 12 using values from a table and a register.
- `update_structure_and_call`: Decrements a 16-bit word at offset 10 of the structure in a0, calls a routine via a pointer in d6 using values from offset 10 and 18, and either increments a 16-bit word at offset 18 or clears several fields and sets a 32-bit pointer at offset 42 of the structure in a0.
- `process_list_and_call`: Iterates through a list pointed to by offset 10 of the structure in a1, calls a routine via a pointer in d6 using values from the list and offset 8 of a2, and updates several fields in the structure in a0.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_word_at_offset`: Tests a word at offset 50 of a structure in a0; if it is not zero, it decrements the word at offset 50 and returns.
- `validate_structure_values`: Decrements a word at offset 10 of a structure in a0, saves two words from that offset into d0 and d1, and jumps to an offset if the values fall outside the range of 16 to 368 for d0 or 32 to 224 for d1.
- `compare_structure_value`: Loads a word from offset 58 of a structure in a0 into d4, compares it to a word from a global variable, and if d4 is higher, branches to another routine; otherwise it sets d7 to 10, compares d1 to 136, and if d1 is below 136, sets d7 to 224 and d6 to 4 before branching to another routine.
- `process_structure_counters`: Decrements a 16-bit word at offset 56 of a structure, compares two 16-bit words from a global variable against two 16-bit words in a0, calls a routine via a pointer in d2, and updates bytes at offsets 54 and 55 of the structure.
- `call_and_adjust_offset`: Adds values to d0 and d1, calls a routine via a pointer in d6, and if the result is not zero, adds a value to offset 12 of a structure and performs arithmetic on d0 based on a byte at offset 54.
- `add_to_structure_on_condition`: Adds 34 to d0, calls a routine via a pointer in d6, and if the original d0 value was negative 34, adds the value in d4 to a 16-bit word at offset 10 of the structure in a0.
- `loop_and_initialize_globals`: Sets two global variables, then enters a loop that calls a routine via a pointer in d6, updates several fields in the structure in a0, and eventually sets a 32-bit pointer at offset 42 of the structure in a0.
- `frag_tst_bpl_jmp`: Short code fragment that tests a fixed memory address, then branches if the last result was not negative, then jumps to another point in the code.
- `calculate_signed_difference`: Tests a longword at a global address and branches if the most significant bit is set; otherwise, it subtracts a global longword from a word in a data register, negates the result if it is negative, and if the result is greater than or equal to 70, clears a word at offset 46 of the structure in a0.
- `update_timer_and_counters`: If a 16-bit word at offset 46 of a structure is zero, it calls a routine via a pointer in d2, adds 25 to the result, and stores the result in offsets 46 and 48 of the structure; otherwise, it decrements a 16-bit word at offset 52 and potentially clears a byte at offset 32 and sets a byte at offset 34.
- `fetch_table_values`: Loads two bytes from tables using a 16-bit word from offset 50 of a structure as an index, and if the second byte is not 127, clears the word at offset 50.
- `update_counters_and_call_routines`: Calculates values using a global variable and a value at offset 48 of the structure in a0, calls several routines, and updates various fields in the structure in a0 based on the state of a global variable and a value at offset 50.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a0, then returns.
- `frag_st_rts`: Short code fragment that sets a fixed memory address to all ones, then returns.
- `insert_into_sorted_list_2`: Calls a routine via a pointer in d6, then performs a linked-list insertion into a global list using values from the structure in a0 and a1.
- `branch_on_structure_sign`: Calls another routine, tests a word at offset 10 of a structure in a0, and jumps to an offset if the word is not positive.
- `decrement_structure_values`: Tests a word at offset 52 of the structure in a0 and branches if it is negative; otherwise, it decrements that word, compares a byte at offset 50 to 1, and if they are not equal, returns, but if they are equal, it decrements a byte at offset 48 of the structure.
- `branch_on_byte_value`: Reads a byte from offset 50 of a structure and branches to another routine based on whether that byte is 1, 2, or 3; otherwise, it decrements a 16-bit word at offset 10 of the structure and returns.
- `decrement_and_call_routine`: Adds values to d0 and d1, calls a routine via a pointer in d6, and if the result is not zero, decrements words at offsets 54 and 10 of a structure if a byte at offset 54 is not negative.
- `clear_structure_fields`: Sets a byte at offset 50 of the structure in a0 to 1, a 16-bit word at offset 52 to 50, a 16-bit word at offset 54 to 27, and clears the 16-bit word at offset 30 and the bytes at offsets 34 and 32.
- `update_structure_counters`: Stores a long value at offset 42 of a structure, sets a byte at offset 34, and if a 16-bit word at offset 54 is not negative, decrements that word and a 16-bit word at offset 12.
- `set_structure_values`: Sets a byte at offset 50 of the structure in a0 to 2, a 16-bit word at offset 52 to 50, and a 16-bit word at offset 54 to 65.
- `decrement_and_increment_offset`: Checks if a word at offset 54 of the structure in a0 is negative; if so, it decrements a byte at offset 54 and increments a word at offset 52, otherwise it returns.
- `initialize_structure_values`: Stores the byte 3 at offset 50 of the structure in a0, then stores the words 20 and 40 at offsets 52 and 54.
- `update_structure_and_call_routine`: Saves two words from the structure in a0 to d0 and d1, adds 44 to d1 and 22 to d0, calls another routine, and if the result is not zero, increments a word at offset 10 and a byte at offset 12 of the structure in a0 before returning.
- `set_structure_fields`: Stores the byte 4 at offset 50 of the structure in a0, stores the longword an address at offset 42, and clears the words at offsets 30, 34, and 32.
- `execute_conditional_logic`: Calls another routine, decrements a byte at offset 60 of a structure, and if certain conditions regarding a calculated value and offset 60 are met, it calls two different routines and updates bytes at offsets 33 and 34 of the structure.
- `frag_subq_bmi_rts`: Short code fragment that subtracts 1 from the memory at an offset from a0, then branches if the last result was negative, then returns.
- `process_data_loop`: Stores the byte 115 at offset 61 of the structure in a0, then enters a loop that calls another routine four times, updating offsets 51, 52, 46, 54, and 52 with specific values and a value from a table.
- `update_position_with_bounds_check`: Decrements a word at offset 82 of the structure in a0 and performs several conditional checks and calls to other routines, eventually updating a value at offset 48 using a table lookup based on a shifted value from offset 48.
- `frag_move_rts`: Short code fragment that copies d1 to the memory at an offset from a0, then copies d2 to the memory at an offset from a0, then returns.
- `frag_move_clr_rts`: Short code fragment that copies 1 to the memory at an offset from a0, then clears the memory at an offset from a0, then returns.
- `decrement_and_branch`: Subtracts 16 from a word in d0 which was loaded from offset 10 of the structure in a0, calls another routine, and then either decrements a word at offset 10 of the structure or jumps to another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_clr_rts_2`: Short code fragment that copies 1 to the memory at an offset from a0, then clears the memory at an offset from a0, then returns.
- `clear_global_bytes`: Clears six consecutive bytes starting at offset 76 of a global table and stores the word 1250 at offset 106 of that table.
- `process_global_tables`: Compares a word at offset 246 of a structure to 3390; if it is greater, it performs several loops that call another routine to copy data from tables to memory locations, updates various global bytes, and handles conditional decrements of a word at offset 42 based on specific comparison values.
- `copy_memory_blocks`: Copies a longword from the pointer at the start of the structure in a0 to the pointer in a1, increments a1 by 244, and then copies the next longword from the structure in a0 to the new location in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_audio_dma_state`: Updates audio parameters and DMA controller settings based on values in registers and a global variable.
- `clear_registers`: Sets registers d0, d1, d2, d3, and d4 to zero and returns.
- `update_dma_and_audio_parameters`: Checks a flag in a global variable to enable DMA via the DMA controller and, if not already enabled, performs a loop that calculates audio values from tables, updates the audio hardware registers, and modifies a global table.
- `frag_lea_bra_2`: Short code fragment that loads the address of a location in the code into a1, then jumps to another point in the code.
- `update_audio_parameters`: Performs a loop that calculates values using a table, updates the AUD0LCH register, and writes to a global table and the DMA controller to manage audio playback.
- `push_registers_to_stack`: Pushes several words from registers a6, d0, a4, and d5 onto the stacks pointed to by a3 and a4.

### R-Type.adf bytes 1024-1536 (code never loaded by a reference run)

- `bk_refill_bit_buffer_3`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### R-Type.adf bytes 143872-144384 (code never loaded by a reference run)

- `clear_cia_interrupts_and_initialize`: Clears the least significant bit of the control registers for both CIA chips and performs three loops: copying sixteen words from a table to an offset in a6, filling a table with zero and increments of 208, and filling another table with zero and increments of 208 up to a count of 271.
- `copy_data_loop`: Copies a 32-bit value from d0 to a destination at an offset specified by a 16-bit value from a pointer in a1, repeating this process in a loop until a negative 16-bit value is encountered in the pointer's memory.
- `handle_keyboard_interrupt_2`: Checks a byte at offset 2048 of a structure in a5 and a sign bit at offset 1792; if certain conditions are met, it performs bitwise operations on a value from a hardware register, modifies values at offsets 420c, 420f, and 5136, and executes a loop that runs 56 times.

### R-Type.adf bytes 156672-157696 (code never loaded by a reference run)

- `process_bit_stream`: Calls two routines to read bits from a buffer and then returns a byte with bits 1 and 7 set.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### R-Type.adf blocks 1759-1759 (bytes 900608-901120) (code never loaded by a reference run, load address unknown)

- `copy_and_process_data_block`: Copies a block of data from a source to a destination, then iterates through the destination to perform conditional lookups and writes bytes based on the results of two subroutines.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `calculate_masked_value`: Performs arithmetic operations on d7 using d1, d3, and d6, then fetches a word from a table using a value derived from d1 and d6, and finally returns the result of a bitwise AND between that word and d0.

### R-Type.adf:blocks 66-275 (bytes 33792-141312)#unpacked

- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `branch_on_word_test`: Saves registers d0 through d7 and a0 through a7, loads a relative address into a6, and if the word at offset 12 of the structure in a6 is negative, it calls another routine and otherwise branches to another routine.
- `check_and_call_subroutines`: Tests two words at offsets 8 and 46 of a structure in a6; if both are non-zero, it calls two subroutines and then returns.
- `decrement_structure_word`: Loads a pointer from a6 into a4, loads a pointer into a5, tests a word at offset 44 of the structure in a4, and if it is zero, branches; otherwise, it decrements the word at offset 44 of a4 and returns.
- `copy_multiple_words`: Copies a word from offset 6 of a5 to offset 44 of a6, then copies nine consecutive words from offset 448 of a4 to offsets 8, 12, 16, 20, 24, 28, 32, 36, and 40 of a5, and calls another routine before returning.
- `iterate_and_call_routine`: Checks a 16-bit word at offset 70 of a structure in a6; if it is not zero, it enters a loop that repeatedly clears a 16-bit word at offset 10 of a structure in a6 and calls another routine, continuing as long as that word is non-zero.
- `reset_object_attributes_and_update`: Clears eight 32-bit values and sets eight 16-bit values to all ones within a structure pointed to by a5.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_pointer_table`: Iterates through a series of eight 16-bit values from a source table in a0, and for each value, if it is not negative, calculates a 32-bit address by adding a base address in a4 to a value fetched from a second table in a1, then stores both the 16-bit value and the calculated address into a structure pointed to by a5.
- `frag_clr_movem_rts`: Short code fragment that clears the memory at an offset from a6, then restores saved registers from the stack, then returns.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a0 to the memory at an offset from a5, then jumps to another point in the code.
- `copy_and_increment_values`: Copies a word from offset 2 of a structure in a0 to offset 6 of a structure in a5 and to offset 44 of a structure in a6, increments a word at offset 4 of the structure in a5, and branches to another routine.
- `compare_and_set_byte`: Compares a byte at offset 72 of a structure in a0 against 128, 255, and 254; if the byte is 254 and a word at offset 104 of the same structure is non-zero, it sets that word to one and moves a byte into d0.
- `decrement_word_on_condition`: Compares a byte at offset 72 of a structure in a0 to 253; if it is not equal, it continues, otherwise it tests a word at offset 106 and, if that word is not zero, decrements it and returns.
- `update_table_entry_with_offset`: Fetches a value from a table using an index stored at offset 104 of the structure in a0, then updates a byte at offset 40 of the structure in a6.
- `fetch_and_store_table_entry`: Compares a byte at offset 40 of a structure in a6 to 245; if not equal, it subtracts 240 from that byte, shifts the result left by two, and uses it as an index to load a longword from a table into a global variable at offset 2 of another structure.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_addi_bra`: Short code fragment that adds 1 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_addi_rts`: Short code fragment that adds 1 to the memory at an offset from a0, then returns.
- `reset_word_if_not_equal`: Compares a word at offset 4 of a structure in a5 with a word at offset 2 of the same structure; if they are not equal, it moves the word from offset 0 to offset 4 and branches to another location.
- `increment_and_call_reset`: Increments a 16-bit word at offset 4 of a structure pointed to by a5, calls another routine, sets a 16-bit word at offset 10 of a structure pointed to by a6 to 1, and returns.
- `decrement_byte_conditionally_3`: Tests a byte at offset 74 of a structure in a0; if it is not zero and not 255, it decrements the byte and branches.
- `frag_move_bra_2`: Short code fragment that copies 255 to the memory at an offset from a0, then jumps to another point in the code.
- `decrement_and_update_structure_entry`: Decrements a byte from a value at offset 35 of a structure in a6 and stores it at offset 74 of a structure in a0, then copies a word from offset 42 of the structure in a6 to offset 104 of the structure in a0 before branching.
- `update_entity_pointer_from_table`: Fetches a byte from offset 39 of a structure, masks it, and stores it at offset 72 of another structure.
- `frag_move_bra_3`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_rts_2`: Short code fragment that copies 255 to the memory at an offset from a0, then returns.
- `check_entity_status_and_branch`: Loads a 32-bit value from an offset of 40 in a6 into d0, checks a 16-bit word at offset 8 of a0, and either calls a subroutine or branches to another location based on whether that word is zero.
- `set_bits_and_modify_byte`: Sets a bit in a byte at the memory location pointed to by a2 and performs five bitwise OR operations on d0 with different constant byte values.
- `call_subroutine_repeatedly`: Loads three addresses into a6, a5, and a4, then calls another routine four times, incrementing a5 by 4 and a4 by 16 before each call.
- `branch_on_word_value`: Tests a 16-bit word at offset 2 of a structure in a5 and branches if it is zero or negative; if the condition is met, it calls four different routines in sequence, otherwise it calls one routine and returns.
- `update_structure_fields`: Tests a 16-bit word at offset 2 of a structure in a5 and branches based on the result; if the value is negative, it clears words at offsets 64 and 66 of the structure, sets words at offsets 2 and 98 to all ones, and returns if the word at offset 66 is zero; otherwise, it decrements the word at offset 66 and returns.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `reset_dma_and_counters`: Updates the DMA control register using a word from an offset of 52 in a structure, clears three bytes at offsets 70, 92, and 112, increments a counter at offset 64, and branches to another routine.
- `frag_move_bra_4`: Short code fragment that copies the memory at an offset from a5 to a fixed memory address, then jumps to another point in the code.
- `update_indexed_pointer`: Masks a 32-bit value from a structure in a6 to 24 bits, adds an offset from a6 to it, and stores the result into a pointer at offset 176 of a structure in a5 and into the location pointed to by a4.
- `frag_move_bra_5`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then copies the memory at an offset from a6 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then jumps to another point in the code.
- `decrement_byte_conditionally_2`: Tests a byte at offset 98 of a structure in a5; if the byte is zero, it branches to another location, and if the byte is 255, it branches to another location; otherwise, it subtracts 1 from the byte at offset 98 and branches to another location.
- `frag_move_bra_7`: Short code fragment that copies 255 to the memory at an offset from a5, then jumps to another point in the code.
- `decrement_and_update_jump_table_index`: Decrements a byte at offset 39 of a structure in a6 and stores the result at offset 98 of a structure in a5, then copies a word from offset 42 of a6 to offset 64 of a5 before branching.
- `initialize_structure_from_table`: Calculates a pointer by adding an index from a byte at offset 41 of a structure in a6 to a base address, then stores this pointer and a value from offset 42 of the structure into a second structure in a5.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a5, then returns.
- `calculate_offset_value`: Adds a byte from offset 41 of a structure in a6 to a byte from offset 17 of a structure in a5, shifts the result left by 1, and uses it as an index into a table to fetch a word.
- `update_structure_value`: Takes a byte from offset 41 of a structure in a6, masks it, shifts it left by 1, and uses it to index a table to fetch a word.
- `update_object_state_and_index`: Copies a byte and a word from a structure pointed to by a6 into a structure pointed to by a5, sets a byte at offset 131 of the a5 structure to 1, and increments a counter at offset 64 of the a5 structure before branching.
- `frag_move_asl_add_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then shifts d0 left by 1, then adds the memory at an offset from a5 to d0, then adds the memory at an offset from a6 to d0, then copies d0 to the memory at an offset from a4, then copies d0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_8`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a4, then copies the memory at an offset from a6 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_9`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5 (4 times in a row), then jumps to another point in the code.
- `update_structure_and_counter`: Stores a 16-bit value split into two bytes at a pointer in a6, clears a bit in a byte at a6 minus one, and if a byte at offset 148 of a5 is non-zero, performs arithmetic on values at offsets 144, 160, and 192 of a5, potentially updating a 16-bit value at offset 6 of a4 and decrementing a byte at offset 147 of a5.
- `update_structure_values`: If a byte at offset 146 of a structure in a5 is non-zero, it adds a signed byte from offset 128 to a 16-bit value at offset 144, and if a 16-bit value at offset 192 is zero, it stores a sum in a location pointed to by a4.
- `update_structure_values_3`: Decrements a byte at offset 131 of a structure in a5 using a byte from offset 130, and if the result is non-zero, performs arithmetic comparisons between words at offsets 160, 192, and 194 of the structure to potentially clear offset 192 and update offsets 194 and 6 of a4.
- `compare_and_branch`: Adds a 16-bit value from offset 192 of a structure in a5 to d0, compares the result to d1, and branches to another routine based on the comparison result.
- `decrement_byte_conditionally`: Tests a byte at offset 112 of a structure in a5 and a byte at offset 113 of the same structure; if both are non-zero, it subtracts 1 from the byte at offset 113 and returns, otherwise it returns.
- `modify_byte_and_store`: Moves a byte from offset 112 to 113 of a structure in a5, subtracts a byte from offset 115 from offset 96, and depending on the comparison with a byte from offset 114, either stores a value at offset 8 of a4 or clears offset 112.
- `modify_and_store_byte`: Adds a byte from offset 115 of a structure in a5 to a byte at offset 96 of the same structure, then compares this result to d0 and stores a masked version of the result in a location pointed to by a4.
- `calculate_table_offset`: Calculates an offset by rotating d0, masking it, and shifting it, then adds this to a pointer in a5.
- `update_table_from_register`: Stores bytes from d0 and d2 into offsets 53, 49, 31, and 23 of a5, calculates an address by adding a scaled value from a6 to a pointer, and stores a value at offset 48 of a5.
- `clear_dma_channel`: The routine selects an entry from a table using the low two bits of the input word and copies the corresponding value into a global variable.
- `store_word_to_memory`: Stores a 16-bit word from d0 into a memory location at an offset from a pointer in a6.
- `copy_table_entries_and_set_flags`: Calls two other routines, copies four table entries selected by a masked index into global words, clears two words in memory, sets a bit in a global variable and writes an all-ones value to a word.
- `reset_audio_channels`: Clears four word-sized values at specific offsets in a table, sets a value to fifteen at a global address, and clears four word-sized values at offsets two, six, ten, and fourteen of a table.
- `write_three_values_to_table`: Stores d0 and d1 into a table at an offset from a6, and stores the constant one into a third location in that table.
- `store_and_update_globals`: Saves registers, copies a global variable to two different memory locations, and then updates the global variable to a new constant value.
- `execute_subsystem_calls`: Saves a6 to the stack, loads a pointer from a table into a6, and copies a longword from an offset of a6 into a global variable before returning; otherwise, it performs several conditional checks, calls three different routines, and tests bits 6 and 7 of a hardware register.
- `test_interrupt_bit`: Saves all registers, reads a word from a hardware register, and if bit 5 is set, calls another routine; otherwise it returns.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `parse_and_load_title_assets`: Parses a string to extract filenames, specifically looking for the sequence 'mdat' to copy it and 'smpl' to a second buffer, then allocates memory and loads the resulting file paths into two separate memory buffers via library calls before calling another routine.
- `free_two_memory_blocks`: Calls a routine twice to free two different memory blocks using pointers and sizes retrieved from a structure in a6.
- `clear_color00_register`: The routine clears the color register by writing a sequence of values to a global variable, effectively resetting COLOR00.
