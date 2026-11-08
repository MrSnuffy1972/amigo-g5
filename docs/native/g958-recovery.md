# Game 958 (JamesPond v2.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00958). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2094250 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 55 of 55 units identical (bytes and relocations), 160 raw-word lines |
| Functions named and described | pass, 141 of 141 named (0 author symbols), 141 of 141 described (14 by rule, 125 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 671 of 671 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2785 of 3375 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 30238 |
| code | 1634 |
| data | 1226439 |
| padding | 867 |
| unused | 835072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 337 | 30 |
| Disk.1:bootblock | 1 | 28 | 3 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |
| logged disk load from Disk.1 boot-block load | 1 | 107 | 2 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 1109 | 30 |
| logged disk load from Disk.1 | 1 | 388 | 29 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 289 | 12 |
| logged disk load from Disk.1 | 1 | 275 | 9 |
| logged disk load from Disk.1 | 1 | 22 | 1 |
| logged disk load from Disk.1 | 1 | 724 | 6 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 59 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |

## Cases handled

- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: file j1 (header 64832 bytes, blocks hold 11224): its blocks are judged as raw blocks outside the filesystem
- JamesPond_v2.0/JamesPond_v2.0.hdf: hard-disk image (AmigaDOS OFS, 2240 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree JamesPond_v2.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: the boot block reads 5120 bytes from disk offset 749056 to an address (found statically); recovered as a unit
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- 50 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Highs: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: 50 logged loads cover 0 bytes of the image
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.20)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 759-768 (bytes 388608-393728) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.73)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.52)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.00)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 858-879 (bytes 439296-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.44)
- JamesPond_v2.0/JamesPond_v2.0_hdf/JamesPond/Disk.1: blocks 883-883 (bytes 452096-452608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.90)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `program_disk_dma_transfer` (Disk.1 +an address loaded at an address (5120 bytes)): Configures the disk DMA pointer, length, and control registers, modifies the audio and disk control registers, and sets the interrupt request register before performing a search through a memory buffer for a specific 16-bit pattern. Kind: disk signature/magic number check. Pass path: the branch that sets _DAT_000029c4 = 2 and returns.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_hardware_and_shift_bits`: Writes values to a structure in memory starting at a fixed address, calls a routine via a pointer in a register, and performs a long sequence of bitwise shifts and bit-testing on a series of consecutive memory locations.
- `call_slot_routine_repeatedly`: Calls a function stored at a fixed offset in an object thirteen times in a row, once per count, using a loop.
- `accumulate_scaled_counter_and_call`: Multiplies a 16-bit field by a fixed factor, adds the product into a 32-bit counter, clears the field, then calls the function stored at a fixed offset in an object.
- `branch_on_flag_call_routine`: Tests a flag word and calls one of two routines accordingly, then returns the incoming register value unchanged.
- `process_scaled_data_loop`: Calculates a value from two global variables, enters a loop that calls a function stored at a fixed offset in an object while updating several counters and a pointer, and finally updates a global variable and clears another.
- `call_handler_with_scaled_size`: Multiplies a count by a fixed size, calls the function stored at a fixed offset in an object with the product and count, then adds the result into a global variable and clears it.
- `patch_function_table`: Iterates through a memory range from a starting address to a limit, checking for a specific 32-bit value and a 16-bit value, then overwrites them with a new word and the address of this routine.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `store_values_to_structure`: Calls another routine, stores the value from d5 into a 16-bit word at offset 100 of a structure in a6, and clears a 16-bit word at offset 102 of that structure.
- `set_dma_control_high`: Increments a value at the end of the stack and writes 34368 to the DMA control register.
- `set_dma_control_low`: Increments a value at the end of the stack and writes 33856 to the DMA control register.
- `enable_dma_channel`: Increments the stack pointer by 2, calls another routine, and writes the value 1024 to the DMA control register.
- `set_blt_address_d0`: Calls another routine and stores the 32-bit value from d0 into the blitter address register.
- `set_blt_address_a1`: Calls another routine and stores the 32-bit value from a1 into the blitter address register.
- `set_blt_address_a2`: Calls another routine and stores the 32-bit value from a2 into the blitter address register.
- `set_blit_depth_from_a1`: Calls a routine to wait for a status flag to clear, then stores the value in a1 into the blitter depth register.
- `clear_blt_mod_address`: Calls another routine and clears the 32-bit blitter modification address register.
- `clear_blt_mod_address_alt`: Calls another routine and clears the 32-bit blitter modification address register at a different offset.
- `set_blt_control`: Calls another routine and writes the value 2544 to the blitter control register.
- `store_long_via_pointer`: Calls another routine and then stores a 32-bit value from a4 into a location at offset 54 of the structure pointed to by a5.
- `call_table_routine_pair`: Calls the function stored at a fixed offset in a table and returns the two data register values unchanged.
- `call_table_routine_if_clear`: Calls the function stored at a second fixed offset in a table only when a flag word is zero, then returns the data registers unchanged.
- `decompress_bitstream_data`: Decompresses a bit-packed stream into a destination buffer by reading bits through a bit-buffer refill routine and applying variable-length decoding logic.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_right`: Shifts a longword bit buffer right by a count, loading the previous stream longword with a set top bit whenever the buffer empties.
- `set_d0_and_branch`: Sets d0 to 3 and performs a branch to another location.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a bitwise OR operation on register d0 with zero.
- `wait_until_status_clear`: Waits in two loops until a status flag in a global variable clears.

