# Game 1348 (treasure trap (1989)(electronic zoo)[cr flt][t +3 adept]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01348). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909504 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (2 by rule, 43 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 195 of 195 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 23 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1006 of 1110 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8082 |
| code | 4090 |
| data | 895169 |
| padding | 115 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf:bootblock | 1 | 110 | 10 |
| logged disk load from Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf boot-block load | 1 | 89 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 196 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 164 | 2 |
| Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf bytes 1024-3072 (code never loaded by a reference run) | 1 | 551 | 23 |

## Cases handled

- treasure trap (1989)(electronic zoo)[cr flt][t +3 adept]/Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf: custom-format (non-DOS) disk, 1753 non-empty blocks: matched against track-loaded segments
- treasure trap (1989)(electronic zoo)[cr flt][t +3 adept]/Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 141 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- treasure trap (1989)(electronic zoo)[cr flt][t +3 adept]/Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf: bytes 1024-3072 look like code no run loaded: recovered position-independently (load address unknown)
- treasure trap (1989)(electronic zoo)[cr flt][t +3 adept]/Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf: 2 logged loads cover 7680 bytes of the image
- treasure trap (1989)(electronic zoo)[cr flt][t +3 adept]/Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- 1 packed track region(s), 889856 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_global_and_wait` (Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf bytes 1024-3072 (code never loaded by a reference run)): This routine modifies a global variable using a bitwise operation and then enters a loop that calls another routine. Kind: disk hardware/checksum trap. Pass path: the branch at an address (bne.b an address) which skips the sabotage/error path.
- `decrement_global_after_wait` (Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf bytes 1024-3072 (code never loaded by a reference run)): This routine modifies a global variable with a bitwise operation, calls another routine, and then decrements a global variable. Kind: disk hardware/sync check. Pass path: sub_02AC.
- `increment_global_after_wait` (Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf bytes 1024-3072 (code never loaded by a reference run)): This routine modifies a global variable with a bitwise operation, calls another routine, and then increments a global variable. Kind: disk hardware/sync check. Pass path: sub_02AC.
- `verify_disk_data_integrity` (Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf bytes 1024-3072 (code never loaded by a reference run)): This routine performs a checksum and bitwise validation of data read from a disk. Kind: disk integrity/checksum check. Pass path: the loop continues to the next sector/block if the XOR checksum and sector count match, eventually returning the total loaded size (sVar1) if the final sector count matches the expected count (_u4__d0_050A == _u4__d0_0508).

## Functions


### Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf:bootblock

- `initialize_boot_sequence`: This routine copies a block of code to a new location and performs several function calls to set up system state.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_linked_routines`: Traverses a linked list of function pointers, calling each routine sequentially until a null pointer is encountered.
- `iterate_null_terminated_string`: This routine iterates through a null-terminated string by calling another routine for each character.
- `return_concatenated_registers`: Returns a 64-bit value formed by concatenating the contents of two 32-bit registers.
- `initialize_system_resources`: Allocates memory and signals, opens a disk device, and performs I/O operations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_unspecified_routine`: This routine calls another routine and is itself called by another routine.
- `enable_dma_controller`: Sets bit 15 of the DMA controller register to enable DMA channels.

### logged disk load from Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf boot-block load

- `shift_and_store_word`: Performs a bitwise right shift on a value and stores the result into a memory location via a pointer.
- `loop_bit_shifts`: This routine performs a loop that right-shifts a value in a register and calls another routine when the value reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_state`: Updates a global variable and uses its bits to conditionally modify a table and execute a function pointer.
- `copy_list_to_stack`: Reads a value from a pointer, clears that pointer, and then copies a sequence of integers from a table into a stack-based structure.
- `increment_global_variable`: Increments a global variable by one.
- `increment_and_return_zero`: Increments a global variable by one and returns zero.
- `copy_data_blocks`: This routine copies blocks of data between a source table and a destination table based on the value of a global variable.
- `copy_data_block`: Copies a block of data from a source table to a destination table based on a bit flag in a global variable.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `update_lookup_tables`: Performs multiple lookups in tables to update a global variable and writes values to a memory range based on specific bit patterns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_high_score_disk`: Checks if a disk contains a valid high score signature and copies string data from a buffer if a specific signature is detected.
- `fetch_next_table_byte`: reads a byte from a table or an offset table and increments a global index.

### Treasure Trap (1989)(Electronic Zoo)[cr FLT][t +3 Adept].adf bytes 1024-3072 (code never loaded by a reference run)

- `copy_code_block`: Copies a block of code to a high memory address after executing several function pointers from a table and modifying a DMA control register.
- `copy_memory_block`: Copies a block of data from a source pointer to a destination pointer for a fixed number of iterations.
- `copy_boot_code`: This routine copies bytes from a source to a destination and performs conditional updates to a data structure.
- `process_string_sequence`: This routine traverses a null-terminated string and calls another routine for every non-zero byte encountered.
- `return_concatenated_registers_2`: Returns the combined contents of two 32-bit registers as a single 64-bit value.
- `concatenate_register_pair`: Combines the contents of two 32-bit registers into a single 64-bit return value.
- `interleave_bit_data`: Combines bits from two source memory locations into a destination location using bitwise masking and shifting.
- `transform_data_block`: Performs a bitwise manipulation on words from two source locations and stores the result in a destination buffer.
- `transform_data_block_2`: Processes a series of words by combining bit-masked values from two different memory offsets into a new location.
- `interleave_bitwise_data`: This routine iterates through two memory buffers, combining bits from a source buffer and a secondary buffer into a destination buffer.
- `check_global_flag`: Checks a bit in a global variable and updates that variable if a specific bit is set.
- `wait_for_cia_bit`: Writes values to hardware registers and enters a loop that waits for a specific bit in a CIA register to be set.
- `update_global_and_wait`: This routine modifies a global variable using a bitwise operation and then enters a loop that calls another routine.
- `decrement_global_after_wait`: This routine modifies a global variable with a bitwise operation, calls another routine, and then decrements a global variable.
- `increment_global_after_wait`: This routine modifies a global variable with a bitwise operation, calls another routine, and then increments a global variable.
- `poll_hardware_status`: Checks a bit in a global variable within a loop and interacts with a CIA register.
- `verify_disk_data_integrity`: This routine performs a checksum and bitwise validation of data read from a disk.
- `compare_case_insensitive_strings`: Compares two null-terminated strings by treating specific character ranges as case-insensitive.
- `process_data_structure`: This routine performs arithmetic on a global variable and populates several other global variables using values from a pointer.
- `increment_table_index`: This routine increments a value in a table and updates a global variable based on a specific constant.
- `call_subroutine_twice`: This routine calls another routine two consecutive times.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `empty_routine`: This routine contains no executable instructions.
