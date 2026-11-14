# Game 1865 (final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01865). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905536 top-level bytes in 81 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 37 raw-word lines |
| Functions named and described | FAIL, 176 of 441 named (0 author symbols), 176 of 441 described (2 by rule, 172 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1431 of 1446 (98.96%) |
| Recovered without load address (counted in proof 1, flagged) | 47376 bytes, 37 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11633 of 11636 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10316 |
| code | 36676 |
| data | 787807 |
| padding | 9297 |
| unused | 61440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:final | 1 | 9234 | 351 |
| Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:FFC! | 2 | 85 | 4 |
| Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1135 | 46 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:FFC!#unpacked | 1 | 1147 | 37 |

## Cases handled

- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bred2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (14988 -> 28678 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:guy.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (23204 -> 46450 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:weapons.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (4380 -> 6646 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bred.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (15676 -> 28422 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:heads.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (7696 -> 8626 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:trainstrap.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (2564 -> 2672 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bigboss.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (18680 -> 34928 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:axl.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17196 -> 33600 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:gymnast.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (18364 -> 40350 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:axl2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17036 -> 33600 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:chars.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (7732 -> 9390 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:cody.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (37776 -> 68972 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:haggarpics.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16200 -> 21882 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:j.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (12192 -> 22100 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:rasta.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (24080 -> 39182 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:giant2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17812 -> 34890 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:chandelier.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (4688 -> 6908 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bred3.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16008 -> 28550 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:guys.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (13244 -> 17766 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:maptrain.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (13760 -> 17062 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:knifeguy.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17744 -> 33564 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:dontusedrugs.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (6708 -> 8452 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:j2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (12440 -> 22236 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:giant.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17324 -> 37092 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:fflogo.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (9972 -> 13506 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:cop.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (26668 -> 41304 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:townmap.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (10168 -> 13694 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bill.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (21856 -> 38870 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:haggar.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (23696 -> 49310 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:knifeguy2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16616 -> 33564 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:swordsman.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (19276 -> 34380 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:guys_ring.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (14484 -> 18256 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:mod.mit: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (132768 -> 180488 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bill2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (20468 -> 38870 bytes)
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:burntguy.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (2744 -> 4876 bytes)
- 35 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 873 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- final fight (1991)(u. s. gold)[cr skr](disk 1 of 2)[a]/Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:FFC!: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 22180 -> 47376 bytes), 10 of 10 windows confirmed in reference RAM, loaded at an address: recovered as u6
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 displacement, 1 immediate+displacement
- 1 packed track region(s), 11264 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `hash_string_and_search_header` (final): Computes a checksum of a null-terminated string and searches for a matching signature within a data structure. Kind: string hash and header search (likely a signature check for a specific disk format or loader). Pass path: DAT_00001af8 = 0; return 0;.
- `validate_and_load_disk_data` (final): Verifies the integrity of a data block using a checksum and marker validation before updating a global table with the processed data. Kind: track/checksum validation. Pass path: d0 == 0.
- `check_joystick_status` (final): Manipulates a bitmask in a global variable and checks a value retrieved from a table against a loop limit. Kind: disk hardware/track check. Pass path: the branch where the check does not trigger a failure (e.g., the code continues execution normally or returns 1/0 based on the specific logic flow).
- `reset_global_values` (final): Sets a global variable to zero and another memory location to all ones while touching a CIAB register. Kind: sabotage path / state reset. Pass path: sub_8652 returns 0, sub_86E4 returns 1, sub_7C86 continues execution.
- `initialize_hardware_parameters` (final): Assigns several values to a global variable and performs multiple calls to other routines before writing values to specific memory locations. Kind: disk hardware parameter initialization/validation. Pass path: the execution continues to the subsequent move instructions (setting hardware registers) if the parameters are accepted.

## Functions


### Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:final

- `verify_and_relocate_payload`: Verifies a specific signature across a series of memory blocks and updates a list of pointers with a new base address.
- `relocate_and_patch_memory`: Verifies a memory pattern, relocates a data block by applying an offset to a list of pointers, and copies a small block of code.
- `verify_and_relocate_memory`: Performs a memory integrity check by writing a pattern to a block of memory and verifying it, then relocates a data block by applying an offset to its contents using a table of pointers.
- `store_register_to_pointers`: Stores a byte from a data register into a memory location pointed to by one pointer, and clears a second memory location pointed to by another pointer.
- `memmove_block_copy`: Copies a block of memory from a source address to a destination address, handling overlapping memory regions by choosing between forward or backward copying directions.
- `copy_memory_block`: Copies a block of words from one memory location to another using a counter.
- `sub_0176`: undescribed.
- `sub_01A4`: undescribed.
- `sub_01C2`: undescribed.
- `sub_021E`: undescribed.
- `sub_0228`: undescribed.
- `sub_0260`: undescribed.
- `write_word_to_pointer`: Loads a value from a global variable and writes a specific word to the memory location pointed to by that value.
- `store_word_at_pointer`: Writes a specific word to the memory location stored in a global variable.
- `blit_sprite_tile_data`: Copies sprite or tile data from a source table into a destination buffer using a nested loop structure.
- `assign_constant_value`: Stores a specific word value into a global variable.
- `parse_sprite_attribute_table`: Iterates through a data table to extract sprite attributes and updates a target structure with coordinate and attribute data.
- `update_display_parameters_2`: Updates a series of memory locations with values derived from a global table and a pointer.
- `update_display_parameters`: Updates a series of memory locations with values derived from a global table and a pointer.
- `reset_global_flag`: Checks the value of a global variable and resets it to zero or one based on specific comparison conditions.
- `calculate_display_coordinates`: Calculates two coordinate values based on input registers and a global offset.
- `calculate_bitmask_values`: Performs bitwise operations on input registers and stores the resulting values into a structure.
- `return_stack_value`: Returns the value currently stored at the top of the stack.
- `copy_memory_blocks`: Performs a nested loop to copy data from a source table to a destination buffer.
- `update_structure_fields_3`: Polls a status bit in a structure and then writes several constant values and register contents to specific offsets within that structure.
- `update_entity_status`: Polls a bitmask in a structure until a specific flag is cleared, then overwrites several fields within that structure with constant values.
- `update_entity_state_parameters`: Polls a status bit in a structure until it clears, then writes several constant values and register contents to specific offsets within that same structure.
- `store_register_values`: Stores two 32-bit values from registers into a structure and writes a constant value to a third offset within that structure.
- `initialize_and_fill_structure`: Waits for a specific bit flag in a structure to clear before initializing several fields within that same structure and performing a bulk memory fill using a repeating pattern of values from RAM.
- `fill_memory_with_pattern`: Waits for a specific bit flag in a structure to clear before initializing several fields within that same structure.
- `initialize_data_structure`: Clears several fields in a structure using zero and sets four specific fields to a constant value of 300.
- `update_bitmask_from_hardware`: Performs a bitwise AND operation on two values and a constant, stores the result in two locations, and updates a global variable by ORing it with a shifted version of the result.
- `process_attribute_table`: Iterates through a table pointed to by a global variable, performing complex calculations to update structure members, bitmasks, and pointers based on values found in the table.
- `set_structure_parameters`: Waits for a specific bit in a structure to clear, then populates several fields in that structure with specific constants and register values.
- `sub_1C80`: undescribed.
- `update_global_byte`: Checks a specific bit in a byte and updates a global variable accordingly.
- `do_nothing`: This routine is empty and performs no operations.
- `reset_structure_and_table`: Repeatedly writes several constant values into a structure and clears a range of memory at a fixed address.
- `sub_1DB6`: undescribed.
- `configure_hardware_and_tables`: Writes values to various hardware registers and populates a table with a specific routine address.
- `fill_data_array`: Fills a table with repeating patterns of incrementing values and constants based on a global variable.
- `sub_1EBA`: undescribed.
- `initialize_zero_table`: Fills a table with alternating values, where one word is an incrementing constant and the next word is zero.
- `clamp_table_values`: Iterates through a table and updates its contents by combining nibbles from a source table, ensuring the resulting values are clamped within specific ranges.
- `initialize_runtime_data_tables`: Populates several memory tables with interleaved incrementing values, constants, and data from a source table.
- `populate_data_table`: Iterates through a loop to interleave values from a source table into a destination table using a specific incrementing pattern.
- `configure_hardware_registers`: Writes specific values to several hardware registers, including CIA and interrupt control registers, and a memory location.
- `update_global_from_input`: Sets a global variable to zero and waits for a specific bit to be set in a global variable before clearing it again.
- `modify_bit_in_table`: Uses a value to calculate a bit position and an offset to set or clear a specific bit within a table.
- `update_value_from_string`: Uses a byte to index into a string table and updates a global variable based on the character found.
- `sub_228A`: undescribed.
- `sub_22D0`: undescribed.
- `write_interrupt_request`: Calls another routine and writes a value to a hardware register.
- `initialize_system_state`: Clears a large block of memory, populates a table with function pointers, and performs a series of hardware and software initializations.
- `populate_function_pointer_table`: Iterates through a memory block to store a sequence of register values and a null pointer into consecutive memory locations.
- `initialize_system_state_2`: Clears a large block of memory, populates a table with function pointers, and performs hardware and software initializations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_null_terminated_strings`: Iterates through a list of pointers to copy null-terminated strings from one memory location to another.
- `call_subroutine_and_recurse`: Calls another routine and then calls itself.
- `execute_stored_callback`: Checks if a global variable contains a non-zero value and, if so, calls the function pointed to by that variable or passes a value to another routine.
- `initialize_structure_fields_2`: Resets several fields within a structure and a separate memory location if a specific global variable is non-negative.
- `initialize_structure_fields`: Checks if a value in a table is non-negative and, if so, resets several fields within that structure and a separate memory location.
- `process_entity_collisions`: Iterates through a list of entities to check for spatial overlaps and specific bitmask conditions.
- `update_entity_state_and_animation`: Updates an entity's internal timer and state based on a table lookup, then modifies structure members and triggers subsequent processing routines.
- `compare_list_elements`: Iterates through a list of structures to perform complex comparisons involving bitmasks, coordinate ranges, and data retrieved from a table.
- `process_complex_list`: Iterates through a list of structures, performing various comparisons and arithmetic operations on their members.
- `sub_2F7C`: undescribed.
- `sub_2F94`: undescribed.
- `sub_2FC0`: undescribed.
- `sub_3024`: undescribed.
- `reset_structure_data`: Clears several fields within a structure and resets a bit in a byte at a specific offset.
- `search_list_for_range`: Iterates through a list of structures to find an entry that satisfies specific bitmask requirements and falls within a defined coordinate range.
- `search_list_elements`: Iterates through a list of data to find elements that satisfy specific range and bitmask conditions.
- `clear_conditional_table_entries`: Iterates through a table and sets an entry to zero if it is non-zero and a specific bit in a corresponding offset is set.
- `zero_memory_block_2`: Fills a block of memory with zeros by iterating through a table in increments.
- `zero_memory_block`: Fills a large block of memory with zeros by iterating through a table in increments.
- `perform_redundant_loads`: The routine performs two consecutive load operations of a global variable into itself without modifying the value.
- `sort_and_process_table`: Filters a table based on range and bitwise criteria, sorts the resulting entries using a bubble sort, and performs memory operations on the sorted data.
- `sub_323C`: undescribed.
- `sub_324E`: undescribed.
- `clear_structure_bit`: Calls another routine and then clears a specific bit in a byte at a fixed offset within a structure.
- `sub_4DEE`: undescribed.
- `sub_4E40`: undescribed.
- `sub_4F8C`: undescribed.
- `sub_5002`: undescribed.
- `sub_506E`: undescribed.
- `sub_50A4`: undescribed.
- `sub_5122`: undescribed.
- `sub_5224`: undescribed.
- `sub_524A`: undescribed.
- `sub_525C`: undescribed.
- `sub_5286`: undescribed.
- `sub_53E8`: undescribed.
- `sub_53F8`: undescribed.
- `sub_5412`: undescribed.
- `sub_5466`: undescribed.
- `sub_546A`: undescribed.
- `update_structure_fields`: Clears specific fields within a structure and updates a value at a fixed offset based on a global variable and a bitwise calculation.
- `nop_padding_sequence`: This routine is empty and consists of instructions that perform no operation.
- `parse_data_table_3`: Iterates through a table of data entries, performing indexed lookups and conditional jumps to populate structures.
- `find_matching_element`: Iterates through a table of pointers and values to find the first element that satisfies a bitwise condition and a specific value match.
- `search_nested_data_tables`: Iterates through a series of nested tables by performing indexed lookups and searching for specific values.
- `process_linked_lists`: Iterates through a list of integers to clear bits in a structure and then processes a second list to set bits in another structure.
- `initialize_and_populate_data_structures`: Clears specific memory fields and populates various data structures by copying values from tables, performing indexed lookups, and updating structure fields based on status bit polling.
- `count_matching_elements`: Iterates through a table of data and increments a counter if a specific byte at an offset within the table entry matches a specific value.
- `filter_objects_to_list`: Iterates through a table of structures and, if a specific bit is set, stores a pointer to the structure in a list.
- `decrement_unnamed_variable`: Decrements a global variable in a loop until it reaches a negative value.
- `update_object_animation_and_velocity`: Updates an object's internal animation frame counter and handles sprite index lookups from a table.
- `get_table_index`: Calculates an index into a table using two input values and returns a masked word from that table if the index is within a specific range.
- `search_nested_table`: Performs a nested loop to search through a table using an index and a masked value retrieved from another routine.
- `modify_structure_offsets`: Updates several word-sized values within a structure using arithmetic operations and conditional negation based on a specific offset.
- `populate_entity_structure_from_table`: Iterates through a list of structures to find an empty entry, then initializes it by copying multiple fields from a data table.
- `initialize_structure_from_table`: Loads a pointer from a table into a structure and populates numerous fields within that structure using data from the pointed-to memory.
- `reset_structure_fields`: Clears several fields within a structure and sets a specific bit in a byte at a given offset.
- `sub_7006`: undescribed.
- `sub_7062`: undescribed.
- `sub_70CA`: undescribed.
- `sub_7106`: undescribed.
- `clear_global_variable`: Sets a global variable to zero and performs several arithmetic operations on it.
- `update_timer_value`: Compares a value in a structure against an input and updates a field with a negative constant if the difference falls within a specific range.
- `sub_7226`: undescribed.
- `clear_and_initialize_memory_buffers`: Clears two contiguous memory blocks by zeroing out a series of longwords and words.
- `recursively_process_global`: Decrements a global variable and, if it reaches zero, performs a series of recursive calls and updates the variable based on a table lookup.
- `trigger_text_display`: Calls another routine and then enters an infinite recursive loop.
- `clear_memory_blocks`: Iterates through a list of pointers to zero out memory and then clears a separate fixed-size buffer.
- `update_linked_list_offsets`: Iterates through a list of pointers, updating their values and decrementing them by a specific offset.
- `traverse_linked_list`: Iterates through a linked list by following pointers and compares a calculated value against a global variable.
- `process_linked_list_flags`: Iterates through a linked list to clear a high bit in a byte at a specific offset, sets a high bit in another byte via a table lookup, and searches for the minimum unsigned word value in a list.
- `process_linked_list_data`: Traverses a linked list, calculates relative offsets, and performs data decompression and byte-swapping.
- `calculate_relative_offsets`: Calculates and stores relative offsets from a base pointer into a destination table, with the calculation method changing based on a global flag.
- `generate_lookup_tables`: Populates a table with incrementing values and generates a second table of bit-manipulated bytes, then uses these to fill a global buffer.
- `update_global_counter`: Executes a function pointer if it is valid and performs complex incrementing, wrapping, and resetting logic on a global variable.
- `write_constant_sequence`: Writes a specific sequence of constant values to a global variable.
- `check_and_assign_pointer`: Checks if a global variable is zero and, if not, assigns the current stack pointer to that global variable before returning the value pointed to by the new address.
- `update_global_with_stack_address`: If a global variable is non-zero, it updates that variable with the address of a local stack buffer.
- `hash_string_and_search_header`: Computes a checksum of a null-terminated string and searches for a matching signature within a data structure.
- `recursively_process_indexed_values`: Checks if a value is below a threshold and, if it differs from a global variable, updates that variable and calls itself recursively with a composite value.
- `validate_and_load_disk_data`: Verifies the integrity of a data block using a checksum and marker validation before updating a global table with the processed data.
- `check_input_status`: Checks bits in a global variable and calls other routines based on their state before storing a byte into a memory location.
- `update_hardware_control_bits`: Modifies specific bits of a global variable based on the value of a register and updates hardware port registers.
- `update_hardware_status`: Modifies a global variable and several other memory locations while interacting with CIAA and CIAB port registers.
- `check_joystick_status`: Manipulates a bitmask in a global variable and checks a value retrieved from a table against a loop limit.
- `reset_global_values`: Sets a global variable to zero and another memory location to all ones while touching a CIAB register.
- `store_indexed_value`: Stores a global variable into a memory location calculated using that same variable as an index.
- `increment_global_counter`: Increments a global variable and performs a check to see if it has wrapped or returned to zero.
- `calculate_string_length`: Iterates through a null-terminated string to determine its length.
- `initialize_hardware_and_memory`: Writes values to specific memory offsets, clears a block of memory, and waits for a bit in a hardware register to clear.
- `initialize_system_memory`: Writes specific values to memory offsets, clears a block of memory, and waits for a hardware register bit to clear.
- `initialize_hardware_parameters`: Assigns several values to a global variable and performs multiple calls to other routines before writing values to specific memory locations.
- `configure_cia_interrupt_and_data`: Sets specific values into memory locations via a pointer and clears a global variable before calling another routine.
- `sub_88B4`: undescribed.
- `populate_memory_grid`: Iterates through a series of memory locations, filling each with a byte retrieved from a table based on a global variable.
- `fill_memory_blocks`: Fills multiple memory blocks with values retrieved from a table based on a global variable.
- `sub_88D2`: undescribed.
- `skip_whitespace_and_copy`: Iterates through a buffer, skipping space characters, and processes data until a specific byte threshold is met.
- `copy_structured_data`: Copies specific bytes from a source table into a destination buffer using a structured loop.
- `transfer_large_blocks`: Transfers data from one memory area to another by copying specific word segments in a loop.
- `sub_899C`: undescribed.
- `sub_89C6`: undescribed.
- `sub_89FE`: undescribed.
- `sub_8A1C`: undescribed.
- `copy_memory_block_2`: Copies sixteen bytes from a source pointer to a destination pointer and sets a global variable to a specific value upon completion.
- `sub_8A50`: undescribed.
- `sub_8A7A`: undescribed.
- `sub_8AC0`: undescribed.
- `sub_8C0A`: undescribed.
- `sub_8C4E`: undescribed.
- `clear_pointer_chain`: Iterates through a linked list of pointers, setting each referenced memory location to zero until a null pointer is encountered.
- `calculate_linear_feedback`: Performs a bitwise shift and conditional XOR operation on a global variable based on a loop count and returns a derived 16-bit value.
- `sub_8CA0`: undescribed.
- `sub_8CE8`: undescribed.
- `sub_8CEA`: undescribed.
- `sub_8CFA`: undescribed.
- `sub_8D16`: undescribed.
- `sub_8D42`: undescribed.
- `sub_8D4E`: undescribed.
- `decompress_and_expand_data`: Copies a block of data into a temporary buffer while performing a byte-swap on each word.
- `copy_and_swap_data_blocks`: Copies a block of data to a new location and performs word-level byte swapping on the copied values using a loop.
- `sub_9078`: undescribed.
- `sub_90A8`: undescribed.
- `sub_90B4`: undescribed.
- `sub_90E6`: undescribed.
- `sub_90F8`: undescribed.
- `sub_9104`: undescribed.
- `sub_9134`: undescribed.
- `sub_9158`: undescribed.
- `sub_9164`: undescribed.
- `sub_9182`: undescribed.
- `sub_9194`: undescribed.
- `sub_91A0`: undescribed.
- `sub_91D0`: undescribed.
- `sub_91E6`: undescribed.
- `sub_91FC`: undescribed.
- `sub_920E`: undescribed.
- `sub_921A`: undescribed.
- `sub_924A`: undescribed.
- `sub_9272`: undescribed.
- `sub_9284`: undescribed.
- `sub_9290`: undescribed.
- `sub_92C0`: undescribed.
- `sub_92D6`: undescribed.
- `sub_92DC`: undescribed.
- `sub_92EA`: undescribed.
- `sub_935A`: undescribed.
- `sub_9382`: undescribed.
- `sub_93A2`: undescribed.
- `sub_93C2`: undescribed.
- `sub_93EC`: undescribed.
- `sub_9402`: undescribed.
- `sub_9406`: undescribed.
- `sub_942C`: undescribed.
- `sub_9442`: undescribed.
- `sub_947A`: undescribed.
- `sub_94AC`: undescribed.
- `sub_955C`: undescribed.
- `sub_9566`: undescribed.
- `sub_95B4`: undescribed.
- `sub_95C4`: undescribed.
- `sub_95EA`: undescribed.
- `sub_9602`: undescribed.
- `sub_9632`: undescribed.
- `sub_9662`: undescribed.
- `sub_970A`: undescribed.
- `sub_9710`: undescribed.
- `sub_9720`: undescribed.
- `sub_9788`: undescribed.
- `process_string_data`: Checks a character at a memory location and iterates through a buffer until a null terminator is reached.
- `perform_recursive_calls`: Checks a global variable and performs multiple recursive calls to another routine based on that value.
- `execute_loop_count`: Executes a loop a fixed number of times based on a local counter.
- `convert_to_ascii`: Recursively calls itself and adds an ASCII offset to a global variable.
- `convert_numeric_string`: Converts a value into ASCII characters by adding an offset to a byte, then iterates through a buffer while skipping whitespace characters.
- `convert_nibbles_to_ascii`: Converts a value into a sequence of ASCII numeric characters by masking and shifting bits.
- `sub_98BE`: undescribed.
- `sub_98F2`: undescribed.
- `sub_991A`: undescribed.
- `sub_995C`: undescribed.
- `sub_99AC`: undescribed.
- `sub_99EC`: undescribed.
- `sub_9A14`: undescribed.
- `sub_9A56`: undescribed.
- `sub_9A78`: undescribed.
- `sub_9AC8`: undescribed.
- `sub_9AE2`: undescribed.
- `increment_and_wrap_global`: Increments a global variable and resets it to one if it reaches specific threshold values.
- `sub_9B54`: undescribed.
- `sub_9B60`: undescribed.
- `sub_9BB4`: undescribed.
- `sub_9D90`: undescribed.
- `sub_9EB2`: undescribed.
- `sub_9FE2`: undescribed.
- `sub_A006`: undescribed.
- `sub_A0F0`: undescribed.
- `sub_A5EE`: undescribed.
- `sub_A62C`: undescribed.
- `sub_A634`: undescribed.
- `sub_A6D6`: undescribed.
- `sub_A6EC`: undescribed.
- `sub_A712`: undescribed.
- `sub_A726`: undescribed.
- `sub_A72C`: undescribed.
- `sub_A736`: undescribed.
- `sub_A73C`: undescribed.
- `sub_A74A`: undescribed.
- `sub_A750`: undescribed.
- `sub_A75A`: undescribed.
- `sub_A7C8`: undescribed.
- `sub_A9C8`: undescribed.
- `sub_AB7E`: undescribed.
- `sub_ABEC`: undescribed.
- `sub_ACA6`: undescribed.
- `sub_AD18`: undescribed.
- `sub_AE0E`: undescribed.
- `sub_AFC8`: undescribed.
- `sub_AFE6`: undescribed.
- `sub_B0B2`: undescribed.
- `sub_B0D8`: undescribed.
- `sub_B0E8`: undescribed.
- `sub_B100`: undescribed.
- `sub_B128`: undescribed.
- `sub_B15E`: undescribed.
- `sub_B18C`: undescribed.
- `sub_B1C0`: undescribed.
- `sub_B1EE`: undescribed.
- `sub_B270`: undescribed.
- `sub_B2A4`: undescribed.
- `sub_B3D6`: undescribed.
- `sub_B412`: undescribed.
- `sub_B450`: undescribed.
- `sub_B484`: undescribed.
- `sub_B48E`: undescribed.
- `sub_B49C`: undescribed.
- `sub_BEEA`: undescribed.
- `sub_BF00`: undescribed.
- `sub_BF1A`: undescribed.
- `sub_BF2A`: undescribed.
- `sub_BF2C`: undescribed.
- `sub_BF5A`: undescribed.
- `sub_BF82`: undescribed.
- `sub_BF96`: undescribed.
- `sub_BFAE`: undescribed.
- `sub_BFC8`: undescribed.
- `sub_C01A`: undescribed.
- `sub_C032`: undescribed.
- `sub_C03C`: undescribed.
- `sub_C046`: undescribed.
- `sub_C050`: undescribed.
- `sub_C08C`: undescribed.
- `sub_C0A4`: undescribed.
- `sub_C0AE`: undescribed.
- `sub_C0B8`: undescribed.
- `sub_C15A`: undescribed.
- `sub_C16C`: undescribed.
- `sub_C172`: undescribed.
- `sub_C176`: undescribed.
- `sub_C734`: undescribed.
- `sub_C7F6`: undescribed.
- `sub_C7F8`: undescribed.
- `sub_C854`: undescribed.
- `sub_C87E`: undescribed.
- `sub_C9D6`: undescribed.
- `sub_CA6E`: undescribed.
- `sub_CA9E`: undescribed.
- `sub_CAC2`: undescribed.
- `sub_CAFE`: undescribed.
- `sub_CB20`: undescribed.
- `sub_CB2C`: undescribed.
- `sub_CB32`: undescribed.
- `sub_CB50`: undescribed.
- `sub_CB56`: undescribed.
- `sub_CC00`: undescribed.
- `sub_CC68`: undescribed.
- `sub_CC98`: undescribed.
- `sub_CCD4`: undescribed.
- `sub_CD84`: undescribed.
- `sub_CDA4`: undescribed.
- `sub_CDAA`: undescribed.
- `sub_CDEA`: undescribed.
- `sub_CE0A`: undescribed.
- `sub_CE42`: undescribed.
- `sub_CE58`: undescribed.
- `sub_CE60`: undescribed.
- `sub_CEA6`: undescribed.
- `sub_CEC6`: undescribed.
- `sub_CEDC`: undescribed.
- `sub_CF0E`: undescribed.
- `sub_CF40`: undescribed.
- `sub_CF68`: undescribed.
- `sub_CF74`: undescribed.
- `sub_CF7C`: undescribed.
- `sub_CF90`: undescribed.
- `sub_CFA6`: undescribed.

### Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:FFC!

- `decompress_bitstream_data`: Decompresses a bit-packed stream into a destination memory buffer using a variable-length coding scheme.
- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_interrupt_parameters`: Sets a pointer and a global variable before executing a trap and returning a value.
- `initialize_hardware_and_tables`: Clears a large memory range, populates several tables with constant values, and configures hardware interrupts and audio volumes.
- `update_structure_fields_2`: Increments a byte in a global variable and, if it reaches a specific value, resets it to zero before writing several multi-byte values to a structure.
- `copy_table_data_by_index`: Searches a table for a matching byte, then copies a block of data from a calculated offset to a destination address.
- `store_multiple_values_to_offset`: Stores several constant values into various offsets relative to a pointer.
- `update_memory_table_via_loop`: If a global variable is zero, the routine performs a loop that calculates and stores values into a memory range using data from two separate tables and an offset.
- `initialize_structure_loop`: Writes several constant values to a structure and then executes a loop that calls another routine a fixed number of times.
- `store_registers_to_memory`: Waits for a specific bit in a memory location to clear, then stores several registers into a structure.
- `copy_data_from_lookup_table`: Iterates through a list of bytes, finds matches in a lookup table, and copies the corresponding data to a destination.
- `store_constants_after_bit_check`: Checks a bit in a memory location and, once cleared, writes several constant values to a structure.
- `increment_counter_and_check_global`: Increments a global counter and, upon reaching a threshold, checks a global variable and updates another global pointer.
- `process_table_entries`: Waits for a bit in a byte to clear, modifies several fields in a structure, and then iterates through a table to call another routine based on specific value comparisons.
- `calculate_and_store_distance_metrics`: Calculates horizontal and vertical differences between two points and stores the resulting dimensions and bitmask into a structure.
- `reset_structure_data_2`: Waits for a bit in a byte to clear and then writes a series of constant values to a structure before calling another routine four times.
- `store_word_to_pointer`: Checks a bit in a byte pointed to by a register and, if not set, stores a 32-bit value and a 16-bit value into a structure at an offset from that pointer.
- `initialize_loop_data`: Waits for a bit to clear, writes constant values to a structure, and then iterates through a table to perform bitwise operations and call another routine.
- `initialize_structure_data`: Checks a bit in a byte pointed to by a register and, if not set, writes several constant values and multiple register values into a structure at an offset from that pointer.
- `increment_table_values`: Iterates through two tables and increments the values in the first table to match the corresponding byte components of the second table.
- `sub_06BC`: undescribed.
- `configure_hardware_interrupts`: Manipulates several hardware registers including DMA and interrupt control registers, performing a loop to check a value before setting specific register bits.
- `initialize_sound_module_from_header`: Silences the four audio channel volumes, copies per-channel offsets from a header into a table, clears four channel records and fills a sample pointer table with values spaced by a header byte.
- `clear_audio_volumes`: Sets four consecutive memory locations, which correspond to audio volume registers, to zero.
- `sub_08C2`: undescribed.
- `advance_sound_sequence_timed`: Repeats the sound sequence step of its neighbour while interacting with hardware registers for raster position and DMA control.
- `process_object_attributes`: Iterates through a list of pointers to update structure members, including bitwise inversions and coordinate adjustments based on internal flags.
- `load_channel_note_from_tables`: Looks up a period value from a table using a sequence byte, then loads a volume-like byte and a second parameter byte from parallel tables into the channel record.
- `apply_channel_slide_step`: Stores the result of a helper into the channel record and, while a slide counter is nonzero, adds the record's slide offset to a pitch word and advances the counter's fractional byte by a step value.
- `select_highest_byte`: Compares bytes within a list and stores the largest byte found into a structure, while decrementing a counter.
- `trigger_channel_note_and_period`: When a note is pending, resets the channel fields, loads the instrument's sample pointer, length and volume from tables, and computes the period from a note table.
- `slide_channel_period_within_limits`: Runs seven helpers, then adds the channel's slide offset to its period, clamping values below 96 to 95 and values above 5759 to 5760.
- `step_channel_counter_pitch`: Compares a channel repeat count with a step counter: while they differ it advances the counter, and when they match it adds a per-tick amount to the channel's pitch word.
- `update_entity_state_attributes`: Extracts a command index from a word within a structure and performs specific operations based on that index.
- `update_value_with_clamping`: Updates a value by adding a signed offset and clamps the result between a minimum and maximum range.
- `update_channel_period_from_table`: Extracts an index and a value from a word, uses the index to look up a value in a table, adds it to a base offset, and writes the result to a channel's period field.
- `sub_0DD0`: undescribed.
- `sub_0DDC`: undescribed.
- `sub_0DE6`: undescribed.
- `copy_byte_to_two_locations`: Copies a single byte from a table into two consecutive byte locations in another structure.
- `sub_0E66`: undescribed.
- `scale_and_store_value`: Stores a short value into a table and then stores its quadruple value, clamped to a maximum, into another location.
- `store_byte_if_nonzero`: Checks if a byte at an offset from a pointer contains a non-zero value and, if so, stores that value into another location.
- `step_channel_modulation_table`: When modulation is enabled it advances a phase counter, and at each phase boundary it adds a table value to the channel's pitch word and resets the phase counter.
- `update_channel_modulation_pitch`: Checks a flag and compares a counter against a threshold to trigger a pitch update.
- `advance_sample_loop_pointer`: Advances a sample-loop counter; at each boundary it loads the next sample entry's pointer and length from a table into the channel's pointer and length fields, and stops when an end marker is read.
- `write_channel_volume_register`: Calls a helper to manage a volume envelope and then writes the channel volume, reduced by a factor of four, into its hardware volume register.
- `run_channel_envelope_phase`: Runs a four-phase volume envelope: it ramps the channel volume up or down by table steps toward each phase level, counts a hold period, and moves to the next phase when each one finishes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_short_routine`: A routine consisting of fifteen instructions that is entered from outside static flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `access_cia_hardware`: A routine consisting of ten instructions that accesses the CIA hardware registers.

### Final Fight (1991)(U. S. Gold)[cr SKR](Disk 1 of 2)[a].adf:FFC!#unpacked

- `sub_0000`: undescribed.
- `sub_0048`: undescribed.
- `sub_0160`: undescribed.
- `sub_01A2`: undescribed.
- `sub_01E6`: undescribed.
- `sub_020C`: undescribed.
- `sub_02CE`: undescribed.
- `sub_031C`: undescribed.
- `sub_033A`: undescribed.
- `sub_0386`: undescribed.
- `sub_03C4`: undescribed.
- `sub_03F6`: undescribed.
- `sub_04A4`: undescribed.
- `sub_0532`: undescribed.
- `sub_057C`: undescribed.
- `sub_058C`: undescribed.
- `sub_0656`: undescribed.
- `sub_06EE`: undescribed.
- `sub_07CC`: undescribed.
- `sub_08A8`: undescribed.
- `sub_08F0`: undescribed.
- `sub_0A20`: undescribed.
- `sub_0ADA`: undescribed.
- `sub_0B1E`: undescribed.
- `sub_0B3E`: undescribed.
- `sub_0BA6`: undescribed.
- `sub_0C6C`: undescribed.
- `sub_0CBC`: undescribed.
- `sub_0CE0`: undescribed.
- `sub_0D2E`: undescribed.
- `sub_0D9A`: undescribed.
- `sub_0DCC`: undescribed.
- `sub_0E9A`: undescribed.
- `sub_0EF0`: undescribed.
- `sub_0F54`: undescribed.
- `sub_0FCC`: undescribed.
- `sub_0FDA`: undescribed.