### Disk.1:bootblock

- `initialize_io_request`: Calls another routine, then enters a loop that performs 14 iterations of calling a library function and updating two values in a structure pointed to by a1, before storing a pointer in a global variable and triggering a trap.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `issue_do_io_request`: Calls the exec DoIO library function and touches the bitplane control register, according to the facts.

### logged disk load from Disk.1 boot-block load

- `shift_and_store_byte`: Loads a longword from the address pointed to by a0, stores the lowest byte from d0 into the word at the address pointed to by a5, and returns with d0 rotated right by one bit.
- `process_boot_data`: Executes a recursive routine that performs bit shifts on a value in d0 and calls another routine based on the result.

### logged disk load from Disk.1

- `update_table_pointer`: Compares a word from a global variable to 4, and if greater, sets a global variable to 1696, another global variable to 42, loads a pointer from a table into a0, and performs several register assignments before branching.
- `branch_to_blitter_logic`: Performs a branch to another location.
- `set_global_values`: Sets a global variable to 440 and returns.
- `calculate_blitter_address`: Loads a pointer from a global variable, calculates an offset using a bitwise AND and a left shift on d0, adds this to the pointer, and branches to another location.
- `program_blitter_registers`: Touches the blitter first and last word masks, modulo and control registers, and the DMA control register, and makes two calls to unnamed routines.
- `initialize_memory_tables`: Fills three tables with 32-bit values of all ones and 16-bit values of all ones in a loop, then calls process_blitter_commands.
- `configure_blitter_masks_and_dma`: Touches the blitter first and last word masks, modulo and control registers, and the DMA control register.
- `update_global_counter`: Decrements a global variable and, if it reaches zero or specific values, performs a table lookup and updates a global variable with a calculated value.
- `configure_blitter_and_dma`: Updates a global byte, manages two counters in global variables, calculates a value based on a table and a global variable, and performs multiple blitter register updates including destination and source pointers before returning.
- `iterate_table_elements`: Compares a word from a table to a value and increments a global variable in a loop, calling update_table_pointer and another routine.
- `setup_blitter_graphics_and_draw`: Calculates source and destination addresses for a blitter operation based on a string-derived index and a global structure, then configures the blitter control, mask, and size registers to perform multiple drawing operations across several memory locations.
- `parse_string_data`: Processes a string by checking for specific characters and updating a global variable based on the character values and pointer increments.
- `process_blitter_commands`: Calls another routine, copies a word from a global variable to another, and then processes a byte from a pointer to update various global variables or add values to them based on specific constant checks.
- `set_blitter_control_and_dma`: Touches the second blitter control register and the DMA control register.
- `program_blitter_registers_and_dma`: Touches the blitter masks, pointers, modulo, control and size registers and the DMA control register, and makes no calls according to the facts.
- `program_blitter_registers_and_dma_2`: Touches the blitter masks, pointers, modulo, control and size registers and the DMA control register, and makes no calls according to the facts.
- `update_blitter_masks_and_registers`: Checks two global variables to either fill a table with all ones or copy bitwise-inverted values from a table to another table, then calls another routine, touches the blitter and DMA control registers, and sets several hardware registers including the copper control and display start registers.
- `initialize_blitter_parameters`: Calls another routine, copies a pointer from a register to a new location, and sets registers D0, D1, and D3 to 10, 15, and 4 before branching to another routine.
- `calculate_bitmask_result`: Performs a bitwise AND on a 16-bit value in d0 with a mask, shifts the result right by 3, multiplies a 16-bit value in d1 by a global variable, and returns the concatenated 32-bit result.
- `configure_display_and_update_state`: Sets hardware registers for color, display start, and copper control, then executes a sequence of subroutines that update various global variables, process text messages, and perform conditional logic based on table lookups and counter values.
- `render_referenced_text_messages`: References the text strings 500,000, EXTRA!, EXIT OPEN! and HURRY UP!, and makes three calls to unnamed routines.
- `setup_and_loop_data`: Calls another routine, performs several writes to a global variable, and then enters a loop that adds 970 to a value and decrements a counter until it reaches -1.
- `reset_game_state_variables`: Clears several 16-bit and 32-bit global variables, sets one to a maximum value, and checks a specific global value.
- `process_unrecovered_leaf_routine`: Updates a global variable based on its current value by branching to different subroutines, performing specific arithmetic, or setting constant values, and then calls another routine before returning.
- `process_unrecovered_leaf_routine_2`: Nineteen-instruction routine, called once, with no calls listed in the facts.
- `call_subroutines_and_clear_word`: Calls two different routines and then clears a 16-bit word in a global variable.
- `process_conditional_logic`: Checks a byte at an offset of a global variable, sets several global values, and calls update_table_pointer, process_unrecovered_leaf_routine_2, and two unnamed routines.
- `update_global_state_and_data_2`: Checks a global variable and, if it equals thirty-four, calls another routine and resets the variable to zero.
- `update_global_state_and_data`: Checks a global variable, calls another routine, and if a specific condition is met, performs a loop that calculates a value and stores it in a table using a pointer.
- `calculate_and_store_table_values`: Calls several routines, calculates a value based on a sum of elements from a table, and then enters a loop that writes values to a table using a pointer.

