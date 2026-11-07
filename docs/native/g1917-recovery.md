# Game 1917 (indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl][t supplex]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01917). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906320 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 52 of 52 named (0 author symbols), 52 of 52 described (3 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 315 of 315 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1214 of 1214 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 962 |
| code | 3996 |
| data | 899160 |
| padding | 2202 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][t Supplex].adf:SUPPLEX | 1 | 99 | 3 |
| Indy-IV | 2 | 267 | 8 |
| Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][t Supplex].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 838 | 40 |

## Cases handled

- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl][t supplex]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][t Supplex].adf:Indy IV/disk_00.cnc: high-entropy data file (7.74 bits/byte), classified as packed data
- runtime segment an address: 65 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)[cr csl][t supplex]/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][t Supplex].adf:Indy IV/Indy-IV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_cia_port_with_delays` (runtime segment an address): Runs two short delay loops, writes a masked pattern to a CIA B port register, runs another delay loop and returns. Kind: disk hardware/timing check. Pass path: lea.l an address.l, a0.

## Functions


### Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][t Supplex].adf:SUPPLEX

- `write_colour_registers_on_entry`: Program entry point that writes to the colour register bank and calls four other routines.
- `refill_bit_buffer_word`: Loads the 32-bit word that precedes the source pointer, clears a 16-bit location through the second pointer, and returns that word shifted right by one bit with the top bit set.
- `discard_bits_from_stream`: Shifts out as many bits as its input count from a bit buffer, calling the refill routine whenever the buffer word becomes empty.

### Indy-IV

- `load_hunks_and_apply_relocations`: Parses AmigaDOS hunk records of a loaded executable.
- `busy_wait_16bit_count`: Burns time in a software delay by decrementing a 16-bit count until it wraps past zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_compressed_stream_run`: Decompresses data by reading variable-length codes through a bit reader and copying runs backwards from already written output into the destination, stopping when the destination pointer reaches its start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `read_bit_sequence_into_word`: Reads a run of bits from a bit buffer, refilling the buffer word from memory when it empties, and assembles them into an accumulator by shifting each new bit in at the low end.
- `execute_trap_instruction`: Triggers a software interrupt and attempts to store a byte into a memory location derived from a register.

### Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts)[cr CSL][t Supplex].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_wait_messages`: Opens and closes exec libraries, locates the current task and waits on its message port.
- `open_library_install_copper_list`: Opens library resources under Forbid and Permit, installs a copper list pointer and enables DMA bits, then walks a table of pointers calling a setup routine for each entry.
- `stub_self_assign_and_return`: Stores a global variable back into itself and returns, so it performs no work.
- `load_and_unpack_compressed_file`: Reads from a file through library calls, checking each result for failure.
- `read_four_bit_field`: Reads a 4-bit field from a bit stream at the current bit position, refilling the stream word first when the bit counter is below seven.
- `read_five_bit_field`: Reads a 5-bit field from a bit stream at the current bit position, refilling the stream word first when the bit counter is below seven.
- `read_six_bit_field`: Reads a 6-bit field from a bit stream at the current bit position, refilling the stream word first when the bit counter is below seven.
- `read_seven_bit_field`: Reads a 7-bit field from a bit stream at the current bit position, refilling the stream word first when the bit counter is below seven.
- `read_eight_bit_field`: Reads an 8-bit field from a bit stream at the current bit position, refilling the stream word first when the bit counter is below seven.
- `read_nine_bit_field`: Reads a 9-bit field from a bit stream at the current bit position, refilling the stream word first when the bit counter is below seven.
- `copy_block_then_read_file`: Copies a source block into a destination buffer, clamping the length to a fixed maximum, then calls a library read routine.
- `clear_global_pointer`: Checks if a global variable contains a valid value, calls a function through a pointer at an offset within that value, and then resets the global variable to negative one.
- `pulse_cia_port_with_delays`: Runs two short delay loops, writes a masked pattern to a CIA B port register, runs another delay loop and returns.
- `allocate_signal_and_init_table`: Allocates an exec signal bit and finds the current task.
- `fill_bytes_with_value`: Writes one byte value to memory repeatedly for a count given in a data register, advancing the destination pointer by one byte each time.
- `copy_block_forward_or_backward`: Copies a byte block from source to destination, copying forward when the destination lies below the source and backward otherwise, so overlapping regions are copied correctly.
- `copy_bytes_backward_by_count`: Copies a block backward from the high end of the source and destination for a given count.
- `jump_through_dispatch_table`: Jumps to one of several routines chosen by a global value through a table, and returns when that routine returns.
- `allocate_and_link_list_node`: Allocates a memory node through the exec allocator, and on success links it into a list by storing two pointer fields and zeroing two others.
- `allocate_node_or_report_error`: Allocates a memory node through the exec allocator and links it into a list on success.
- `concatenate_two_strings_into_buffer`: Copies one zero-terminated string into a fixed buffer, appends a second zero-terminated string after it, and passes the buffer to a library routine.
- `clear_longword_table_reset_counter`: Zeroes a fixed table of 32-bit words and then resets a global counter to zero twice.
- `bump_counter_around_two_calls`: Increments a global counter, calls a routine, resets the counter to zero and calls another routine.
- `accumulate_mouse_port_movement`: Reads the mouse-port movement register twice.
- `enable_interrupt_bits`: Calls two routines, copies a global value through a temporary, and writes an interrupt-enable value with the set-on-write bit to the interrupt enable register.
- `read_interrupt_request_state`: Reads the interrupt enable and interrupt request state registers, with two calls around the reads.
- `touch_interrupt_request_register`: Touches the interrupt request register and makes one call.
- `examine_two_instruction_stub`: No pseudo-code could be recovered.
- `decode_keyboard_serial_byte`: Reads a byte from a CIA serial data register, inverts and rotates it into a key code, and updates a per-key state table according to the key-down or key-up bit.
- `store_key_state_and_wait`: Stores the low seven bits of a key code, saves the previous state of that key's entry and sets the entry from the press or release bit.
- `enable_interrupts_after_two_calls`: Calls two routines and then writes a fixed interrupt-enable value to a global.
- `disable_interrupts_master_bit`: Writes a fixed value with the set-or-clear bit clear to the interrupt enable register, which disables the bit it names.
- `enable_interrupts_master_bit`: Writes a fixed value with the set-or-clear bit set to the interrupt enable register, which enables the bit it names.
- `setup_interrupt_server_node`: Clears a small interrupt-server structure, stores a code address and a priority value in it, then opens a system resource.
- `release_stored_resource_handle`: If a stored resource handle is set, calls two library functions on it.
- `wait_forever_after_calls`: Calls two routines and then spins in an endless loop, so it never returns.
- `stub_return_without_work`: The decompiled body is a bare return, so the pseudo-code shows no work.
- `copy_strided_words_into_table`: Copies a run of 16-bit words into a global table, stepping the source by four bytes and the table by two.
- `silence_audio_channels`: Clears the DMA enable bits for the four audio channels and sets the volume register of each channel to zero.
- `store_longwords_at_table_offsets`: Walks a table of offset and value entries and writes each 32-bit value into a second table at the offset given by its entry.
