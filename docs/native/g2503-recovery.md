# Game 2503 (deliverance (1992)(21st century)(disk 1 of 3)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02503). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906864 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 113 raw-word lines |
| Functions named and described | pass, 188 of 188 named (0 author symbols), 188 of 188 described (6 by rule, 56 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1080 of 1080 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4272 of 4272 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6592 |
| code | 5334 |
| data | 885788 |
| padding | 6078 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance | 2 | 267 | 8 |
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:FairLight | 2 | 267 | 8 |
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 123 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 263 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 417 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 304 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 5 |
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance#unpacked | 1 | 2170 | 87 |
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:FairLight#unpacked | 2 | 381 | 28 |

## Cases handled

- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_10: high-entropy data file (7.98 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_11: high-entropy data file (7.96 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_13: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_14: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_17: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_18: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_19: high-entropy data file (7.95 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_21: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_22: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_00: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_01: high-entropy data file (7.95 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_02: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_05: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_06: high-entropy data file (7.99 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_07: high-entropy data file (7.94 bits/byte), classified as packed data
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:DE_09: high-entropy data file (7.99 bits/byte), classified as packed data
- runtime segment an address: 91 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 252 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 108 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 147 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 18196 -> 32024 bytes), 13 of 14 windows confirmed in reference RAM: recovered as a hunk executable
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:FairLight: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 20056 -> 49392 bytes), 11 of 12 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 branch displacement
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_copy_data` (runtime segment an address): Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range. Kind: disk hardware/track check. Pass path: the branch at an address (after bne.b).
- `read_drive_status_bits` (runtime segment an address): Polls a CIA status register for 32 samples, accumulating the bits into a mask, and records a result byte; it repeats until the returned code equals one. Kind: Disk hardware status check (CIAB_PRB read via read_drive_status_bits). Pass path: The routine reads the drive status bits (CIAB_PRB) in a loop, then u6__sub_0368() returns a value; the check passes when (short)uVar1 == 1, which exits the do/while loop and stores bVar2 / 1 at (unaff_A4 + 8)..
- `configure_hardware_and_copy_data_2` (FairLight#unpacked): Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range. Kind: disk hardware check. Pass path: The routine configures hardware and performs operations on CIAB_PRB, likely checking for specific hardware state or presence of a disk. The call to `u9__sub_0_06AC()` and the setup of custom registers suggest this is part of a disk verifica.
- `initialize_dma_and_audio_channels` (FairLight#unpacked): Writes the DMA and interrupt control registers, the CIA B port registers and the audio-channel registers, then waits for the vertical beam position counter to reach a threshold. Kind: disk hardware check. Pass path: The routine initializes DMA and audio channels, manipulates CIAB_PRB (port B) to set up hardware state, and configures timers and audio DMA channels. If the check passes, it continues execution normally without branching to a sabotage path .

## Functions


### Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance

- `load_amiga_hunk_file`: Parses AmigaDOS hunk records for code, data, uninitialised data and 32-bit relocation entries, allocating memory for each section through exec, copying its contents, applying relocation offsets and freeing the scratch buffer.
- `spin_delay_counter`: Busy-waits by decrementing a 16-bit counter until it passes zero, with no other calls.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrunch_backward_bitstream`: Unpacks a compressed block by reading bits from a stream, copying literal bytes and back-references backwards from the end of the output buffer until the output pointer reaches the start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_multiword_bit_buffer`: Shifts a multiword bit buffer across data registers using the extend flag, pulling bits from the preceding longword as needed, for a number of steps held in a counter.
- `raise_trap_invalidate_cache`: Raises a trap with exception number 6 and then invalidates a range of cache lines starting at a given address.

### Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:FairLight

- `decrunch_and_relocate`: Processes a compressed bitstream, unpacking literal bytes and back-references into a buffer while tracking positions and sizes.
- `decrement_register_loop`: Counts down a register from a starting value to zero using a loop that decrements it each iteration.
- `empty_return_routine`: An empty routine that returns immediately without performing any operation.
- `copy_bytes_backward_loop`: Reads a length value, branches when it is non-zero, and then copies blocks of bytes backward through a pointer while tracking a flag.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `shift_multiword_bit_buffer_2`: Shifts a multiword bit buffer across data registers using the extend flag, pulling bits from the preceding longword as needed for a given count.
- `consume_bits_from_buffer`: Consumes a number of bits from a multiword bit buffer and packs them into an accumulator register, refilling the buffer from a preceding longword when it runs empty.
- `install_bitstream_trap`: Executes a trap instruction that is used as a jump vector by the caller's bitstream decoder.

### Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_then_trap_zero`: Calls another routine, stores a fixed constant into a global variable, then executes trap zero and returns zero.
- `wait_for_hardware_flag`: It polls a bit in a global variable until it becomes set, then executes a sequence of other routines and loops.
- `configure_hardware_and_copy_data`: Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range.
- `update_memory_values`: Writes several specific values to a memory block pointed to by a register.
- `check_and_increment_byte`: Checks a byte at an offset; if it is negative one, it increments a global variable and loops until the byte is zero.
- `check_byte_and_decrement_memory`: Checks a byte at an offset and, if it is not negative one, decrements a word at a memory location relative to the program counter.
- `clear_memory_range`: Fills a range of memory starting at a specific address with zeros using a loop.
- `copy_data_from_table`: Reads a sequence of bytes from a global variable to determine how many bytes to copy from a table into a destination memory range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores a pointer to the current routine in a global variable and executes a trap instruction.
- `poll_a5_state_bytes`: Sets a state byte and a counter word in the structure addressed by the A5 register, polls that byte through nested loops with other routine calls between polls, and runs a countdown of 250 steps before a second polling phase.
- `toggle_flag_and_update_table`: Toggles a flag word; when it becomes zero, it advances a counter by two and writes a shared value into four table entries spaced sixteen bytes apart.
- `toggle_flag_and_fill_buffer`: Toggles a flag word; when it becomes zero and a countdown word stays non-negative after subtracting two, it writes a shared value into four table entries spaced sixteen bytes apart.
- `unpack_nibble_table_words`: Clears a 32-entry region through a helper call, then expands each of 32 source words into four stored values, each holding the negated low nibble, plus masked low, middle and high nibble fields.
- `find_max_nibble_value`: Scans the four nibbles of a 16-bit word and keeps the largest one in an accumulator register.
- `update_countdown_timer_table`: Decrements a countdown; when it reaches zero it resets it to four and walks a table of 32 six-word entries, decrementing active timers and, for idle entries, comparing a bit mask against a control word and updating it.
- `decrement_nibble_table_entries`: Decrements a countdown; when it reaches zero it resets it to four and steps through 32 words, subtracting one from each non-zero high, middle and low nibble.
- `initialize_dma_and_copper`: Saves the custom chip state, disables interrupts and DMA, configures the CIA control registers, polls the raster position until a set line is reached, writes audio channel setup entries, and re-enables DMA.
- `build_incrementing_word_table`: Fills a table with incrementing word values and the two halves of an address that advances by forty bytes per step, then fills eighty word pairs with incrementing values and zeros, and stores the table pointer and final value in a structure.
- `restore_dma_and_interrupt_state`: Disables DMA and interrupt enables through the custom registers, restores saved values from globals, sets two global flags, then re-enables DMA by writing a fixed value.
- `store_pointer_and_word_fields`: Copies a longword from one structure into a pointer field of another and stores a word value in a second field.
- `set_intreq_values`: Writes two constant byte values to global variables that stand in for the interrupt request register.
- `set_cia_intreq_bits`: Writes two constant values to global variables that stand in for the CIA and interrupt request registers, then returns.
- `stub_unrecovered_short_routine`: Decompilation timed out, and the listed facts give only a five-instruction size and one caller, so no behaviour is recovered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_then_loop_forever`: Disables interrupts and DMA, installs a trap handler, clears a low-memory table, waits for an interrupt request bit, runs a long sequence of helper calls that configure the hardware, then loops forever.
- `initialize_hardware_setup_sequence`: Clears a low-memory table, runs a trap, waits for an interrupt request bit, then makes a long sequence of helper calls that configure the hardware before looping forever.
- `clear_global_state`: Writes zero to a global variable five times and returns.
- `set_flag_and_table_byte`: Writes a constant byte to a global variable, then stores a byte derived from its sign into a table at an offset given by the low seven bits of that variable.
- `decode_joystick_port_movement`: Reads the joystick port movement register into a global, converts its bits with a Gray-code style step and a sign-extended shift and mask sequence, and stores the resulting direction value.
- `read_joy0dat_and_accumulate`: Reads the mouse port movement register, splits it into low and high bytes, passes each through a helper, accumulates the results into a global, and returns the sum in both halves of a 64-bit value.
- `wrap_short_difference`: Subtracts one short value from another and wraps a difference outside the range around plus or minus 126 by adding or subtracting 256, then returns the wrapped value.
- `read_keyboard_serial_code`: When the serial data interrupt flag is clear, reads a keyboard serial byte, decodes it through a table into state bits, and acknowledges the interrupt; otherwise it calls a handler through a stored pointer.
- `loop_forever_stub`: Loops forever without doing anything, so it is an idle loop rather than a recovered function.
- `stub_infinite_idle_loop`: The recovered listing is only an infinite do-nothing loop, even though the listed facts show 244 instructions that touch the colour, interrupt and raster registers, so no further body is recovered.
- `clear_zero_longword_block`: Writes zero to a block of 186 longwords starting at a fixed low-memory address, one entry at a time.
- `configure_blitter_registers`: Writes specific values to several hardware registers at fixed offsets of the custom hardware base, configuring blitter operation parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_no_op_stub`: Returns without doing anything, so it is a no-op stub.
- `format_digits_and_poll_ports`: Formats a number from a global variable as two decimal digits in a text buffer, then runs a polling loop over CIA-B port bits and small helper routines, comparing text against a stored string and copying a record into a buffer.
- `modify_byte_flags`: Reads a byte from a memory location, sets its high bits, clears a subset of them, and sets another group of bits in sequence.
- `noop_four_instructions`: This routine contains four instructions that execute and then return without performing any observable action on memory, registers, or hardware.
- `read_and_verify_disk_sector`: Sets up a disk read through the custom chip registers, scans the raw track buffer for the MFM sync word, copies the sector data and checks it against a checksum made from alternate bits, returning when the check passes.
- `return_immediately`: It executes five instructions and then returns without modifying any data or calling anything.
- `step_drive_head_to_track`: Compares a stored track position with a new one and steps the disk head that many times in the matching direction, updating the drive control flags before and after the steps.
- `select_drive_and_wait_ready`: Clears and sets drive select bits in a CIA port register, writes values to CIA timer and control registers, then waits until the status bits for drive ready and index settle.
- `start_drive_motor_and_count`: Clears and sets control bits on a drive port, increments a step counter, writes control values to CIA registers, and waits until a status bit clears.
- `write_drive_control_and_wait`: Writes zero and then the input byte to CIA port registers at fixed offsets, sets a control register to a fixed value, and waits until its busy bit clears.
- `read_drive_status_bits`: Polls a CIA status register for 32 samples, accumulating the bits into a mask, and records a result byte; it repeats until the returned code equals one.
- `update_drive_port_bits`: Clears one bit of a port byte selected by a register value, sets the low three bits, then clears a second bit chosen from the same value plus three.
- `set_drive_port_bits`: Sets three bits of a port byte and clears one bit chosen from a register value plus three.
- `set_drive_status_bits`: Sets the bit selected by a register value in a byte twice, calls another routine, then sets three low bits in a second byte.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 120 into (a0), then returns.
- `stub_unrecovered_fifteen_instructions`: Decompilation timed out, and the listed facts give only a fifteen-instruction size and one call into another routine, so no behaviour is recovered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_colour_and_dma_registers`: Decompilation timed out; the listed facts show it writes the background colour register and the DMA and interrupt enable registers, with one call into another routine.
- `configure_beam_control_register`: Decompilation timed out; the listed facts show it writes the beam control register and makes one call into another routine.
- `stub_unrecovered_seven_instructions`: Decompilation timed out, and the listed facts give only a seven-instruction size and one call into another routine, so no behaviour is recovered.
- `stub_unrecovered_four_instructions`: Decompilation timed out, and the listed facts give only a four-instruction size and one call into another routine, so no behaviour is recovered.
- `stub_unrecovered_seven_instructions_2`: Decompilation timed out, and the listed facts give only a seven-instruction size and one caller, so no behaviour is recovered.

### Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance#unpacked

- `calculate_and_set_register`: Performs bitwise operations on values from a table, and if certain conditions are met, writes a value to a hardware register and calls another routine.
- `copy_table_and_jump`: Disables interrupts, clears a memory location using a loop, checks a byte at an offset from a pointer, and if not equal to 50, copies 8192 words from a source to a destination before jumping to a location at offset 256.
- `copy_table_entries_to_memory`: Copies 7778 longwords from a table to a destination memory location pointed to by a4, then sets d4 to the value of a4 and branches to another routine.
- `update_table_entry`: Adds a value from a register to an element in a table and reads the next value from a buffer into the register, then branches back if the value is positive.
- `update_table_values_2`: Processes a loop of data updates in a table, adding values from registers and performing arithmetic operations on array elements.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `read_joy0dat_and_accumulate_2`: Reads the mouse port movement register, splits it into low and high bytes, passes each through a helper routine, accumulates the results into a global variable, and returns the sum in both halves of a 64-bit value.
- `normalize_word_value`: Subtracts a word value from another word register, branches if positive, compares the result with negative 127, branches if greater, adds 256 to the result before branching.
- `validate_range`: Compares a word value with 127 and branches if less than, then subtracts 256 from the value before returning.
- `execute_function_pointer_2`: Sets a word value in memory, checks if a global variable contains a positive value, and calls the function pointed to by that variable if it.
- `update_global_byte`: Reads a byte from a table using a register value as an index, combines it with another byte through bitwise operations, and stores the result in a global variable.
- `execute_function_pointer`: Sets a word value in memory, checks if a global variable contains a positive value, and calls the function pointed to by that variable if it does.
- `clear_interrupts_and_loop`: Clears all interrupt requests and enables all interrupts, then enters an infinite loop that continuously writes a fixed value to a color register.
- `clear_interrupts`: Writes a word value to multiple hardware registers related to interrupts and color settings, then enters an infinite loop.
- `process_bit_operations`: Performs bitwise AND and OR operations on memory locations based on shifting and conditional logic derived from data in global tables.
- `update_counter_and_process_data`: Updates a counter in a global variable, performs bit operations on memory locations, and processes data from tables with conditional logic.
- `update_and_process_data`: Updates a counter, performs bit operations on memory locations, and processes data from global tables with conditional logic.
- `decrement_and_process`: Increments a counter, decrements values in global data structures, and performs bit operations on memory locations based on shifting and conditional logic.
- `setup_blitter_parameters`: Configures blitter registers for bit block transfer operations using input values, then performs a loop of blitter operations until a counter reaches zero.
- `execute_blitter_setup`: Calls another routine and then calls a second routine with specific values in registers d0 through d4.
- `perform_blitter_transfers`: Calls a routine with specific register values, calculates an offset into a table using a value from a global variable, and then calls the same routine again with different register parameters.
- `copy_memory_block`: Copies 68 longwords from the memory location pointed to by a0 to the memory location pointed to by a1 using a series of longword move operations.
- `validate_and_process_memory`: Loads an address from a global variable into a register, tests a byte value at an offset, branches if the value is zero, performs arithmetic operations on registers, and calls another routine with multiple parameters.
- `update_memory_structure`: Copies data between memory locations, performs arithmetic operations on a value from the source structure, calculates an offset using division, and modifies specific memory locations at that offset.
- `initialize_game_object`: Clears specific global variables, calls another routine to insert a node into a list, stores register values into specific structure offsets, and initializes various fields of a data structure with predefined values.
- `update_and_validate`: Reads values from global variables, performs arithmetic operations including subtraction and multiplication by two, checks conditions against thresholds, and updates a global variable with either a fixed value or a negated result based on comparison outcomes.
- `negate_and_read`: Negates a value from memory using an offset from a register and stores the result in another register.
- `validate_value_range`: Compares a value with negative 1024 and branches if greater, then sets the value to negative 240 before continuing.
- `validate_and_update_values`: Negates a value from memory, stores it, performs conditional arithmetic operations on global variables, compares values with thresholds, and updates memory locations based on comparison results.
- `load_and_negate_value`: Loads a 16-bit value from memory using an offset from a register, negates that value, and branches unconditionally to another location.
- `validate_and_limit_value`: Compares a 32-bit value with a fixed negative constant, branches if the value is greater, loads another fixed constant into the register, and branches unconditionally to another location.
- `negate_and_store`: Negates a 16-bit value in a register, uses it as an offset to read from a memory location, and stores the result into a global variable.
- `fill_memory_with_zeros`: Writes zero values to a sequence of memory locations by looping 5119 times and incrementing a pointer by 10 each iteration.
- `initialize_rendering_params`: Sets up rendering parameters based on configuration data, calculates offsets for graphics buffers, and configures bit block transfer settings including source and destination modulo values.
- `update_render_parameters`: Updates rendering parameters based on global variables, stores calculated values into two memory locations, and configures custom hardware registers for bitmap operations.
- `clamp_and_accumulate_2`: Clamps a 16-bit value in a global variable between -112 and 0, then accumulates it with another global variable while updating multiple rendering parameters.
- `clamp_and_accumulate`: Clamps a 16-bit value in a global variable between -112 and 0, then accumulates it with another global variable while updating multiple rendering-related parameters.
- `update_globals_and_branch`: Reads and modifies multiple global variables based on conditional comparisons, updates memory locations with values from other global variables, and performs arithmetic operations including subtraction and bit shifts before branching to different code paths.
- `update_global_variables`: Performs arithmetic operations on global variables, updates memory locations with computed values based on conditional checks, and initializes several global variables with specific values.
- `initialize_registers_and_flags`: Initializes various global variables with different values based on conditional checks, performs bit operations, and updates multiple memory locations with computed results.
- `initialize_rendering_2`: Sets up rendering parameters based on a condition, updates pointers to graphics data structures, and configures multiple rendering stages with specific offsets and values.
- `initialize_rendering`: Sets up rendering parameters based on a condition, updates pointers to graphics data structures, and configures multiple rendering stages with specific offsets and data transfers.
- `initialize_rendering_3`: Sets up rendering parameters based on a condition, updates pointer values in a structure with address register contents, and continues updating until specific bit conditions are met.
- `update_pointer_with_condition_2_2`: Updates pointer values in a structure with address register contents, sets flags, and continues updating until specific bit conditions are met.
- `update_pointer_with_condition_2`: Updates pointer values in a structure with address register contents, sets flags, and continues updating until specific bit conditions are met, then performs conditional operations on array elements.
- `update_pointer_with_condition`: Updates a pointer in a structure with the value of an address register, sets a flag to 1, and continues updating until a specific bit condition is met.
- `initialize_linked_list`: Sets up a linked list structure by updating pointers in a loop that runs 60 times, then updates global pointers and clears a 16-bit value at a specific location.
- `setup_and_branch`: Loads an address into an address register and branches to another routine without making a function call.
- `process_function_pointer_chain`: Processes a chain of function pointers stored in memory by calling each function until reaching a null pointer, then returns to the previous stack frame.
- `allocate_structure_entry`: Checks if a counter at a global location is less than 60, and if so, performs linked list operations to insert a new structure entry, initializes its fields with zeros, and sets a flag byte.
- `remove_node_from_list`: Removes a node from a linked list structure when the list count is less than 60, initializes node fields with zero values, and sets a flag byte to 255.
- `store_structure_data`: Stores the contents of d0 and d1 registers into offsets 8 and 12 of the structure in a0, stores a1 into offset 58, stores a pointer from a table into offset 66, stores a value from that table into offset 54, and clears offsets 72 and 33 of the structure in a0.
- `update_structure_from_table`: Updates a structure's fields with values from another structure, computes a new value using data from a table, and clears two specific fields in the target structure.
- `toggle_and_process`: Toggles a byte value in memory, calls another routine, toggles the same byte value again, then returns.
- `save_and_test_flag`: Saves a word value from a register into a global variable, then tests a 16-bit value at offset 108 of a structure and branches if the value is zero.
- `load_and_test_flag_2`: Loads a memory address into address register a0 and tests a 16-bit value at offset 6c of that structure, branching to another routine if the value is non-zero.
- `load_and_test_flag`: Loads a memory address into address register a0 and tests a 16-bit value at offset 6c of that structure, branching to another routine if the value is not zero.
- `check_and_branch`: Loads an address into an address register, tests a 16-bit value at an offset and branches if not zero, otherwise continues to another routine.
- `compare_and_branch`: Compares a register value with byte values at specific offsets in multiple memory locations, branches to another routine if any comparison is not met, and otherwise continues execution with a branch to a different location.
- `check_and_clear_flag_2`: Tests a 16-bit value at offset 108 of the structure in a0 and branches to offset 14 on zero condition.
- `test_and_branch_bit_2`: Tests bit 10 of data register d1 and branches to another routine if the bit is clear, otherwise continues execution.
- `test_and_branch_bit`: Tests bit 10 of data register d1 and branches to another routine if the bit is clear, otherwise continues execution.
- `test_and_branch`: Tests bit 11 of the byte in d1 and branches to offset 14 if it is clear, otherwise it returns.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `check_and_clear_flag`: Tests a 16-bit value at offset 108 of the structure in a0 and branches to offset 14 on zero condition.
- `initialize_graphics_data`: Writes byte values from registers to specific offsets in a structure, loads a pointer from a table based on a computed index, and stores the result of a calculation at offset 2 of the structure.
- `initialize_audio_hardware_3`: Clears a byte array using a loop, populates hardware registers with values from a table, initializes fields in a structure with values from a table, and calls another routine twice to process audio data.
- `parse_and_dispatch_table_entries`: Saves all registers on the stack, stores a pointer from a0 to offset 20 of the structure in a6, adds 256 to a1 and reads a 16-bit value from a1 to a register.
- `initialize_structure_fields`: Copies a longword from a location offset 16 in a6 to offset 10 in a0, then clears several fields at offsets 16, 51, 107, 40, 34, 36, 38, 42, 32, 31 in a0, copies a longword from offset 20 in a6 to offset 18 in a0, and finally stores the values from d5, d6, and d7 into offsets 14, 54, and 52 of a0 respectively.
- `initialize_hardware_parameters_2`: Loads values from a structure into hardware registers for audio channels, performs conditional writes to DMA control register, and returns.
- `decrement_timers`: Decrements a 16-bit counter and a byte counter in a global structure, then branches based on their values to update DMA and position data.
- `update_position_and_dma_3`: Reads a byte from a buffer, updates position and DMA-related data based on its value, and advances the buffer pointer.
- `process_dma_state`: Processes DMA state based on a lookup table index, updating various structure fields and handling DMA control register operations.
- `update_dma_and_position_2_4`: Updates DMA and position data based on input values and state flags, modifying memory locations through pointer arithmetic and conditional logic.
- `update_collision_data_3_2`: Loads a byte value from memory, tests another byte for zero, branches if not zero, adds the loaded value to a memory location, and then modifies data based on bit testing and shift operations.
- `update_collision_data_3`: Loads a byte value from memory, tests another byte for zero, branches if not zero, adds the loaded value to a memory location, and then modifies collision detection data by subtracting values from memory locations, decrementing counters, and updating flags based on comparison.
- `update_collision_data_2`: Loads a byte value from memory, tests another byte for zero, branches if not zero, adds the loaded value to a memory location, and then branches to a specific address.
- `update_collision_data`: Modifies collision detection data by subtracting values from memory locations, decrementing counters, and updating flags based on comparison results.
- `update_dma_parameters_3`: Updates DMA parameters by modifying word values at specific offsets in a structure based on comparisons between register contents and memory values, then increments a counter and stores a byte from a buffer.
- `update_dma_parameters_2`: Performs arithmetic operations on register values, compares results with another register, updates memory locations with calculated values, and branches based on comparison outcomes.
- `update_dma_control_register_3_2`: Copies a word value from a register to offset 26 of a structure, loads another address into a3 register, reads a byte from memory using a word pointer, and performs arithmetic operations on the result.
- `update_counter_and_compare`: Adds a value from one register to another, compares the result with a third register, branches if the result is positive or zero, stores the third register's value into a memory location, then branches unconditionally to the end.
- `update_dma_control_register_3`: Copies a word value from a register to offset 26 of a structure, loads another address into a3 register, reads a byte from memory using a word index, and updates control registers based on comparison results.
- `update_dma_control_register_2_2`: Loads a memory address into register a3, adds an 8-bit value at offset 1e of the structure in a0 to a register, tests an 8-bit value at offset 6a of that structure and branches to offset 50 if zero, otherwise copies that value to a register and branches to offset 5a.
- `update_dma_control_register_2`: Subtracts an 8-bit value at offset 46 of the structure in a1 from a register, and branches to offset 8 if the result is positive.
- `update_dma_control_register`: Subtracts an 8-bit value at offset 46 of the structure in a1 from a register, and branches to offset 8 if the result is positive.
- `set_dma_control_bit`: Loads a word value from memory into a register, performs a bitwise OR operation with another word value at a fixed offset, and stores the result back to that memory location.

### Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:FairLight#unpacked

- `call_then_trap_zero_2`: Calls another routine, pushes a memory address onto the stack, stores that address into a global variable, executes a trap-zero instruction, and returns zero.
- `wait_for_hardware_flag_2`: Polls bit 6 of a hardware register, then executes a sequence of subroutines based on the results of various conditional branches and bit tests, finally returning via an interrupt return.
- `configure_hardware_and_copy_data_2`: Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range.
- `update_memory_values_2`: Writes several specific values to a memory block pointed to by a register.
- `check_and_increment_byte_2`: Checks a byte at an offset; if it is negative one, it increments a global variable and loops until the byte is zero.
- `process_compressed_data`: Loads a byte value from a structure, compares it with negative one, branches if not equal, loads another address into a register, subtracts a fixed value from memory, and returns.
- `clear_memory_range_2`: Fills a range of memory starting at a specific address with zeros using a loop.
- `configure_hardware_and_copy_data_2_2`: Loads two memory addresses into address registers a0 and a1, initializes data registers d0 and d1 to zero, copies bytes from a0 to d0 while checking for positive values, and branches based on comparison results.
- `validate_and_calculate`: Compares a byte value with 254 and branches to another routine if not equal, otherwise performs pointer arithmetic and calculates a word difference.
- `initialize_loop`: Branches to a loop if a condition is met, then increments a pointer and sets a counter to zero before branching elsewhere.
- `copy_and_increment_loop`: Copies bytes from a table to memory locations while incrementing pointers and performing loop iterations based on counter values.
- `set_global_and_trap_2`: Stores the address of its own routine in a global variable and then issues a trap-zero instruction.
- `process_a5_state_loops`: Sets a byte and a word at fixed offsets in a structure addressed by A5 and runs nested polling loops that wait for those bytes to change, calling other routines during each wait.
- `toggle_flag_and_update_table_2`: Toggles a flag word in the structure addressed by A5, and when the flag becomes zero it advances a counter by two and writes one value into four table entries spaced sixteen bytes apart at a fixed offset of 168 bytes.
- `toggle_flag_and_fill_buffer_2`: Toggles a flag word at a fixed offset in the A5 structure, and when it becomes zero and a counter stays non-negative after subtracting two, it writes one value into four entries of the table at offset 168 with a stride of sixteen bytes.
- `unpack_compressed_table`: Calls another routine 32 times, then walks 32 source words and writes their 4-bit fields into a destination array by masking and shifting.
- `find_max_nibble`: Scans the four nibbles of a 16-bit word and keeps the largest one in a register, running exactly four times once per four-bit segment.
- `decrement_and_branch`: Decrements a 16-bit value at offset 384 of the structure in a5 and branches to another routine if the result is zero.
- `process_nested_loops_2`: Executes nested loops that manipulate values in tables and increment pointers based on specific conditions, performing bit operations and comparisons.
- `process_nested_loops`: Executes nested loops that manipulate values in tables and increment pointers based on specific conditions, performing bit operations and conditional updates.
- `decrement_and_check_counter`: Decrements a 16-bit counter at offset 384 of the structure in a5 and branches to another routine if the result is zero, otherwise returns immediately.
- `process_and_accumulate_bits`: Initializes a counter and processes 32 words from a table, clearing specific bit patterns from each word and accumulating the results in register d2 before returning.
- `initialize_dma_and_audio_channels`: Writes the DMA and interrupt control registers, the CIA B port registers and the audio-channel registers, then waits for the vertical beam position counter to reach a threshold.
- `build_copper_style_word_table`: Fills a global table with four groups of word pairs holding descending or incrementing values together with the high and low halves of a fixed address, then writes 80 words of incrementing values alternated with zeros; it stores the table pointer and final counter into the structure addressed by a6.
- `restore_dma_and_interrupt_state_2`: Disables DMA and interrupt enables through the custom registers, restores saved values from globals, sets two global flags, then re-enable DMA by writing a fixed value.
- `copy_and_store_values`: Copies a 32-bit value from one memory location to another, stores a 16-bit value into a different memory location, and returns.
- `lbl000352`: Writes the constant 255 to one variable and 32 to a global variable, which the listed facts associate with the interrupt request register.
- `configure_cia_interrupt_control`: Saves registers to the stack, reads a byte from a hardware register, writes a fixed constant to a memory location, restores registers from the stack, writes another fixed constant to a hardware register, and returns from the interrupt.