### logged disk load from Disk.1

- `set_hardware_registers`: Writes 51905 to a hardware register, writes a pointer to a global variable, calls three different routines, and writes 11457 to a hardware register before returning.
- `configure_display_and_copper`: Compares a long value at a pointer against three constants, sets a pointer in a global variable based on the result, and then touches the bitplane modulo and control registers, the data fetch start and stop registers, the display window start and stop registers, the DMA control register and the copper jump strobe, and makes one call to an unnamed routine.
- `set_coprocessor_registers`: Stores the contents of a register into a coprocessor register and stores the contents of a global variable into another coprocessor register.
- `move_register_to_memory`: Stores the contents of a register into a memory location.
- `access_background_colour_register`: Copies 32 words from a source pointer to both the background colour register and another memory location using a loop.
- `run_unrecovered_tiny_leaf`: Five-instruction routine, executed in the reference run, with no calls or registers listed in the facts.
- `clear_and_branch_on_flag`: Clears a 16-bit word in a global variable, tests it, and either returns immediately or branches to another routine.
- `configure_cia_timer_interrupts`: Touches the high and low halves of CIA-A timer B, its control and interrupt control registers, the copper list one high register, the DMA control register, and the interrupt enable and request registers.
- `access_cia_interrupt_registers`: Touches the CIA-A control register A and interrupt control register, together with the interrupt request register, while performing a loop that clears a bit in a global variable.
- `restore_stack_and_copy_register_2`: Stores a long value from a register into a memory location, restores the stack pointer from a value on the stack, and then branches to another routine.
- `restore_stack_and_copy_register`: Stores a word from a register into a memory location, restores the stack pointer from a value on the stack, and copies a word from the video port to another memory location.
- `access_keyboard_serial_register`: Saves registers, reads a byte from the CIA-A interrupt register, performs bitwise operations on it, writes the result to a global variable, and if certain conditions are met, calls two routines before returning via an interrupt return.
- `access_joystick_and_fire_state`: Touches the joystick movement register, the CIA-A port register holding fire buttons and disk status, the interrupt request registers, the playfield control register and colour register 31.
- `compare_and_jump`: Compares a word at a global variable to 1, and if it is not equal, branches to another location; otherwise it performs a long jump to a specific address.
- `mask_interrupt_requests_2`: Compares a word at a memory location to 2 and either branches to another routine or jumps to a long address.
- `mask_interrupt_requests`: Reads the interrupt request register, masks bits 8 through 11, and writes the result to the interrupt mask register.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_display_state_and_registers`: Updates a global variable based on a character comparison and, if specific characters are matched, performs bitwise OR operations on various memory locations and the beam enable register.
- `push_d0_and_branch`: Pushes the current value of register d0 onto the stack, loads the constant 3840 into d0, and performs a short branch to another location.
- `access_background_colour_register_2`: Performs a loop that writes values to the background colour register based on a bitwise AND of a constant and a decrementing counter.
- `update_color_register`: Calls another routine, writes the resulting word from d0 to a hardware color register, and branches to itself.
- `update_global_longword`: Loads a longword from a global variable, shifts it left by one bit, and checks bit 23; if bit 23 is not set, it checks bit 1 and, if that is also not set, writes the modified longword back to the global variable.
- `increment_and_store_value`: Tests bit 1 of a register, increments the register and stores it in a global variable if the bit is not set, otherwise it clears a memory range using a pointer in a0 and a1.
- `process_unrecovered_leaf_routine_3`: Compares a byte in d0 against several constants and, depending on the match, clears or stores bytes in two global variables.
- `program_blitter_registers_and_dma_3`: Touches the blitter control, modulo, destination pointer and size registers and the DMA control register in a nine-instruction routine called once.
- `store_word_to_global`: Stores a 16-bit word from a global variable into another global variable and returns immediately.
- `decompress_bitstream_to_buffer`: Reads a bitstream from a source buffer, using bit-shifting and calls to bit-reader routines to extract values, and writes the resulting bytes into a destination buffer.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_right_2`: Shifts a longword bit buffer right by a count specified in d0, loading the previous longword from a pointer in a0 with a set top bit whenever the buffer empties, and repeats this process for the number of iterations specified in d1 minus one.

