# Game 2503 (deliverance (1992)(21st century)(disk 1 of 3)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02503). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906864 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 109 raw-word lines |
| Functions named and described | pass, 73 of 73 named (0 author symbols), 73 of 73 described (5 by rule, 19 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1071 of 1071 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1716 of 1716 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6592 |
| code | 5310 |
| data | 885812 |
| padding | 6078 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance | 2 | 267 | 8 |
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:FairLight | 2 | 267 | 8 |
| Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 127 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 263 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 408 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 304 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 5 |

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
- runtime segment an address: 106 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 147 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deliverance (1992)(21st century)(disk 1 of 3)[cr flt]/Deliverance (1992)(21st Century)(Disk 1 of 3)[cr FLT].adf:Deliverance: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_copy_data` (runtime segment an address): Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range. Kind: disk hardware/track check. Pass path: the branch at an address (after bne.b).
- `read_drive_status_bits` (runtime segment an address): Polls a CIA status register for 32 samples, accumulating the bits into a mask, and records a result byte; it repeats until the returned code equals one. Kind: Disk hardware status check (CIAB_PRB read via read_drive_status_bits). Pass path: The routine reads the drive status bits (CIAB_PRB) in a loop, then u6__sub_0368() returns a value; the check passes when (short)uVar1 == 1, which exits the do/while loop and stores bVar2 / 1 at (unaff_A4 + 8)..

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
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

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
