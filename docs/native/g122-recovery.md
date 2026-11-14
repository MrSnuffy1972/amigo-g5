# Game 122 (Shadoworlds v1.0 0030): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00122). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2757215 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 66 raw-word lines |
| Functions named and described | FAIL, 308 of 567 named (0 author symbols), 308 of 567 described (9 by rule, 182 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1594 of 1647 (96.78%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7507 of 7507 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 33518 |
| code | 14532 |
| data | 811163 |
| padding | 18 |
| unused | 1897984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Shadoworlds.Slave | 1 | 130 | 11 |
| Disk.1:bootblock | 1 | 8 | 2 |
| logged disk load from Disk.1 | 1 | 2406 | 193 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 404 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 112 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1198 | 74 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 457 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1471 | 117 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 41 | 22 |
| logged disk load from Disk.1 | 1 | 1238 | 105 |

## Cases handled

- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: custom-format (non-DOS) disk, 1565 non-empty blocks: matched against track-loaded segments
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.3: custom-format (non-DOS) disk, 0 non-empty blocks: matched against track-loaded segments
- 5 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 383 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 85 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 184 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 422 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 325 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 28 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.1: 122 executed PCs whose RAM was overwritten before any snapshot match its bytes 11280-16992: recovered as a unit run at an address
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.1: 6 logged loads cover 0 bytes of the image
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.034, entropy 4.19)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 6-9 (bytes 3072-5120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.01)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 14-15 (bytes 7168-8192) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.39)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 20-21 (bytes 10240-11264) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.22)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 26-26 (bytes 13312-13824) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.18)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 32-33 (bytes 16384-17408) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.28)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 38-39 (bytes 19456-20480) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.46)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 44-44 (bytes 22528-23040) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.11)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 50-50 (bytes 25600-26112) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.35)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 56-56 (bytes 28672-29184) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.19)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 62-62 (bytes 31744-32256) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.05)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 68-68 (bytes 34816-35328) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.07)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 74-74 (bytes 37888-38400) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.71)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 80-80 (bytes 40960-41472) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.38)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 86-86 (bytes 44032-44544) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.38)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 92-132 (bytes 47104-68096) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.70)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 221-229 (bytes 113152-117760) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.74)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 253-259 (bytes 129536-133120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.22)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 270-282 (bytes 138240-144896) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.83)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 287-965 (bytes 146944-494592) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.18)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 970-971 (bytes 496640-497664) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.75)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 976-977 (bytes 499712-500736) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 982-984 (bytes 502784-504320) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.14)
- Shadoworlds_v1.0_0030/Shadoworlds/Disk.2: blocks 988-1759 (bytes 505856-901120) hold ASCII text
- 1 packed track region(s), 6656 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialise_custom_chip_registers_2` (Disk.1 +an address loaded at an address (11264 bytes)): Stores unnamed routine addresses into a low-memory vector area, then writes display, colour, bitplane, DMA and CIA control registers with fixed values. Kind: hardware/register initialization for custom chip/disk controller. Pass path: execution continues to the next instruction in the caller after the routine returns.
- `set_copper_list_pointer_2` (runtime segment an address): Writes its long argument to the first copper list pointer register and returns. Kind: disk hardware/ADKCON check. Pass path: execution continues to normal track loading.
- `store_register_value` (runtime segment an address (play-long-joy)): Stores a value from a register into a memory location. Kind: checksum trap / integrity check. Pass path: the instruction immediately following the call (assuming no crash/jump to error handler).
- `initialize_hardware_state` (Disk.1 +an address run at an address (5712 bytes, matched to executed PCs)): Populates a series of memory locations with specific values and configures hardware registers including bitplane, color, and DMA control registers. Kind: hardware state/environment initialization. Pass path: execution continues to the next instruction in the caller (likely the main loader loop).

## Functions


### Shadoworlds.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `init_clear_memory_and_copy`: Writes zero longwords from a low address up to the top of the first megabyte, then writes a jump instruction and target into a fixed low cell and calls a system trap.
- `patch_return_and_jump_stubs`: Writes a return instruction word and jump instruction words into fixed global cells and stores a fixed value.
- `copy_block_after_trap_call`: Writes jump instruction words into fixed global cells, calls a system trap, and copies a block of 10652 bytes from one address to another.
- `install_jump_stub_table_entries`: Calls one unnamed routine, then writes instruction words for a jump, a return, a no-operation and a subroutine call into fixed global cells.
- `call_service_spin_if_flagged`: Calls a service routine through a function pointer with a fixed request code and two zero arguments.
- `set_global_constant`: Stores a specific constant value into a global variable.
- `check_save_tag_return_status`: Calls a tiny routine, then returns its value when a global long equals the text SAVE and a global word equals the text MS. Otherwise it returns a fixed error value of minus three.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `call_service_return_second_register`: Calls one service routine through a function pointer in a global table, then returns the value of its second data register unchanged.
- `call_service_forward_on_success`: Calls a service routine through a function pointer with a fixed request code and two zero arguments.

### Disk.1:bootblock

- `execute_boot_call`: A one-instruction routine that is part of a boot block sequence and makes a single call to another routine.
- `call_unnamed_routine_from_boot`: Seven-instruction boot block routine that makes one call to an unnamed routine and has two callers.

### logged disk load from Disk.1

- `copy_block_after_trap_call_2`: Writes a function address into a global cell, calls a system trap, and copies a block of 10652 bytes from one address to another.
- `copy_block_then_release_wait`: Copies a block of 10652 bytes from one address to another, writes a stack-saved value, then waits for a global word to clear.
- `patch_constants_then_wait_flag`: Writes fixed constants into two code cells, calls a routine and keeps its result, clears three global cells, then writes two more constants.
- `sub_025A`: undescribed.
- `sub_025C`: undescribed.
- `sub_0266`: undescribed.
- `sub_02A4`: undescribed.
- `sub_030A`: undescribed.
- `sub_030E`: undescribed.
- `sub_0312`: undescribed.
- `sub_0316`: undescribed.
- `sub_031A`: undescribed.
- `sub_031E`: undescribed.
- `sub_0324`: undescribed.
- `sub_0428`: undescribed.
- `sub_0452`: undescribed.
- `sub_045A`: undescribed.
- `sub_0484`: undescribed.
- `sub_0492`: undescribed.
- `sub_0518`: undescribed.
- `sub_0546`: undescribed.
- `sub_0614`: undescribed.
- `sub_0678`: undescribed.
- `sub_06A4`: undescribed.
- `sub_06DC`: undescribed.
- `sub_06FA`: undescribed.
- `sub_0700`: undescribed.
- `sub_0728`: undescribed.
- `sub_0738`: undescribed.
- `sub_0742`: undescribed.
- `sub_074A`: undescribed.
- `sub_075E`: undescribed.
- `sub_0780`: undescribed.
- `sub_0798`: undescribed.
- `sub_0802`: undescribed.
- `sub_081A`: undescribed.
- `sub_0860`: undescribed.
- `sub_08A0`: undescribed.
- `sub_08AC`: undescribed.
- `sub_08B2`: undescribed.
- `sub_08EC`: undescribed.
- `sub_0912`: undescribed.
- `sub_091E`: undescribed.
- `sub_0924`: undescribed.
- `sub_093C`: undescribed.
- `sub_097A`: undescribed.
- `sub_0998`: undescribed.
- `sub_09E0`: undescribed.
- `sub_0A04`: undescribed.
- `sub_0A1E`: undescribed.
- `sub_0A4C`: undescribed.
- `sub_0AF2`: undescribed.
- `sub_0AFC`: undescribed.
- `sub_0BE4`: undescribed.
- `sub_0BF6`: undescribed.
- `sub_0C06`: undescribed.
- `sub_0CA8`: undescribed.
- `sub_0CB2`: undescribed.
- `sub_0D52`: undescribed.
- `sub_0D94`: undescribed.
- `sub_0DF8`: undescribed.
- `sub_0E24`: undescribed.
- `sub_0E5A`: undescribed.
- `sub_0E78`: undescribed.
- `sub_0E7E`: undescribed.
- `sub_0EA6`: undescribed.
- `sub_0EB6`: undescribed.
- `sub_0EC0`: undescribed.
- `sub_0EC8`: undescribed.
- `sub_0EDC`: undescribed.
- `sub_0EFA`: undescribed.
- `sub_0F1A`: undescribed.
- `sub_0F6A`: undescribed.
- `forward_call_to_unnamed_routine`: Two-instruction routine that calls one unnamed routine and returns, with one caller.
- `configure_cia_a_port`: Sets a state byte, and when its low bit is set configures the CIA-A control and port registers.
- `reset_cia_state_bytes`: Clears a global state byte and a lookup entry, sets an enable bit in one hardware byte register and writes fixed values into three more at offsets from a base pointer.
- `service_cia_b_interrupt_flags`: Sets a global to the constant 8192, clears its low bit, and calls one unnamed routine when that bit was set.
- `mask_button_state_and_call`: Masks a global state byte to its low three bits, calls one unnamed routine, then stores zero into the byte.
- `store_sixteen_into_global`: Sets a global cell to the constant 16 and returns.
- `execute_conditional_callback`: Checks if a global variable is zero, executes a subroutine if true, and then updates the global variable by setting its high byte to 255 before clearing it.
- `call_unnamed_when_global_zero`: When a global byte is zero, calls one unnamed routine, writes a word whose high byte is 255 into the global, and then clears it.
- `clear_global_when_nonzero`: Returns at once when a global byte is zero.
- `program_cia_b_or_clear`: When a global byte is zero, sets it to 255 and then to 1, keeps the top two bits of one CIA-B control byte while setting bit three, and writes a 16-bit value into two adjacent byte registers.
- `clear_global_after_stack_store`: Stores a stack address into a global cell and then sets the cell to zero.
- `write_cia_b_word_registers`: Keeps the top two bits of one CIA-B control byte while setting bit three, sets a global to 129, and writes the low and high bytes of a data register into two adjacent byte registers.
- `call_unnamed_when_global_set`: Calls one unnamed routine when a global byte is nonzero and otherwise returns.
- `sub_15E4`: undescribed.
- `sub_15F4`: undescribed.
- `wait_for_non_zero`: Loops until a global variable is no longer zero.
- `sub_162E`: undescribed.
- `check_flag_and_branch`: Checks a global variable and a byte at an offset from a pointer to determine whether to execute a sequence of subsequent routine calls.
- `call_routine_sequence_with_flag`: Calls unnamed routines in order, passing a data register value to the second.
- `call_status_update_handler`: Calls a routine that stores a value in a global cell and subsequently updates a table slot using an index derived from a register.
- `store_slot_when_status_ok`: Stores a value in a global cell and calls two unnamed routines, then stores a fixed value into a table slot whose index is derived from a register and returns zero.
- `sub_16B8`: undescribed.
- `store_argument_and_call_three`: Stores its data register argument in a global cell, then calls three unnamed routines in sequence and returns.
- `run_bounded_counter_with_status`: Stores a register value in a global and calls unnamed routines to check a frame byte, returning status codes of minus four or minus three when checks fail.
- `run_bounded_counter_tail`: Shares the status and counter loop of its neighbour without the initial global store and frame checks.
- `return_status_minus_three`: Returns a value consisting of a status code minus three in the high word and an argument in the low word.
- `run_bounded_counter_return_zero`: Runs the same bounded byte counter loop as its neighbour, returning zero when its loop exits through the zero path and the result of an unnamed call otherwise.
- `run_counter_loop_until_flag`: Returns the result of one unnamed call when a flag is set.
- `call_two_return_stack_value`: Calls two unnamed routines and returns the value passed on the stack unchanged.
- `forward_call_to_unnamed_routine_2`: Short routine that calls unnamed routines and returns, with two callers.
- `dispatch_when_index_in_range`: Checks that a register index lies within a window of four values.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 251 into d0, then returns.
- `validate_then_run_counter_loop`: Returns minus three when a zero-flag check fails and minus four when a frame byte is nonzero.
- `run_counter_after_byte_check`: Returns minus four when a frame byte is nonzero.
- `forward_call_to_unnamed_routine_3`: Short routine that calls one unnamed routine and returns, with one caller.
- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 252 into d0, then returns.
- `access_hardware_registers`: This routine executes without performing any operations on the registers or memory.
- `store_registers_into_record`: Stores two data register values, a second word and a byte into a record at fixed offsets, with no calls.
- `init_record_then_run_counter`: Reads two saved fields from a record, writes a set of fixed control bytes into it and calls an unnamed routine with the values.
- `execute_complex_logic`: A fourteen-instruction routine that is called by several different procedures.
- `update_data_structures`: Performs multiple word and longword writes to various memory locations derived from registers.
- `write_fixed_word_pattern_tail`: Calls an unnamed routine, then writes the same fixed 16-bit pattern and record values as its neighbour into a buffer and a record.
- `poll_status_bit_with_timeout`: Polls a status bit in a record up to fourteen times, calling an unnamed routine on each pass and waiting on a global flag.
- `update_flag_bit_from_result`: Calls an unnamed routine and folds bit zero of its byte result into a bit at an index taken from a data register, updating a byte at a base pointer.
- `copy_512_block_and_advance`: Sets a global to 255, copies one 512-byte block from a source pointer into a destination computed from a byte index times 512 plus a fixed base, then advances the source pointer.
- `copy_512_byte_block_direct`: Copies one 512-byte block from a source pointer to a destination pointer and advances both, then reduces a remaining count by 512, leaving zero when the count was 512 or less.
- `scan_sync_pattern_then_copy`: Scans a raw data stream for a fixed sync pattern, skipping repeated patterns.
- `combine_masked_bit_words`: Returns a bitwise combination: a mask in one data register is ANDed with a word read through one pointer and ORed with the same mask ANDed with a word read through a second pointer that is doubled.
- `call_two_unnamed_routines`: Calls two unnamed routines in sequence and returns.
- `set_low_bit_then_call`: Sets bit zero of a byte at a fixed offset from a base pointer, then calls an unnamed routine.
- `write_word_bytes_wait_flag`: Clears a global flag, writes the low and high bytes of a data register into two byte registers at fixed offsets from a base pointer, then spins until the global flag becomes nonzero.
- `update_control_byte_bits`: Keeps the top three bits of one byte at a fixed offset from a base pointer while setting bit three, and writes the constant 130 into another byte at a fixed offset.
- `call_two_unnamed_routines_2`: Calls two unnamed routines in sequence and returns.
- `clear_flag_write_word_bytes`: Calls an unnamed routine, clears a global flag, and writes the low and high bytes of a data register into two byte registers at fixed offsets from a base pointer.
- `scan_words_until_target_match`: Loops reading 16-bit values through an unnamed routine until one equals a target stored in a frame field.
- `scan_words_until_target_tail`: Shares the target-scan loop of its neighbour without the first pair of calls.
- `clear_bit_in_control_byte`: Rewrites a control byte at a fixed offset from a base pointer by masking it, setting a fixed set of bits, and clearing one bit chosen by the low five bits of a data register.
- `clear_control_bit_alt`: Rewrites a control byte at a fixed offset from a base pointer by masking it, setting a different fixed set of bits, and clearing one bit chosen by the low five bits of a data register.
- `arm_bit_and_poll_status`: When a slot byte indexed by a register is zero, calls an unnamed routine, marks the slot, and rewrites a control byte with bits set and one cleared.
- `pulse_control_byte_and_wait`: Rewrites a control byte with a fixed set of bits and rewrites it again from its previous value, then calls an unnamed routine.
- `rewrite_control_byte_bit_mask`: Rewrites a control byte at a fixed offset from a base pointer four times with bitwise masks and sets, ending with a fixed set of bits.
- `clear_slot_set_control_bit`: Clears a slot byte indexed by a data register, rewrites a control byte with bits set and one bit cleared and set from a mask chosen by that register, then calls an unnamed routine.
- `compare_bytes_stop_at_marker`: Compares up to four bytes of two data register values from the most significant byte down.
- `tiny_moveq_rts_5`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `sub_1CAA`: undescribed.
- `call_routine_four_times`: Calls one unnamed routine four times in a counted loop and returns.
- `toggle_field_bit_around_call`: Saves a frame field, calls an unnamed routine for a value, stores that value with bit one toggled into the field, then calls another unnamed routine and restores the saved field.
- `set_control_bit_by_status`: Rewrites a control byte with a fixed set of bits and one bit cleared by a register index.
- `poll_status_bits_arm_record`: Polls status bits through several unnamed routines.
- `clear_frame_word_by_index`: Writes zero to a 16-bit slot in a frame array whose index comes from a data register offset by minus three, then returns.
- `read_frame_word_by_index`: Returns a 16-bit word from a frame array whose index comes from a data register offset by minus three.
- `write_frame_word_by_index`: Writes the data register word into a frame array slot whose index comes from a data register offset by minus three, then returns.
- `increment_frame_word_by_index`: Adds one to a 16-bit frame array slot whose index comes from a data register offset by minus three, then returns the data register value.
- `no_op_routine`: The routine is empty and performs no instructions.
- `call_routine_four_in_row`: Calls one unnamed routine four times in a row, then returns.
- `verify_paired_routine_results`: Calls two unnamed routines and compares their results; when they match it calls both again and compares once more.
- `xor_fold_longwords_masked`: XORs together a run of longwords read through a pointer, with the run length taken from a data register divided by four, then returns that fold ANDed with a data register mask.
- `scan_status_bits_into_slots`: When a global byte is zero, loops over four slot bytes, clearing each slot and setting it to 255 while calling unnamed routines, and sets a slot to zero when two status bits are both present.
- `call_pair_when_global_set`: When a global byte is nonzero, calls two unnamed routines, clears the global, and returns.
- `stub_empty_return_routine`: Empty routine that returns immediately, with one caller.
- `fill_buffer_alternating_pattern`: Writes 3332 longwords of an alternating bit pattern through a buffer pointer held in a frame record, four at a time.
- `write_register_block_words`: Calls an unnamed routine, stores a frame field into a pointer slot, and writes a fixed set of 16-bit values at offsets from a base pointer.
- `write_sync_pattern_then_call`: Calls an unnamed routine, writes a fixed 32-bit value made of two copies of a 16-bit sync pattern into a buffer, and calls another unnamed routine.
- `xor_fold_longwords_offset`: XORs a run of longwords read through a pointer, starting from the first value, with the run length taken from a data register divided by four minus two.
- `update_flag_bit_from_neighbour`: Calls two unnamed routines, then clears bit seven of a byte at a pointer and sets it again when bit six is clear and bit zero of the preceding byte is clear.
- `update_flag_bit_no_calls`: Clears bit seven of a byte at a pointer and sets it again when bit six is clear and bit zero of the preceding byte is clear.
- `merge_bits_from_neighbour`: Combines a data register mask and value with a shifted exclusive-or, clears the top bit when bit zero of the preceding byte is set, and stores the 32-bit result through a pointer.
- `program_transfer_registers_with_waits`: Initialises a record with a fixed sentinel and zeroed fields, then stores pointer and count values into it.
- `call_two_unnamed_routines_3`: Calls two unnamed routines in sequence and returns, with one caller.
- `nop_padding_sequence`: This routine is empty and consists only of padding instructions used to fill a gap in the executable.
- `set_control_bits_return_word`: Sets a fixed set of bits in a control byte at a fixed offset from a base pointer and clears one bit chosen by a data register's low five bits.
- `set_control_bits_return_input`: Sets a fixed set of bits in a control byte at a fixed offset from a base pointer and returns its word input unchanged.
- `wait_global_word_then_set`: Spins until a global word reads zero, then sets it to 255 and returns.
- `clear_global_cell`: Sets a global cell to zero and returns.
- `stub_infinite_loop_only`: Contains only an endless loop with no exit and no calls in the facts.
- `set_global_sequence_then_call`: Sets a global cell to one, increments it, calls one unnamed routine, then sets the cell to 255.
- `sub_2396`: undescribed.
- `sub_23BA`: undescribed.
- `initialise_custom_chip_registers`: Stores unnamed routine addresses into a low-memory vector area, then writes display, colour, bitplane, DMA and CIA control registers with fixed values.
- `set_copper_list_pointer`: Writes its long argument to the first copper list pointer register and returns.
- `initialise_custom_chip_registers_2`: Stores unnamed routine addresses into a low-memory vector area, then writes display, colour, bitplane, DMA and CIA control registers with fixed values.
- `nop_placeholder`: This routine is empty and performs no operations.
- `sub_259E`: undescribed.
- `sub_25D0`: undescribed.
- `sub_2602`: undescribed.
- `sub_2672`: undescribed.
- `sub_268A`: undescribed.
- `sub_274E`: undescribed.
- `sub_279E`: undescribed.
- `sub_27BC`: undescribed.
- `sub_27E2`: undescribed.
- `sub_280C`: undescribed.
- `sub_286C`: undescribed.
- `sub_2892`: undescribed.
- `sub_28BE`: undescribed.
- `sub_28CE`: undescribed.
- `sub_28EA`: undescribed.
- `sub_290C`: undescribed.
- `sub_2922`: undescribed.
- `sub_2932`: undescribed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_global_word_then_set_2`: Spins until a global word reads zero, then sets it to a fixed value and returns.
- `clear_global_variable`: Sets a global variable to zero.
- `execute_infinite_loop`: This routine enters an infinite loop that prevents further execution.
- `set_global_sequence_then_call_2`: Sets a global cell to one, increments it, calls one unnamed routine, then sets the cell to 255.
- `preserve_global_around_call`: Calls an unnamed routine, saves a global cell, calls another unnamed routine, then restores the saved value.
- `store_offset_sum_then_call`: Stores the low word of a long argument in a global cell, adds three constants totalling 48000 to it, then calls one unnamed routine.
- `initialise_custom_chip_registers_3`: Stores a fixed word into eight low-memory slots and sets two global values, then writes display, colour, bitplane, DMA and CIA control registers with fixed values.
- `set_copper_list_pointer_2`: Writes its long argument to the first copper list pointer register and returns.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `run_until_zero_result`: Initialises three global cells, then repeatedly calls two unnamed routines and a third routine that returns a value, looping until that value is zero.
- `run_until_zero_result_variant`: Initialises three global cells with a different starting value for one of them, then repeatedly calls two unnamed routines and a third routine that returns a value, looping until that value is zero.
- `scale_packed_nibble_table`: Rescales sixteen packed 12-bit words, each holding three 4-bit fields, by a factor taken from a global cell, writing the results into a global table.
- `sub_051A`: undescribed.
- `poll_values_until_zero_loop`: Calls several unnamed routines, reads a field from a record, and fetches another value when its low bit is clear.
- `xor_into_longword_at_pointer`: XORs a data register value into the longword at a pointer and returns.
- `copy_block_or_strided_words`: When a flag bit is clear, copies 2000 blocks of four longwords between two buffers.
- `clear_four_strided_long_buffers`: Zeroes two thousand longwords of a global buffer one at a time, clearing three other copies of that buffer located eight, sixteen and twenty-four thousand bytes further on within the same loop.
- `return_data_registers_unchanged`: Performs no work and returns the incoming values of the two data registers unchanged as a pair, so callers get back what they passed in.
- `convert_field_to_word_count`: Reads a 16-bit field from the record addressed by the address register, adds fifteen and shifts right by four, returning the number of sixteen-unit columns needed to cover that value.
- `copy_sixteen_words_fixed`: Copies sixteen consecutive words from the block addressed by the address register, starting four bytes in, into a fixed buffer at a constant location.
- `double_words_into_global_table`: Reads sixteen words from a fixed source buffer, doubles each one, and stores them into a global table at four-byte intervals.
- `double_record_words_strided`: Doubles sixteen words read from the block addressed by the address register and writes them to a destination pointer at four-byte intervals, starting two bytes in.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `fill_strided_words_with_value`: Stores the low word of the second data register into sixteen words spaced four bytes apart, starting two bytes into the destination pointer, then returns.
- `spin_until_counter_reaches_limit`: Loops with no other work while a global counter is less than a 16-bit limit read from the record addressed by a fixed pointer register, then returns the first data register unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_port`: Clears four consecutive memory locations and modifies a hardware port register.
- `sub_004A`: undescribed.
- `reset_audio_and_dma_2`: Resets audio volumes and DMA control registers, finds a maximum value in a table, and clears a series of memory locations.
- `reset_audio_and_dma`: Sets four audio volume registers and a DMA control register to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_function_pointer`: Checks a global variable against several constant values and uses it as an index into a table of function pointers to call another routine.
- `sub_00C0`: undescribed.
- `sub_00F6`: undescribed.
- `sub_0108`: undescribed.
- `sub_07AC`: undescribed.
- `sub_0812`: undescribed.
- `sub_0858`: undescribed.
- `sub_0872`: undescribed.
- `sub_0890`: undescribed.
- `sub_08FA`: undescribed.
- `sub_0930`: undescribed.
- `sub_0976`: undescribed.
- `sub_0990`: undescribed.
- `sub_09AE`: undescribed.
- `sub_0A04`: undescribed.
- `sub_0A4A`: undescribed.
- `sub_0A64`: undescribed.
- `sub_0C44`: undescribed.
- `sub_0C7A`: undescribed.
- `sub_0CB8`: undescribed.
- `sub_0D3C`: undescribed.
- `sub_0D4E`: undescribed.
- `sub_0DE0`: undescribed.
- `sub_0DF4`: undescribed.
- `sub_0DFA`: undescribed.
- `sub_0E08`: undescribed.
- `sub_0E96`: undescribed.
- `sub_0EBC`: undescribed.
- `sub_0EE4`: undescribed.
- `sub_0EEE`: undescribed.
- `sub_0F0C`: undescribed.
- `sub_0F2E`: undescribed.
- `sub_0F48`: undescribed.
- `sub_0F7C`: undescribed.
- `sub_0F8C`: undescribed.
- `sub_0FB4`: undescribed.
- `sub_0FBE`: undescribed.
- `sub_0FC8`: undescribed.
- `sub_0FD2`: undescribed.
- `sub_0FF2`: undescribed.
- `sub_1012`: undescribed.
- `sub_1032`: undescribed.
- `sub_1D2C`: undescribed.
- `sub_1D4A`: undescribed.
- `sub_1D64`: undescribed.
- `sub_1D98`: undescribed.
- `copy_and_dispatch_function`: Copies a block of data from a global variable to a table, updates several memory locations with constant values, and calls a function from a table using a calculated index.
- `sub_208E`: undescribed.
- `sub_20CC`: undescribed.
- `sub_2112`: undescribed.
- `sub_213E`: undescribed.
- `sub_216A`: undescribed.
- `sub_2196`: undescribed.
- `sub_21C2`: undescribed.
- `sub_222A`: undescribed.
- `sub_2282`: undescribed.
- `sub_22DA`: undescribed.
- `sub_2310`: undescribed.
- `sub_23BA`: undescribed.
- `sub_2410`: undescribed.
- `sub_2690`: undescribed.
- `sub_2712`: undescribed.
- `sub_2720`: undescribed.
- `sub_273A`: undescribed.
- `sub_2754`: undescribed.
- `sub_276E`: undescribed.
- `sub_278A`: undescribed.
- `sub_27BA`: undescribed.
- `sub_27D6`: undescribed.
- `sub_27F2`: undescribed.
- `clear_memory_table`: If a global variable is non-zero, it clears a series of memory locations in a loop.
- `set_global_flag_2`: Sets a global variable to a specific value if it is currently zero.
- `update_joystick_coordinates`: Reads hardware registers for joystick and mouse movement, calculates new coordinate values with clamping, and updates a global variable with a bitmask.
- `copy_memory_block`: Copies a sequence of data from a source pointer to two different destination memory locations using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sub_0000`: undescribed.
- `search_table_for_bounds`: Iterates through a table of four-word entries to find a record where two specific coordinate values fall within a defined range.
- `sub_0038`: undescribed.
- `sub_0058`: undescribed.
- `sub_005E`: undescribed.
- `parse_nested_data_structures`: Parses a string containing nested bracketed or braced data structures to calculate offsets and lengths.
- `initialize_data_structure`: Calculates an offset into a table and performs nested loops to fill memory with specific values and function pointers.
- `initialize_data_structure_variant`: Fills a memory block with constant values and populates a table with specific bytes and function pointers using nested loops.
- `parse_data_string`: Iterates through a memory buffer looking for specific delimiter bytes and performs bitwise operations on a destination buffer using a source table.
- `blit_masked_bitplane_rows`: Walks eight 40-byte rows of a 4-bitplane screen buffer, using a 32-byte source record per row that supplies one byte per plane plus a mask derived as the complement of the OR of those bytes.
- `apply_bitwise_mask_to_pointer`: Iterates through a loop to apply bitwise operations to a destination buffer using data from a source table.
- `process_nested_data`: Stores several values into global variables, parses data structures, and performs conditional function calls based on bitwise results.
- `update_status_byte`: Checks a global variable and specific bits in a memory structure to perform a bitwise XOR on a byte and update a global flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_constant_to_memory`: Stores a constant value into a memory location calculated from a register.
- `update_global_state_via_register`: Performs bitwise operations on a global variable and updates a memory location using a value from a register.
- `update_global_state`: Modifies a global variable and performs bitwise operations to update a memory location and a calculated value.
- `update_stack_and_status`: Modifies a global variable using a bitwise OR with a return address, performs bitwise masking, and conditionally calls another routine.
- `check_hardware_status`: Checks specific bits in a hardware register and conditionally calls another routine or modifies a value.
- `save_masked_state_and_advance`: Keeps only bits zero and two of a shared word, calls another routine, copies the masked value into a second shared word, increments a pointer-sized shared value and clears a further shared word.
- `set_interrupt_request`: Stores a constant value into a global variable that affects the interrupt request register.
- `sub_03D6`: undescribed.
- `trigger_system_trap`: Executes a software interrupt trap and sets a global variable to a constant value.
- `set_global_flag`: Stores a constant value into a global variable.
- `copy_stack_to_memory`: Copies a block of data from the stack to a global variable and then searches for a specific code pattern in memory.
- `store_word_from_register`: Stores a word from a register into a global variable.
- `execute_infinite_loop_2`: This routine enters an infinite loop that prevents further execution of the program flow.
- `set_timeout_and_wait_flag`: If a shared word is zero, sets a second shared word to nine thousand and waits until a shared flag byte is cleared.
- `update_flag_state`: Checks if a global variable is non-zero and another is zero, then sets the second variable to a specific value and clears a third.
- `initialize_flag_word`: Checks a global variable and, if zero, sets a global word to a specific value and clears another.
- `reset_stack_pointer`: Updates a stack pointer and several global variables based on the state of existing flags and memory values.
- `reset_timer_state`: Sets several global variables and hardware bitmasks when a specific flag is zero, otherwise updates a stack pointer and clears a flag.
- `clear_pending_timer_request`: Records the current stack address in a shared word and clears the pending word.
- `update_hardware_and_memory`: Modifies a bitmask in a hardware register and stores a word from a register into two consecutive memory locations.
- `run_if_flag_set`: Calls another routine only when a global variable is nonzero.
- `sub_0568`: undescribed.
- `sub_057A`: undescribed.
- `sub_057C`: undescribed.
- `sub_0592`: undescribed.
- `sub_05AE`: undescribed.
- `clear_memory_and_flag`: Clears a byte at a specific offset in a structure and resets a global integer to zero.
- `sub_05D0`: undescribed.
- `sub_05FC`: undescribed.
- `sub_0608`: undescribed.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `write_data_to_table`: Writes four values to a table using a pointer.
- `sub_083E`: undescribed.
- `sub_088A`: undescribed.
- `sub_08B6`: undescribed.
- `sub_08C2`: undescribed.
- `sub_090C`: undescribed.
- `sub_093A`: undescribed.
- `sub_094A`: undescribed.
- `sub_0974`: undescribed.
- `sub_09B8`: undescribed.
- `sub_09BE`: undescribed.
- `sub_0A02`: undescribed.
- `sub_0A0E`: undescribed.
- `sub_0A42`: undescribed.
- `sub_0A50`: undescribed.
- `sub_0A5E`: undescribed.
- `sub_0A7C`: undescribed.
- `sub_0A80`: undescribed.
- `sub_0A8E`: undescribed.
- `sub_0AB0`: undescribed.
- `sub_0ACA`: undescribed.
- `sub_0AE0`: undescribed.
- `sub_0AEA`: undescribed.
- `sub_0B02`: undescribed.
- `sub_0B06`: undescribed.
- `sub_0B20`: undescribed.
- `sub_0B42`: undescribed.
- `sub_0B62`: undescribed.
- `sub_0B76`: undescribed.
- `sub_0B8A`: undescribed.
- `sub_0BD0`: undescribed.
- `sub_0C08`: undescribed.
- `sub_0C32`: undescribed.
- `sub_0C68`: undescribed.
- `sub_0C80`: undescribed.
- `sub_0C84`: undescribed.
- `sub_0C8C`: undescribed.
- `sub_0CAA`: undescribed.
- `sub_0CC4`: undescribed.
- `sub_0D02`: undescribed.
- `sub_0D28`: undescribed.
- `sub_0D66`: undescribed.
- `sub_0D6E`: undescribed.
- `sub_0D7A`: undescribed.
- `sub_0D86`: undescribed.
- `sub_0D96`: undescribed.
- `sub_0DA6`: undescribed.
- `sub_0DB2`: undescribed.
- `sub_0DD8`: undescribed.
- `sub_0E14`: undescribed.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `sub_1124`: undescribed.
- `sub_1138`: undescribed.
- `wait_then_set_marker_word`: Waits until a shared word is zero, then sets it to a value whose high byte is 255 and whose low byte is zero.
- `reset_global_integer`: Sets a global integer variable to zero.
- `initialize_pointer_table_2`: Sets several global variables to a constant value and populates a series of memory locations with values from a table.
- `initialize_pointer_table`: Sets a global variable and a local variable to a constant and populates a series of memory locations with sequential pointers.
- `initialize_data_structures`: Populates a series of memory locations with various constants, pointers, and zeroed values using a loop and direct assignments.
- `store_register_value`: Stores a value from a register into a memory location.
- `return_without_action`: The routine consists of only two instructions, a return with no body, so it performs no work and simply returns to its caller.
- `initialize_game_state_and_buffers`: Copies data from one memory block to another and clears several large memory regions.
- `execute_pause_sequence`: Performs a series of conditional checks and bitwise operations on a global variable before executing a long sequence of other routines.
- `execute_callback_sequence`: Executes a sequence of several different routines and performs multiple writes to a global variable.
- `process_data_structure`: Performs conditional checks on a global variable and, if a specific value is found via a pointer, executes a sequence of other routines and updates a global variable.
- `decrement_global_and_call`: Decrements a global variable if it is non-zero and then calls two other routines.
- `reset_global_and_call`: If a global variable is non-zero, it sets it to a specific value, calls two routines, and then resets the global variable to zero before calling a final routine.
- `initialize_and_clear_buffers`: Performs arithmetic on a global variable, clears several large memory blocks using a loop, and modifies several global bytes before executing other routines.
- `sub_1D4A`: undescribed.
- `loop_until_flag_clears`: Executes a sequence of routines and enters a loop that repeatedly calls other routines until a global variable reaches zero.
- `execute_chained_callbacks_2`: Sets a global variable to zero and executes a loop that calls another routine a fixed number of times while incrementing a counter.
- `execute_chained_callbacks`: Decrements a global variable and calls three different routines if the variable is non-zero.
- `copy_data_blocks`: Copies data from four different offsets of a global variable into a destination table using a loop.
- `write_and_wait_flag`: Writes several values to a memory block and then enters multiple loops that wait for a specific bit in a global variable to clear.
- `write_and_wait_flag_alt`: Writes a constant and several other values to a memory block and then waits for a specific bit in a global variable to change via multiple loops.
- `copy_and_wait_data`: Writes several values to a block of memory and then enters multiple loops that wait for a specific bit in a global variable to change.
- `copy_and_wait_data_variant`: Writes two identical values from a global variable to a memory block and then waits for a specific bit in a global variable to change.
- `copy_and_wait_data_inverse`: Writes a constant value and a global variable to a memory block and then waits for a specific bit in a global variable to change.
- `copy_and_wait_for_bit`: Copies a block of data to a new location, writes several values to memory, and then enters multiple loops that wait for a specific bit in a global variable to clear.
- `calculate_offset_values`: Calculates two values based on a global variable and a table, then stores them in two global variables.
- `clear_memory_blocks_2`: Clears two separate blocks of memory by iterating through them and setting multiple longwords to zero.
- `return_immediately_2`: This routine performs no operations and returns immediately to its caller.
- `scale_data_values`: Iterates through a source list of shorts and stores the value multiplied by two into two different locations in a global variable.
- `sub_20AA`: undescribed.
- `set_global_and_spin`: Sets a global word to a value with its high byte filled with ones, then busy-waits until that word becomes zero.
- `rearrange_memory_table`: Iterates through a table of longwords, performing a series of shifts and stores to reorganize the data.
- `initialize_and_spin_wait`: Increments a global variable, performs arithmetic on it, sets four memory locations based on the result, and then enters a busy-wait loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Performs a series of conditional checks, memory copies, and software interrupts before patching code and waiting on a flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_instruction`: The routine is empty and performs no operations.
- `sub_0002`: undescribed.
- `sub_0006`: undescribed.
- `sub_000A`: undescribed.
- `sub_000E`: undescribed.
- `sub_0012`: undescribed.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work, so it does nothing when entered.
- `sub_001A`: undescribed.
- `sub_001E`: undescribed.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning to the caller.
- `sub_0026`: undescribed.
- `sub_002A`: undescribed.
- `sub_002E`: undescribed.
- `sub_0032`: undescribed.
- `sub_0036`: undescribed.
- `sub_003A`: undescribed.
- `sub_003E`: undescribed.
- `sub_0042`: undescribed.
- `sub_0046`: undescribed.
- `sub_004A`: undescribed.
- `sub_004E`: undescribed.
- `sub_0052`: undescribed.

### logged disk load from Disk.1

- `initialize_hardware_state`: Populates a series of memory locations with specific values and configures hardware registers including bitplane, color, and DMA control registers.
- `return_no_op_stub`: The routine consists of a single return instruction and performs no work, so it acts as an empty placeholder.
- `execute_trap_and_copy`: Triggers a software trap and then performs a bulk copy of data from a source table into four separate destination tables.
- `bulk_copy_and_scale`: Copies data from a source table to several destination tables in a loop and then scales a series of words by two before storing them in a new location.
- `execute_infinite_loop_3`: The routine consists of an empty loop that prevents further execution by trapping the processor in an infinite cycle.
- `execute_empty_call`: This routine performs no operations and immediately returns to the caller.
- `update_status_bits`: Performs bitwise operations on a global variable and modifies specific bytes in memory based on the resulting bit patterns.
- `initialize_status_bytes`: Sets a global variable to zero, writes zero to a memory location, and modifies two bytes at specific offsets using bitwise OR and constant assignments.
- `modify_cia_port`: Checks a bit in a global variable, modifies that variable based on the bit, and manipulates bits in a hardware port.
- `clear_cia_port_bits`: Checks bit one of a CIA port byte, sets the byte to all ones if that bit is set, then clears bits one and two of it.
- `mask_and_increment_state`: Masks a global variable with a constant, calls another routine, increments the global variable, and clears a different global variable.
- `set_interrupt_request_2`: Sets a global variable to sixteen.
- `sub_0650`: undescribed.
- `set_global_byte`: Sets a global variable to a specific byte value.
- `set_global_to_ff`: Stores the value two hundred and fifty-five into a global variable.
- `copy_and_search_code`: Copies a block of memory starting from a register to a global variable and then increments a pointer until it finds a specific code address.
- `store_word_to_global`: Copies a 16-bit value from a data register into a global variable.
- `execute_infinite_loop_4`: The routine enters an infinite loop that prevents further execution.
- `set_timeout_and_wait_flag_2`: If a shared word is zero, sets a second shared word to nine thousand and waits until the shared flag byte is cleared.
- `set_shared_flag_word`: If a global variable is non-zero, it stores a word composed of a high byte and that global variable into a memory location and clears a second global variable.
- `update_hardware_and_stack`: If a global variable is zero, it modifies a hardware port, stores a value into two memory locations, and sets a global variable to a specific constant.
- `update_hardware_and_stack_alt`: Modifies a hardware port and stores a byte and its shifted counterpart into two memory locations if a global variable is zero.
- `reset_pending_timer_request`: Stores the current stack pointer into a global variable and then clears that same variable to zero.
- `update_port_and_global`: Modifies a bit in a hardware port register, sets a global variable to a specific value, and stores a word into two consecutive memory locations.
- `run_if_flag_set_2`: Calls another routine only when a specific global variable is non-zero.
- `sub_07E2`: undescribed.
- `sub_07F4`: undescribed.
- `sub_07F6`: undescribed.
- `sub_080C`: undescribed.
- `sub_0828`: undescribed.
- `sub_0832`: undescribed.
- `copy_block_or_return_error`: Returns an error code of minus eight when a shared word is zero.
- `call_another_routine`: Calls another routine and returns.
- `store_result_into_indexed_slot`: Saves its fifth data register into a shared word, calls a helper for a 64-bit result and a second helper with its low half, then stores the saved word into a table slot chosen from the seventh register word, reduced by three and scaled by four.
- `sub_08EA`: undescribed.
- `store_word_run_three_calls`: Stores its data-register argument into a shared word and then makes three helper calls.
- `verify_byte_and_loop`: Checks a byte at a pointer for a specific value and, if valid, enters a loop that increments a byte at a pointer based on bitwise status checks.
- `validate_and_increment_byte`: Checks a byte at a pointer and a sign bit, then enters a loop to increment a byte at a pointer while performing bitwise comparisons.
- `return_negative_one`: Returns a value of negative one.
- `execute_conditional_loop`: Checks a bit in a byte and enters a loop that increments a value at a pointer while performing bitwise comparisons and conditional subroutine calls.
- `process_complex_logic`: Executes a sequence of subroutines including table copying, device state updates, and a loop that increments a byte at a pointer until a specific condition is met.
- `execute_subroutines`: Calls two other routines in sequence before returning.
- `return_error_and_toggle_bit`: Returns a value of negative one and toggles a specific bit within a byte at a memory location determined by a pointer.
- `compare_rotated_words_2`: Compares two 32-bit values by checking their high bytes and performing byte rotations, returning negative five if a specific condition is met.
- `return_negative_five`: Returns a value of negative five.
- `scan_for_sync_with_status`: Returns error codes of minus three or minus four when a status flag is not set or a status byte is nonzero.
- `touch_hardware_registers`: Writes values to the CIAA and CIAB Port A registers and a custom chip register.
- `store_multiple_values`: Stores several values from registers into a structure pointed to by a register.
- `begin_track_read_request`: Reads a stored address and length from a block, resets several bytes to fixed values, calls another routine, and then repeats a helper call with a byte counter until it reaches eleven.
- `check_value_range`: Checks if a longword in d3 matches any of a series of predefined longword constants, returning a specific value in d7 if a match is found or otherwise returning without any modification to d7.
- `update_memory_values`: Performs multiple writes to memory locations using values from a pointer and several constants.
- `program_disk_dma`: Waits for a hardware status bit, then writes several 16-bit values to memory locations and hardware registers.
- `wait_for_disk_ready_bit`: Performs up to fifteen passes checking a status bit, then programs disk DMA control values and returns either zero or an original argument.
- `update_hardware_register`: Retrieves a value from a table and uses it to perform bitwise operations on a hardware register.
- `copy_source_table`: Copies thirty-two words from a source pointer to a destination calculated from a global variable and a base address.
- `copy_memory_block_2`: Copies a block of 32 words from a source table to a destination using a loop.
- `manage_dynamic_buffer_list`: Iterates through a linked list of memory blocks to find a specific marker and then performs data copying or reallocation.
- `calculate_bitwise_expression`: Returns a value calculated by combining bitwise AND and OR operations on two pointers and an input value.
- `update_device_state`: Clears the lowest bit of a byte at a specific offset, increments that byte, and then writes a sixteen-bit value to a device register.
- `modify_device_byte`: Clears the lowest bit of a byte at a specific offset and then increments that byte by one.
- `write_word_wait_for_flag`: Clears the wait flag, writes the low and high bytes of its argument to two device registers, then spins until another routine sets the flag.
- `modify_device_registers`: Updates a byte at a specific offset by masking bits and adds a constant, then writes a byte to another offset.
- `write_device_word`: Writes a 16-bit value to device registers after clearing a wait flag.
- `clear_flag_write_word_bytes_2`: Clears a wait flag and writes the low and high bytes of a 16-bit value to two separate device registers.
- `loop_until_match`: Compares a value from a table against a short integer and loops through a sequence of values until a match is found, conditionally checking bit states.
- `process_conditional_logic`: Checks a value in a table and enters a loop that compares a value against a global variable, conditionally calling another routine based on the parity of a retrieved word.
- `clear_bit_in_memory`: Clears a specific bit in a byte at a memory offset using a bitmask derived from a register.
- `clear_bit_with_mask`: Clears a specific bit in a byte at a memory offset after applying a bitwise OR mask.
- `modify_byte_with_bits`: Checks a byte in a table, sets it to a specific value, modifies bits in another byte based on an index, and loops until a bit in a pointer is cleared.
- `update_byte_and_loop`: Modifies a byte at a specific offset using bitwise operations and then executes a loop for a fixed number of iterations.
- `manipulate_byte_bits`: Performs a series of bitwise OR and AND operations on a byte at a memory offset and returns a word.
- `toggle_device_bit`: Clears a byte at a specific offset, sets its highest bit, performs bitwise masking, and then toggles a specific bit determined by a value.
- `compare_rotated_words`: Compares two 32-bit values by checking their high bytes and performing byte rotations.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `sub_0EE4`: undescribed.
- `invoke_helper_indices_three_six`: Calls another routine for each index from three through six.
- `toggle_table_word`: Fetches a word from a table using a register offset, performs a bitwise XOR with the value, and stores the result back into the same table location.
- `update_device_register`: Calculates a new value by combining a register and a byte, updates a device register, and modifies several memory locations based on a bit test.
- `start_request_if_ready`: Checks status bits of a device register and, if specific conditions are met, fills a request block with a length, a counter, a pointer, and a count before calling another routine.
- `clear_table_entry`: Stores a zero word into a table at an offset determined by a register.
- `get_table_entry`: Returns a word from a table at an offset determined by a register.
- `set_table_entry`: Stores a word from a register into a table at an offset determined by a register.
- `increment_table_entry`: Increments a word stored in a table at an offset determined by a register.
- `frag_move_subq_add_cmp_rts`: Short code fragment that copies d7 to d0, then subtracts 3 from d0, then adds d0 to d0, then compares d6 with an indexed entry from a6, then returns.
- `toggle_cia_bits`: Performs four consecutive calls to another routine.
- `verify_checksum_results`: Performs bitwise calculations and XOR checksums on a table, returning a 64-bit value based on whether two sets of results match.
- `xor_checksum_array`: Performs a bitwise XOR accumulation on a series of words in a table and masks the result.
- `collect_six_position_status`: Collects status for six positions.
- `reset_global_flag`: Checks if a global variable is non-zero, and if so, calls two other routines and clears that global variable.
- `nop_sequence`: The routine consists of two instructions that perform no operation and return immediately.
- `fill_buffer_alternating_bits`: Fills a buffer with a pattern of alternating one and zero bits in every long, covering 832 groups of four longs, then makes eleven pairs of calls to other routines.
- `configure_hardware_registers`: Copies a word from a table to a memory location and writes several specific values to multiple memory offsets.
- `write_constant_to_memory_2`: Writes a specific 32-bit constant to a memory location and executes a loop three times.
- `xor_checksum_array_offset`: Calculates a bitwise XOR sum of a word sequence starting from a specific memory location.
- `update_bitfield_value_2`: Updates a byte by conditionally setting its highest bit based on the state of its own bit and a bit in the preceding byte.
- `modify_byte_flags`: Modifies a byte by masking bits and conditionally setting the highest bit based on the values of the byte and its predecessor.
- `update_bitfield_value`: Performs bitwise operations on a value and stores the result through a pointer.
- `configure_dma_parameters`: Writes multiple values to a structure in memory and modifies the DMA controller register.
- `apply_bit_modifications`: Updates a byte at a pointer location by setting specific bits and modifies a table entry by combining register bits with existing table data.
- `modify_byte_bits`: Sets specific bits within a byte at a location determined by a pointer.
- `modify_byte_with_mask`: Combines bits from a register and a byte from a table, then stores the result back to that table.
- `set_byte_bits`: Sets specific bits in a byte located at an offset from a pointer.
- `wait_and_set_marker_word`: Polls a global variable until it reaches zero, then updates it to a specific marker value.
- `frag_sf_rts`: Short code fragment that clears a fixed memory address, then returns.
- `set_global_flag_3`: Sets a global variable to one and returns a value concatenated from two registers.
