# Game 1916 (indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01916). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906320 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 62 of 62 named (0 author symbols), 62 of 62 described (2 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 294 of 294 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1329 of 1329 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1894 |
| code | 4730 |
| data | 897446 |
| padding | 2250 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indy-IV | 6 | 232 | 6 |
| Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1087 | 55 |

## Cases handled

- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL].adf:Indy IV/disk_00.cnc: CNC data cruncher (xfd)-packed, unpacked with ancient (786219 -> 3736 bytes)
- runtime segment an address: 65 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL].adf:Indy IV/Indy-IV: Imploder (xfd) stream found by ancient scan (51748 -> 111660 bytes) but only 9 of 49 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL].adf:Indy IV/Indy-IV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `delay_and_set_cia_port` (runtime segment an address): Waits through three short countdown delays and, between them, writes a masked value to the port B data register of a CIA chip, then returns. Kind: timing/disk hardware check. Pass path: lea.l an address.l, a0.

## Functions


### Indy-IV

- `run_program_startup_calls`: Program entry point.
- `decrement_until_minus_one`: Repeatedly decrements a data register, starting from its existing value, until it reaches negative one, then returns.
- `run_two_pass_accumulate_loop`: Calls an indirect routine twice, then runs two passes.
- `stub_zero_result_return`: Merges zero into a result register and returns.
- `copy_bytes_backward_bit_driven`: Copies bytes from an input pointer that moves backward into an output pointer, using a bit buffer kept in globals to decide how many bytes each copy covers and calling other routines to fetch the next bits or count.
- `push_registers_to_stack`: Increments d0 by 8, performs a bitwise OR on d0 with 34, and pushes the resulting byte, the byte in d1, and the word in d2 onto the stack pointed to by a4, then restores d0 through d3, d5 through d6, and a3 through a4 from the stack before branching to another routine.

### Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_wait_port`: Calls exec routines to find the current task, open and close libraries and wait on a message port, and writes the DMA control and copper list pointer registers.
- `release_table_and_loop_forever`: If a global is set, calls two library routines and stores the result.
- `stub_self_assignment_return`: Assigns a global variable to itself and returns, so it has no effect; the routine is effectively a stub.
- `update_linear_congruential_generator`: Multiplies a 16-bit value from a global variable by 1469, adds 18269, stores the result back to the global variable, and returns the value shifted right by one.
- `format_two_digit_decimal_text`: Writes the tens and units of a value into a fixed text buffer as characters when it differs from another value, then looks up a value in a table to call library routines and returns a status.
- `read_four_bit_field`: If a global counter is below seven, calls another routine first.
- `read_five_bit_field`: If a global counter is below seven, calls another routine first.
- `read_six_bit_field`: If a global counter is below seven, calls another routine first.
- `read_seven_bit_field`: If a global counter is below seven, calls another routine first.
- `read_eight_bit_field`: If a global counter is below seven, calls another routine first.
- `read_nine_bit_field`: If a global counter is below seven, calls another routine first.
- `copy_buffer_then_read_chunk`: Copies a block of bytes into a buffer, then reads up to a fixed chunk size through a library call.
- `close_file_if_open`: If a stored file handle is not the sentinel value of minus one, closes it through a library call and resets the handle to that sentinel value.
- `delay_and_set_cia_port`: Waits through three short countdown delays and, between them, writes a masked value to the port B data register of a CIA chip, then returns.
- `reorder_and_dispatch_buffer`: Stores three values into global variables, calls another routine with a constant value of 974, reorders four bytes within a memory block using a series of byte moves, and then performs two subsequent calls to another routine before jumping to a location stored in a global variable.
- `update_and_swap_buffer_data`: Stores two pointers into global variables, calls a routine with a constant of 621, and if successful, updates a global pointer with the result.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `load_address_and_branch`: Calls another routine, loads a long value from a global variable into a0, and branches to another routine.
- `allocate_signal_and_init_block`: Allocates an exec signal bit and, on success, zeroes a block of memory, stores the signal number and the current task pointer into it and links its fields.
- `perform_io_and_cleanup`: Attempts to open a device and, upon failure, performs a sequence of I/O operations using a structure in a1 to write specific command codes, data, and bit-shifted values before closing the device and performing bitwise operations on the CIA register via a loop.
- `initialize_io_request_and_execute_io`: Stores three values from registers into global variables, then attempts to open a device using an IO request structure.
- `load_address_and_branch_2`: Calls another routine, loads a long value from a global variable into a0, and branches to another routine.
- `fill_bytes_with_value`: Writes a byte value to a run of consecutive memory locations, with the count held in a register.
- `copy_bytes_overlap_safe`: Copies a run of bytes between two regions.
- `copy_bytes_backward`: Copies a counted run of bytes from the end of a source region backward into the end of a destination region, so overlapping regions are handled.
- `dispatch_through_jump_table`: Calls a routine selected through a jump table, with the entry indexed by a value.
- `allocate_tracked_memory_block`: Allocates a memory block through the exec library and, on success, stores its address and size into a record in a list and advances the list pointer.
- `allocate_memory_or_report_error`: Allocates a memory block through the exec library and records it in a list.
- `concatenate_strings_into_buffer`: Copies one zero-terminated string into a fixed buffer and appends a second string after it, then calls other routines.
- `clear_long_table_and_flag`: Writes zero to thirty-two consecutive longwords at a fixed address and clears a global variable.
- `increment_counter_around_calls`: Increments a global counter, calls one routine, resets the counter to zero and then calls another routine before returning.
- `store_byte_to_global`: Stores the byte from d0 into a global variable and returns.
- `update_and_reset_global_byte`: Increments a 16-bit global variable, calls another routine, stores 32639 into a global variable, and calls another routine before returning.
- `read_joystick_and_status`: Increments a 16-bit global variable, checks a bit in the CIAA register based on d0, and then performs several bitwise checks on the JOY0DAT register to build a bitmask in d0.
- `store_joystick_data`: Calls another routine, reads a 16-bit value from the JOY0DAT register, and stores the low and high bytes into a table at offset 0, then repeats this process for the JOY1DAT register at offset 6 before calling another routine and returning.
- `poll_joystick_and_pot_buttons`: Checks a value in a register and, if not equal to one, loads a table into registers, tests bit six of the CIAA register, and tests bit two of the custom chip register.
- `check_hardware_status`: Loads values from a table, checks bit 7 of the CIAA register, writes 52224 to the POTGO register, and checks bit 6 of the POTGOR register to increment a value in d2.
- `track_port_movement_deltas`: Reads the movement counter registers for the mouse and joystick ports, stores their low and high bytes, and adds each change since the previous reading to a running total for each port, for two entries.
- `store_word_to_global`: Stores the word from d0 into a global variable and returns.
- `copy_globals_then_enable_interrupts`: Copies global variables around in a shuffle and writes an interrupt enable value that sets the master and external enable bits.
- `read_interrupt_status_registers`: Reads the interrupt enable and interrupt request registers and calls two other routines.
- `access_interrupt_request_register`: Accesses the interrupt request register and calls one other routine.
- `stub_two_instruction_routine`: A two-instruction routine that the decompiler did not decode, so only its single call site is known and its behaviour is not shown.
- `decode_serial_input_byte`: Reads a byte from the serial data register, rotates and inverts it into a code, updates a state table entry for that code, then waits for the serial transfer flag before resetting the control bits.
- `record_key_state_and_char`: Stores a masked code and the previous table entry, updates the table entry from the top bit of the input byte and, when the entry is set, records the matching character from a text table.
- `call_then_enable_interrupts`: Calls another routine, copies a global value to itself and writes an interrupt enable value that sets the master and external enable bits.
- `disable_external_interrupt_bit`: Writes an interrupt enable value that clears the external interrupt bit without setting the enable bit, switching that interrupt off.
- `enable_master_external_interrupts`: Writes an interrupt enable value with the master and external enable bits set to the interrupt enable register.
- `open_resource_and_init_block`: Initialises fields of a memory block, stores a routine address in it, opens a named system resource through the exec library and, if that succeeds, calls two routines of the resource.
- `call_resource_routines_if_set`: If a stored resource pointer is set, calls two routines through its function table and returns.
- `call_routines_then_loop_forever`: Calls other routines and then loops forever.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `copy_every_other_word_reversed`: Optionally selects a source pointer from a flag bit, then copies sixteen words in reverse order, taking one word from each pair of words in a source block into a global area.
- `silence_audio_channels`: Writes the DMA control register with the four audio channel bits cleared, which disables those channels, and zeroes the volume registers of the four audio channels.
- `write_indexed_table_entries`: Loops over a fixed table of index and value pairs, storing each value as a longword at an address offset by its index, for a count taken from a global variable.
