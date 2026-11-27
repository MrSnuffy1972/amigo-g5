# Game 3953 (hot rubber (1991)(palace)(disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03953). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1126776 top-level bytes in 98 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 447 of 447 named (0 author symbols), 447 of 447 described (129 by rule, 308 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1521 of 1523 (99.87%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 21822 of 21877 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 802575 |
| code | 96726 |
| data | 187009 |
| padding | 22546 |
| unused | 17920 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:GP.PRG | 2 | 267 | 8 |
| Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 323 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 46 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 58 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 225 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 55 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 233 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 277 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 122 | 4 |
| Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:GP.PRG#unpacked | 3 | 10008 | 192 |
| unit:u0#packproof-None#unpacked | 3 | 10008 | 192 |

## Cases handled

- hot rubber (1991)(palace)(disk 1 of 2)/Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:SPR_END.IM1: high-entropy data file (7.63 bits/byte), classified as packed data
- extended reference run ignored: it does not re-execute 1453 of the 1559 PCs the canonical run logged (262238 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 262101 of its 262238 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- prefix run f150 ignored: it logged 262132 PCs the canonical run never executed (not a prefix of it)
- prefix run f75 ignored: it logged 262132 PCs the canonical run never executed (not a prefix of it)
- 1265 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 12 first-run image unit(s) in 1 version group(s); 2126 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- hot rubber (1991)(palace)(disk 1 of 2)/Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:GP.PRG: PowerPacker 4.0 (xfd) stream found by ancient scan (42064 -> 108936 bytes) but only 0 of 47 sampled windows are in reference RAM: not used
- GP.PRG: code lead from fmtread (108936 bytes, hunk exe), 0 of 47 windows confirmed in reference RAM (hunk exe, relocatable: no RAM confirmation needed): recovered
- u0#packproof-None: code lead from fmtread (108936 bytes, hunk exe), 0 of 47 windows confirmed in reference RAM (hunk exe, relocatable: no RAM confirmation needed): recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 11 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 7 jump target, 4 immediate
- hot rubber (1991)(palace)(disk 1 of 2)/Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:GP.PRG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_port_b` (first-run image an address): Modifies a hardware port register by setting specific bits in a loop with multiple iterations. Kind: disk hardware check (CIAB_PRB). Pass path: The routine reads CIAB_PRB multiple times and modifies it, suggesting a check against expected port states. A successful check would likely proceed without triggering a trap or error, with the final call to u13__sub_026E() being the pass pa.
- `setup_dma_transfer_2` (GP.PRG#unpacked): Configures DMA channels by setting up hardware registers with specific values, performs multiple DMA I/O operations, and copies data between memory locations. Kind: trackdisk.device I/O with sync/checksum verification. Pass path: The routine succeeds if the data read from the disk matches expected values (i.e., the loop comparing pcVar5 and pcVar8 does not jump to u14__l0_2F08). If successful, it continues to call u14__sub_0_3050()..
- `configure_dma_channels` (GP.PRG#unpacked): Sets up DMA channel configurations by writing specific register values, performs DMA I/O operations with different parameters, and calls a routine to update port registers. Kind: trackdisk device I/O check. Pass path: The routine performs multiple DoIO calls with different parameters (uRam0100db78 values 5, 2, and 4), suggesting a validation or verification step on the disk track. If these operations succeed without error, the check passes and execution .
- `manage_dma_operations` (GP.PRG#unpacked): Manages DMA operations by setting a global flag, performing permit and forbid operations, executing multiple DMA I/O requests with various parameters, and clearing the global flag. Kind: trackdisk.device I/O with delay loops. Pass path: The routine executes DoIO calls and ends with exec_Forbid() and u14__sub_0_3050(), suggesting a check that must pass before proceeding. If the check fails, execution may jump to a sabotage path or halt..
- `update_port_registers` (GP.PRG#unpacked): Updates hardware port registers by reading current values, masking bits with specific patterns, and writing modified values to control peripheral devices. Kind: disk hardware check. Pass path: The routine updates CIAB_PRB multiple times, likely checking for specific hardware states or responses from the disk drive, and calls u14__sub_0_FA44() after each update. A successful check would proceed through all iterations without branc.
- `setup_dma_transfer_3` (u0#packproof-None#unpacked): Configures DMA channels by setting up hardware registers with specific values, performing multiple I/O operations through the exec library, and copying data between memory locations. Kind: trackdisk.device I/O with sync/checksum validation. Pass path: The routine succeeds if the data read from the disk matches expected values (u15__d0_2F1C vs u15__d0_2F32), and the final branch after the loop does not jump to u15__l0_2F08. The check passes when all bytes match, allowing execution to cont.
- `configure_dma_channels_2` (u0#packproof-None#unpacked): Sets up DMA channel configurations by writing specific register values, performing I/O operations through the exec library, and updating hardware port registers for DMA transfers. Kind: trackdisk device I/O with DMA channel configuration. Pass path: The routine configures DMA channels and performs multiple DoIO calls to trackdisk.device, suggesting a disk integrity or track verification check. The successful path would proceed through the DoIO operations without triggering protection m.
- `manage_dma_access` (u0#packproof-None#unpacked): Manages DMA access by setting a global flag to maximum value, calling forbid and permit functions from the exec library, performing multiple I/O operations with different parameters, and clearing the global flag before returning. Kind: trackdisk.device I/O with sync/checksum verification. Pass path: The routine executes DoIO calls on a trackdisk device request structure, likely performing a read with checksum or sync validation. A successful check would proceed through the normal execution path (e.g., after the final DoIO), and the 'pa.
- `update_port_registers_2` (u0#packproof-None#unpacked): Updates hardware port registers through multiple iterations of bitwise AND/OR operations, setting specific bit patterns for peripheral control. Kind: disk hardware check. Pass path: The routine updates CIAB_PRB multiple times and calls u15__sub_0_FA44() each time, likely checking for specific hardware states or responses from the disk drive. A successful check would proceed through all iterations without triggering a t.

## Functions


### Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:GP.PRG

- `process_hunk_headers`: Reads and processes AmigaDOS hunk headers from a data structure, allocating memory blocks for each hunk and copying data into them.
- `decrement_and_loop`: Decrements a word-sized counter in a register until it reaches zero, performing a simple loop operation without any branching or external calls.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_and_shift_data`: Decodes data by performing bitwise operations, shifting values right, and accumulating results while manipulating memory pointers.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `bit_shift_and_accumulate`: Performs bitwise operations on data in registers and memory, shifting bits right and left while accumulating results in a counter, and updates a hardware color register.
- `load_and_or`: Loads a word from memory location d4 and ors it with the immediate value 136 into register d1, then executes a trap instruction.

### Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blitter`: Sets up blitter registers for graphics operations using values loaded from global variables, then waits for blitter completion before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_color_palette`: Clears the color palette entries and initializes them with scaled values from a source table, then copies the results to another location.
- `scale_color_components`: Processes color palette entries by scaling their red, green, and blue components and writes the results to a color register table.
- `decode_and_process_data`: Validates data format by comparing against a specific signature, copies data blocks to output locations, processes byte sequences with conditional logic, and calls another routine for coordinate updates.
- `update_coordinates`: Updates coordinate values based on input parameters and returns a specific value when certain conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movem_clr_lea_move_movea_jsr_ori`: Short code fragment that saves registers on the stack, then clears a fixed memory address (2 times in a row), then loads the address of a fixed memory address into a0, then clears the memory at an offset from a0, then loads the address of a fixed memory address into a1, then copies 5 to the memory at an offset from a1, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_move_movea_jsr_ori`: Short code fragment that copies 2 to the memory at an offset from a1, then copies 512 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_move_movea_jsr_ori_2`: Short code fragment that copies 4 to the memory at an offset from a1, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `copy_string_data`: Copies character data from one memory location to another, handling string length and null termination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_port_b`: Modifies a hardware port register by setting specific bits in a loop with multiple iterations.
- `frag_movem_movea_moveq_jsr_ori`: Short code fragment that saves registers on the stack, then copies an address to a1, then loads 0 into d0, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_movea_jsr_ori`: Short code fragment that copies 0 to a1, then calls another routine, then merges 0 into d0.
- `frag_move_jsr_ori`: Short code fragment that copies d0 to the memory at an offset from a1, then calls another routine, then merges 0 into d0.
- `frag_move_clr_lea_jsr_ori_rts`: Short code fragment that copies an address to the memory at an offset from a1, then clears d0, then clears d1, then loads the address of a fixed memory address into a0, then calls another routine, then merges 0 into d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_disk_data`: Processes disk-related data by checking conditions and performing operations on memory locations.
- `frag_tst_beq_bmi_move_movea_jsr_ori`: Short code fragment that tests d0, then branches if the last result was zero, then branches if the last result was negative, then copies a fixed memory address to d3, then copies a fixed memory address to d2, then copies a fixed memory address to d1, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_tst_beq_bmi_move_movea_jsr_ori_2`: Short code fragment that tests d0, then branches if the last result was zero, then branches if the last result was negative, then copies a fixed memory address to d1, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_clr_movem_rts`: Short code fragment that clears a fixed memory address, then clears d0, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `xor_memory_block`: XORs bytes in a memory block with a fixed value, iterating through the specified range.
- `initialize_structure`: Initializes a structure at a fixed address with values from input registers and stores a pointer to this structure in another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_interrupt_handler`: Initializes an interrupt server by setting up a data structure with specific byte values and pointers, then adds it to the system's interrupt handling mechanism.
- `setup_interrupt_vector`: Initializes interrupt vector table entries and sets up an interrupt handler for a specific interrupt number.
- `frag_lea_move_clr_moveq_jsr_ori`: Short code fragment that loads the address of a fixed memory address into a1, then copies 2 to the memory at an offset from a1, then copies 100 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then clears the memory at an offset from a1, then copies an address to the memory at an offset from a1, then loads 8 into d0, then calls another routine, then merges 0 into d0.
- `frag_lea_move_clr_moveq_jsr_ori_2`: Short code fragment that loads the address of a fixed memory address into a1, then copies 2 to the memory at an offset from a1, then copies 100 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then clears the memory at an offset from a1, then copies an address to the memory at an offset from a1, then loads 9 into d0, then calls another routine, then merges 0 into d0.
- `frag_lea_move_clr_moveq_jsr_ori_3`: Short code fragment that loads the address of a fixed memory address into a1, then copies 2 to the memory at an offset from a1, then copies 100 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then clears the memory at an offset from a1, then copies an address to the memory at an offset from a1, then loads 10 into d0, then calls another routine, then merges 0 into d0.
- `execute_rendering_function`: Executes a rendering function with calculated parameters after checking if certain conditions are satisfied.
- `validate_and_return`: Checks if a global variable is zero and exits with an error if true, otherwise returns two 32-bit values from registers D0 and D1.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_registers`: Initializes audio hardware registers based on global variable states and writes specific values to custom hardware addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware`: Saves registers, calls a routine to initialize hardware, performs multiple iterations of setting up and calling another routine with different parameters, then restores registers and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_data`: Initializes game data structures by copying configuration values from a table, setting up global variables with specific constants, and performing several hardware register updates.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `blit_dma_setup`: Saves all registers, sets up DMA transfer parameters for three blits using specific hardware registers, performs the blits by updating source and destination pointers, then restores registers and returns.
- `blit_dma_setup_reverse`: Saves all registers, sets up DMA transfer parameters for three blits using specific hardware registers, performs the blits by updating source and destination pointers in reverse order, then restores registers and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `single_dma_setup`: Saves all registers, sets up a single DMA transfer with specific parameters using hardware registers, waits for DMA completion, then restores registers and returns.
- `coprocessor_dma_loop`: Saves all registers, waits for DMA to complete, performs a loop of memory operations involving coprocessor registers, then enters an infinite loop.
- `frag_movem_movea_jsr_ori`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_move_jsr_movem_rts`: Short code fragment that copies the memory at an offset from a5 to a fixed memory address (2 times in a row), then calls another routine (4 times in a row), then restores saved registers from the stack, then returns.
- `frag_movea_moveq_jsr_ori`: Short code fragment that copies an address to a1, then loads 0 into d0, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0.
- `frag_movea_jsr_ori_2`: Short code fragment that copies d0 to a6, then copies the memory at an offset from a6 to a0, then calls another routine, then merges 0 into d0.
- `frag_movea_move_rts`: Short code fragment that copies an address to a5, then copies a fixed memory address to a fixed memory address, then copies an address to a fixed memory address, then copies 32800 to the memory at an offset from a5, then copies 1 to a fixed memory address, then returns.
- `increment_counters`: Saves registers d0 and a5, reads interrupt status register, increments various global counters based on conditions, updates a table with values from another table, then jumps to a fixed address.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then copies an address to a fixed memory address, then returns.
- `frag_movem_movea_move_jsr_ori`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a6, then copies 80076 to d0, then copies 65538 to d1, then calls another routine, then merges 0 into d0.
- `frag_addi_move_movea_jsr_ori`: Short code fragment that adds 38 to d0, then copies d0 to a fixed memory address, then adds 40038 to d0, then copies d0 to a fixed memory address, then copies a fixed memory address to a6, then copies 100 to d0, then copies 65538 to d1, then calls another routine, then merges 0 into d0.
- `frag_movea_clr_jsr`: Short code fragment that copies an address to a1, then clears d0, then calls another routine.
- `setup_coprocessor`: Copies data from memory location 38(a4) to a global variable, fills an array with word values and their swapped counterparts, sets up coprocessor registers for display, clears color table entries, then restores registers and returns.
- `read_joystick_input`: Clears several joystick input buffers, reads joystick data from hardware registers, processes the input to calculate movement values, stores results in global variables, then restores registers and returns.
- `frag_movem_andi_lea_move_tst_rts`: Short code fragment that saves registers on the stack, then masks d0 with 255, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d0, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d0, then restores saved registers from the stack, then tests d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_color_table`: Saves registers d0-d1/a0-a1, copies data from a parameter pointer to multiple color table locations, performs 16 iterations of copying and updating values, then restores registers and returns.
- `initialize_coprocessor`: Configures coprocessor registers including DMA control and sets up a loop that waits for DMA completion before entering an infinite loop.
- `frag_movem_subq_bmi_move_cmp_beq_dbra_r`: Short code fragment that saves registers on the stack, then subtracts 1 from d0, then branches if the last result was negative, then copies a fixed memory address to d1, then compares d1 with a fixed memory address, then branches if the last result was zero, then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `setup_blitter`: Initializes blitter registers with specific values for bit block transfer operations, then waits for DMA completion before returning.

### Hot Rubber (1991)(Palace)(Disk 1 of 2).adf:GP.PRG#unpacked

- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `decode_and_copy_data`: Decodes compressed data from a source buffer and copies it to a destination buffer using a lookup table, handling both literal and compressed sequences.
- `update_sprite_parameters`: Updates sprite rendering parameters including position, size, and clipping values based on input data and global configuration.
- `update_blitter_parameters`: Updates blitter parameters based on global variables and configures DMA operations for graphics rendering.
- `setup_blitter_dma`: Configures blitter registers for DMA operations using values from global variables, sets up multiple blitter operations with specific moduli and addresses, and waits for blitter completion between operations.
- `update_global_counter`: Updates a global variable with data from a table and sets a word value in another global variable, then returns the original value from register d0.
- `perform_bitwise_screen_operations`: Performs bitwise AND and OR operations on memory locations using values from two tables, applying different operations to various screen regions based on input dimensions and offsets.
- `bitwise_screen_operations`: Performs bitwise AND and OR operations on memory locations using values from two tables, with different operations applied to various screen buffer offsets.
- `conditional_memory_write`: Compares input parameters against stored values, performs bit manipulation on memory locations, and either returns immediately or jumps to another routine based on comparison results.
- `frag_lea_move_and_movem_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to d3, then masks the memory at a0 with d3, then restores saved registers from the stack, then returns.
- `frag_move_jmp`: Short code fragment that copies d1 to the memory at a0 (advancing a0), then jumps to another point in the code.
- `frag_move_jmp_2`: Short code fragment that copies d1 to the memory at a0 (advancing a0), then jumps to another point in the code.
- `frag_move_jmp_3`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (2 times in a row), then jumps to another point in the code.
- `frag_move_jmp_4`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (2 times in a row), then jumps to another point in the code.
- `frag_move_jmp_5`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (3 times in a row), then jumps to another point in the code.
- `frag_move_jmp_6`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (3 times in a row), then jumps to another point in the code.
- `frag_move_jmp_7`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (4 times in a row), then jumps to another point in the code.
- `frag_move_jmp_8`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (4 times in a row), then jumps to another point in the code.
- `frag_move_jmp_9`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (5 times in a row), then jumps to another point in the code.
- `frag_move_jmp_10`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (5 times in a row), then jumps to another point in the code.
- `frag_move_jmp_11`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (6 times in a row), then jumps to another point in the code.
- `frag_move_jmp_12`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (6 times in a row), then jumps to another point in the code.
- `frag_move_jmp_13`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (7 times in a row), then jumps to another point in the code.
- `frag_move_jmp_14`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (7 times in a row), then jumps to another point in the code.
- `frag_move_jmp_15`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (8 times in a row), then jumps to another point in the code.
- `frag_move_jmp_16`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (8 times in a row), then jumps to another point in the code.
- `frag_move_jmp_17`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (9 times in a row), then jumps to another point in the code.
- `frag_move_jmp_18`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (9 times in a row), then jumps to another point in the code.
- `update_collision_bounds`: Updates boundary coordinates for collision detection based on input parameters and clamps values against global limits.
- `update_screen_bounds`: Updates screen boundary coordinates using values from a table, configures bit block transfer registers, and performs bitwise screen operations based on calculated dimensions.
- `interpolate_coordinates`: Calculates interpolated coordinates between two points using integer arithmetic and stores the results in a table.
- `coordinate_interpolation`: Calculates interpolated coordinates between two points using bit shifting and comparisons, returning the final calculated x coordinate value.
- `bitmapped_pattern_processing`: Processes a pattern from memory through bitwise operations including rotation, negation, and masking, then applies the result to multiple screen buffer locations with conditional OR operations.
- `update_pattern_coordinates`: Updates pattern coordinates by incrementing x and y values, wrapping x coordinate when it exceeds 320 pixels, and restores original coordinates on zero byte encounter.
- `data_transformation_loop`: Transforms data by copying and mapping bytes through a lookup table, performing multiple iterations of byte manipulation and memory writes in a nested loop structure.
- `setup_color_palette`: Configures color palette entries by processing RGB values through multiplication, division, and bit manipulation before storing results in hardware registers.
- `update_color_palette_2`: Updates color palette entries by processing RGB values through multiplication, division, and bit manipulation while decrementing a counter until it reaches zero.
- `initialize_frame_timing`: Clears a table of 10 longwords and initializes frame timing variables by reading vertical position register values, performing multiplication operations, and updating frame timing variables.
- `parse_object_data`: Parses structured object data from a buffer, copying specific values to a destination and processing pixel data with conditional logic based on flags.
- `conditional_register_setup`: Sets up register values based on conditional checks against input parameters, returning specific values or zero depending on whether certain conditions are met.
- `memory_block_copy`: Copies data blocks from one memory location to another, initializing multiple screen buffer locations with values from a source table and setting several global variables to specific values.
- `frag_movem_jsr_move_rts`: Short code fragment that saves registers on the stack, then calls another routine, then copies 0 to a fixed memory address, then restores saved registers from the stack, then returns.
- `validate_and_store_coordinates`: Validates coordinate boundaries and stores adjusted coordinates in global variables while setting up parameters for object rendering operations.
- `shift_and_combine`: Processes pixel data by shifting values based on a global variable, combining results through bitwise operations, and updating multiple memory locations.
- `copy_object_data`: Copies structured data from a source buffer to a destination buffer with conditional logic based on a control flag, processing multiple data elements in a loop.
- `store_object_data`: Writes object state information to a global buffer with different memory access patterns depending on a condition flag, handling 10-element data structures.
- `update_pattern_coordinates_2`: Updates pattern coordinates by incrementing x and y values, wrapping x coordinate when it exceeds 320 pixels, and restores original coordinates after processing.
- `frag_clr_tst_beq_addq_bra_rts`: Short code fragment that clears d0, then tests the memory at a0 (advancing a0), then branches if the last result was zero, then adds 1 to d0, then jumps to another point in the code, then returns.
- `shift_and_clear_buffer`: Performs a series of memory shifts and clears across multiple data locations in a loop, moving data from higher addresses to lower addresses while zeroing the original locations.
- `frag_move_dbra_rts`: Short code fragment that copies d1 to the stack, then copies 9 to d1, then copies the memory at a0 (advancing a0) to the memory at an offset from a1 (4 times in a row), then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d1, then copies the top of the stack to d1, then returns.
- `setup_dma_transfer_2`: Configures DMA channels by setting up hardware registers with specific values, performs multiple DMA I/O operations, and copies data between memory locations.
- `configure_dma_channels`: Sets up DMA channel configurations by writing specific register values, performs DMA I/O operations with different parameters, and calls a routine to update port registers.
- `manage_dma_operations`: Manages DMA operations by setting a global flag, performing permit and forbid operations, executing multiple DMA I/O requests with various parameters, and clearing the global flag.
- `update_port_registers`: Updates hardware port registers by reading current values, masking bits with specific patterns, and writing modified values to control peripheral devices.
- `initialize_system_resources`: Initializes system resources by opening libraries and devices, setting up communication ports, and establishing task references for system operations.
- `manage_file_operations_2`: Opens a file for reading, reads data into memory, and closes the file handle, returning a status code indicating success or failure of each operation.
- `close_file_handle`: Closes a file handle by calling an operating system routine with specific parameters, then restores saved register values from the stack before returning.
- `process_disk_access`: Checks for specific disk file headers and handles file operations including opening, reading, and closing files based on validation results.
- `xor_data_block`: Applies XOR encryption to a memory block by toggling the high bit of each byte in a loop until reaching a specified end address.
- `setup_object_parameters`: Configures object properties based on parameter values, storing register contents into specific memory locations and initializing various fields including flags and counters.
- `configure_dma_interrupts`: Sets up DMA and interrupt configurations for different hardware channels by writing control values to custom registers based on parameter input.
- `setup_audio_interrupts`: Configures multiple audio interrupt handlers by setting up interrupt structures with specific parameters, installing them using system calls, and initializes audio channel registers for sound playback.
- `update_object_counter`: Increments a counter in a global variable and updates a table index when the counter reaches a certain value, then calls another routine with calculated parameters.
- `update_audio_channels`: Updates audio channel settings based on global variable states, writing volume, period, and length values to custom hardware registers for four audio channels.
- `manage_audio_dma`: Manages audio DMA operations by checking a counter value, resetting audio registers when the counter reaches zero, enabling DMA channels and interrupts, and calling an audio update routine when needed.
- `frag_subq_move_moveq_rts`: Short code fragment that subtracts 1 from a fixed memory address, then copies 128 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `handle_dma_channel_1`: Manages the first DMA channel configuration by checking a counter value and setting appropriate register values for DMA and interrupt control.
- `frag_subq_move_moveq_rts_2`: Short code fragment that subtracts 1 from a fixed memory address, then copies 256 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `handle_dma_channel_2`: Manages the second DMA channel configuration by checking a counter value and setting appropriate register values for DMA and interrupt control.
- `frag_subq_move_moveq_rts_3`: Short code fragment that subtracts 1 from a fixed memory address, then copies 512 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `handle_dma_channel_3`: Manages the third DMA channel configuration by checking a counter value and setting appropriate register values for DMA and interrupt control.
- `frag_subq_move_moveq_rts_4`: Short code fragment that subtracts 1 from a fixed memory address, then copies 1024 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `install_interrupt_handlers`: Installs interrupt handlers by calling library functions to set vector pointers for multiple interrupt levels and removes a server for a specific interrupt level.
- `initialize_rubber_effect`: Initializes collision boundary coordinates and sets up a series of operations to update collision detection data based on input parameters.
- `process_collision_grid`: Processes collision detection grid entries by calculating coordinates, validating ranges, and invoking a collision update routine for each valid entry.
- `update_collision_flags`: Updates collision flags in a grid structure based on input coordinates and bit manipulation operations, clearing specific bits in the process.
- `update_sprite_patterns`: Processes sprite pattern data by applying bitwise operations to coordinate values, updating pattern coordinates based on input parameters, and handling lap and gear display text updates.
- `copy_sprite_data`: Copies sprite data from a source table to a destination buffer using indexed addressing and multiple byte transfers.
- `update_velocity`: Updates velocity values based on input parameters and performs boundary checks, modifying position and related counters in a data structure.
- `frag_move_ori_btst_andi_bclr`: Short code fragment that copies the memory at a0 to the memory just below a3 (moving a3 down), then merges 50 into an indexed entry from a2 (4 times in a row), then tests bit d0 of the memory at an offset from a4, then masks the memory at a0 (advancing a0) with 600, then masks the memory at a0 (advancing a0) with 400, then clears bit d0 of the memory at a0 (2 times in a row).
- `update_position_and_collision`: Updates position coordinates based on vector calculations and performs collision detection with boundary checks.
- `update_position_data`: Reads position coordinates from memory, performs vector arithmetic operations including multiplication, subtraction, and addition with fixed-point scaling, then stores the results to global variables.
- `process_input_coordinates`: Loads input coordinates from memory, performs vector arithmetic operations including multiplication, subtraction, and addition with fixed-point scaling, then stores the results to global variables.
- `calculate_index_offset`: Calculates an index offset based on input values, performs bounds checking, and returns scaled offsets for memory access.
- `handle_input_buttons`: Checks multiple hardware input buttons by calling a routine for each button, sets corresponding global flags to either 1 or 65535 based on the button states, and returns immediately after processing all buttons.
- `sort_and_rearrange_data`: Sorts data in a table using bubble sort algorithm and rearranges elements based on comparison results.
- `update_position_flags`: Updates position-related flags and boundary checks based on calculated offsets and input values, then adjusts a velocity value with bit shifting before returning.
- `frag_tst_bpl_addi_bra_cmp_bmi_subi_rts`: Short code fragment that tests d0, then branches if the last result was not negative, then adds 720 to d0, then jumps to another point in the code, then compares d0 with 720, then branches if the last result was negative, then subtracts 720 from d0, then jumps to another point in the code, then returns.
- `frag_jsr_cmp_bmi_subi_rts`: Short code fragment that calls another routine, then compares d0 with 361, then branches if the last result was negative, then subtracts 720 from d0, then returns.
- `perform_fixed_point_multiplication`: Performs fixed-point multiplication and division operations with bit shifting and scaling, returning scaled results.
- `frag_move_lsl_lea_movea_rts`: Short code fragment that copies a0 to the stack, then copies d0 to d1, then shifts d0 left by 1, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d0, then shifts d1 left by 1, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d1, then copies the top of the stack to a0, then returns.
- `calculate_scaled_value`: Calculates a scaled value using division and lookup table access, applying corrections based on sign and magnitude comparisons.
- `perform_vector_operations`: Performs vector arithmetic operations including multiplication, subtraction, addition, and bit shifting with fixed-point scaling.
- `execute_vector_arithmetic`: Executes vector arithmetic operations involving multiplication, subtraction, addition, and bit shifting with fixed-point scaling.
- `scale_and_divide_values`: Scales input values by a fixed amount, performs division operations, and returns combined quotient and remainder results.
- `update_sprite_positions`: Updates sprite position data by calculating scaled differences, performing vector arithmetic operations, and storing results in a structured format.
- `process_collision_detection_2`: Tests collision between two game objects using vector arithmetic and scaling, updates object states based on collision results, and modifies movement parameters for both objects.
- `process_collision`: Tests for collision between two game objects and updates their state variables accordingly.
- `check_collision`: Performs collision detection between two objects using coordinate comparisons and updates object state variables when a collision is detected.
- `check_collision_with_offset`: Performs collision detection with an additional offset parameter, updating object state variables when the collision condition is met.
- `update_game_state`: Updates game state variables including sprite parameters and global configuration based on input data and performs conditional rendering updates.
- `update_player_animation`: Updates player animation state and coordinates based on input and collision data, managing sprite positioning and palette settings while handling game state transitions.
- `update_sprite_animation`: Updates sprite animation state by modifying counters and pointers based on collision data, then configures graphics rendering parameters including DMA settings and color palette entries before returning.
- `update_game_state_2`: Updates game state variables including sprite positions and collision detection parameters, processes input flags, and manages animation frame indices while handling boundary checks and coordinate adjustments.
- `initialize_game_state`: Initializes game state variables including screen buffer contents and collision detection parameters, then configures color palette entries and updates sprite data based on input conditions.
- `update_player_sprites`: Configures sprite parameters for two players based on game state, updates pattern coordinates, and sets up blitter operations for sprite rendering.
- `update_sprite_and_dma`: Configures sprite rendering parameters based on input values and global state, then performs bit manipulation on memory locations before setting a DMA control register.
- `update_sprite_data_3`: Updates sprite rendering parameters including position, size, and clipping values based on input data and global configuration, then configures blitter DMA registers with specific values and waits for DMA to complete before setting a final status flag.
- `update_player_positions`: Updates sprite position parameters for player objects based on game state and input values, configuring coordinates and dimensions before rendering.
- `find_collision_index`: Searches through a set of collision boundaries to find which index matches the given coordinates and returns that index.
- `setup_dma_channels`: Configures four DMA channels by calling a setup routine with different parameters and clears specific hardware registers after each configuration.
- `clear_audio_flags`: Clears multiple audio-related flags in global variables and calls another routine, then returns.
- `initialize_audio_system`: Sets up DMA channels for audio processing, clears a global variable, and installs interrupt handlers for audio system operations.
- `manage_audio_parameters`: Updates audio parameters based on conditional checks, configures hardware registers for sound generation, and manages audio channel states.
- `calculate_division_result`: Calculates a division result based on input value and multiple conditional branches, returning a computed value in registers.
- `process_sprite_data_2`: Processes sprite configuration data including position, size, and scaling parameters, then updates global rendering variables and calls a rendering routine.
- `scale_sprite_data`: Processes sprite dimension values to determine scaling parameters and updates screen boundary coordinates.
- `update_rendering_parameters`: Updates rendering parameters by testing a bit condition and modifying hardware registers, then copies data between memory locations using calculated offsets and pointer arithmetic.
- `update_rendering_state`: Updates rendering parameters based on a bit test condition, modifying memory locations with calculated values and advancing pointers through multiple iterations.
- `initialize_structure_data`: Initializes a data structure by clearing memory blocks and setting specific values at predetermined offsets.
- `frag_jsr_move_lea_rts`: Short code fragment that calls another routine, then copies a fixed memory address to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then calls another routine (2 times in a row), then returns.
- `initialize_palette`: Initializes color palette entries by processing RGB values through multiplication, division, and bit manipulation while decrementing a counter.
- `initialize_dma_transfer`: Sets up DMA transfer parameters by writing specific values to hardware registers, performs multiple DMA I/O operations, and copies data between memory locations.
- `write_word_sequence`: Writes eight specific word values to a memory location starting at a global variable address, then sets another global variable to -1.
- `initialize_graphics_system_2`: Initializes graphics hardware by clearing color registers, setting up blitter parameters for DMA operations, configuring display registers including color palette entries, and performing various initialization tasks related to screen rendering and audio flags.
- `update_color_palette_4`: Processes color data by multiplying values from a source table by two and storing results in two destination tables.
- `update_rubber_effect`: Updates rubber effect rendering parameters including position, size, and clipping values based on input data and global configuration.
- `toggle_flag_and_wait`: Toggles a 16-bit flag in a global variable using XOR operation, then waits for the flag to become zero by polling its value.
- `update_object_state`: Updates object state variables based on conditional checks and increments counters, with different behavior for different states.
- `process_input_data_2`: Processes joystick and mouse input data by reading from hardware registers, storing values to memory locations, and managing different input states based on conditional tests.
- `check_input_states`: Tests multiple input conditions using lookup table access, sets flags based on the results of these tests, and manages state variables accordingly.
- `update_object_position`: Updates object position coordinates and related flags by copying input values to memory locations, performing arithmetic operations on a counter, and calling multiple subroutines for collision detection.
- `update_rubber_parameters`: Updates rubber effect rendering parameters including position, size, and clipping values based on input data and global configuration.
- `update_game_objects`: Copies data between memory locations, updates counters, and manages object states when specific conditions are met during game processing.
- `sort_and_process_data_2`: Clears state variables, processes data through a sorting routine, and rearranges elements in tables based on comparison results.
- `configure_rendering_parameters`: Configures rendering parameters for objects by setting various hardware register values based on conditional checks.
- `initialize_sprite_data_3`: Initializes sprite parameter structures with default values including position, size, and color settings.
- `update_race_graphics_2`: Updates race-related graphics parameters including sprite data, color palettes, and display settings.
- `setup_sprite_data`: Initializes sprite rendering parameters including position, size, and clipping values.
- `initialize_race_display`: Sets up display parameters for race-related text elements including position, size, and character data, then configures and renders multiple numeric characters and text strings at various screen coordinates.
- `initialize_display_elements`: Sets up display parameters for multiple text elements including position, size, and color values, then processes pattern coordinates and updates sprite rendering parameters.
- `initialize_graphics_system_3`: Initializes graphics hardware by setting up display registers, configuring color palettes, and preparing sprite data structures.
- `copy_data_to_blitter`: Configures blitter registers and performs data transfer operations to graphics hardware using multiple DMA cycles.
- `copy_data_to_blitter_reverse`: Configures blitter registers and performs data transfer operations to graphics hardware in reverse direction using multiple DMA cycles.
- `allocate_sound_buffers`: Allocates memory for sound-related data using the Exec library's AllocMem function with specific sizes and flags, storing the returned pointers in global variables.
- `set_game_flags`: Sets several game state variables including FADC, FADE, FAE6, FAE8, and FAE4 to specific values.
- `clear_sprite_data`: Clears multiple word values in a memory area starting at a global variable address.
- `initialize_graphics_2`: Sets up graphics-related global variables with specific values including a screen mode, a palette table pointer, and various display parameters.
- `load_and_decode_sprite_data`: Decodes compressed sprite data from a source buffer into a destination buffer, handling both literal and compressed sequences.
- `initialize_sprite_data`: Initializes sprite data structures by setting values at specific offsets, handling conditional logic for different sprite types.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `initialize_game_structure`: Initializes a game data structure by setting default values at specific offsets, calculating index offsets for memory access, and performing arithmetic operations on timing variables.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 0 into d0, then copies 1499 to d1, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d1, then returns.
- `setup_blitter_dma_2`: Configures blitter DMA registers including BLTCON0, BLTCON1, BLTDMOD, BLTDPTH, and BLTSIZE with specific values, waits for DMA to complete before and after the setup, then returns.
- `update_sprite_data`: Updates sprite position and animation data by swapping pointers, incrementing counters, and performing bit operations on sprite-related variables in memory.
- `disable_interrupts`: Disables system interrupts by calling the Forbid function, reads hardware register values, and sets up interrupt vectors for system operations.
- `frag_movea_moveq_jsr_move_rts`: Short code fragment that copies 70 to a1, then loads 0 into d0, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then copies d0 to a6, then copies the memory at an offset from a6 to a0, then calls another routine (2 times in a row), then returns.
- `frag_movea_move_rts_2`: Short code fragment that copies an address to a5, then copies a fixed memory address to a fixed memory address, then copies 61156 to a fixed memory address, then copies 32800 to the memory at an offset from a5, then copies 1 to a fixed memory address, then returns.
- `handle_input_event`: Processes input events by checking joystick status, updating counters, and managing various game state variables based on input conditions.
- `frag_move_rts_2`: Short code fragment that copies a fixed memory address to a fixed memory address, then copies 61324 to a fixed memory address, then returns.
- `process_joystick_input`: Reads joystick data from CIA registers, performs bit manipulation to determine input direction, updates a lookup table entry, and sets a flag based on the processed input.
- `frag_lea_move_bra`: Short code fragment that loads the address of a fixed memory address into a0, then copies 0 to an indexed entry from a0, then jumps to another point in the code.
- `frag_movem_jmp`: Short code fragment that restores saved registers from the stack, then jumps to another point in the code.
- `initialize_graphics`: Allocates memory for graphics structures using system calls, initializes display registers including copper lists and bitplane configurations, then clears specific memory areas.
- `read_joystick_data`: Reads joystick and mouse input data from hardware registers, processes button states and movement values, calculates differences between consecutive readings, and stores results in global variables.
- `lookup_table_access`: Uses a byte value as an index to access two lookup tables, performs a series of table lookups with the indexed values, and returns the final result from the second table.
- `access_lookup_tables`: Performs conditional lookups in multiple tables based on input values, processes the retrieved data through arithmetic operations, and manages state flags.
- `execute_function_pointer`: Calculates an address using input parameters and a lookup table, retrieves a function pointer from another table, then calls that function with specific parameters.
- `frag_and_rts`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_2`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_2`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_3`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_3`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_4`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_4`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_or_and_rts_5`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_and_or_rts_5`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_or_and_rts_6`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_and_or_rts_6`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `frag_or_and_rts_7`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `frag_and_or_rts_7`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `frag_or_rts`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `calculate_frame_time`: Reads vertical position register values to initialize a time counter, performs multiplication operations, and updates frame timing variables for display synchronization.
- `perform_multiplication`: Performs signed 16-bit multiplication with carry handling, processes multiple values through arithmetic operations, and returns the final result in a register.
- `update_color_palette`: Copies color values from input data to multiple display registers including COLOR00 through COLOR15 and a custom palette location, updating the graphics color settings.
- `process_sprite_data`: Processes sprite information from a buffer, calculates offsets and counts entries, then copies data between memory locations while updating global variables with computed values.
- `copy_sprite_chunk`: Copies a fixed-size chunk of sprite data from one memory location to another using a loop that processes 20 words, then updates global pointers and DMA control registers.
- `wait_for_dma_ready`: Waits for a specific DMA status bit to clear by polling a hardware register, then enters an infinite loop.
- `wait_for_dma_ready_2`: Checks DMA status repeatedly until a specific bit is set, then decrements a counter and branches based on its value before returning.
- `setup_blitter_parameters`: Configures blitter hardware registers with predefined values for graphics operations, including control registers, address pointers, and size parameters.

### unit:u0#packproof-None#unpacked

- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `decode_and_copy_data_2`: Decodes compressed data from an input stream, copies decoded bytes to output locations, and maintains a circular buffer with rotation logic for pattern matching.
- `update_collision_data_2`: Updates collision-related parameters based on input values and configures blitter settings for graphics rendering.
- `adjust_blitter_moduli`: Adjusts blitter modulus values based on global variables and sets bit 0 of a status byte.
- `setup_blitter_operations`: Configures blitter registers for graphic operations using values derived from global variables and structure offsets, then performs multiple blitter operations with varying parameters before returning.
- `update_global_counter_2`: Updates a global variable with data from a table based on a calculated index, and sets a status flag when the index changes.
- `process_bitwise_operations`: Performs bitwise AND and OR operations on memory locations using values from two tables, with different operations applied to various screen regions.
- `apply_bitmask_pattern`: Applies bitwise operations to memory locations using indexed values from tables, modifying pixel data patterns in a graphics buffer.
- `process_collision_data`: Processes collision detection data by calculating coordinates and applying bitwise operations to memory locations based on input parameters.
- `frag_lea_move_and_movem_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to d3, then masks the memory at a0 with d3, then restores saved registers from the stack, then returns.
- `frag_move_jmp_19`: Short code fragment that copies d1 to the memory at a0 (advancing a0), then jumps to another point in the code.
- `frag_move_jmp_20`: Short code fragment that copies d1 to the memory at a0 (advancing a0), then jumps to another point in the code.
- `frag_move_jmp_21`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (2 times in a row), then jumps to another point in the code.
- `frag_move_jmp_22`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (2 times in a row), then jumps to another point in the code.
- `frag_move_jmp_23`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (3 times in a row), then jumps to another point in the code.
- `frag_move_jmp_24`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (3 times in a row), then jumps to another point in the code.
- `frag_move_jmp_25`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (4 times in a row), then jumps to another point in the code.
- `frag_move_jmp_26`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (4 times in a row), then jumps to another point in the code.
- `frag_move_jmp_27`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (5 times in a row), then jumps to another point in the code.
- `frag_move_jmp_28`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (5 times in a row), then jumps to another point in the code.
- `frag_move_jmp_29`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (6 times in a row), then jumps to another point in the code.
- `frag_move_jmp_30`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (6 times in a row), then jumps to another point in the code.
- `frag_move_jmp_31`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (7 times in a row), then jumps to another point in the code.
- `frag_move_jmp_32`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (7 times in a row), then jumps to another point in the code.
- `frag_move_jmp_33`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (8 times in a row), then jumps to another point in the code.
- `frag_move_jmp_34`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (8 times in a row), then jumps to another point in the code.
- `frag_move_jmp_35`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (9 times in a row), then jumps to another point in the code.
- `frag_move_jmp_36`: Short code fragment that copies d1 to the memory at a0 (advancing a0) (9 times in a row), then jumps to another point in the code.
- `update_collision_bounds_2`: Updates boundary coordinates for collision detection based on input parameters and clamps values against global limits.
- `update_screen_bounds_2`: Updates screen boundary coordinates using values from a table, configures bitblt operations with specific registers, and performs bitwise screen manipulations based on calculated dimensions.
- `interpolate_coordinates_2`: Calculates interpolated coordinates between two points using integer arithmetic and stores the results in a table.
- `calculate_midpoint_coordinates`: Calculates midpoint coordinates between two points using bit shifting and compares them to determine a final coordinate pair.
- `apply_bitwise_filter`: Applies bitwise filtering operations to memory locations based on input data, performing bit rotation and logical operations on pixel data.
- `update_pattern_position`: Updates pattern coordinates by incrementing X and Y values, and handles coordinate overflow by adjusting the Y value and resetting the X value when the threshold is exceeded.
- `process_data_block`: Processes a data block by reading from a source table, applying byte lookups, and copying results to a destination buffer with multiple iterations.
- `setup_color_palette_2`: Configures color palette entries by processing RGB values through multiplication and division operations, then writes results to hardware color registers.
- `update_color_palette_3`: Updates color palette entries by processing RGB values through multiplication and division operations, then writes results to hardware color registers with decrementing intensity values.
- `initialize_frame_timing_2`: Initializes frame timing variables by clearing a table of 10 longwords, reading vertical position register values to set up a time counter, performing multiplication operations, and updating frame timing variables.
- `parse_object_data_2`: Parses structured object data from a buffer, copying and processing multiple data elements including conditional byte operations and subroutine calls.
- `check_loop_counters`: Checks three loop counters and returns immediately if any of them is not equal to one, otherwise returns negative one.
- `copy_and_initialize_table`: Copies data from one table to another and initializes specific memory locations with predefined values, setting up graphics or display parameters.
- `frag_movem_jsr_move_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then copies 0 to a fixed memory address, then restores saved registers from the stack, then returns.
- `validate_and_store_coordinates_2`: Validates coordinate values against boundary limits, calculates memory offsets, and stores object state information to global buffers with conditional logic.
- `process_pixel_data`: Processes pixel data through multiple memory locations using bitwise operations and shifts, updating values based on a global variable and a shift amount stored in a register.
- `copy_object_data_2`: Copies structured data from a source buffer to a destination buffer with different memory offsets based on a counter and conditional logic, then updates pointers and decrements the counter.
- `store_object_data_2`: Stores register values into memory locations based on a parameter, initializes multiple fields including flags and counters, and returns.
- `initialize_sprite_data_2`: Initializes sprite position and dimension data for multiple sprite objects, setting their coordinates and sizes based on input parameters and performing boundary checks.
- `frag_clr_tst_beq_addq_bra_rts_2`: Short code fragment that clears d0, then tests the memory at a0 (advancing a0), then branches if the last result was zero, then adds 1 to d0, then jumps to another point in the code, then returns.
- `copy_and_zero_block`: Copies data from one memory location to another while zeroing the source locations, processing ten 32-bit words in a loop.
- `frag_move_dbra_rts_2`: Short code fragment that copies d1 to the stack, then copies 9 to d1, then copies the memory at a0 (advancing a0) to the memory at an offset from a1 (4 times in a row), then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d1, then copies the top of the stack to d1, then returns.
- `setup_dma_transfer_3`: Configures DMA channels by setting up hardware registers with specific values, performing multiple I/O operations through the exec library, and copying data between memory locations.
- `configure_dma_channels_2`: Sets up DMA channel configurations by writing specific register values, performing I/O operations through the exec library, and updating hardware port registers for DMA transfers.
- `manage_dma_access`: Manages DMA access by setting a global flag to maximum value, calling forbid and permit functions from the exec library, performing multiple I/O operations with different parameters, and clearing the global flag before returning.
- `update_port_registers_2`: Updates hardware port registers through multiple iterations of bitwise AND/OR operations, setting specific bit patterns for peripheral control.
- `initialize_system_resources_2`: Initializes system resources by opening libraries and devices, setting up ports, and storing handles in global variables.
- `open_and_close_file`: Opens a file using a system call with specific parameters, stores the returned handle in a global variable, and then closes the file handle if it was successfully opened.
- `close_file_handles`: Closes a file handle by calling the DOS library Close function with specific parameters, then restores saved register values before returning.
- `manage_file_operations`: Handles file opening and reading operations by calling DOS library functions, manages system permissions, updates display elements, and processes audio channel settings.
- `xor_memory_range`: XORs bytes in a memory range with a fixed value, iterating from one address to another until the end is reached.
- `setup_object_parameters_2`: Sets up object parameters based on input values, storing data into different memory locations depending on a condition parameter.
- `configure_dma_interrupts_2`: Configures DMA and interrupt settings in hardware registers based on an input parameter, clearing specific flags and setting control values.
- `setup_audio_interrupts_2`: Configures multiple audio interrupt handlers by setting up data structures for different DMA channels, registers them with the system, and initializes global variables related to audio processing.
- `update_object_counter_2`: Increments a counter in a global variable and checks if it matches a value from a table; if so, it updates the counter to 11 and calls another routine with values fetched from the table.
- `update_audio_channels_2`: Updates audio channel settings for four separate channels based on values from a global variable, writing to custom hardware registers including volume, period, and length fields, then sets a new value in the global variable.
- `update_audio_dma`: Updates audio channel settings by writing to DMA and interrupt control registers, and conditionally calls another routine based on a global variable's value.
- `frag_subq_move_moveq_rts_5`: Short code fragment that subtracts 1 from a fixed memory address, then copies 128 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `handle_dma_channel_1_2`: Handles DMA channel 1 configuration by checking a counter value and setting up interrupt and DMA control registers accordingly.
- `frag_subq_move_moveq_rts_6`: Short code fragment that subtracts 1 from a fixed memory address, then copies 256 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `handle_dma_channel_2_2`: Handles DMA channel 2 configuration by checking a counter value and setting up interrupt and DMA control registers accordingly.
- `frag_subq_move_moveq_rts_7`: Short code fragment that subtracts 1 from a fixed memory address, then copies 512 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `handle_dma_channel_3_2`: Handles DMA channel 3 configuration by checking a counter value and setting up interrupt and DMA control registers accordingly.
- `frag_subq_move_moveq_rts_8`: Short code fragment that subtracts 1 from a fixed memory address, then copies 1024 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `install_interrupt_handlers_2`: Installs interrupt handlers by calling library functions to set vector pointers for multiple interrupt levels and removes a server for a specific interrupt level.
- `update_rubber_bounds`: Updates boundary coordinates for collision detection based on input parameters and clamps values against global limits.
- `process_collision_grid_2`: Processes collision detection for grid-based objects by iterating through coordinates, performing arithmetic operations on position values, and calling a routine to update collision flags when conditions are met.
- `check_and_clear_flag`: Checks a bit in a grid-based structure using input coordinates, and if the bit is set, clears it in a related data field.
- `update_race_graphics`: Processes and updates race-related graphics data including pattern coordinates, sprite information, and display elements based on game state variables.
- `copy_sprite_data_2`: Copies sprite data from a source table to a destination buffer using indexed addressing and multiple byte transfers.
- `update_velocity_2`: Updates velocity values based on input parameters and performs boundary checks, modifying position and related state variables accordingly.
- `update_display_buffer`: Updates display buffer bytes with specific bit operations and performs conditional checks on memory locations.
- `update_collision_data`: Processes collision detection values by calculating offsets, updating position components, and managing counters.
- `update_position_data_2`: Loads word values from memory locations offset by 10 and 12 from a0 into d0, stores them at fixed offsets, then performs arithmetic operations including multiplication, division, and bit shifting on vector components.
- `validate_and_process_input`: Reads a word value from memory offset by 10 from a0, checks if another global variable is zero, decrements the value if it's negative, and stores the result at a fixed offset.
- `calculate_table_indices`: Calculates table indices and performs arithmetic operations to determine memory offsets for data access.
- `handle_input_events`: Saves d0 to the stack, calls a routine to read input data, then reads multiple joystick button states from hardware registers.
- `sort_and_process_data`: Sorts data in a table using bubble sort algorithm and processes elements with bit manipulation and memory operations.
- `update_position_bounds`: Updates position boundaries based on calculated offsets and clamps values within specific ranges, modifying global variables for position tracking.
- `frag_tst_bpl_addi_bra_cmp_bmi_subi_rts_2`: Short code fragment that tests d0, then branches if the last result was not negative, then adds 720 to d0, then jumps to another point in the code, then compares d0 with 720, then branches if the last result was negative, then subtracts 720 from d0, then jumps to another point in the code, then returns.
- `frag_jsr_cmp_bmi_subi_rts_2`: Short code fragment that calls another routine, then compares d0 with 361, then branches if the last result was negative, then subtracts 720 from d0, then returns.
- `perform_fixed_point_multiplication_2`: Performs fixed-point multiplication operations on word values and returns extended long results.
- `load_table_entry`: Loads word values from two different tables using indexed addressing with shift operations on input parameters, then returns the second loaded value.
- `perform_arithmetic_operations`: Performs arithmetic operations including negation, bit shifting, division, and table lookups with conditional logic.
- `execute_vector_arithmetic_2`: Executes vector arithmetic operations using multiplication, subtraction, addition, and bit shifting with extended results.
- `calculate_collision_data`: Performs fixed-point arithmetic calculations on coordinate differences and stores results in multiple registers before returning.
- `scale_and_divide_coordinates`: Shifts input values left by 8 bits, then performs division operations on them with a common divisor, storing the quotient in the lower word of a register.
- `update_sprite_data_6`: Updates sprite position and scaling parameters based on vector calculations and global state, then stores the results in a structured output buffer.
- `update_collision_state`: Tests for collision conditions between two game objects and updates their state flags and position values accordingly.
- `check_collision_2`: Tests for collision between two game objects and updates their states accordingly.
- `process_collision_detection`: Loads coordinate data from tables, performs arithmetic operations on them, compares results with thresholds, and updates game state variables when conditions are met.
- `handle_object_collision`: Processes collision detection logic for objects, reads a global counter, performs coordinate comparisons, and modifies object state variables based on the results.
- `update_sprite_data_4`: Updates sprite rendering parameters including position, size, and clipping values based on input data and global configuration, then calls a rendering routine.
- `update_player_data`: Updates player-related data including position coordinates, state flags, and collision indices based on input and game logic.
- `update_sprite_animation_2`: Updates sprite animation state by modifying global variables, processing coordinate data, and managing frame counters based on input conditions.
- `update_rubber_sprite`: Updates rubber sprite position and animation data by processing input coordinates, calculating midpoints, and configuring sprite rendering parameters.
- `initialize_game_state_2`: Initializes game state variables including screen coordinates and sprite positions, configures color palette entries with specific RGB values, and sets up DMA transfer parameters for graphics operations.
- `update_player_sprites_2`: Updates sprite parameters for two players based on game state, including position, size, and clipping values, then configures blitter parameters for graphics rendering.
- `update_rubber_sprites`: Updates sprite parameters based on input data and global configuration, including position, size, and clipping values.
- `update_sprite_data_5`: Updates sprite rendering parameters including position, size, and clipping values based on input data and global configuration, then configures blitter DMA registers with specific values and waits for DMA to complete before returning.
- `update_player_sprites_3`: Configures sprite parameters for two players based on game state, updates pattern coordinates, and sets up blitter operations for sprite rendering.
- `find_collision_zone`: Iterates through a list of boundary coordinates, compares input values against them, and returns an index when a matching range is found or zero if none match.
- `initialize_dma_channels`: Configures DMA interrupt settings for four hardware channels by calling a setup routine with channel numbers 0 through 3, then clears specific custom register words.
- `clear_audio_flags_2`: Clears multiple audio-related flags in global variables and calls another routine, then returns.
- `setup_audio_dma`: Initializes audio DMA settings for multiple channels by calling a configuration routine with channel numbers 0 through 3, clears a global word variable, and installs interrupt handlers.
- `manage_audio_parameters_2`: Updates audio parameter values from memory tables, configures DMA settings for audio channels based on conditions, and writes updated values to custom audio registers when certain flags are clear.
- `calculate_scroll_offset`: Divides input by a constant, performs conditional division based on a value, adjusts a counter using bit operations, and updates a scroll position variable.
- `process_sprite_data_3`: Processes sprite configuration data to calculate rendering parameters including position, size, and clipping values, then updates global variables with these computed values for sprite display.
- `update_screen_bounds_3`: Updates screen boundary coordinates using values from a table, configures bit block transfer registers, and performs bitwise screen operations.
- `update_rendering_parameters_2`: Configures bitplane DMA parameters and copies data between memory locations using calculated offsets, then calls other routines to handle rendering updates based on conditional tests.
- `configure_rendering`: Tests a specific bit in a global structure and modifies multiple hardware-related variables accordingly, then updates memory pointers and register values in sequence.
- `initialize_memory_block`: Clears memory locations in a block with zero values, writes specific data patterns to multiple memory addresses, and advances pointers through a loop structure.
- `frag_jsr_move_lea_rts_2`: Short code fragment that calls another routine, then copies a fixed memory address to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then calls another routine (2 times in a row), then returns.
- `update_game_state_3`: Updates game state variables including sprite positions and collision detection parameters, processes input flags, and manages animation frames.
- `setup_dma_transfer_2_2`: Configures DMA channels by setting up hardware registers with specific values, performs multiple DMA I/O operations, and copies data between memory locations using loop counters.
- `initialize_collision_data`: Writes a sequence of word values to a global variable table starting at offset ed4c, then sets a specific global variable to negative one.
- `initialize_graphics_system_4`: Initializes multiple graphics-related hardware registers and global variables, including blitter parameters, color settings, and DMA configuration.
- `update_display_pattern`: Processes pattern coordinate updates by copying and scaling data from one memory location to another, then calls multiple routines to handle audio, input, and sprite management.
- `update_rubber_parameters_2`: Updates rubber effect rendering parameters including position, size, and clipping values based on input data and global configuration.
- `configure_dma_channels_3`: Sets up DMA channels by calling a configuration routine with specific parameters, manages a global flag through bitwise operations, and loops until the flag becomes zero.
- `update_object_state_machine`: Processes state transitions for objects in a loop, updates counters, reads lookup table values, and modifies object properties based on current state and counter values.
- `copy_input_data`: Reads joystick and mouse input data from hardware registers and copies it to memory locations based on conditional tests, with different code paths for various input states.
- `check_input_states_2`: Tests multiple input states by calling a lookup routine with specific codes, sets global word variables to -1 or 1 based on the results of these tests, and returns immediately.
- `update_position_and_velocity`: Copies input velocity values to position tracking variables, performs calculations on a counter variable, and calls multiple routines to update movement and collision detection.
- `update_rubber_sprite_data`: Updates sprite rendering parameters including position, size, and clipping values based on input data and global configuration, then processes collision detection grid entries by calculating coordinates and validating ranges.
- `update_display_buffers`: Copies data from a lookup table to two display buffer locations, updates counters and flags when a condition is met, and resets a counter when it reaches a maximum value.
- `process_input_data`: Clears two global word variables, tests input conditions from memory tables, sets flags based on test results, and performs data rearrangement operations when certain conditions are met.
- `configure_dma_interrupts_3`: Updates DMA and interrupt configurations for different hardware channels by writing control values to custom registers based on parameter values and conditional comparisons.
- `initialize_sprite_data_4`: Initializes sprite parameter structures by setting multiple memory locations to zero or specific values, and configures color palette entries for sprite rendering.
- `initialize_race_sequence`: Initializes race-related display elements including sprite parameters, color palettes, and pattern coordinates.
- `initialize_sprite_data_5`: Writes fixed values to multiple sprite configuration offsets in a global structure, then reads a word from a table using a value from another global variable as an index, and stores the result at a specific offset.
- `initialize_race_display_2`: Sets up display parameters for race-related text elements including position, size, and color attributes, then renders numeric characters from 1 to 4 at multiple locations on screen.
- `initialize_display_elements_2`: Sets up display parameters for multiple text elements including position, size, and color values, then writes numeric data to a buffer using a loop that processes player scores.
- `initialize_disk_access`: Sets up disk file headers and initializes data structures for file operations, including configuring file type and version fields.
- `perform_dma_transfer`: Configures bitplane DMA transfer parameters, waits for hardware readiness, performs multiple data transfers with incrementing addresses, and returns after completing all iterations.
- `setup_blitter_dma_3`: Initializes blitter registers for memory transfer operations and waits for DMA to become ready before configuring the blitter with source and destination pointers, then performs multiple blitter operations in a loop.
- `allocate_sound_buffers_2`: Allocates multiple memory buffers of specific sizes using the AllocMem library function and stores the resulting pointers in global variables.
- `set_player_animation_flags`: Sets several global variables to specific values and performs conditional jumps based on their current states.
- `update_game_state_4`: Updates game state variables including sprite positions and collision detection parameters, processes input flags, and manages animation frames.
- `initialize_graphics_data`: Sets up graphics-related global variables with specific values including a word-sized counter, a word-sized width value, and several word-sized height and position values, then returns.
- `load_and_decode_sprite_data_2`: Decodes compressed sprite data from a source buffer into a destination buffer using a lookup table, handling both literal and compressed sequences.
- `update_sprite_data_2`: Updates sprite configuration data by reading from global tables, setting up sprite parameters including position and color values, and manages sprite state flags for different sprite types.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `initialize_game_structure_2`: Sets up game data structure values including flags, counters, and memory offsets, performs bit shifting operations, and initializes timing variables based on conditional comparisons.
- `frag_lea_moveq_move_dbra_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads 0 into d0, then copies 1499 to d1, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d1, then returns.
- `configure_blitter_single`: Sets up blitter registers for a single memory transfer operation, waits for DMA readiness, configures the blitter with source and destination addresses, then waits again before returning.
- `update_dma_control`: Updates DMA control registers and manages memory pointers in a loop, performing bit manipulation operations on data values and updating memory locations with calculated results.
- `configure_interrupts`: Disables system interrupts, reads current interrupt and DMA register values, stores them in global variables, then calls several system routines to initialize hardware resources.
- `frag_movea_moveq_jsr_move_rts_2`: Short code fragment that copies 70 to a1, then loads 0 into d0, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then copies d0 to a6, then copies the memory at an offset from a6 to a0, then calls another routine (2 times in a row), then returns.
- `frag_movea_move_rts_3`: Short code fragment that copies an address to a5, then copies a fixed memory address to a fixed memory address, then copies 61156 to a fixed memory address, then copies 32800 to the memory at an offset from a5, then copies 1 to a fixed memory address, then returns.
- `increment_counter_and_check`: Increments a global counter and performs conditional checks on multiple variables, updating various global counters and modifying a value based on comparison results.
- `frag_move_rts_3`: Short code fragment that copies a fixed memory address to a fixed memory address, then copies 61324 to a fixed memory address, then returns.
- `process_joystick_input_2`: Processes joystick input by reading from CIA registers, performing bit operations to determine input states, and updating global variables with computed values based on input conditions.
- `frag_lea_move_bra_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies 0 to an indexed entry from a0, then jumps to another point in the code.
- `frag_movem_jmp_2`: Short code fragment that restores saved registers from the stack, then jumps to another point in the code.
- `initialize_graphics_system`: Allocates memory for graphics resources, initializes hardware registers including coprocessor and display registers, sets up color tables, and configures bitplane parameters for screen rendering.
- `read_joystick_positions`: Reads joystick and mouse input data from hardware registers, processes input states using bitwise operations, and updates global variables with computed movement values based on input differences.
- `lookup_table_access_2`: Performs nested table lookups using a byte value as an index to access data in two different tables, returning the final result after processing through multiple lookup operations.
- `access_lookup_table`: Performs table lookups using input data as an index, processes conditional branches based on test results, and returns a byte value from a lookup table after clearing specific flags.
- `execute_function_pointer_2`: Calculates an address offset based on input parameters, accesses function pointers from a table, and executes the function at that calculated address with specific parameter values.
- `frag_and_rts_2`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_8`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_8`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_9`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_9`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_10`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_10`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_or_and_rts_11`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then masks the memory at a4 with d3, then returns.
- `frag_and_or_rts_11`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_or_and_rts_12`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_and_or_rts_12`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_or_and_rts_13`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then masks the memory at a3 with d3, then merges d2 into the memory at a4, then returns.
- `frag_and_or_rts_13`: Short code fragment that masks the memory at a1 with d3, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `frag_or_and_rts_14`: Short code fragment that merges d2 into the memory at a1, then masks the memory at a2 with d3, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `frag_and_or_rts_14`: Short code fragment that masks the memory at a1 with d3, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `frag_or_rts_2`: Short code fragment that merges d2 into the memory at a1, then merges d2 into the memory at a2, then merges d2 into the memory at a3, then merges d2 into the memory at a4, then returns.
- `calculate_frame_timestamp`: Reads vertical position register values to generate a timestamp, stores this value in a global variable, and performs arithmetic operations to update the timestamp with a carry bit.
- `calculate_product`: Performs signed multiplication of two 16-bit values from memory locations, stores the result on the stack, and applies a sign correction based on comparison results.
- `update_colors`: Copies color values from a source buffer to multiple color registers in the hardware, updating 16 color entries with doubled values from the input data.
- `setup_dma_transfer`: Configures DMA transfer parameters for graphics rendering by setting up control words and copying data between memory locations while managing loop counters.
- `wait_for_dma_ready_3`: Waits for a specific bit in the DMA control register to clear, then performs a series of memory operations including data copying and register updates before returning.
- `wait_for_dma_ready_4`: Waits for a specific DMA status bit to clear by continuously reading a hardware register until the bit is zero, then enters an infinite loop.
- `check_condition_loop`: Decrements a counter and checks if it's negative, then enters an infinite loop if the condition is met, otherwise continues execution.
- `configure_blitter`: Sets up blitter registers with predefined values for graphics operations, copies source and destination addresses from the stack, and waits for DMA ready status before returning.
