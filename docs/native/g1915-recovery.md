# Game 1915 (indiana jones and the fate of atlantis - arcade (1992)(lucasarts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01915). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906320 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 62 of 62 named (0 author symbols), 62 of 62 described (2 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 294 of 294 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1329 of 1329 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1894 |
| code | 4730 |
| data | 896436 |
| padding | 2236 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indy-IV | 6 | 232 | 6 |
| Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1087 | 55 |

## Cases handled

- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts).adf:Indy IV/disk_00.cnc: CNC data cruncher (xfd)-packed, unpacked with ancient (786219 -> 3736 bytes)
- runtime segment an address: 65 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts).adf:Indy IV/Indy-IV: Imploder (xfd) stream found by ancient scan (50784 -> 111660 bytes) but only 9 of 49 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- indiana jones and the fate of atlantis - arcade (1992)(lucasarts)/Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts).adf:Indy IV/Indy-IV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `delay_and_write_cia_bits` (runtime segment an address): Runs two short countdown loops, writes a masked value into the CIA B port B register, runs one more countdown, and returns. Kind: timing/delay loop for disk hardware synchronization. Pass path: the instructions immediately following the call (lea.l an address.l, a0).

## Functions


### Indy-IV

- `run_entry_sequence_then_free`: Program entry point.
- `decrement_until_minus_one`: Decrements a local counter until it reaches negative one, then returns.
- `accumulate_table_two_passes`: Runs two passes over a table, calling an unnamed routine through a pointer on each step.
- `enter_stream_decoder_stub`: A two-instruction entry stub whose listed facts show one call to a single-instruction routine.
- `unpack_lz_stream_backward`: Decodes a packed bit stream of literal bytes and back-references, writing output from the end backward.
- `push_and_branch`: Adds 6 to d0, performs a bitwise OR on d0 with 150, pushes d0, d1, and d2 onto the stack, restores several registers from the stack, and branches to another routine.

