# Game 2391 (Mercenary v1.2 0955): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02391). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (855183 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 120 raw-word lines |
| Functions named and described | pass, 200 of 200 named (0 author symbols), 200 of 200 described (27 by rule, 173 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1101 of 1101 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 27392 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3417 of 3419 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 26710 |
| code | 33894 |
| data | 723898 |
| padding | 25 |
| unused | 70656 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 209 | 12 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3147 | 168 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 19 |
| Disk.1 bytes 20224-20480 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 24832-25344 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 33792-46336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 198400-198656 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 203008-203520 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 206336-206592 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 206848-207104 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 210432-210688 (code never loaded by a reference run) | 1 | 27 | 1 |
| Disk.1 bytes 211968-224512 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: raw disk image (362496 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Mercenary_v1.2_0955/Mercenary_v1.2_0955.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Mercenary_v1.2_0955_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (play-rs-m1500): 930 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 20224-20480 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 24832-25344 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 33792-46336 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 198400-198656 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 203008-203520 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 206336-206592 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 206848-207104 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 210432-210688 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: bytes 211968-224512 look like code no run loaded: recovered position-independently (load address unknown)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: 10 logged loads cover 33536 bytes of the image
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: 130 track-loaded chunks found in recovered runtime segments and classified through them
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 24-44 (bytes 6144-11520) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.70)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 48-50 (bytes 12288-13056) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.94)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 58-60 (bytes 14848-15616) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.18)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 95-95 (bytes 24320-24576) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.20)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 198-335 (bytes 50688-86016) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.12)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 368-384 (bytes 94208-98560) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.73)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 400-504 (bytes 102400-129280) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.36)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 528-542 (bytes 135168-139008) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.63)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 544-557 (bytes 139264-142848) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.57)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 560-577 (bytes 143360-147968) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.02)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 583-585 (bytes 149248-150016) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 588-588 (bytes 150528-150784) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.22)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 592-598 (bytes 151552-153344) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 602-634 (bytes 154112-162560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.19)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 640-667 (bytes 163840-171008) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.75)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 688-740 (bytes 176128-189696) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.76)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 744-746 (bytes 190464-191232) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.95)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 754-757 (bytes 193024-194048) outside the filesystem hold data, not code (opcode density 0.018, entropy 3.46)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 791-791 (bytes 202496-202752) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.20)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 894-1031 (bytes 228864-264192) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.12)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1064-1080 (bytes 272384-276736) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.73)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1096-1200 (bytes 280576-307456) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.38)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1224-1238 (bytes 313344-317184) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.63)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1240-1253 (bytes 317440-321024) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.53)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1256-1273 (bytes 321536-326144) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.02)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1279-1281 (bytes 327424-328192) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1284-1284 (bytes 328704-328960) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.22)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1288-1294 (bytes 329728-331520) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1298-1330 (bytes 332288-340736) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.20)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1336-1363 (bytes 342016-349184) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.76)
- Mercenary_v1.2_0955/Mercenary_v1.2_0955_hdf/Mercenary/Disk.1: chunks 1384-1415 (bytes 354304-362496) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.45)
- 8 never-loaded region(s), 27136 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_game_state` (runtime segment an address (play-rs-m1500)): Calls several routines, toggles a bit in a hardware register, and loops while a comparison between a0 and a1 is not met, finally returning a value from a global variable. Kind: checksum/data-dependent trap. Pass path: the loop continues if the XORed CIAB_PRB value and the manual decrement loop do not cause a branch or exit condition failure, specifically looking for the loop to terminate via the extraout_A0 < extraout_A1 condition.
- `initialize_audio_hardware_2_3` (runtime segment an address (play-rs-m1500)): Writes several constants to custom hardware registers, calls another routine, and then writes further constants to custom hardware registers before returning. Kind: disk hardware/track/sync check. Pass path: The routine configures specific hardware registers (ADKCON, DMACON, DSKPTH, DSKSYNC, DSKLEN) to a fixed state and then calls a subsequent routine (u2__sub_502E) to validate if the hardware/track/sync or checksums match this hardcoded config.
- `configure_dma_and_vblank` (runtime segment an address (play-rs-m1500)): Sets several custom hardware registers, including DMA controller and dsk registers, and calls a routine that waits for vblank. Kind: disk hardware/track check. Pass path: the routine sets specific hardware registers (ADKCON, DMACON, DSKPTH, DSKLEN) to a fixed state, which is characteristic of a check for a specific disk/track or a way to configure the drive for a specific read, rather than a generic load..
- `clear_ciab_bits` (runtime segment an address (play-rs-m1500)): Sets a byte in the CIAB_PRB register, clears several bits, sets bit 2, and calls another routine. Kind: bit-masking/sabotage path. Pass path: u2__sub_523C().
- `set_ciab_portb_bits` (runtime segment an address (play-rs-m1500)): Sets or clears certain bits in the CIAB_PRB register in a specific sequence and then returns. Kind: bit-masking/side-effect check. Pass path: the routine sets specific bits in CIAB_PRB, likely a way to force a known state or check if certain lines (like a certain track or certain sensor/bit) are active or inactive, acting as a way to verify hardware or a specific disk-read state..
- `set_ciab_portb_mask` (runtime segment an address (play-rs-m1500)): Modifies the CIAB_PRB register by setting bit 7, clearing bit 3, and then setting bit 3, then returns. Kind: bit manipulation/masking. Pass path: CIAB_PRB bit 3 and 7 set, bit 3 cleared/set, bit 3 cleared/set.
- `clear_cohort_status` (runtime segment an address (play-rs-m1500)): Clears a bit in the CIAB_PRB register and then sets a different bit in the same register, then returns. Kind: bit-masking/sabotage path. Pass path: the routine modifies a hardware-related register (CIAB_PRB) via bitwise operations, a common way to set a 'valid' or 'checked' flag or to disable certain hardware-level checks/interrupts after a successful read or a specific track-check pas.
- `poll_joystick_button` (runtime segment an address (play-rs-m1500)): Clears bit 1 and bit 0 of the CIAB_PRB register, calls a routine, sets bit 0, calls another routine, and increments a global variable. Kind: manual/button check or sabotage path. Pass path: the code modifies CIAB_PRB (a hardware register or memory-mapped I/O or a dummy variable acting as a button/status flag) and then calls a routine (sub_5226) which likely checks if a specific bit or value was set, potentially acting as a way.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_interrupt_mask_and_dma`: Disables interrupts via the certain hardware register and configures a DMA-based memory transfer.
- `execute_indirect_call`: Executes a function call using a pointer retrieved from a table.
- `modify_memory_via_offset`: writes a constant value to a memory location calculated by adding an offset to a register-based pointer.
- `set_global_value`: Sets a global variable to a constant value.
- `set_interrupt_register`: Sets a global variable to a fixed value and modifies the interrupt request register.
- `set_global_constant`: sets a global variable to a constant value.
- `check_status_and_call`: Checks a bit in a global variable and, if certain conditions involving a hardware register and a loop are met, calls a function via a pointer from a table.
- `toggle_bits_from_table`: Sets a global variable to zero, calls another routine, calls a function from a table, and then performs multiple bitwise XOR operations on a stack value and a pointer-based memory location.
- `toggle_bits_after_call`: Calls another routine, calls a function from a table, and then performs multiple bitwise XOR operations on a stack value and a pointer-based memory location.
- `update_ascii_char`: Increments a global variable, wraps it if it exceeds a certain value, and stores the result as an ASCII character in a global variable.
- `copy_and_configure_dma`: Copies data from a global source to a destination, performs a block-based memory copy, clears a hardware register, and configures a DMA-related register with a bit-shifted value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_and_initialize_tables`: Copies blocks of data between high memory locations and populates several tables with specific values.
- `perform_division_and_set_bits`: Performs a 16-bit division using a value from a pointer in a4 and d0, then performs a long sequence of bitwise OR operations on d7.
- `initialize_data_table_2`: Populates a series of memory locations with values and pointers extracted from a data block.
- `execute_infinite_loop`: Enters an infinite loop that performs no operations and never returns.
- `perform_count_loop`: Increments a local variable in a loop until it reaches a specific value.
- `initialize_memory_buffers_2`: Fills several memory areas with specific patterns, including zeroed bytes and copied word data from a source table.
- `initialize_memory_buffers`: Fills two large memory areas with repeating patterns of zeros and specific word values using a loop.
- `update_memory_with_swapped_bits`: Loops 6 times, performing bitwise OR and AND operations on memory at a0 and a1, and swapping bits in d1 and d2.
- `update_buffer_bitmasks_2`: Checks a global variable, and if certain conditions are met, clears a global variable and loops through two buffers to apply bitwise OR or AND operations to certain offsets.
- `update_buffer_bitmasks`: If a certain global variable is non-zero, it clears that variable and then loops through two memory buffers, applying bitwise OR or bitwise OR/AND operations to specific offsets, then returns.
- `check_bitmask_conditions`: Tests certain bytes in memory and branches to other routines based on the results.
- `apply_bitmask_to_buffers`: Saves registers, calls another routine while setting D1, D2, D3, and D4 to specific constants, then restores registers and returns.
- `apply_bitmask_to_buffers_alt`: Saves registers, calls another routine while setting D1, D2, D3, and D4 to specific constants, then restores registers and returns.
- `initialize_bitmask_constants`: Saves any existing register values to the stack, loads four constant bitmasks into registers d1 through d4, calls update_bitmask_table, and restores the original register values before returning.
- `update_bitmask_table_2`: Saves all d0-d4 and a0-a1 registers to the stack, initializes d1, d2, d3, and d4 with specific constants, calls another routine, and restores the registers before returning.
- `update_bitmask_table`: Iterates through two parallel memory buffers, applying bitwise OR and AND operations to specific bit-fields or bit-masks at certain offsets within each buffer-element, then increments the pointers and repeats for a fixed number of iterations before returning.
- `increment_global_with_mask`: Increments a 32-bit global variable by 256 and applies a bitwise AND mask.
- `initialize_global_values_2`: Checks a word in a global variable; if it is not zero and not negative, it shifts a value from a global variable, adds a constant, and subtracts that value from the global variable; otherwise, it clears the global variable and a certain word in a global variable before returning.
- `initialize_global_values`: Sets several global variables to specific 32-bit or 16-bit constants and clears a byte in a global variable.
- `decrement_and_update_structure_counters`: Decrements a 16-bit value at offset 9766 in a structure, and if not zero, updates several 32-bit pointers or counters by adding values from other offsets in the same structure.
- `modify_memory_and_call`: Shifts a word in a0 right by two, uses the result as a pointer to write a byte to a memory location, clears a word at an offset of that pointer, pushes a0 to the stack, calls another routine, restores a0, and branches to another instruction.
- `configure_audio_and_dma_channels`: Checks if certain pointers or values in a structure in a0 fall within a specific range, then calculates a new address or index, updates a global variable, and configures several audio or DMA registers including period, length, and channel, before calling another routine.
- `check_and_reset_status`: Tests a word at an offset of 11364 from a pointer in a0 and, if it is negative, sets a byte in a global variable to 255, clears a 16-bit value at an offset of 9766 from a0, and otherwise returns to another routine.
- `call_and_branch_with_constant`: Calls a routine and, if the zero flag is not set, loads a constant into d1 and branches to another routine.
- `set_d1_constant`: Calls another routine, then if a condition is met, loads a constant into d1 and branches to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_counter_and_update_byte`: Decrements a 16-bit counter in a global variable and, if the counter is not zero, loads a pointer from a global variable to update a byte at that pointer's location with a byte from another global variable, provided a certain byte comparison passes; otherwise, it checks the least significant bit of the decremented counter and returns or loops back to the start of the routine.
- `update_conditional_byte`: Compares a byte from a global variable with a byte from a pointer, and if certain bit conditions or sign flags are met, decrements a 16-bit global variable and branches to another routine.
- `increment_global_pointer`: Increments a 32-bit global variable and branches to another routine.
- `decrement_global_counter_conditionally`: Decrements a 16-bit global variable by one and branches to another routine if the negative flag or overflow flag or certain condition is met, otherwise it jumps to a subsequent instruction.
- `update_global_pointer_2`: Increments a global variable, then checks a certain condition involving a subtraction; if certain conditions are met, it modifies a byte in a buffer using a bitwise OR and then modifies a byte in that buffer using a bitwise AND before branching to another routine.
- `mask_and_store_byte`: Reads a byte from a global variable, masks it with a bitmask, and stores the masked value into a destination pointer held in a5.
- `setup_audio_and_clear_memory`: Checks a byte in a global variable and branches if non-zero; otherwise, it sets a pointer in a4, calls two other routines, and if a certain condition is met, clears several hardware registers and certain global variables, then sets several hardware registers and certain global variables, and finally branches to another routine.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_audio_and_dma`: If a certain global byte is non-zero, it returns; otherwise, it resets certain audio registers, configures DMA channels, and then calls another routine to perform a loop that checks a memory range for matching bytes and writes a decreasing sequence of bytes to a global table before branching to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `reset_audio_and_set_frequency`: Checks several global flags and, if certain conditions are met, resets certain audio registers, clears a global variable, and calculates a new frequency or period or a scaled value based on a pointer-offset and stores it in a global variable before calling another routine or returning.
- `calculate_and_store_scaled_value`: Performs bitwise shifts and extensions on d0 based on a value derived from d7, stores the result in a structure, and then updates d7 using a global value and d5, or clears d0 if the result is negative.
- `calculate_rotated_value`: Swaps d7, negates it, and adds a constant; if the result is not negative, it extends d0, shifts d0 left by two, and shifts d0 right by a value derived from d7, then stores the result in a structure at an offset of a0, and subsequently performs further arithmetic and stores a value to a pointer, before potentially clearing d0 and branching.
- `calculate_and_store_rotated_value`: Swaps d7, negates it, and adds a constant; if the result is positive, it extends d0, shifts d0 left by two, and shifts d0 right by the value in d7, then stores the result in a pointer offset, copies three long-words from a global pointer, and branches to another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_register_offsets`: Subtracts values from memory-referenced pointers in a0 and a4, then moves the results into a1, a2, and a3.
- `rotate_and_update_audio_registers`: Iterates a fixed number of times, performing a bitwise left rotation on a word read from a global pointer, writes the result to two audio-related registers, and updates the pointer to a new address masked with a specific bitmask before returning.
- `calculate_and_apply_bitmask`: Performs bitwise arithmetic and shifts on values retrieved from memory tables to compute a mask.
- `loop_until_bit_clears`: Repeatedly writes a word and a constant to memory locations while calling another routine, continuing as long as a specific bit in a global variable is set, then waits for that bit to clear before performing a final write and call.
- `update_audio_parameters`: Checks a global variable and, if it does not match a specific value, performs a series of calculations involving memory lookups and bitwise operations to update the global variable and hardware audio registers.
- `modify_audio_settings`: If a global variable does not match a specific value, it calculates a new value based on a memory lookup and updates the global variable and hardware registers; otherwise, it clears the global variable.
- `interpolate_and_update_table`: Calculates an interpolated value between two entries in a data table using bitwise shifts and scaling, then updates a destination table with the result.
- `modify_bitfield_and_call_5`: Toggles bits in four memory locations based on a register and performs an indirect call using a calculated pointer.
- `clear_bits_in_loop`: Clears bits in a structure at an offset from a2 and loops using d5.
- `set_bit_and_interpolate`: Sets a specific bit in a byte and updates a destination table with an interpolated value from a source table before calling another routine.
- `clear_bit_and_interpolate`: Clears a specific bit in a byte and updates a destination table with an interpolated value from a source table before calling another routine.
- `modify_bitfield_and_call_2`: Sets or clears specific bits in four different memory locations based on a register and then performs an indirect function call.
- `set_and_clear_bits_in_loop`: Sets or clears bits in a structure at offsets from a2 and loops using d5.
- `clear_bit_and_interpolate_alt`: Clears a bit in a primary byte and another in a secondary byte, then updates a destination table with an interpolated value.
- `clear_bits_and_interpolate`: Clears bits in multiple memory locations and updates a destination table with an interpolated value from a source table.
- `update_bit_and_interpolate`: Sets and clears specific bits in four different memory locations based on a bit index, then performs an interpolation calculation and executes a subroutine via a pointer.
- `set_clear_bit_and_decrement_loop`: Sets bit 4 and clears bit 6 of a byte at an offset from a pointer in a2, then decrements a loop counter and returns.
- `modify_bitfield_and_call_3`: Updates bitfields in a memory structure using a register value and executes a function via a calculated address.
- `modify_bitfield_bits`: Clears bit 8 or 24 of a byte at an offset of 8192 or 16384 from a pointer in a2, sets bit 24 of a byte at an offset of 24576 from a2, and loops using d5.
- `modify_bitfield_and_call`: Updates specific bits in a memory structure based on a register value and then performs an indirect function call using calculated data.
- `modify_bitfield_bits_alt`: Sets bit 2 or 8 of a byte at an offset of 2 or 8 from a2, clears bit 16 of a byte at an offset of 4 or 16384 from a2, sets bit 24 of a byte at an offset of 24576 from a2, and loops using d5.
- `clear_multiple_bits_and_interpolate`: Clears bits in several memory offsets and performs a table interpolation and update before calling another routine.
- `set_multiple_bits_and_interpolate`: Sets bits in multiple memory locations and updates a destination table with an interpolated value from a source table.
- `modify_bitfield_and_call_4`: Manipulates bits at specific offsets within a memory block and triggers an indirect call using data from a table.
- `set_bitfield_bits`: Sets bit 8, 16, or 24 of a byte at an offset of 8192, 16384, or 24576 from a2, and loops using d5.
- `update_bitmask_and_interpolate_table`: Sets or clears specific bits across four distinct memory blocks using a bit index, then performs a weighted interpolation between two table values to update a destination entry.
- `set_byte_bits_and_interpolate`: Sets bits at specific offsets within a byte array and performs a table interpolation and update.
- `parse_bytecode_instruction`: Reads a value from a global pointer and executes different logic based on the lower bits of that value.
- `calculate_string_length`: Iterates through a null-terminated string to calculate its length and stores a modified version of the length in a global variable.
- `store_word_to_global`: Stores a word from a register into a global variable after clearing it.
- `dereference_and_update_pointer`: Loads a value from a location pointed to by a global variable and then updates that global variable with a new pointer.
- `update_pointer_data`: Compares memory contents against a global variable and, depending on the result, stores a pointer or a word into a global variable.
- `update_pointer_data_2`: Compares a word in memory against a global variable and updates that global variable with either a pointer or a word.
- `execute_and_increment_pointer`: Calls a function through a pointer and stores the result plus four in a global variable.
- `call_and_increment_pointer`: Calls a routine at the address in a0, increments a0 by 4, and branches back to a previous instruction if a zero flag is set.
- `rotate_and_update_pointer`: Saves a0 to the stack, loads a pointer from a global variable, rotates a word, extracts a word, modifies a word with a mask and a constant, stores it back to a global variable, restores a0, and compares a word from a pointer to a word from a register.
- `initialize_audio_hardware_2_2`: Loads a byte from a pointer in a0, compares it to a byte in a global variable, and if not equal, calls another routine and then branches to another routine.
- `check_pointer_byte`: Loads a pointer from a global variable, adds a constant, and checks if the byte at that pointer is negative, otherwise it branches to a previous instruction.
- `copy_data_to_offset`: Calculates an offset based on a value in a register and copies a longword from a source to that calculated destination.
- `update_pointer_data_3`: Compares a value derived from a memory pointer against a global variable to determine how to update a global pointer structure.
- `update_bcd_values`: Calculates a pointer, updates a global pointer, and performs a series of bcd-adjusted additions or writes to bytes at offsets relative to a calculated pointer using values from a pointer in a0.
- `store_and_update_pointer`: Calculates a pointer, stores a word from a pointer in a0 into that pointer, and updates a global pointer with a new address.
- `or_and_update_pointer`: Ors a word from a pointer in a0 into a global variable and updates a global pointer with a new address.
- `and_not_and_update_pointer`: Reads a word from a pointer in a0, inverts it, ands it with a global variable, then updates a global pointer with a new address.
- `load_and_compare_word`: Loads a word from a pointer in a0, performs a bitwise AND with a constant, and branches or tail-branches to another routine based on the result.
- `compare_global_word`: Loads a word from a global variable, compares it to another global variable, and branches or tail-branches to another routine based on the result.
- `fetch_indexed_value`: Uses a value from a pointer to index into a table, potentially performing a second lookup, and stores the result in a global variable.
- `update_pointer_table`: Calculates three destination addresses based on a pointer in a0 and a base address, copies a word from a source pointer to a destination, and then performs a series of BCD-style additions using a pointer-based table before storing a new pointer in a global variable.
- `add_to_global_pointer`: Loads a word from a pointer in a0, adds it to a global variable, and updates a global pointer with the new address.
- `compare_word_with_limit`: Loads a word from a pointer in a0, compares it to a global variable, and branches or tail-branches to another routine based on the carry flag.
- `store_word_at_offset`: Stores a word from a pointer into a memory location offset by a large constant.
- `store_byte_at_offset`: Calculates a destination address using a pointer in a0 and a constant, copies a byte from a pointer in a0 to that address, and updates a global pointer.
- `modify_global_pointer_logic`: Loads a pointer from a global variable, checks a byte at an offset, and if certain conditions are met, updates a global variable and a pointer-based table with a new address or a masked value.
- `set_global_constant_2`: Sets a global variable to a constant value and updates a global pointer with the value in a0.
- `traverse_and_store_word`: Compares a byte from a table, and if not equal, calculates a series of destination addresses using a pointer in a0 and stores a word from a pointer in a0 to a calculated address before updating a global pointer.
- `apply_mask_to_table`: Compares a word from a global variable, calculates a destination address in a table, and if a byte from a pointer in a0 is not negative, or-masks that byte into the table before updating a global pointer.
- `tiny_and_rts`: Small routine of 2 instructions that masks (a1) with d0, then returns.
- `update_audio_parameters_2`: Calculates new period and volume values based on a global state and table lookups, then updates hardware registers for audio and color.
- `configure_hardware_registers`: Updates various hardware registers including audio and DMA settings based on values derived from a global variable.
- `modify_and_store_address`: Checks a global variable and either adds a constant to it or simply stores its address into another global variable.
- `initialize_audio_hardware`: Writes a series of constant values to various audio, DMA, and interrupt control hardware registers.
- `update_audio_and_color`: Performs complex bitwise manipulations on a global variable and updates various audio and color hardware registers.
- `update_audio_period_and_registers`: Updates the audio period register and a custom hardware register based on values derived from a global variable and a lookup table.
- `configure_audio_hardware`: Updates audio period and custom hardware registers based on a global variable and a lookup table.
- `copy_table_bytes`: Calculates an offset from a table using a register and copies five bytes from that table to a destination buffer.
- `copy_bytes_from_table`: Copies several individual bytes from a table at an offset determined by a register into a destination buffer.
- `branch_copy_data`: Checks the sign of a register to decide whether to copy five XORed bytes or five raw bytes from a table to a destination.
- `xor_table_bytes`: Calculates an offset from a table and copies five bytes to a destination, XORing each byte with a constant value.
- `copy_data_from_table`: Calculates an offset from a register and copies five bytes from a table to a destination pointed to by another register.
- `update_memory_from_index`: Calculates a new index based on a global variable, updates that variable, and copies several words from a source table to a destination table using the new index.
- `update_sprite_attribute_tables`: Calculates an index based on a global variable and performs a bulk transfer of word-sized data from a source table to two distinct destination memory regions.
- `count_matching_bytes`: Saves registers, then loops through a memory range to check if bytes match a value from a global variable, incrementing a counter and writing a value to a global variable if a match is found, before restoring registers and calling another routine.
- `compare_three_values`: Compares the absolute value of a value in a1 against d1, then compares the absolute value of a2 against d1, and finally compares the absolute value of a3 against d1, returning if any absolute value is less than d1.
- `update_global_pointer`: Sets bit 6 of a global variable, then if that bit was not set, it reads a word from a table using d1, updates a global pointer to point to a new location, and stores the previous pointer value and the new word into a global structure.
- `set_global_value_2`: Checks if a global variable is zero, and if so, loads a word from a table at an offset determined by d1 into that global variable and sets the global variable to one.
- `initialize_audio_hardware_2`: Sets several audio and DMA registers, including AUD1PER, AUD0LCH, AUD0LEN, AUD0VOL, AUD1LCH, AUD1LEN, and DMACON, and clears a global variable if a certain global or register check passes.
- `push_and_loop`: Pushes the value in d0 onto the stack and then enters an infinite loop.
- `calculate_range_limit`: Calculates a new range or index limit based on a register value and a global variable, then updates a global word with a bit-rotated or transformed value.
- `calculate_table_index_range`: Calculates a new index or range limit based on a register value and a global variable, then updates a global word with a bit-rotated or transformed value from a table.
- `set_global_word`: Compares d1 to a constant and branches if less; otherwise, it clears a word in a global table and branches to another instruction.
- `update_table_with_rotated_data`: Calculates a table index based on a register value, reads a word from a table, and pushes several bit-rotated versions of that word onto the stack.
- `encode_and_push_bit_rotated_data`: Calculates an offset into a table, reads a word, and pushes several bit-rotated versions of that word onto the stack.
- `set_player_score_2`: Decrements d5 and d4, then enters a nested loop that calls another routine, clears a range of memory, and modifies certain bytes in a table based on values from a global variable; if a certain condition is met, it updates a global word and a global long pointer before returning.
- `set_player_score`: If d1 is non-zero, it walks forward through a short table starting at d7 until it finds a matching entry or reaches the end pointer a4; if a match is found it returns without writing; otherwise it stores the d1 value at the location pointed to by a4.
- `expand_and_store_vectors_data`: Reads a configuration or size-related value from a table, then uses it to populate a destination array with a sequence of transformed or bit-manipulated 32-bit values.
- `expand_and_store_vectors_2`: Calculates a bit-shifted or bit-rotated bitmask or value based on a source byte and stores the resulting 32-bit word into a destination array, while also performing bit-shifts and sign-extensions on data read from a source buffer and storing the results into a secondary destination array.
- `expand_and_store_vectors`: Calculates a bit-shifted or bit-rotated bitmask or value based on a source byte and stores the resulting 32-bit word into a destination array, then checks a global variable to decide whether to continue or terminate.
- `unpack_and_store_vectors`: Loops through a source buffer, extracting or transforming bytes into 32-bit words and storing them in a destination array, then checks a global byte and potentially calls another routine.
- `loop_on_subroutine_result`: Calls a routine and, if the zero flag is not set, repeats the call and then returns.
- `update_pointer_data_6`: Loads a pointer from a global variable, decrements a long-word at an offset of that pointer, and if the result is negative, clears that long-word.
- `update_pointer_data_5`: Decrements a register, adds a constant to another, writes the result to an offset of a pointer in a0, swaps a register, and then checks a value to potentially update a pointer or modify a register and write it back to an offset of a pointer in a0.
- `update_pointer_data_4`: Decrements d0, adds a constant to d1, writes d1 to an offset of a pointer in a0, swaps d0, writes d0 to an offset of a pointer in a0, decrements a0, and then checks if d0 is positive before returning.
- `set_global_value_2_2`: Checks if a certain word in a global variable is zero; if so, it calls another routine and then checks if a certain word in a global variable is equal to a constant before returning.
- `initialize_game_state`: Calls several routines, toggles a bit in a hardware register, and loops while a comparison between a0 and a1 is not met, finally returning a value from a global variable.
- `transform_and_store_word_2`: Fills a memory range with a constant, copies several longwords from a source to a destination, calls another routine multiple times with different byte values, and then enters a loop that adds a byte from a pointer to d7, rotates d7, and calls another routine before finally performing a bit shift and calling another routine before returning.
- `populate_bit_table`: Fills a table starting at a certain address with a sequence of words, where each word contains a bit-reversed or bit-shifted representation of the current loop index.
- `transform_and_store_word`: Reads a word from a table using d1 as an index, then performs bitwise operations involving d2 and d3, and stores the resulting word at the location pointed to by a4.
- `process_bit_rotation`: Loads a pointer, then loops to perform a series of bit-shifts and bit-rotations on a value read from memory, storing the result back to memory, before checking a global variable and potentially performing further bit-shifts in a loop.
- `update_global_pointer_3`: Compares a byte from a pointer in a4 to a byte in a global variable; if they match, it enters a loop that calculates a bit-rotated value using a byte from a pointer and a running sum, then checks if a certain value matches a byte from a pointer before writing a constant to a global word and returning.
- `initialize_audio_hardware_2_3`: Writes several constants to custom hardware registers, calls another routine, and then writes further constants to custom hardware registers before returning.
- `configure_dma_and_vblank`: Sets several custom hardware registers, including DMA controller and dsk registers, and calls a routine that waits for vblank.
- `wait_for_vblank_loop`: Loads a large constant into a counter and repeatedly decrements it while checking if bit 1 of a custom register is set; if the bit is set, the routine returns early, otherwise it continues until the counter reaches zero and then returns.
- `wait_for_interrupt_bit`: Loads a large constant into d0 and repeatedly decrements it while checking if bit 4 of a custom interrupt register is set; if the bit is set, the routine returns early, otherwise it continues until the counter reaches zero and then returns.
- `clear_ciab_bits`: Sets a byte in the CIAB_PRB register, clears several bits, sets bit 2, and calls another routine.
- `set_ciab_portb_bits`: Sets or clears certain bits in the CIAB_PRB register in a specific sequence and then returns.
- `set_ciab_portb_and_update`: Writes a constant byte to the CIAB_PRB register, clears several bits, sets bit 2, and calls another routine.
- `set_ciab_portb_mask`: Modifies the CIAB_PRB register by setting bit 7, clearing bit 3, and then setting bit 3, then returns.
- `set_ciab_prb_and_init`: Writes a constant byte to the CIAB_PRB register, clears several bits, sets bit 2, and calls another routine that waits for a button release.
- `clear_cohort_status`: Clears a bit in the CIAB_PRB register and then sets a different bit in the same register, then returns.
- `check_hardware_bit`: Tests bit 4 of a hardware register and, if not set, calls another routine and loops back to the start of this routine.
- `clear_global_counter`: Clears a 32-bit global variable and jumps to another routine.
- `compare_and_branch`: Loads a word from a global variable into d0, then branches if d0 is zero, calls two other routines, and branches if d0 still equals the loaded word.
- `set_ciab_portb_bits_2`: Sets bit 1 and clears bit 0 of the CIAB_PRB register, calls a routine, sets bit 0, calls another routine, and decrements a global variable.
- `poll_joystick_button`: Clears bit 1 and bit 0 of the CIAB_PRB register, calls a routine, sets bit 0, calls another routine, and increments a global variable.
- `wait_game_frames`: The routine contains a decrementing loop that counts down from a constant value to zero and then returns the value originally in D0 without modifying it.
- `wait_for_button_release`: Loops until bit 5 of the CIAA_PRA register is cleared, then returns.
- `initialize_and_update_data`: Calls three other routines, then checks if d0 is equal to a constant; if not, it decrements d0 and checks if it is less than a constant, potentially writing d0 to a global array before calling three more routines and finally jumping to an indirect address.
- `set_register_value`: Loads a constant into a register and calls another routine.
- `update_array_from_pointers`: Calls three other routines, then checks if d0 equals a constant; if not, it decrements d0 and checks if it is less than a constant, potentially writing d0 to a global array before calling three more routines and then copying four long-words from one memory location to another before jumping to an indirect address.
- `process_bit_manipulation_loop`: Checks a bit in a global variable, and if set, executes a loop that rotates a byte, inverts it, and writes a new value to a global variable, then returns.
- `process_bit_and_loop`: Checks a bit in a global word, and if certain conditions are met, enters a loop that modifies a byte from a global address and repeats until a counter reaches zero.
- `clear_flag_and_transform`: Checks a bit in a global variable, and if set, performs a loop to decrement a value before clearing the global variable and applying a bitwise transformation.
- `reset_global_variable`: Sets a global variable to zero.
- `update_global_flags`: Writes a word to a global address, reads a byte from a global address, and if that byte is not zero, clears a global byte and sets a bit in another global word before branching.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `compare_d0_value`: Compares a byte in d0 to a constant and branches if it matches or if it matches another constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### Disk.1 bytes 210432-210688 (code never loaded by a reference run)

- `update_global_value`: Performs bitwise operations on a byte through a pointer and updates a global variable based on comparison logic.