### logged disk load from Disk.1

- `set_audio_channel_zero_sample`: Loads a longword from a memory location into the audio channel zero location register, sets the audio channel zero length register to sixteen, and clears a 16-bit word in a global variable before returning.
- `set_audio_channel_one_sample`: Loads a longword from a memory location into the audio channel one location register, sets the audio channel one length register to sixteen, and clears a 16-bit value at a global address before returning.
- `set_audio_channel_two_sample`: Sets the audio channel two location register to a table address, sets the audio channel two length register to sixteen, and clears a 16-bit value at a global variable address before calling another routine.
- `set_audio_channel_three_sample`: Sets the audio channel three location register using a pointer from a table, sets the audio channel three length register to sixteen, and clears a 16-bit value in a global variable before returning.
- `set_audio_volume_one`: Sets the first audio volume register to one and clears the second audio volume register, then returns.
- `set_audio_volume_two`: Sets the first audio volume register to two and clears the second audio volume register, then returns.
- `set_audio_volumes_two_three`: Sets the first audio volume register to four and clears the second audio volume register, then sets the first audio volume register to eight and clears the third audio volume register, then returns.
- `clear_audio_registers`: Performs several bitwise OR operations on d0, checks a global word, and if non-zero, clears four longwords in a table and calls seven different routines before branching.
- `update_sprite_data_loop`: Checks a global variable and enters a loop that processes data from a table, updates sprite attributes and positions, and performs complex pointer arithmetic and table lookups before returning.
- `update_audio_registers`: Checks two global variables, calculates an offset based on a byte from a table, and writes a word to a destination in a table while updating a pointer in a hardware register.
- `update_audio_parameters`: Checks several global variables and, if conditions are met, calculates values from a table and writes two words to two different destination tables and updates two hardware registers.
- `call_one_unnamed_routine`: Copies data from a source in a0 to four destination areas starting at a specific address using a loop, then copies data from a specific address back to a0 using a second loop, and calls another routine.