### Indiana Jones and the Fate of Atlantis - Arcade (1992)(LucasArts).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_os_library_setup_sequence`: Makes a sequence of operating system library calls covering multitasking control, task lookup, library open and close, and message port waiting.
- `start_copper_and_run_table`: Opens operating system libraries and forbids multitasking.
- `stub_unreached_gap_code`: A two-instruction stub in a gap between routines that no call reaches.
- `update_linear_congruential_generator`: Multiplies a 16-bit value from a global variable by 1469, adds 18269, stores the result back to the global variable, and returns the value shifted right by one.
- `format_digits_and_decode_block`: Writes a two-digit decimal form of the value into a text buffer, then decodes a block by copying runs of earlier output and reading table entries.
- `read_four_bit_field`: Refills the bit source through an unnamed routine when the bit counter is below seven, advances the counter, and returns a four-bit field taken from the current word.
- `read_five_bit_field`: Refills the bit source through an unnamed routine when the bit counter is below seven, advances the counter, and returns a five-bit field taken from the current word.
- `read_six_bit_field`: Refills the bit source through an unnamed routine when the bit counter is below seven, advances the counter, and returns a six-bit field taken from the current word.
- `read_seven_bit_field`: Refills the bit source through an unnamed routine when the bit counter is below seven, advances the counter, and returns a seven-bit field taken from the current word.
- `read_eight_bit_field`: Refills the bit source through an unnamed routine when the bit counter is below seven, advances the counter, and returns an eight-bit field taken from the current word.
- `read_nine_bit_field`: Refills the bit source through an unnamed routine when the bit counter is below seven, advances the counter, and returns a nine-bit field taken from the current word.
- `copy_block_then_start_program`: Copies a block of bytes from a source to a destination, then calls an operating system library routine.
- `close_file_handle_if_open`: Closes a stored file handle through an operating system library call when it is not the sentinel value, then marks it as closed.
- `delay_and_write_cia_bits`: Runs two short countdown loops, writes a masked value into the CIA B port B register, runs one more countdown, and returns.
- `reorder_and_dispatch_buffer`: Stores three values into global variables, calls another routine with a constant value of 974, reorders four bytes within a memory block using a series of byte moves, and then performs two subsequent calls to another routine before jumping to a location stored in a global variable.
- `update_and_swap_buffer_data`: Stores two pointers into global variables, calls a routine with a constant of 621, and if successful, updates a global pointer with the result.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_error_handler`: Calls another routine, loads a pointer into a0, and branches to another routine.
- `allocate_signal_and_init_block`: Allocates an operating system signal and, on success, clears a block of bytes and fills in fields of a structure.
- `perform_io_and_cleanup`: Attempts to open a device and, upon failure, performs a sequence of I/O operations using a structure in a1 to write specific command codes, data, and bit-shifted values before closing the device and performing bitwise operations on the CIA register via a loop.
- `perform_io_and_delay`: Opens a device using a pointer from a global variable, then performs a sequence of I/O operations involving bitwise manipulation of a value and shifting it before closing the device and calling delay_and_write_cia_bits.
- `call_error_handler_alt`: Calls another routine, loads a pointer into a0, and branches to another routine.
- `fill_memory_with_byte`: Writes a byte value into a count of consecutive bytes starting at a destination pointer.
- `copy_block_handling_overlap`: Copies a block of bytes between two pointers, choosing a forward or backward copy so that overlapping regions are copied correctly.
- `copy_block_backward`: Copies a block of bytes from its end toward its start using a count and two pointers, and is called by five unnamed routines.
- `dispatch_indirect_call_stub`: A stub that makes an indirect call through a table entry computed from a shared value and then returns.
- `allocate_memory_block`: Calls a memory allocation routine and stores the resulting address and size into a global list.
- `allocate_memory_or_report_error`: Allocates a memory block through the operating system's AllocMem call and records its address, size and two zero words in a list.
- `concatenate_strings_into_buffer`: Copies one zero-terminated string and then a second one into a fixed buffer, passes the buffer to an unnamed routine, and runs the shared exit path.
- `clear_long_word_table`: Clears a fixed block of 32 long words and resets two shared values to zero.
- `bump_counter_around_call`: Increments a shared counter, calls an unnamed routine, resets the counter to zero, and calls a second unnamed routine.
- `store_byte_to_global`: Stores a byte from d0 into a global variable and returns.
- `update_and_reset_global_byte`: Increments a 16-bit global variable, calls another routine, stores 32639 into a global variable, and calls another routine before returning.
- `read_joystick_and_status`: Increments a 16-bit global variable, checks a bit in the CIAA register based on d0, and then performs several bitwise checks on the JOY0DAT register to build a bitmask in d0.
- `store_joystick_data`: Calls another routine, reads a 16-bit value from the JOY0DAT register, and stores the low and high bytes into a table at offset 0, then repeats this process for the JOY1DAT register at offset 6 before calling another routine and returning.
- `poll_joystick_and_pot_buttons`: Checks a value in a register and, if not equal to one, loads a table into registers, tests bit six of the CIAA register, and tests bit two of the custom chip register.
- `check_hardware_status`: Loads values from a table, checks bit 7 of the CIAA register, writes 52224 to the POTGO register, and checks bit 6 of the POTGOR register to increment a value in d2.
- `update_mouse_movement_deltas`: Reads the mouse port movement register for two ports and updates per-port position and delta fields in a small table.
- `store_word_to_global`: Stores a 16-bit value from d0 into a global variable and returns.
- `reenable_interrupt_system`: Calls an unnamed routine, shuffles a shared value through a second location, then writes the interrupt-enable value with its set bit to re-enable the interrupt system.
- `check_interrupt_request_state`: Reads the interrupt enable and request registers and calls two unnamed routines.
- `read_interrupt_request_bits`: Reads the interrupt request register and calls one unnamed routine.
- `test_interrupt_bit`: Tests a specific bit within a value and returns the result.
- `decode_keyboard_scancode`: Reads the keyboard serial data register, converts the raw code into a key number, and updates a key-state table entry from the press or release bit.
- `record_key_state_change`: Stores a key code and its previous state into a table and updates the key state from the top bit of the raw code, using a key map referenced in the text.
- `call_then_enable_interrupts`: Calls two unnamed routines and then writes the interrupt-enable value with its set bit, switching the master interrupt enable on.
- `disable_interrupt_master`: Clears the master interrupt-enable bit by writing the interrupt-enable register without its set bit.
- `enable_interrupt_master`: Writes the interrupt-enable value with its set bit for the master interrupt enable, switching the interrupt system on.
- `open_resource_and_init_fields`: Clears a few shared fields and stores the address of an unnamed routine in a shared slot.
- `close_resource_if_open`: When a stored handle is set, calls two operating system library routines to release it.
- `run_routines_then_halt`: Calls two unnamed routines and then loops forever.
- `stub_empty_return_routine`: Its pseudo-code is an immediate return with no body, although the listed instruction count is larger, so no behaviour can be stated beyond that.
- `copy_saved_words_into_table`: Copies sixteen-bit words from a source area into a shared table, choosing an alternate source pointer when the low bit of the argument is set.
- `silence_audio_channels`: Disables the four audio DMA channels and sets their volume registers to zero.
- `copy_indexed_long_words`: Walks a table of index and value pairs and writes each four-byte value into a destination at the given index offset.
