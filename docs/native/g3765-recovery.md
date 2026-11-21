# Game 3765 (galdregon's domain (1989)(pandora)[cr black dragon]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03765). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1817216 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (9 by rule, 22 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 276 of 276 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 978 of 978 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9317 |
| code | 3258 |
| data | 904758 |
| padding | 299 |
| unused | 899584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf:bootblock | 1 | 63 | 1 |
| logged disk load from Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf boot-block load | 1 | 92 | 2 |
| logged disk load from Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 157 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 664 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- galdregon's domain (1989)(pandora)[cr black dragon](disk 2 of 2)/Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1752 non-empty blocks: matched against track-loaded segments
- galdregon's domain (1989)(pandora)[cr black dragon](disk 1 of 2)/Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf: the boot block reads 2560 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- galdregon's domain (1989)(pandora)[cr black dragon](disk 1 of 2)/Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 92 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 182 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+displacement
- galdregon's domain (1989)(pandora)[cr black dragon](disk 1 of 2)/Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf: 2 logged loads cover 0 bytes of the image
- galdregon's domain (1989)(pandora)[cr black dragon](disk 2 of 2)/Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 2 of 2).adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.58)
- galdregon's domain (1989)(pandora)[cr black dragon](disk 2 of 2)/Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 2 of 2).adf: blocks 10-1759 (bytes 5120-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.13)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_sprite_dma` (runtime segment an address): Updates DMA settings for a sprite based on its type and position, then configures audio hardware registers accordingly. Kind: disk hardware ADKCON check. Pass path: The check passes if the ADKCON register is set correctly based on sprite data, and the routine updates DMA and ADKCON registers accordingly. The successful path involves setting up DMA channels and updating ADKCON with values derived from s.

## Functions


### Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf:bootblock

- `initialize_io_structure`: Initializes IO request structure fields with specific values, allocates memory, performs IO operations, and processes boot block data to set up memory regions for subsequent operations.

### logged disk load from Galdregon's Domain (1989)(Pandora)[cr Black Dragon](Disk 1 of 2).adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_rem_int_server`: Wrapper that calls the exec library function RemIntServer and returns its result to the caller.
- `update_input_state`: Reads joystick and mouse input states, updates movement counters and button statuses in a global structure, then configures custom hardware registers and executes an IO operation based on input conditions.
- `toggle_and_execute_io`: Toggles a bit in a structure, updates a counter, reads from a buffer, and executes an IO operation when certain conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_io_execution`: Performs an IO operation only when a condition is true, specifically when a zero flag indicates a particular state.
- `frag_movem_move_beq_movea_addq_jsr_rts`: Short code fragment that saves registers on the stack, then copies a0 to d0, then branches if the last result was zero, then copies the memory just below a0 (moving a0 down) to d0, then copies a0 to a1, then adds 4 to d0, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `execute_two_io_operations`: Executes two sequential IO operations with different command values and returns the results of these operations.
- `execute_io_with_bit_manipulation`: Sets a bit in a structure, performs an IO operation with specific parameters, updates a byte value, and clears the previously set bit.
- `set_io_parameter`: Sets a 16-bit parameter in an IO request structure based on a multiplication result, then executes the IO operation.
- `initialize_io_request`: Sets up an IO request structure with command 5, executes a DoIO operation, clears a memory buffer of 128 words, then sets up another IO request with command 2 and executes it.
- `execute_io_and_get_result`: Executes an IO operation with a specific command and returns a value from a field of the IO request structure.
- `allocate_and_copy_data`: Allocates memory, initializes it with a specific value, copies data from a table to the allocated memory, then frees the memory and performs several operations on different data structures.
- `copy_memory_block`: Copies bytes from one memory location to another in a loop, handling a specific block size of 5632 bytes before calling another routine and continuing the copy operation.
- `update_and_call_io`: Updates fields in an IO request structure with values from a table, increments a pointer by a fixed amount, then executes the IO operation.
- `allocate_and_transfer_data`: Allocates memory, initializes it, copies data from a table to the allocated memory, then performs multiple operations on different data structures before freeing the memory.
- `update_io_with_fixed_size`: Updates fields in an IO request structure with values from a table, increments a pointer by a fixed amount, and executes the IO operation with command 2.
- `copy_memory_with_reset`: Copies bytes from one memory location to another in a loop, resetting the counter after reaching 5632 bytes and calling another routine before continuing the copy operation.
- `keyboard_interrupt`: CIA-A serial port interrupt routine: takes the key code the keyboard sent, does the handshake and passes it to the key handler.
- `update_game_state`: Increments a global counter and resets it to zero when reaching six, then updates audio registers and handles audio channel data.
- `update_position`: Updates position values based on input parameters and writes results to global variables.
- `frag_add_cmpi_bmi_move_rts_clr`: Short code fragment that adds d0 to the memory at an offset from a6, then compares the memory at an offset from a6 with 64, then branches if the last result was negative, then copies 64 to the memory at an offset from a6, then copies the memory at an offset from a6 to the memory at an offset from a5, then returns, then clears d0, then copies the memory at an offset from a6 to d0.
- `update_audio_counter`: Reads a byte from a structure, processes it to calculate a value, updates a field in another structure, and manages audio hardware registers through multiple calls.
- `update_sprite_dma`: Updates DMA settings for a sprite based on its type and position, then configures audio hardware registers accordingly.
- `update_audio_registers`: Writes a 16-bit value from a register to an audio register at a specific offset, then advances pointers and loops back to process multiple audio registers.
- `frag_move_movea_cmp_ble_rts`: Short code fragment that copies a5 to the stack, then copies a location in the code to a5, then compares d0 with the memory at an offset from a5, then branches if the comparison was less or equal, then copies the memory at an offset from a5 to d0, then copies the top of the stack to a5, then returns.
- `frag_move_subq_mulu_movea_lea_rts`: Short code fragment that copies d0 to the stack, then subtracts 1 from d0, then multiplies d0 by 18 (unsigned), then copies the memory at an offset from a5 to a0, then loads the address of an indexed entry from a0 into a0, then copies the top of the stack to d0, then returns.
- `calculate_collision_result`: Performs collision detection calculations using values from memory structures, updates a result based on comparisons and arithmetic operations, and returns a combined value with a fixed bit pattern.
- `frag_lea_dbra_suba_clr_movem_rts`: Short code fragment that loads the address of the memory at an offset from a1 into a1, then loops back while counting down d1, then subtracts a0 from a0, then clears d0, then restores saved registers from the stack, then returns.
- `frag_sub_movea_subq_mulu_lea_move_movem`: Short code fragment that subtracts the memory at an offset from a0 from d1, then copies the memory at an offset from a5 to a0, then subtracts 1 from d0, then multiplies d0 by 18 (unsigned), then loads the address of an indexed entry from a0 into a0, then copies d1 to d0, then restores saved registers from the stack, then returns.
- `frag_movea_subq_mulu_lea_move_sub_movem`: Short code fragment that copies the memory at an offset from a5 to a0, then subtracts 1 from d0, then multiplies d0 by 18 (unsigned), then loads the address of an indexed entry from a0 into a0, then copies the memory at an offset from a1 to d0, then subtracts d2 from d0, then restores saved registers from the stack, then returns.
- `decrement_counters`: Checks a bit in a structure, then iterates through a linked list of structures, decrementing various 16-bit counters within those structures based on specific conditions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