### logged disk load from Disk.1

- `call_one_unnamed_routine_2`: Calls three different routines, pushes a word to the stack, pops it back into d0, checks a global word, and then enters a loop that calls another routine based on a byte in a global variable.
- `initialize_and_execute_main_loop`: Initializes data tables and sets a copper list pointer, then enters a nested loop structure that repeatedly calls various subroutines and updates a global variable based on the results of a table scan and a counter loop that runs up to one hundred and fifty-four times.
- `initialize_data_tables`: Performs several memory operations including copying sixteen words, initializing a global table, initializing a parameter table, clearing four blocks of 1500 words, and copying data within a loop.
- `process_unrecovered_leaf_routine_4`: Copies sixteen words from a source pointed to by a0 to a destination starting at an offset from a pointer in a1, using a loop.
- `initialize_global_table_one`: Calls another routine and then writes a series of words to a sequence of global variables starting at offset 5f6, before branching to another routine.
- `initialize_parameter_table`: Calls a library routine, writes a series of word values to a block of memory starting at a global variable, and branches to another routine.
- `process_unrecovered_leaf_routine_5`: Clears four blocks of memory, each consisting of 1500 words, using a loop that iterates 1500 times.
- `copy_data_and_loop_with_calls`: Initializes a series of global variables, copies a block of four words from one memory location to another, and enters a loop that repeatedly updates global variables, calls three different routines, and performs arithmetic on a global variable based on a value returned from another routine.
- `set_copper_list_two_pointer`: Touches the high location register of copper list two in a nineteen-instruction routine called once.

### logged disk load from Disk.1

- `generate_bit_pattern_tables`: Iterates through a loop to generate bit-manipulated patterns, storing a transformed byte into one memory location and a shifted version of that byte into another memory location until a counter reaches zero.

### logged disk load from Disk.1

- `disable_interrupts_and_call_routines`: Disables interrupts, loads two pointers into registers, calls three routines, loads a value into a register, and branches to another routine.
- `process_unrecovered_leaf_routine_6`: Eight-instruction routine, called once, with no calls listed in the facts.
- `execute_system_initialization`: Performs a sequence of operations including clearing memory, calling multiple routines, and configuring disk DMA and audio registers.
- `program_disk_dma_transfer`: Configures the disk DMA pointer, length, and control registers, modifies the audio and disk control registers, and sets the interrupt request register before performing a search through a memory buffer for a specific 16-bit pattern.
- `read_vertical_position_register`: Touches the vertical position read register in a fifteen-instruction routine called once.
- `shift_bit_stream_backwards`: Performs a series of bitwise left shifts on a sequence of 16-bit words stored in memory pointed to by a5, where the most significant bit of each word is shifted into the least significant bit of the preceding word in the sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_blt_parameters`: Calls a routine, sets two hardware registers to all ones, calculates a value based on a global variable and d0, clears a hardware register, sets another hardware register, runs a loop using a decrementing counter, and calls another routine.
- `copy_to_cop2lch`: Calls a routine and then copies a 32-bit value from a global variable to a hardware register.
- `clear_interrupt_bit`: Calls a routine, clears bit 6 of a hardware register, and writes the value 8 to another hardware register.
- `setup_blt_registers`: Calls a routine, writes 2304 to a hardware register, clears another hardware register, and calls another routine.
- `call_external_subroutine`: Calls an external subroutine located at a fixed address.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_cop2lch_address`: Loads an address into a0 from a table, copies it to a hardware register, and returns.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_branch_sequence`: Returns immediately, then sets the processor status register, loads two addresses into a0 and a1, and calls three different routines before branching to another location.
- `write_hardware_registers`: Moves four consecutive words from the address pointed to by a1 into hardware registers at offsets 27a6, 27a8, 27aa, and 27ac, then calls three other routines before returning.
- `execute_branch_sequence_2`: Calls several routines, sets a global variable to an address, and performs a sequence of calls including execute_branch_sequence, write_hardware_registers, and clear_and_branch.
- `clear_and_branch`: Clears three 16-bit words at specific locations, calls two unnamed routines, sets a 16-bit word to zero, and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
