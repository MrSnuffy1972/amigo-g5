# Game 1918 (indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl][v systemz v4.0]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01918). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906320 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (2 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 294 of 294 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1089 of 1089 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2650 |
| code | 3716 |
| data | 897726 |
| padding | 2228 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indy-IV | 6 | 233 | 7 |
| Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][v SystemZ v4.0].adf:bootblock | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 837 | 40 |

## Cases handled

- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl][v systemz v4.0]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][v SystemZ v4.0].adf:Indy IV/disk_00.cnc: high-entropy data file (7.74 bits/byte), classified as packed data
- runtime segment an address: 65 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl][v systemz v4.0]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][v SystemZ v4.0].adf:Indy IV/Indy-IV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `delay_and_set_cia_port` (runtime segment an address): Waits through three short countdown delays and, between them, writes a masked value to the port B data register of a CIA chip, then returns. Kind: timing/delay-based hardware check. Pass path: lea.l an address.l, a0.

## Functions


### Indy-IV

- `run_program_startup_calls`: Program entry point.
- `decrement_until_minus_one`: Repeatedly decrements a data register, starting from its existing value, until it reaches negative one, then returns.
- `run_two_pass_accumulate_loop`: Calls an indirect routine twice, then runs two passes.
- `stub_zero_result_return`: Merges zero into a result register and returns.
- `copy_bytes_backward_bit_driven`: Copies bytes from an input pointer that moves backward into an output pointer, using a bit buffer kept in globals to decide how many bytes each copy covers and calling other routines to fetch the next bits or count.
- `copy_reversed_byte_quads`: Copies groups of four bytes from a source pointer that moves backward into an output pointer that moves forward, refilling a bit buffer from globals and stopping when a counter reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][v SystemZ v4.0].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_wait_port`: Calls exec routines to find the current task, open and close libraries and wait on a message port, and writes the DMA control and copper list pointer registers.
- `release_table_and_loop_forever`: If a global is set, calls two library routines and stores the result.
- `stub_self_assignment_return`: Assigns a global variable to itself and returns, so it has no effect; the routine is effectively a stub.
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
- `allocate_signal_and_init_block`: Allocates an exec signal bit and, on success, zeroes a block of memory, stores the signal number and the current task pointer into it and links its fields.
- `fill_bytes_with_value`: Writes a byte value to a run of consecutive memory locations, with the count held in a register.
- `copy_bytes_overlap_safe`: Copies a run of bytes between two regions.
- `copy_bytes_backward`: Copies a counted run of bytes from the end of a source region backward into the end of a destination region, so overlapping regions are handled.
- `dispatch_through_jump_table`: Calls a routine selected through a jump table, with the entry indexed by a value.
- `allocate_tracked_memory_block`: Allocates a memory block through the exec library and, on success, stores its address and size into a record in a list and advances the list pointer.
- `allocate_memory_or_report_error`: Allocates a memory block through the exec library and records it in a list.
- `concatenate_strings_into_buffer`: Copies one zero-terminated string into a fixed buffer and appends a second string after it, then calls other routines.
- `clear_long_table_and_flag`: Writes zero to thirty-two consecutive longwords at a fixed address and clears a global variable.
- `increment_counter_around_calls`: Increments a global counter, calls one routine, resets the counter to zero and then calls another routine before returning.
- `track_port_movement_deltas`: Reads the movement counter registers for the mouse and joystick ports, stores their low and high bytes, and adds each change since the previous reading to a running total for each port, for two entries.
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
