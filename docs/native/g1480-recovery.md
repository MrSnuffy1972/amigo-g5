# Game 1480 (combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01480). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1845920 top-level bytes in 67 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 442 of 442 named (0 author symbols), 442 of 442 described (18 by rule, 420 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2038 of 2039 (99.95%) |
| Recovered without load address (counted in proof 1, flagged) | 59860 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8000 of 8002 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 508541 |
| code | 29662 |
| data | 882668 |
| padding | 15449 |
| unused | 409600 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:TPX! | 2 | 267 | 8 |
| Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:Echo | 2 | 117 | 2 |
| Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:TUDE | 3 | 2068 | 158 |
| Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:cap.amg | 2 | 263 | 7 |
| Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:EasyInstaller | 4 | 2200 | 121 |
| Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1428 | 51 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 120 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 39 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 82 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 794 | 48 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 574 | 30 |
| spanish.lan (raw code file, load address unknown) | 1 | 0 | 0 |
| italian.lan (raw code file, load address unknown) | 1 | 0 | 0 |
| campaign (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/f18_n8p4.pcc: high-entropy data file (7.80 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/STARTUP.PCC: high-entropy data file (7.82 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F18_RR4P.PCC: high-entropy data file (7.80 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/PRINTER.PCC: high-entropy data file (7.76 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F14_L_4P.PCC: high-entropy data file (7.75 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F14_RL4P.PCC: high-entropy data file (7.81 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/f18hang.pcc: high-entropy data file (7.83 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F14_R_4P.PCC: high-entropy data file (7.75 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/f14_n8p4.pcc: high-entropy data file (7.82 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F18ZOOM.PCC: high-entropy data file (7.83 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/f14hang.pcc: high-entropy data file (7.77 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/PSYLOG16.PCC: high-entropy data file (7.71 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F18_L_4P.PCC: high-entropy data file (7.74 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/gfx/F18_R_4P.PCC: high-entropy data file (7.74 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 645 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 35 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 39 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 70 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 359 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 29 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 188 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/language/spanish.lan: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/language/italian.lan: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 2 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 2 of 2).adf:Cap/campaign: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 1 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:TPX!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- combat air patrol (1992)(psygnosis)(m5)[cr toxic - prelux](disk 1 of 2)/Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:cap.amg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 59860 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_and_manage_graphics_context` (runtime segment an address (play-rs-x25k)): Initializes several data tables and global variables, manages hardware DMA and interrupt settings, and enters a loop that repeatedly calls various subroutines with different constant values. Kind: disk/track/hardware validation loop. Pass path: the loop continues through various hardware/mode checks (u7__sub_0576) until u7__sub_03D8 is non-zero, which breaks the loop and proceeds to normal execution.
- `set_global_constants` (runtime segment an address (play-long-joy)): Writes a fixed constant into a global variable, then rewrites that same variable using a bitwise OR, AND, and OR sequence of constants. Kind: disk hardware/parameter initialization. Pass path: the instruction immediately following the call (the code continues execution normally).

## Functions


### Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:TPX!

- `unpack_amigados_hunk_data`: Parses a data stream containing AmigaDOS hunk type codes to allocate memory and relocate data segments.
- `decrement_loop`: Decrements a word value in a loop until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Performs bitwise operations and memory shifts on a data stream, moving bytes based on bit-flag conditions.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs complex bitwise shifts and rotations on data based on a variable number of bits, updating a status register and a target value through a loop.
- `modify_memory_and_cache`: Performs a trap, modifies a byte at a pointer location using an XOR operation, and stores a value into a global structure.

### Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block_6`: Copies a sequence of four words from a source address to a destination address using a loop.

### Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:TUDE

- `wait_and_reply_port_message`: Finds the current task, waits on its message port and then forbids task switching while it takes a message and replies to it.
- `initialize_configuration_system`: Opens required libraries and loads configuration data from a string into a global buffer.
- `parse_assignment_lines`: Iterates through a string looking for equals signs and newline characters, copying content following an equals sign to a new location and decrementing a counter.
- `search_string_for_key`: Searches through a string for a specific sequence of characters while ignoring case and whitespace, returning a pointer to the match or updating a counter.
- `load_configuration_data`: Calls library functions to retrieve data, calculates an offset, and copies a series of null-terminated strings into a global buffer.
- `reallocate_and_patch_memory_block_4`: Allocates a new memory block based on a calculated size and copies data from a variable-length stream into it.
- `reallocate_and_patch_memory_block_3`: Allocates a new memory block based on a calculated size and copies data from a variable-length stream into it.
- `copy_memory_block_4`: Copies a block of memory from a source to a destination and updates two pointers within a global structure.
- `copy_memory_block`: Copies a block of memory from a source to a destination using a loop and updates a pointer in a global structure.
- `reallocate_and_patch_memory_block_2`: Allocates a new memory block and copies data from a variable-length stream into it.
- `reallocate_and_patch_memory_block`: Allocates a new memory block and copies data from a variable-length stream into it.
- `parse_and_reallocate_data_structure`: Parses a variable-length data stream to extract values and updates a memory block.
- `resize_and_checksum_memory_block`: Resizes a memory block by allocating a new destination, copying existing data, and calculating a checksum for the new block.
- `reallocate_and_checksum_memory_2`: Resizes a memory block by allocating a new destination, copying existing data, and calculating a checksum for the new block.
- `reallocate_and_checksum_memory`: Resizes a memory block by allocating a new destination, copying existing data, and calculating a checksum for the new block.
- `reallocate_and_checksum_memory_block`: Determines an appropriate memory block size based on system availability and copies data to a new destination.
- `allocate_and_checksum_memory`: Determines a memory block size based on system availability and copies data to a destination.
- `allocate_and_verify_memory_block`: Iteratively queries memory availability to find a suitable block, then copies data to the destination and performs a bitwise complement checksum verification.
- `update_checksum_and_copy_data`: Copies a block of data from a global variable to a destination address and calculates a bitwise complement checksum of a memory range.
- `calculate_and_reset_state`: Sums a series of sixteen-bit values from a table, stores the bitwise complement of the sum in a global variable, and performs an indirect call.
- `reset_and_indirect_call`: Resets the system state and then performs an indirect call to an address retrieved from a jump table.
- `free_memory_and_call`: Calls a routine to free memory and then calls another unnamed routine.
- `set_status_bit_high`: Checks if a value in a global variable exceeds a threshold and, if so, sets a specific bit in a memory location.
- `set_flag_bit`: Sets a specific bit in a memory location using a bitwise OR operation.
- `execute_callback_sequence`: Calls a function pointer from a global structure, performs a memory copy if the result is non-zero, and executes a sequence of subsequent function calls and memory updates.
- `swap_memory_blocks_2`: Copies a block of data from one location to another and then copies it back in reverse order.
- `nop_placeholder_routine`: This routine is empty and performs no operations.
- `update_trap_parameter`: Stores the address of a global variable into a location indexed via a pointer within a table and executes a system trap.
- `trigger_system_trap`: Stores a pointer to a global variable into a memory location and executes a system trap.
- `execute_conditional_callback`: Checks a bit in a global variable and, if set, calls a function pointer before storing the result into a memory location.
- `parse_command_string_2`: Skips leading spaces in a string, finds a newline or space, and replaces it with a null terminator before calling a function via a pointer retrieved from a table.
- `parse_option_string`: Searches for a space or newline character in a string and replaces the found character with a null terminator.
- `verify_option_file_access`: Checks for the existence or accessibility of an option file and triggers an error message if the file cannot be opened.
- `patch_code_block`: Copies a block of instructions to a destination address and updates two specific offsets within that destination using values retrieved from a global variable.
- `patch_runtime_code`: Copies a fixed block of instructions to a destination address retrieved from a pointer.
- `patch_code_block_2`: Copies a fixed-size block of instructions to a destination address retrieved from a function return value.
- `patch_runtime_code_block`: Copies a fixed-size block of instructions from the current routine to a destination address retrieved through a series of function calls.
- `apply_screen_patch`: Copies a block of code to a destination address retrieved from a global variable and updates specific offsets within that destination.
- `check_kickstart_version`: Verifies if the system is running Kickstart 2.0 or higher by evaluating a string.
- `search_instruction_pattern`: Iterates through a list of memory locations looking for a specific pattern.
- `traverse_instruction_list`: Iterates through a list of memory locations, checking for a specific pattern and updating a pointer.
- `set_byte_bit`: Sets the second bit of a byte at a specific offset from a pointer.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself before returning.
- `initialize_callback_state`: Calls a function via a pointer from a global variable, performs several memory writes to a structure, and executes multiple function pointers.
- `clear_global_variable`: Sets a global variable to zero.
- `execute_callback_chain`: Retrieves a pointer from a global variable to execute a sequence of function calls.
- `update_callback_parameters`: Retrieves a pointer from a global variable, modifies several fields within a structure, and executes multiple function pointers.
- `set_global_constant`: Sets a global variable to a constant value.
- `set_bit_via_callback`: Calls a function pointer, performs a bitwise OR on a global variable, and then calls another function pointer.
- `clear_bit_via_callback`: Calls a function pointer, performs a bitwise AND to clear a bit in a global variable, and then calls another function pointer.
- `execute_sequential_display_callbacks`: Executes two separate display-related routines in succession.
- `call_update_display_state`: This routine acts as a wrapper that calls a function to update a display state structure, execute a series of function pointers, and perform a bitwise mask operation on a memory byte.
- `execute_display_callbacks_2`: Executes two sequentially stored function pointers and updates a display state structure by modifying a memory block via a bitwise mask.
- `execute_empty_recursion`: This routine calls itself twice and then returns.
- `call_display_update`: This routine acts as a wrapper that calls a function to update a display state structure.
- `execute_recursive_calls_2`: This routine calls itself twice before returning.
- `execute_display_callbacks`: Calls two separate routines sequentially.
- `dispatch_callback_chain`: Stores a word into a global variable and executes a sequence of function pointers if a specific condition is met.
- `execute_display_and_callback_updates`: Calls a routine to update a display state and then calls another routine to process callback data.
- `trigger_callback_chain`: Updates the display state and executes a sequence of function pointers stored within a global variable based on a conditional check.
- `execute_callback_chain_2`: Stores a word into a global variable and conditionally executes a sequence of function pointers retrieved from that variable.
- `process_callback_data`: Stores a word into a global variable and, if a condition is met, executes a series of function pointers retrieved from that variable.
- `set_callback_bit`: Retrieves a pointer from a global variable and sets a specific bit in a word at an offset from that pointer before executing function pointers.
- `execute_nested_callback_chain`: Retrieves a value from a function pointer stored in a global variable and stores it in a memory location.
- `execute_multiple_callbacks`: Stores a word into a memory location and then executes three different function pointers retrieved from various locations.
- `process_memory_block`: Iterates through a memory range in increments, executes a function pointer for each step, and updates a global variable.
- `process_large_memory_block`: Iterates through a large memory range, executes a function pointer for each step, and updates a global variable to a new limit.
- `patch_memory_block`: Copies a block of code to a newly allocated memory location and stores two values at specific offsets within that block.
- `copy_memory_block_5`: Repeatedly attempts to allocate memory, copies a block of code to a new location, and stores two values at specific offsets within that block.
- `test_register_bit`: Checks if a specific bit is set in a register and returns zero if it is set.
- `test_bit_and_return`: Checks if a specific bit is set in a register and returns zero if it is, otherwise it terminates execution.
- `nop_padding_sequence`: This routine is empty and contains no executable instructions.
- `update_data_pointers`: Checks two global values and, if both are non-zero, updates a pointer and increments a value before clearing a third value.
- `recursively_update_config_pointers`: Checks if two specific pointers in a data structure are non-zero, then recursively calls itself before updating a pointer with a new address and incrementing another pointer by a fixed offset.
- `copy_string_until_newline`: Copies characters from a source pointer to a destination pointer until a null terminator or a newline character is encountered.
- `call_conditional_function`: Checks a bit in a global variable and, if set, executes a function pointer retrieved from a global structure.
- `execute_conditional_sequence`: Checks a bit in a global variable and, if set, clears a memory location and executes three different function pointers from a global structure.
- `execute_callback_sequence_2`: Stores a value, executes two function pointers from a global structure, and then executes a third function pointer.
- `set_flag_one`: Stores the value one into a memory location at an offset from a register.
- `set_flag_zero`: Stores the value zero into a memory location at an offset from a register.
- `store_constant_three`: Stores the value three into a memory location at a fixed offset from a pointer.
- `store_constant_two`: Stores the value two into a memory location at a fixed offset from a pointer.
- `store_all_ones`: Stores a value consisting of all ones into a memory location at a fixed offset from a pointer.
- `execute_nested_conditional_callbacks`: Executes a function pointer from a global structure and updates a value with the result.
- `process_conditional_structure`: Calls a function pointer from a structure, and if the result is non-zero, updates a value and executes two more function pointers from that structure before calling two final function pointers from a global variable.
- `update_display_state`: Updates a value in a structure, executes several function pointers, and modifies a byte in a memory block using a bitwise mask.
- `configure_display_state`: Checks a value against specific constants and, if matched, executes several function pointers and writes values to hardware registers.
- `configure_display_and_dma`: Configures bitplane and DMA registers, then executes several function calls through pointers retrieved from a global variable.
- `copy_code_block`: Copies a block of code from the current routine to a new memory location and updates a structure field.
- `mask_function_result`: Calls a function and returns the result masked with a value to keep only the lower four bits.
- `mask_low_nibble`: Returns the result of a function call masked with a value to keep only the lower four bits.
- `check_sum_and_reset`: Checks if the sum of several global variables is non-zero and, if so, calls another routine.
- `reset_system_state`: Clears several memory locations, modifies hardware registers, and performs an indirect call.
- `reset_and_call_indirect`: Writes zero to several memory locations, modifies hardware registers, and performs an indirect call.
- `execute_indirect_call`: Performs a call to an address retrieved from a jump table.
- `allocate_and_initialize_task_buffer_3`: Allocates a memory block and clears it using a loop.
- `traverse_and_clear_task_list`: Iterates through a linked list of structures to find a valid entry, then clears a memory block using a loop.
- `initialize_and_populate_task_list`: Clears a memory block using a loop and then iterates through a linked list to populate structures with specific values and function pointers.
- `allocate_and_initialize_task_buffer`: Allocates a memory block via a function pointer, clears it using a loop, and updates a linked list of structures.
- `initialize_task_buffer_and_execute_chain`: Allocates and clears a memory block, then populates a linked list of structures with specific data values.
- `allocate_and_initialize_task_buffer_2`: Allocates a memory block, clears it using a loop, and updates a linked list of structures.
- `initialize_and_process_task_list`: Clears a memory block and updates a linked list of structures by setting specific field values and executing indirect function calls.
- `process_data_loop`: Iterates through a loop updating structure fields with specific values and performs indirect calls based on memory contents.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `update_task_pointers`: Calls two functions via a global pointer and updates specific offsets within a structure using values from another structure.
- `update_status_word_one`: Modifies a 32-bit value at a specific offset from a pointer by masking bits and applying a bitwise OR with a constant.
- `update_status_word_two`: Updates a 32-bit value at a fixed offset from a pointer using a bitwise mask and a different constant.
- `update_status_word_three`: Performs a bitwise mask and OR operation on a 32-bit value located at an offset from a pointer.
- `update_status_word_four`: Applies a bitwise mask and a constant value to a 32-bit integer at a specific offset from a pointer.
- `set_status_word_direct`: Overwrites a 32-bit value at a specific offset from a pointer with a fixed constant value.
- `store_constant_eight_thousand`: Stores the value eight thousand into a memory location at a fixed offset from a pointer.
- `execute_conditional_callbacks`: Executes a function pointer, checks a bit in a global variable, and either executes another function pointer or updates a value with a masked result.
- `execute_two_callbacks`: Calls two different functions using pointers stored at offsets within a global variable.
- `copy_memory_block_2`: Allocates memory via a function call, copies a block of data from one location to another, and updates a structure field.
- `process_linked_list_nodes`: Traverses a linked list and performs conditional memory copying and pointer updates based on values found in a global variable.
- `process_linked_list`: Iterates through a structure, performing arithmetic on values and copying a block of memory based on conditional checks.
- `process_linked_list_and_copy`: Traverses a linked list, performs arithmetic on node values, and conditionally copies a block of memory to a new location before executing an indirect call.
- `initialize_and_branch`: Copies a block of code to a new location, updates a global structure, and performs an indirect call based on a comparison.
- `execute_indirect_call_2`: Stores a value into a memory location and then performs a call using an address retrieved from a global variable.
- `call_global_pointer`: Executes a function call using an address stored in a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_memory_string`: Performs several operations involving pointer traversal, updates memory values, and writes a specific string into a buffer.
- `manage_memory_block`: Updates a global structure, modifies a linked list, populates a memory block with a specific string, and executes functions via a global pointer.
- `walk_list_until_flagged_node`: Follows a chain of linked nodes from a fixed field, advancing until a node's flag byte has bit one set, then returns.
- `store_complemented_word_checksum`: Adds up 23 consecutive 16-bit words from a fixed offset in the working block and stores the one's complement of the sum at another fixed offset.
- `traverse_and_copy_linked_list`: Iterates through a linked list to perform arithmetic updates on elements and copies a block of code to a new destination.
- `update_list_pointers`: Iterates through a linked list to copy a block of code to a new destination and updates a pointer in a global variable.
- `process_linked_list_2`: Traverses a linked list while performing arithmetic on node values and conditionally copies a block of memory and executes an indirect call.
- `traverse_and_initialize_memory`: Iterates through a linked list, modifies data based on node contents, and copies a block of code to a destination retrieved from a global structure.
- `copy_and_execute_function_pointer`: Copies a block of instructions from the current location to a new memory address retrieved from a pointer within a global structure.
- `perform_indirect_jump`: Calculates a value, stores it in a memory location, and executes a function call using an address from a global variable.
- `call_global_function`: Calls a function stored in a global variable.
- `initialize_memory_block`: Performs complex initialization of a memory block, including setting up a string and calling multiple subroutines.
- `initialize_memory_block_alt`: Initializes a memory block by writing specific values and strings, then executes several function pointers from a global structure.
- `initialize_data_structure`: Modifies a linked list, writes a string to a memory block, and executes multiple functions through a global pointer.
- `traverse_linked_list_until_flagged`: Follows a chain of linked nodes starting from a pointer in a data structure, advancing until a node's flag byte has bit one set.
- `calculate_complemented_word_checksum`: Sums 23 consecutive 16-bit words from a fixed offset in a data block and stores the one's complement of the result at another fixed offset.
- `update_linked_list_nodes`: Iterates through a linked list to perform arithmetic updates on node fields based on the difference between two values.
- `execute_callback_logic`: Walks a linked list to perform arithmetic on node fields and copies a block of memory to a destination retrieved from a global variable.
- `copy_and_call_data`: Iterates through a linked list, performs arithmetic on node data, copies a memory block, and executes an indirect function call.
- `execute_list_callback_logic`: Walks a linked list, performs arithmetic on node fields, and copies a block of memory to a new address before performing an indirect function call.
- `copy_and_execute_code`: Copies a block of memory from one location to another and then executes a function via a pointer stored in a global variable.
- `invoke_global_function`: Stores a calculated value in a memory location and executes a function call via a pointer stored in a global variable.
- `call_global_function_alt`: Executes a function pointer retrieved from a global variable.
- `initialize_memory_block_v2`: Sets up a memory region with specific data and strings before invoking functions via a global pointer.
- `initialize_memory_string_table`: Initializes a series of memory locations with specific values and populates a data structure with a null-terminated string.
- `update_list_and_string`: Adjusts a linked list, writes a specific string to a memory region, and executes functions through a global pointer.
- `walk_list_until_flagged_node_alt`: Traverses a linked list by checking a specific bit in a flag byte at an offset within each node.
- `store_complemented_word_checksum_alt`: Calculates a checksum by summing a sequence of 16-bit words and storing the bitwise inverted result.
- `update_string_metadata`: Calculates the length of a null-terminated string and stores it in a table alongside several other values, then performs conditional function calls based on a returned value.
- `execute_sequential_calls`: Performs two consecutive calls to another routine.
- `validate_input_range`: Compares a value against two specific offsets and executes three calls to another routine if the comparison fails.
- `find_null_terminator`: Iterates through a null-terminated string and then executes a function pointer stored at an offset from a base address if a specific value is non-zero.
- `execute_offset_functions`: Checks if a value at a specific offset is non-zero and, if so, calls two different functions located at offsets from a base address.
- `dispatch_conditional_callbacks`: Checks two conditions against a global variable and, if both are met, executes three function pointers retrieved from offsets within that global variable.
- `update_handler_state`: Executes a function pointer retrieved from a table and stores the result in a global array and a structure.
- `call_recursive_function`: The routine calls itself and then returns.
- `cycle_offset_functions`: Saves two values from a structure, executes a function pointer to update those values, and then calls several other routines in a loop before restoring the original values.

### Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:cap.amg

- `parse_amigados_hunk_table`: Parses a data stream containing AmigaDOS hunk type codes to perform memory relocation and data copying.
- `decrement_counter_loop`: Decrements a value in a register until it reaches zero using a loop.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `process_bit_stream_3`: Performs bitwise operations and conditional memory copying based on values extracted from a data stream.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bitstream_to_register`: Reads a sequence of bits from a source buffer and shifts them into a destination register.
- `decompress_bit_stream`: Performs bitwise shifts and rotations on a data stream to extract values based on a variable bit length.

### Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:EasyInstaller

- `process_object_hierarchy`: Recursively traverses a linked structure of objects and executes functions based on the retrieved data.
- `traverse_object_hierarchy`: Recursively traverses a linked structure of objects by following pointers stored at specific offsets.
- `process_amigados_hunk_hierarchy`: Recursively traverses a linked structure of objects by following pointers and executing a function table based on the retrieved data.
- `dispatch_recursive_function_table`: Checks a global variable and performs a recursive call if a specific condition is met.
- `copy_memory_block_3`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `extract_line_string`: Copies characters from a source pointer to a destination buffer until a newline character or a maximum length is reached.
- `parse_command_string`: Parses a string to extract arguments, handling quoted substrings and skipping whitespace.
- `validate_library_initialization`: Checks the return value of a function call and compares a global variable against a value retrieved from a structure.
- `parse_dos_hunks`: Iterates through a memory structure to identify specific AmigaDOS hunk types.
- `parse_dos_hunks_2`: Processes a data structure by checking for specific hunk type codes.
- `reset_progress_display`: Sets a global variable to zero and executes two library functions.
- `execute_graphics_and_text_calls`: Calls two indirect functions to perform graphical scrolling and text printing.
- `parse_amiga_dos_hunks`: Iterates through a message or memory structure to identify specific AmigaDOS hunk types.
- `call_raw_do_fmt`: Library stub: loads its arguments and the exec library base, calls RawDoFmt and returns its result to the caller.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `allocate_and_initialize_memory`: Allocates a memory block via a library call and stores a value at the start of the new buffer.
- `free_memory_block`: Calls a library function to release a memory block if the provided pointer is not zero.
- `search_table_for_value`: Searches through a table of nine-byte entries to find a match for a given value.
- `parse_numeric_string`: Iterates through a string and converts ASCII digits into a numeric integer value.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `update_progress_display`: Clamps a value to a maximum of one hundred, stores it in a global variable, and performs graphical drawing and formatted text output.
- `execute_recursive_dispatch`: Performs a recursive function dispatch by checking a global variable and executing indirect calls based on returned values and table offsets.
- `process_indirect_calls`: Executes a series of indirect function calls using values derived from a global variable and various offsets.
- `dispatch_table_functions`: Traverses a structure to perform recursive calls and executes multiple indirect functions through pointers stored in a global variable.
- `validate_dos_hunk_types`: Checks AmigaDOS hunk type codes within a memory structure to identify executable or relocation segments.
- `parse_amiga_dos_hunks_2`: Iterates through a memory structure to identify and validate AmigaDOS hunk type codes, specifically looking for executable or relocation segments.
- `validate_amiga_hunk_types`: Iterates through a memory structure to identify and validate AmigaDOS hunk type codes.
- `parse_dos_hunks_and_append_info`: Iterates through AmigaDOS memory hunks to identify executable or relocation segments.
- `parse_dos_hunks_3`: Iterates through an AmigaDOS memory structure to identify and process specific hunk types, such as executable loading or relocation codes.
- `validate_dos_hunks_and_format_path`: Iterates through a memory structure to identify specific AmigaDOS hunk types, such as executable or relocation segments.
- `format_path_string`: Checks if a global string ends with a specific character and appends a slash if it does not.
- `dispatch_recursive_table`: Executes a recursive function dispatch based on a global variable and performs multiple indirect calls through pointers.
- `execute_recursive_dispatch_table`: Performs a complex sequence of recursive calls and indirect function executions based on values retrieved from a global variable and various memory structures.
- `dispatch_indirect_calls`: Performs a series of indirect calls and conditional branches based on values retrieved from a global variable and other memory locations.
- `compare_and_dispatch`: Compares two values and executes another routine if they are not equal, followed by a recursive function dispatch and indirect calls.
- `execute_conditional_functions`: Executes functions through pointers retrieved from a global variable and performs conditional logic based on the results of those calls.
- `execute_indirect_calls`: Performs a recursive function dispatch and executes a series of indirect calls using values from a global variable.
- `initialize_data_table`: Populates a table with specific constant values and pointers based on a counter, then performs a recursive function dispatch.
- `initialize_and_dispatch_object_table`: Initializes a series of data structures within a table based on a counter, setting various offsets and pointers, and then recursively calls itself to increment the counter.
- `configure_intuition_gadgets`: Populates a list of gadget structures by iterating through a loop and assigning specific attributes and dimensions.
- `process_amiga_hunk_list`: Iterates through a list of memory hunks to identify and handle executable and relocation segments.
- `parse_amigaos_hunk_list`: Iterates through a list of memory hunks by checking type codes to identify specific executable or relocation segments.
- `process_gadget_table`: Iterates through a table to clear entries and update a bit field based on a specific bit in a structure member.
- `process_conditional_logic`: Performs conditional checks and executes a recursive function from a table based on a global variable and integer comparisons.
- `execute_conditional_logic`: Performs a series of conditional checks and nested routine calls based on register values and global state.
- `execute_recursive_logic`: Performs conditional checks and executes functions from a table based on integer comparisons and global variable states.
- `dispatch_main_logic`: Evaluates a global variable to determine whether to call a specific routine or execute a function via a pointer.
- `execute_object_traversal`: Traverses a linked structure and executes functions from a table based on the results of a pointer-based lookup.
- `execute_library_call`: Calls a function from a library and then proceeds to evaluate a global variable to decide on further routine execution.
- `execute_recursive_dispatch_logic`: Checks a global variable to conditionally execute a function via an offset, then performs recursive calls and potential jump table lookups based on return values.
- `dispatch_library_calls`: Executes specific library functions based on a pointer and performs a recursive table dispatch if certain conditions are met.
- `call_library_functions`: Calls two functions from a library using an offset from a register before proceeding to conditional logic.
- `set_window_titles`: Executes a routine, clears a variable, and calls a library function to set window titles before continuing with conditional logic.
- `parse_dos_hunks_4`: Iterates through a data structure to identify specific AmigaDOS hunk types and updates global state based on their values.
- `parse_hunk_data`: Iterates through data structures, comparing specific values against AmigaDOS hunk type codes to determine processing paths.
- `initialize_library_functions`: Calls routines to initialize a library and checks a library function return value to set a global flag.
- `dispatch_logic_entry`: An entry point that checks a global variable to decide between calling another routine or executing a function via a pointer.
- `execute_recursive_dispatch_2`: Performs a recursive function dispatch by checking a global variable and calling functions from a table based on returned indices.
- `process_dos_hunks`: Iterates through memory structures to identify specific hunk types and manages state flags.
- `handle_recursive_dispatch`: Checks a global variable to determine whether to perform a recursive function call or update a state flag.
- `execute_recursive_dispatch_3`: Evaluates a global variable to determine whether to call a function from a table or perform recursive calls.
- `set_flag_and_dispatch`: Sets a global flag to one and then performs a series of function calls involving a global variable and a table.
- `update_state_flag`: Updates a global state variable and executes a sequence of functions through a recursive dispatch process.
- `execute_recursive_dispatch_4`: Uses a global variable to index into a table and execute functions through a recursive process.
- `set_flag_and_dispatch_2`: Updates a global variable and executes a sequence of function calls using pointers from a global variable.
- `dispatch_recursive_table_lookup`: Checks a global variable and performs a recursive call if a specific condition is met, eventually using a return value as an index into a function table for an indirect call.
- `compare_string_lengths`: Compares a value against a library function result and performs a recursive dispatch if they do not match.
- `execute_recursive_dispatch_5`: Calls another routine before performing a recursive dispatch based on the state of a global variable.
- `dispatch_on_equality`: Checks if a value matches a library function result and executes a specific function if the equality condition is met.
- `call_library_and_dispatch`: Calls a function from a library before entering a recursive dispatch routine that uses a global variable.
- `execute_recursive_dispatch_6`: Initiates a recursive function call sequence that uses a global variable to access a function table.
- `calculate_string_length`: Iterates through a null-terminated string to determine its length.
- `initialize_ui_window_structure`: Allocates memory for a new data structure and populates it with various configuration fields, including a string copy.
- `copy_structure_fields`: Copies multiple longwords and words from one memory location to another, effectively duplicating a data structure.
- `dispatch_installer_messages`: Iterates through a message queue and executes different logic branches based on the message type.
- `process_installer_ui_events`: Processes user input events, such as double-clicks, to update an internal state machine and manage a list of UI elements.
- `release_installer_resources`: Checks a value, executes several library functions, and clears a field in a structure.
- `cleanup_resources`: Releases memory, closes a window, and clears a value in a global structure.
- `allocate_and_relocate_data_block`: Allocates a new memory buffer and copies a block of data from a global variable into it.
- `initialize_data_structure_2`: Copies a block of data from a global variable into a newly allocated memory buffer.
- `sort_memory_blocks`: Iterates through a list of memory blocks and compares their contents to determine order.
- `swap_memory_blocks`: Swaps two adjacent 16-byte memory blocks by iterating through word-sized chunks.
- `return_immediately_3`: This routine performs no operations and immediately returns to its caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `process_installer_file_list`: Iterates through a table of file entries to perform string operations and manage a list of items.
- `format_text_list_buffer`: Iterates through a linked list of data structures to copy string content into a formatted buffer.
- `parse_and_install_file_path`: Parses a string to identify directory separators and manipulates path components.
- `process_installer_data_stream`: Iterates through a data structure to copy blocks of memory into a destination buffer while updating internal counters and status flags.
- `parse_file_path_and_status`: Checks the status of a file or directory and processes its path string.
- `truncate_at_newline`: Searches for a newline character in a string and replaces it with a null terminator.
- `initialize_library_environment`: Initializes system libraries and sets up window titles.
- `display_error_message`: Sets a value to zero and performs several indirect function calls, including one related to a text string.
- `print_text_list`: Iterates through a loop to call a text printing routine multiple times using data from a table.
- `handle_installation_error`: Displays an error message and terminates the installation process when a file check or status flag fails.
- `process_installer_file_list_2`: Iterates through a list of file structures to check extensions and status flags.
- `check_file_extension`: Checks if a null-terminated string ends with the specific sequence of characters representing a file extension.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `append_path_string`: Appends a source string to a destination string, ensuring a directory separator is present if the destination does not end with a colon or slash.
- `append_path_component`: Ensures a string ends with a directory separator before appending another string to it.
- `apply_sign_to_math`: Performs bitwise OR operations on three memory locations and returns the result of another routine with a sign determined by the input registers.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `apply_division_sign`: Determines the sign of the result by checking the signs of two input values and negates the result of another routine if the signs differ.
- `perform_long_division`: Calculates the quotient and remainder of a 32-bit division using a loop or direct arithmetic depending on the divisor.
- `swap_and_rotate_words`: The routine swaps the high and low words of a value in a register, then repeatedly doubles a pair of words while propagating carry, subtracting a constant if the intermediate value exceeds a threshold, and decrements a counter fifteen times.
- `manage_task_memory`: Allocates or modifies memory and task attributes based on specific register values, returning a status code.
- `clear_memory_block`: Stores four zeroed longwords into a memory block starting at an offset from a pointer.
- `return_constant_three`: Returns the constant value three.
- `modify_task_attribute`: Finds a task and writes a specific byte value to an offset within that task's structure.
- `return_constant_one`: Loads the value one into a register and returns to the caller.
- `calculate_installer_window_dimensions`: Iterates through a list of window attributes to determine the maximum width and height, then calculates specific display offsets based on these dimensions.
- `process_window_attribute_list`: Iterates through a linked list of window structures to query attributes and calculate display bounds.
- `traverse_window_list_and_query_attributes`: Iterates through a linked list of structures, accessing specific offsets to retrieve integer and short values.
- `process_window_list`: Iterates through a list of structures and performs actions based on specific values found within the elements.
- `iterate_window_list`: Iterates through a list of structures, checking specific flags and calling a routine to close windows.
- `skip_string_elements`: Iterates through a memory block using a pointer and stops when a byte value greater than ten is encountered.
- `update_clamped_values`: Calculates new values for a table based on offsets and limits, decrementing them in loops until they reach specific thresholds or zero.
- `calculate_display_bounds`: Calculates and stores several offset and boundary values into a series of global variables based on input parameters.
- `render_text_buffer`: Uses graphics library calls to set drawing modes and colors, then iterates through a string to manipulate characters in a buffer.
- `empty_routine`: This routine contains no executable instructions and performs no operations.
- `tiny_ori_nop`: Small routine of 2 instructions that merges 0 into d0, then does nothing for one instruction.

### Combat Air Patrol (1992)(Psygnosis)(M5)[cr Toxic - Prelux](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_manage_graphics_context`: Initializes several data tables and global variables, manages hardware DMA and interrupt settings, and enters a loop that repeatedly calls various subroutines with different constant values.
- `process_input_and_interrupts`: Checks a bit in a global variable and performs several routine calls before writing constant values to a global variable.
- `update_status_string`: Checks bits in a global variable to select and write a three-character ASCII string to a destination pointer.
- `tiny_and`: Small routine of 1 instruction that masks d0 with d0.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d1.
- `update_status_string_2`: Selects a three-character string based on a bitmask in a global variable and stores it in a memory location.
- `update_status_string_3`: Writes specific ASCII characters to a buffer depending on the state of various bits in a global variable.
- `update_status_string_4`: Determines a three-character sequence using bitwise logic on a global variable and writes it to a pointer.
- `update_status_string_5`: Updates a character buffer with a three-byte sequence determined by the contents of a global variable.
- `update_status_string_6`: Performs bitwise checks on a global variable to decide which three-character ASCII sequence to write to a pointer.
- `write_status_string`: Writes a three-character ASCII string to a memory location based on the state of bits in a global variable.
- `empty_routine_2`: The routine is empty and performs no operations.
- `copy_routine_code`: Copies a sequence of instructions from one memory location to another.
- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current execution flow.
- `poll_input_status`: Monitors a memory location and a hardware register to check for specific bit states, returning a status value.
- `decrement_triple_memory_locations`: Decrements a byte and subtracts that value from three separate memory locations.
- `update_accumulated_values`: Increments a byte at a memory location and adds the resulting value to three other memory locations.
- `increment_and_accumulate`: Increments a byte at a pointer and adds the new value to three other memory locations.
- `decrement_and_subtract`: Subtracts a byte from three memory locations and then decrements the original byte.
- `wait_for_byte_change`: Loops until a specific byte at an offset from a pointer reaches a certain value.
- `copy_and_rearrange_memory`: Copies a block of memory to a new location, then copies a second block from a source to a destination, and finally copies a third block in reverse order.
- `adjust_table_values`: Increments or decrements specific word values within a table based on the state of a global variable.
- `increment_state_counters`: Increments several global counters based on conditional checks of a local variable.
- `transfer_data_blocks`: Copies multiple longwords from a source table to a destination table in a loop.
- `clear_memory_block_2`: Fills a large block of memory with zeros.
- `initialize_constant_table`: Populates a memory area with a repeating sequence of specific constant values.
- `populate_value_table`: Stores a sequence of incrementing values into a table using a pointer derived from a global variable.
- `update_sprite_data`: Uses a counter and a global index to copy data from a source table into a destination buffer.
- `set_structure_fields`: Writes several specific constant values into a memory structure at various offsets.
- `traverse_nested_strings`: Iterates through a nested structure of memory blocks delimited by specific byte values.
- `copy_structured_data`: Copies eight blocks of data from one memory location to another using calculated offsets.
- `populate_word_table`: Fills a table with values derived from an incrementing integer, splitting each integer into two words.
- `accumulate_and_store_values`: Stores the high and low words of an integer into a table and then adds a constant to that integer for the next iteration.
- `initialize_integer_table`: Stores a series of 32-bit integers into a table by splitting them into high and low words.
- `update_status_buffer`: Copies a three-byte sequence from a source pointer to a destination buffer based on various conditional checks.
- `increment_table_values_2`: Increments four different byte values at specific offsets within a table using a loop.
- `store_split_longwords`: Splits a longword into two separate shorts and stores them into a table in a loop.
- `copy_memory_block_7`: Copies a block of data from a source to a destination by moving words backwards.
- `clear_memory_buffers`: Searches a table for a specific byte value and, if found, clears several memory buffers using a loop.
- `reset_globals_to_constant`: Writes zero to four global variables and writes a constant value to a fifth global variable.
- `update_audio_parameters`: Modifies a global variable and calls routines to update clamped numerical values and configure audio sample properties via hardware registers.
- `update_parameter_and_audio_state`: Modifies a clamped numerical value based on command nibbles and updates associated audio sample parameters.
- `configure_audio_sample_parameters`: Parses command data to select audio sample properties from a lookup table and updates hardware registers for audio channel length and location.
- `update_clamped_parameter_value`: This routine modifies a value within a structure based on a command nibble, applying clamping logic to keep the value within specific bounds.
- `copy_parameter_offset`: Copies a word from one memory location to another using offsets from two different base pointers.
- `search_and_assign_table_entry`: Searches a table for a value matching a specific criteria and updates a structure with the result.
- `update_clamped_value`: Adjusts a value by adding or subtracting a byte and then searches a table to find a corresponding entry.
- `calculate_scaled_value`: Performs bitwise operations and scaling on data from a structure to update a destination word.
- `process_command_nibble_data`: Parses a command byte by extracting a nibble to determine an operation type.
- `set_dma_channel`: Extracts a nibble from a byte and stores it into a memory location.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_complex_logic_2`: A sixteen-instruction routine that calls another routine.
- `increment_counter_periodically`: Increments a counter when a flag is set and a running value is a multiple of eight, then copies that value into another location.
- `execute_complex_logic`: A routine consisting of seventy instructions that calls two other routines.
- `traverse_linked_list`: Iterates through a memory range, following pointers stored in a table to traverse a structure until a negative value is encountered.
- `copy_buffer_into_destination`: Selects a source buffer based on a flag in a hardware register, then copies ten two-byte values from a source buffer into a destination buffer in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_record_fields`: Walks a fixed-size array of records, initializing each one by copying a pair of bytes into several fields, setting other fields to constants, and recording a per-record marker indexed by those copied bytes.
- `clear_word_block`: It writes zeros across a contiguous block of four-word entries, clearing five consecutive words per iteration for a fixed number of iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditionally_set_register_field`: Checks a global flag, saves a 16-bit value from a memory offset, performs a sequence of subroutine calls while toggling that memory value, and finally restores the original value.
- `init_screen_and_decode`: Invokes a sequence of setup and initialization routines that configure screen globals and decode a run-length compressed stream, then returns.
- `copy_table_rows`: Copies rows of words from one global table into another, moving a fixed number of words per row and advancing both source and destination by fixed strides for each of the requested rows.
- `count_string_table_offsets`: It walks a byte string, adding offsets from a table for each valid character and skipping others, then returns a shifted difference of a passed value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_video_registers`: Initializes a set of custom video registers and DMA channel modulation values from fixed constants, then enters a busy loop that spins until a status register bit reaches a threshold.
- `populate_dispatch_table_2`: Stores a sequence of constant values into a series of global variables and then executes a routine via a pointer stored in a global variable.
- `populate_dispatch_table`: Stores several constant values into a sequence of global variables before calling a routine via a pointer stored in a global variable.
- `update_hardware_and_call`: Writes values to several global variables and hardware registers before calling a routine via a pointer.
- `configure_blitter_blanking`: Waits for the display to reach a blanking interval, then programs the blitter channel registers and runs a loop that advances two source pointers by a fixed stride while decrementing a counter.
- `setup_display_scaling`: It sets up a screen mode by writing to the display control custom registers, then repeatedly scales a global byte by a fixed factor, adds an offset, and stores the result into a global word while invoking a helper routine, and finally splits a 32-bit accumulator into high and low halves storing them as consecutive word pairs into a global table four times.
- `init_screen_globals`: Initializes a set of global variables and registers, then calls a helper routine that repeatedly scales a global byte by a fixed factor, adds an offset, and stores the result into a global word while invoking another helper.
- `tiny_ori_2`: Small routine of 1 instruction that merges 30 into -(a4).
- `update_bcd_counters`: Adds bytes from a source table to four consecutive global BCD variables and adjusts them.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `update_bcd_counters_offset`: Adds bytes from a source table with a different offset to four consecutive global BCD variables and adjusts them.
- `update_table_index_and_copy_data`: Increments a global counter and uses its value to perform table lookups via calculated offsets.
- `process_table_indices_2`: Checks a global value against specific constants to calculate an index into a table, then conditionally copies a sequence of words from one table to another.
- `clear_global_word`: Sets a global word to zero.
- `set_global_byte`: Sets a global byte to a specific value.
- `process_table_indices`: Checks a global value against specific constants to calculate an index into a table, then conditionally copies a sequence of words from one location to another.
- `copy_sixteen_words`: Copies sixteen words from a source table to a destination table.
- `set_function_pointers`: Stores a routine address and a constant value into two global variables.
- `set_function_pointers_duplicate`: Assigns a specific routine address and a constant value to two global memory locations.
- `store_count_and_return`: Stores the incoming word from a register into a global variable, stores a fixed count of fifteen into another global variable, and then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `interpolate_table_values_2`: Iterates through a source table and a destination table, adjusting the destination values toward the source values.
- `interpolate_table_values_loop`: Iterates through a table of words and adjusts each value toward a source value by incrementing or decrementing nibbles, bytes, and words.
- `interpolate_table_values_fixed`: Adjusts a sequence of word values in a table toward a single source value by modifying their individual nibble, byte, and word components.
- `interpolate_table_values`: Iterates through a source table and a destination table, adjusting the destination values toward the source values.
- `increment_table_values`: Iterates through two tables and increments or decrements word values based on their nibble and byte components.
- `store_four_word_pairs`: Splits a 32-bit accumulator into high and low halves and stores them as consecutive word pairs into a global table four times.
- `split_words_into_table`: Splits 8 32-bit values from a source table into 16-bit halves.
- `copy_selected_table_slots_2`: Copies 16 words from a table selected by a shift of a register value into a global variable.
- `copy_selected_table_slots`: Copies 16 words from a table selected by a shift of a register value into a global variable.
- `copy_selected_table_clear_slots`: Copies 16 words from a table selected by a masked register value into a global variable.
- `pack_values_into_bytes`: Combines two input values and packs the low bits of each into four consecutive bytes through a pointer, setting flag bits in the fourth byte based on the carry-out of the summed values.
- `decrement_and_loop_variable`: Decrements a value stored in a memory location and repeats the process until the value reaches zero.
- `infinite_loop_halt`: The routine executes an unconditional infinite loop, causing the processor to stall indefinitely.
- `update_display_and_interrupt_state`: Modifies hardware registers for display control, color, and interrupt management while updating global counters and table indices.
- `dispatch_indirect_call_2`: Evaluates a global variable and hardware register status to determine a jump target and executes an indirect call.
- `update_flight_variables`: Modifies four global variables by adding or subtracting specific amounts and clamps them within defined minimum and maximum ranges.
- `update_flight_control_variables`: Processes joystick input to increment or decrement four global variables within specific clamped ranges.
- `update_joystick_parameters`: Reads input from a hardware register and adjusts several global variables within specific bounds.
- `set_global_and_spin`: Sets a global word to a value with its high byte filled with ones, then busy-waits until that word becomes zero.
- `reset_global_and_set_registers`: If a global variable is non-negative, it sets that variable to zero and writes a specific constant to two consecutive memory locations.
- `initialize_blt_parameters`: Sets several memory locations to specific values and waits for a global flag to change state.
- `update_blt_parameters_in_loop`: Busy-waits on a global flag and then enters a loop that updates several memory locations and hardware registers used for bitblock transfers.
- `copy_table_blocks`: Copies a block of words from a source table into a destination table, processing a fixed number of words per iteration and repeating for a fixed number of rows, then returns.
- `toggle_flag_and_call`: Toggles a global flag around a sequence of calls, then returns without touching the passed data.
- `or_words_in_table_rows`: It reads four words from a source array and ORs them together into a destination word.
- `decode_rle_from_struct`: Reads several fields from a structure, then repeatedly decodes a run-length compressed stream by copying literal bytes and expanding back-references into a destination buffer until a counter is exhausted.
- `find_list_entry`: Walks a linked list of records by following each node's next pointer until a matching value is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: The routine is empty and performs no actions before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_then_set_flag_block`: Clears a ten-word block of memory by zeroing each location, then sets the first word to one and the second word to all ones, optionally starting from the same block if a status flag is nonzero.
- `copy_byte_pairs_from_table`: Increments a global counter and reads a pointer from an indexed table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_4`: Small routine of 1 instruction that merges 1 into d0.
- `interpret_bytecode_stream_2`: Processes a stream of bytes by executing different operations such as incrementing globals, loading values from memory, or performing indirect calls based on opcodes.
- `interpret_bytecode_stream`: Iterates through a sequence of bytes, executing specific operations based on opcode values.
- `execute_indirect_call_4`: Sets a global variable to a specific value and performs an indirect call using a table offset derived from a byte in a register.
- `interpret_bytecode_variant`: Processes a byte stream by interpreting opcodes to load 32-bit values into a global variable, modify other globals, or perform indirect calls.
- `dispatch_indirect_call`: Loads a value from a pointer into a global variable, clears that variable, and performs an indirect call to a destination calculated from a table offset.
- `parse_bytecode_stream`: Processes a stream of bytes by interpreting specific opcodes to modify global variables or perform indirect calls.
- `parse_bytecode_stream_variant`: Processes a stream of bytes by interpreting specific opcodes to modify global variables or perform indirect calls, with a different handling of one specific opcode.
- `execute_indirect_call_3`: Clears a global variable and performs an indirect call to a destination calculated from a table offset and a value in a register.
- `execute_indirect_jump_via_table`: Loads a value from a pointer, clears a global variable, and then performs an indirect call to a destination calculated from a table offset.
- `apply_masked_stream`: Advances a stream pointer and adjusts a length based on a table lookup, then applies a bitwise mask to a block of words in a nested loop.
- `process_audio_initialization`: Retrieves a task and message via system library calls, then performs a recursive initialization sequence using a global table and calls a routine that executes a stored pointer.
- `initialize_audio_hardware_objects`: This routine performs a recursive initialization sequence by retrieving object pointers from a global table and calling associated methods to configure hardware-related structures.
- `initialize_system_resources`: Opens required system libraries and resources, then performs a recursive initialization sequence that configures hardware interrupts and sets up global state structures.
- `copy_table_block`: Waits until a global flag clears, then copies a block of words from a source table into a destination table.
- `process_input_event_codes`: Evaluates a value returned from a function call against specific constants to trigger different sequences of subroutines and global variable updates.
- `dispatch_recursive_state_handler`: Updates a function pointer with its own address and executes a series of conditional branches based on values returned from a table.
- `execute_recursive_logic_2`: Assigns a routine address to a global variable, calls a routine via a pointer, and performs a series of conditional recursive calls and global variable assignments based on the returned value.
- `clear_and_recurse_globals`: Checks if the sum of two global variables is zero, then clears those variables and calls itself and another routine multiple times.
- `execute_sequence_of_routines`: Calls a function via a pointer and, if a global flag is zero, executes a specific sequence of other routines and functions.
- `set_global_constants`: Writes a fixed constant into a global variable, then rewrites that same variable using a bitwise OR, AND, and OR sequence of constants.
- `dispatch_based_on_result`: Calls a routine via a pointer and stores either five or zero into a global variable depending on the returned value before calling two other routines.
- `call_stored_pointer`: Loads a function pointer from a global variable, calls it, and stores the returned byte into a global variable.
- `process_string_via_device`: Walks through a null-terminated string stored in a global variable and, for each character, writes a pattern of ones into a device structure and installs a return address pointing back into this routine.
- `enter_infinite_loop`: This routine enters an infinite loop that prevents further execution.
- `tiny_tst_rte`: Small routine of 2 instructions that tests (a7)+, then returns from the exception.
- `call_offset_function`: Calls a function located at an offset from a value stored in a global variable.
- `update_data_structure_2`: Iterates through a structure to decrement values and conditionally updates various fields or calls other routines based on the state of specific counters.
- `update_data_structure`: Compares a value from a source to a destination, and if they differ, updates the destination with the new value and performs bitwise shifts and assignments to other memory locations.
- `return_from_subroutine_2`: The routine consists of a single instruction that returns control to the caller.
