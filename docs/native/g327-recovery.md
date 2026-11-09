# Game 327 (TubeWarriors v1.0 AGA): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00327). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5263680 top-level bytes in 122 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 484 of 484 named (0 author symbols), 484 of 484 described (301 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 998 of 1000 (99.80%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8774 of 8775 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32980 |
| code | 39640 |
| data | 5191048 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 790 | 61 |
| tube | 5 | 7698 | 136 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 127 | 127 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 69 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 80 | 80 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA.hdf: hard-disk image (AmigaDOS OFS, 5760 blocks): its filesystem parses and all 60 files in it are byte-identical to the extracted tree TubeWarriors_v1.0_AGA_hdf, which is recovered file by file; no block outside the filesystem holds data
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/caciara.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (2526 -> 5312 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/caciara2.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (2630 -> 5552 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/diskchange1.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1004 -> 51328 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/diskchange2.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1048 -> 51328 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/diskchange3.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1042 -> 51328 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/dynabyte.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (8966 -> 41024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/facce.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (29538 -> 78752 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione1.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (9810 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione2.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (9448 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione3.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (14420 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione4.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (9028 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione5.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (9436 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione6.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (11528 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione7.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (11606 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/faccione8.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (11306 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/facebox.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (2688 -> 4160 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/ghork.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (134938 -> 427104 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/instruments.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (144846 -> 163996 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/intro.mod.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (180652 -> 223928 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/kenju.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (77058 -> 273312 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/kyoshi.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (110466 -> 362912 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/level1.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (50476 -> 164864 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/level2.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (80614 -> 164864 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/level3.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (56988 -> 164864 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/level4.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (75696 -> 164864 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/mako.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (105626 -> 336800 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/mappa.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (44372 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/metrobrush.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (9532 -> 45024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/metrobrush1.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (8440 -> 41504 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/metrobrush2.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1640 -> 4544 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/mitzuko.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (90742 -> 313568 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/music1.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1754 -> 11324 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/music2.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1788 -> 8252 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/music3.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (1788 -> 8252 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/music4.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (3086 -> 17468 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/ouch1.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (3438 -> 3860 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/ouch3.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (2806 -> 2696 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/ranza.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (119062 -> 371616 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/robby.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (115904 -> 402016 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/satoru.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (104178 -> 283488 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/score.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (2584 -> 7264 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/swish.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (3344 -> 3572 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/titolo.sprite.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (25128 -> 77120 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/versus.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (37088 -> 82944 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0000.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45908 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0001.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (46680 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0002.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (46088 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0003.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45790 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0004.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (46562 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0005.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (46510 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0006.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45570 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0007.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45856 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0008.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45868 -> 65024 bytes)
- TubeWarriors_v1.0_AGA/TubeWarriors_v1.0_AGA_hdf/TubeWarriorsAGA/data/data/tubeintro/TUBE0009.raw.crm: CrM!: Crunch-Mania standard-mode-packed, unpacked with ancient (45732 -> 65024 bytes)
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 41 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_minus_one_in_d0`: Loads the immediate value minus one into data register d0 and returns to the caller, providing a failure or default return value.
- `call_a0_offset_vectors_then_jump`: Stores a0 into a global, calls function pointers at offsets 96, 52 and 100 from a0, then jumps indirectly through a pointer based on a global plus two.
- `clear_ciaa_fire_button_bit`: Clears bit zero of the CIAA port A register at the fire button address, then returns to the caller.
- `load_a3_with_2mb_base`: Loads the immediate value an address into address register a3 and returns to the caller.
- `derive_a0_and_a4_from_pc_relative_pointer`: Reads a longword from a pc-relative location, adds an address to form a0, copies it to a4 and adds another an address to a4, then returns.
- `read_status_word_clear_bits_4_to_6`: Loads a longword from a pc-relative location into d0, clears bits 4 through 6 with an and-immediate, and returns the masked value in d0.
- `test_interrupt_request_and_rte`: Reads the interrupt request register at the custom chip base, restores d0-d1, a0-a1 and a5-a6 from the stack, then returns from the exception with rte.
- `clear_bit4_and_store_word_flag`: Clears bit 4 of the byte at offset 455 in a1, writes d4 to a fixed memory word, sets the following byte to all ones, and returns.
- `store_pointer_and_copy_field_to_flag_area`: Writes d2 to offset 460 of a0, copies the longword at offset 50 of a1 to offset 132, stores d2 and a flag in fixed memory, then skips four stack bytes and returns.
- `store_d4_to_a0_offset_and_pc_table`: Clears bit 13 of d4, writes the result to offset 476 of a0 and to a pc-relative word, sets the following byte to 255, then skips two bytes on the stack and returns.
- `copy_a0_byte6_to_a6_offset_2048`: Reads the byte at offset 6 of a0 into d0 and stores it at offset 2048 of a6, then returns to the caller.
- `set_vposr_bit12_from_pc_flag`: Reads the VPOSR hardware register into d0, compares a pc-relative longword with an address, and sets or clears bit 12 of d0 accordingly before returning.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `decrement_d0_set_borrow_byte`: Subtracts one from the low longword in d0, then sets the low byte of d0 to all ones if the subtraction borrowed or to zero otherwise.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compute_scaled_offset_from_pc_table`: Loads a pointer from a pc-relative table into a0, reads the longword it points to into d0, multiplies d0 by one thousand four hundred forty and adds the longword at offset four of a0.
- `add_scaled_a0_offset_to_d0_store_a2`: Divides the longword at offset eight of a0 by fifty, sign-extends the quotient, adds it to d0, stores the result at offset sixty-four of a2, then restores d2 and a2-a3 and returns.
- `toggle_bit7_at_a3_offset_61`: Flips bit 7 of the byte at offset 97 in the a3 structure, then sets d0 to all ones if the old bit was clear, otherwise zero, and returns.
- `store_bit_test_result_to_a1_offset`: Clears d0, reads a bit index from a3 offset 99 and a bit mask from pc-relative memory, tests the bit, stores the inverted zero flag as a longword at a1 offset 32, then skips the return address adjustment.
- `push_pc_relative_long_plus_offset`: Pushes the longword at the program counter relative location onto the stack, adds a fixed immediate value to it, and returns, leaving the adjusted value on the stack.
- `set_state_ptr_to_offset_14c`: Loads the address of the structure field at offset 332 into a0, stores it at offset 16 of the a3 context, sets bit 1 of the byte at offset 96, skips two bytes on the stack and returns.
- `toggle_bit_and_advance_counter`: Reads a bit index from offset 99 of a3, clears that bit in a global bitmap, and if it was set, sets bit 1 at offset 96, conditionally increments the counter at offset 60 and calls a routine before clearing the bit.
- `store_whdboot_string_pointer`: Loads the address of the WHDBoot.ss string into a0, writes that pointer into memory at offset minus 216 from a0, then loads another nearby address into d1 and returns.
- `open_dos_library_and_call_vectors`: Saves registers, opens dos.library via exec OldOpenLibrary using a name string, then calls two library vectors with the passed arguments and returns.
- `allocate_and_enqueue_whdload_structures`: Allocates two memory blocks with AllocMem, fills in tags and pointers, opens expansion.library, sets up a jump table and enqueues a message on the whdload device list.
- `open_dos_and_jump_to_offset`: Calls the dos.library open vector with a name string, loads a pointer from offset 22 of the returned structure and jumps to it.
- `open_dos_and_create_message_port`: Opens dos.library, allocates a message port named DOS, stores it in a5, patches the task's port list, then waits for and dispatches incoming messages until a matching command arrives.
- `read_joystick_and_compare_state`: Calls two input routines, copies d7 into d0 and d1, loads the current state from offset 28 of a4, calls a comparison routine, shifts the result right by two and branches to the main loop or exit.
- `read_input_and_exit_with_minus_one`: Calls an input routine, copies d7 into d0, calls another routine, sets d0 to minus one and branches back to the main loop exit.
- `load_and_validate_game_state`: Calls three setup routines, checks the returned structure's flag at offset 28, invokes a2 callbacks at offsets 124 and 88, then calls a cleanup routine and returns minus one or zero in d0.
- `cleanup_and_return_zero_with_mask`: Moves d7 into d0, calls the cleanup routine, sets d0 to zero and d1 to the bitwise complement of 39, then branches back to the common exit.
- `copy_state_or_whdload_name`: Calls two setup routines; if a0 is zero it copies the WHDLoad string into d7 offset 8, otherwise copies 57 longs from a0 offset 24 into d7, then calls two more routines and returns minus one.
- `build_path_and_call_twice`: Calls a setup routine, then uses d7 as an object pointer to call a function through a2, and calls another routine twice with a0 pointing at offsets 8 and 144 from that object before branching away.
- `extract_dirname_and_call`: Calls a check routine, scans the string at offset 4 of the d7 object for its length, walks back to the last slash, copies the directory prefix onto the stack, calls a routine with it, and stores results at offsets 12 and 16 of a4.
- `compare_two_object_strings`: Calls two routines to fetch two objects, compares the strings at offset 4 of each byte by byte, and returns all-ones in d0 when they match or zero when either is missing or they differ.
- `find_first_nonempty_string_in_list`: Loads a1 into d0, returns zero if a1 is null or the byte at a1 is zero, otherwise follows the pointer at offset 4 of a1 and returns zero in d0 and d1 before branching away.
- `copy_stream_bytes_into_buffer`: Reads the current stream position and length from the structure at a4, clamps the copy size, advances the position, copies that many bytes into the buffer at offset 24 of a4, and returns the copied count in d0.
- `advance_stream_position_and_update_max`: Adds the length at offset 28 of a4 to the stream position at offset 20 of the structure, stores the new position, updates the maximum at offset 148 if exceeded, and returns the length in d0.
- `set_d2_minus_two_and_branch`: Loads minus two into data register d2 and branches unconditionally to the following routine, setting up its entry state.
- `open_handle_store_result_or_fail`: Calls setup helpers with d7, tests the returned longword, stores it at offset 36 of the d7 structure on success, and returns minus one on failure.
- `close_handle_and_chain_to_open`: Calls helpers with d7, closes the returned handle via a2 and the OS library, then branches into the open-handle routine to continue.
- `load_a4_offset20_and_call_c12`: Loads the longword at offset 20 from a4 into d0, calls the routine at c12, then sets d0 to 255 and branches to 738.
- `load_a4_fields_and_add_a0_offset148`: Loads a0 from offset 20 of a4, d2 from offset 24, d1 from offset 28; if d1 is positive, adds the longword at offset 148 of a0 to d2, then branches to b28.
- `add_a0_offset20_to_d2_and_clamp`: Adds the longword at offset 20 of a0 to d2, compares with offset 148 of a0, and if not above stores d2 there, returns d0 and clears d1 before branching to 734.
- `set_d0_ff_and_d1_e4_branch`: Loads 255 into d0 and 36 into d1, inverts the low byte of d1 to 219, then branches to another routine.
- `load_a4_offset20_shifted_left_two`: Reads the longword at offset 20 from a4, shifts it left by two bits into d7, and returns to the caller.
- `load_a4_offset24_shifted_left_two`: Reads the longword at offset 24 from a4, shifts it left by two bits into d7, and returns to the caller.
- `load_a4_offset_1c_shifted_left_two`: Reads the longword at offset 28 from the a4 base pointer, shifts it left by two bits into d7, and returns.
- `shift_bytes_backward_and_store_count`: Copies bytes from a0 forward into the preceding positions until a zero byte is seen, then writes the byte count into the slot before the shifted block and returns.
- `open_and_initialize_audio_channel`: Calls a setup routine, opens a library resource, fills a structure with channel fields and saved registers, and returns the structure pointer or zero on failure.
- `close_library_and_return`: Tests d0 for a library base; if non-zero, pushes the library's base pointer from offset four, calls the close-library vector with d0 set to -4, then pops the base and jumps to the expunge vector.
- `build_path_and_trim_slashes`: Allocates memory via the exec allocator for a combined directory and filename, copies both parts separated by a slash, then collapses duplicate slashes and strips trailing ones, returning the buffer or an error code.
- `check_whdboot_files_present`: Calls a file-existence check for WHDBoot.exe and WHDBoot.ss using the given name and length pairs, returning zero only if both checks succeed.
- `compare_null_terminated_strings`: Saves a0 on the stack, compares bytes at a0 and a1 until a mismatch or a zero byte, then restores a0 and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_pc_relative_long_plus_32`: Pushes the longword stored at the program-counter-relative location onto the stack, adds 32 to it, and returns.
- `open_dos_library_and_run_game`: Saves registers, opens dos.library, loads the Tube executable, runs it via its entry vector, then closes the library and exits.
- `zero_filled_data_area_ef8`: Twelve zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_word_data_slot_f28`: A single zero word that decodes as ori.b #0,d0; it is a zero-filled data slot referenced by address, not executable code.
- `patch_three_embedded_branch_opcodes`: Saves all registers, locates two code blocks via a helper, then replaces three embedded branch opcode longwords at offsets in the found block with no-operation pairs, restoring registers and returning.
- `find_node_by_id_in_linked_list`: Walks a linked list starting at a0, comparing each node's previous word to d0; returns the matching node pointer in a0 and d0, or zero when the list terminator is reached.

### tube

- `open_dos_and_graphics_libraries`: Opens dos.library and graphics.library via exec.OpenLibrary, stores their bases in globals, and copies the graphics library's two fields at offsets 38 and 34 into globals; returns 1 on failure.
- `allocate_buffers_and_load_game_assets`: Allocates memory with exec.AllocMem, clears and copies screen buffers, loads the game's raw and crm asset files, sets up copper and interrupt state, and initializes the level data structures.
- `main_game_frame_update`: Runs the per-frame game logic: initializes copper and display, updates player and enemy state, handles input flags, and calls many subroutines to draw and animate the playfield.
- `level_start_reset_and_display_setup`: Disables interrupts, sets the copper list and bitplane pointers, clears state buffers, initializes player and enemy variables, and jumps to the main frame loop.
- `advance_level_and_scroll_display`: Increments the level counter, sets scroll and bitplane registers, copies sixteen words from the level data into the copper list, and returns.
- `reset_tube_warriors_level_state`: Decrements a counter, calls two setup routines, copies 16 words from one table to another, clears eight word variables, sets two words to 34816 and derives a masked word, then conditionally adjusts two coordinates and sets two words to 40.
- `spawn_tube_segment_chain`: Looks up a segment record from a table indexed by d0, computes its length and flags, writes position and length fields, then fills four linked segment entries in the a1 output list with coordinates adjusted by the segment width.
- `draw_tube_segment_with_shadows`: Checks two enable flags, masks d0, halves d1 and d2, fetches a segment record, calls four blit routines for shadow and body, then writes position, length and flag bytes into the a2 segment buffer and a1 output list.
- `swap_and_init_two_tube_pointer_sets`: Saves registers, rotates three word globals, copies two eight-long pointer blocks to backup areas, then indexes the table at a0 by d0 to fill two pointer sets with header size and data offsets.
- `init_second_tube_pointer_set`: Saves registers, indexes the table at a0 by d0, reads the masked header size into d1, and fills the second pair of pointer sets with data offsets without touching the first pair.
- `swap_globals_and_render_two_tubes`: Swaps many word, byte and long globals in pairs, sets a flag byte to two, then twice calls the pointer-setup routine and a render routine for the two tube pointer sets.
- `copy_sprite_ptrs_and_call_blit`: Copies four longwords from the d3 structure and four from a fixed table into globals, optionally subtracts four from the second set, then pushes arguments and calls the blit routine.
- `wait_for_vertical_blank_loop`: Loads the graphics library base from address 8 into a6, calls the vertical blank wait function at offset minus 384, and repeats while the returned value is less than d1.
- `cycle_three_sprite_positions`: Increments a counter, and every twentieth call rotates three pairs of longword globals and writes computed x and y words into display copper list registers.
- `restore_system_and_free_audio_buffers`: Frees many memory blocks via exec.FreeMem, clears two pointers, disables and re-enables interrupts through INTENA/INTREQ, restores the old level 3 vector, and calls two subroutines with arguments 1 and 3.
- `init_playfield_and_clear_tilemap`: Calls a setup routine, ORs mask bits into eight rows of four bitplanes, computes two tile pointers from indices, copies tile data, stores screen dimensions, and clears a tilemap buffer.
- `blit_tiles_into_bitplanes_and_buffers`: Copies tile data from a source buffer into four bitplane rows, remaps bytes through a lookup table, and duplicates the playfield into several destination buffers using nested dbra loops.
- `jump_set_flag_bit1`: Jumps to the shared flag routine that sets bit 1 of the d4 flag byte when d0 is nonzero, otherwise clears it, and stores 2 in a global.
- `jump_set_flag_bit3`: Jumps to the shared flag routine that sets bit 3 of the d4 flag byte when d0 is nonzero, otherwise clears it, and stores 8 in a global.
- `adjust_sprite_position_and_frame`: Uses d0 and d3 to update the nibbles of bytes 2 and 3 at a0, then adds a table offset to four coordinate pairs at offsets 6 to 26 of a0.
- `update_tube_sprite_offsets`: Adds d0 to the low nibble at offset 3 of a0, and on carry adjusts the byte at offset 2 and writes four sprite coordinate pairs from a table indexed by d3 into a0.
- `tube_warrior_retreat_and_flip_direction`: Subtracts d0 from the byte at offset 3 of the object at a0, or on underflow rebuilds the nibble and steps the byte at offset 2 down by 68, wrapping to 204 and toggling the direction bit and screen offsets.
- `tube_warrior_advance_and_flip_direction`: Adds d0 to the byte at offset 3 of the object at a0, and on carry duplicates the low nibble, steps the byte at offset 2 up by 68, wrapping to 136 and toggling the direction bit and screen offsets.
- `select_sprite_base_by_4c_bits`: Reads the low three bits of the longword at address 76, compares them with 4, and loads either the pointer at 1644 or the pointer at 1652 plus 8 into a1 before continuing.
- `copy_eight_offsets_added_to_d3_4c`: Loads the word at address 76 into d1 sign-extended, then for eight longwords from (a1) adds d1 and stores the high and low words into a0 offsets 2 through 62.
- `copy_four_offsets_after_skipping_five`: Skips five longwords from (a1), then for four longwords adds d1 and stores the high and low words into a0 offsets 2 through 30, returning.
- `set_sprite_positions_from_index`: Multiplies d0 by 80, adds the result to four longwords from a table and stores the split high and low words into sprite fields at offsets 2 through 30 of a0.
- `wait_for_vblank_and_ack_interrupt`: Sets the byte at 5521 to 1, spins until it clears, calls a routine, waits until the word at 5392 is zero, clears 5521 and writes 4 to INTREQ to acknowledge the interrupt.
- `vblank_interrupt_copy_scroll_registers`: Acknowledges interrupt 4 in INTREQ, copies the words at 634, 638 and 640 into 32622, 32590 and 32606, stores the masked OR of 634 and 636 into 30402, sets 643 to 1, restores registers and returns from the exception.
- `vblank_interrupt_dispatch`: Reads INTREQR, acknowledges vertical blank or copper interrupts in INTREQ, decrements two counters, calls sound and display routines, then restores registers and returns from the exception.
- `irq_restore_regs_and_rte`: Tests bit 6 of d0; if clear, pops d0-d4 and a0-a3 from the stack and returns from the interrupt with rte, otherwise branches away to other code.
- `copper_list_upload_and_blitter_restart`: Checks INTREQR bit 14, toggles the copper list pointer, acknowledges the interrupt, then copies a copper list from memory into the custom chip registers and restarts the blitter before returning from exception.
- `level3_interrupt_acknowledge_rte`: Tests bit 7 of d0, saves d0-d3/a0-a3, reads INTREQR, checks bit 5 and branches to the level 3 handler if set, otherwise restores registers and returns from interrupt.
- `tube_rotate_left_right_update`: Checks direction bits, decrements or increments the rotation counter and angle, redraws the tube segments via helper calls, adjusts the scroll offset and returns.
- `set_tube_color_nibble_all_segments`: Masks d3 to a nibble, writes the doubled nibble to the color register and loops over 35 tube segments setting the low nibble of each segment color byte.
- `apply_palette_and_scroll_offsets`: Reads the shared parameter word, writes nibble-derived colour bytes into 40 palette entries spaced 40 bytes apart, then stores three scroll positions computed from the parameter into the display register pairs.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; it is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_word_second`: Another single zero word decoding as ori.b #0,d0; a zero-filled data area referenced by address, not executable code.
- `open_debug_and_alloc_two_buffers`: Saves registers, opens the debug library, allocates two memory buffers via exec.AllocMem, and on failure frees them and returns zero; otherwise stores the buffer pointers and returns the second allocation.
- `push_d1_and_load_delay_count`: Pushes the current d1 register onto the stack and loads d1 with the immediate value an address, leaving it for the caller to use as a delay count.
- `decompress_rle_stream_to_buffer`: Decodes a run-length compressed byte stream from a2 into the buffer ending at a1, using d1 and d2 as offsets, writing bytes backwards and returning the last decoded word in d0.
- `shift_bitstream_word_with_refill`: Decrements d7; when it reaches zero reloads d7 with 16 and shifts the 32-bit d6 right by one, swapping in a new word from -(a2); otherwise returns d0 unchanged after shifting d0 right by one.
- `extract_masked_bitfield_word`: Copies d6 to d0, shifts d6 right by d1, subtracts d1 from d7, rotates d6 by d7 and refills from -(a2), then masks d0 with a table word indexed by doubled d1.
- `intro_sequence_disk_and_copper_setup`: Runs the tube intro: allocates and frees chip memory, loads raw.crm and sprite files, builds copper lists and bitplanes, polls joystick fire and disk status, and enables interrupts before jumping to the next stage.
- `load_tube_level_and_setup_display`: Loads level graphics and sprite files, copies tile rows into chip memory, installs the copper list and interrupt handler, and runs the main input and selection loop.
- `fade_palette_colors_down`: Repeatedly decrements the red, green and blue nibbles of each palette entry in the color table and calls the graphics library to update the display.
- `spawn_three_sprite_objects`: Reads a sprite descriptor from the object table, sets its position and color fields, then writes three linked sprite entries into the caller's structure.
- `init_tube_segment_display_entries`: Looks up a tube segment pointer from a table indexed by d0, stores its address in globals, and fills four sprite entries with height, position and control bytes.
- `toggle_and_update_four_tube_segments`: Fetches a tube segment pointer by d0, flips the high bit of its height word, and calls the update routine on four consecutive segment entries.
- `vblank_irq_swap_tube_buffers`: Acknowledges the interrupt, counts to three, then swaps two pairs of word buffers across twenty-seven entries and returns from the exception.
- `wait_for_flag_then_clear`: Spins while the byte at absolute address 6059 is zero, then writes zero to that byte and returns.
- `load_and_display_congratulations_screen`: Allocates memory, builds a copper list, copies a bitmap into a buffer, disables and enables the OS, installs the copper list, waits for the copper, frees memory and jumps back to the main loop.
- `blit_text_glyphs_into_bitmap`: Reads characters from a string, looks up each glyph in a font table, and ORs or ANDs the glyph rows into two bitmap planes at the given offsets.
- `clear_tube_columns_by_scroll_offset`: Saves registers, reads the scroll offset from the object, computes a row offset, then fills four plane buffers with zero or all-ones words across 110 rows to blank the tube columns.
- `save_registers_and_jump_to_clear`: Moves the stack pointer into a0, pushes d0-d7 and a1-a6 onto the stack, then branches to the shared clear routine.
- `blit_tube_columns_with_flip`: Saves registers, clamps the vertical position, copies three 20-byte rows per column from source planes into the destination planes, toggles the flip word, and fills the last column with all-ones.
- `blit_8x8_tile_rows_to_screen`: Copies eight rows of tile pixels from source to destination bitplanes using per-row offsets, toggling the copper color register after each row.
- `blit_8x8_tile_rows_with_clip`: Copies eight rows of five longwords from source to destination bitplanes, clipping the row count to the 180-line screen limit and toggling the copper color register per row.
- `clear_8x8_tile_rows_with_clip`: Writes zero longwords into eight rows of destination bitplanes, clipping the row count to the 180-line screen limit and toggling the copper color register per row.
- `clear_tube_columns_and_flip_mask`: Saves registers, computes a column offset from the tube width and scroll position, then writes zeroed longwords down each of four columns and toggles a mask word in memory.
- `build_and_draw_tube_segments`: Saves registers, reads tube and sprite descriptors, computes segment positions and blitter words into the copper list, and calls the segment draw routine for each active bit.
- `reset_tube_state_and_jump`: Saves registers, clears the tube state word in memory, then jumps to the tube setup routine to continue initialization.
- `setup_two_blit_planes_from_rect`: Reads rectangle fields from the caller frame, computes blit modulo and control words, fills two blitter descriptors in the shared structure and invokes the blit routine twice.
- `setup_two_blit_planes_from_sprite`: Reads sprite header words and three pointers, builds two blitter descriptors with fixed shifts and modulo, then calls the blit routine for each.
- `wait_blitter_and_start_copy_blit`: Waits until the blitter busy bit clears, then programs blitter control, modulo, pointers and size registers to start a copy blit.
- `save_regs_and_set_render_mode`: Copies a7 into a0, pushes d0-d7 and a1-a6 onto the stack, then stores the longword 1 at the global render mode flag.
- `render_tube_scanline_spans`: Reads tube geometry from the structure at a0, adjusts scroll offsets and clip words in globals, then fills copper span entries via a2 and calls the span writer for each row.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this address is a zero-filled data area referenced by other code, not executable instructions.
- `ori_b_immediate_data_byte`: Performs a bitwise OR of the immediate byte value 178 into the low byte of d0, then falls through into the following routine.
- `set_clip_limit_and_jump`: Stores the long immediate value 45 at absolute address 22320, then branches unconditionally to the code at 22352.
- `render_tube_segment_scanlines`: Clips a tube segment against stored start and end limits, fills copper list entries with bitplane pointers, colors and scroll values, then calls the line blitter for each of four scanline groups.
- `queue_display_list_entry`: Waits while the DMACONR bit 14 is set, then copies a 36-byte display list entry from the current pointer to the next slot, clears its last word, advances the pointer and increments the entry counter.
- `expand_eight_bytes_via_lookup_table`: Reads eight source bytes, maps each through a byte lookup table in reverse order, writes eight expanded bytes to the destination, and repeats for the given count.
- `blit_tube_sprite_clipped`: Checks the sprite rectangle against the tube bounds, sets the collision flag, then shifts and ORs table-translated pixel bytes into the destination bitplane words for the overlapping rows.
- `clear_playfield_bitplane_block`: Saves registers, takes the plane pointer from the caller's stack frame, adds 1024 bytes and writes 45 longword groups of zero registers backwards, clearing a block of playfield memory.
- `clear_bitplane_pixel_across_four_planes`: Builds a rotated bit mask from d0 and d1, then ANDs it into nine word offsets spaced 56 bytes apart in each of four bitplane pointers, clearing one pixel column.
- `set_bitplane_pixel_across_four_planes`: Builds a rotated longword mask from d0 and d1, then ORs it into nine longword offsets spaced 56 bytes apart in each of four bitplane pointers, setting one pixel column.
- `init_audio_channels_and_voice_tables`: Stores the module base pointer, scans 128 bytes for the highest value, builds 31 voice pointer entries, clears four audio volume registers, sets CIAA port bit, and calls four voice setup routines.
- `silence_all_audio_channels`: Clears the volume registers of all four audio channels and writes 15 to DMACON to disable the audio DMA channels before returning.
- `advance_audio_sequence_step`: Saves registers, increments the sequence counter byte, compares it with the limit byte, resets it when exceeded, and branches to the next or restart sequence routine.
- `init_audio_channels_from_song_data`: Checks four channel-enable bits in a global flag and, for each set bit, points a5 at that channel's audio hardware and calls the voice setup routine with a per-channel structure.
- `update_audio_voice_and_advance_song`: Loads a voice structure from song data, sets sample pointers and lengths in the audio hardware, handles effects, writes DMACON, advances the song position and returns.
- `dispatch_audio_effect_by_type`: Calls the voice update helper, reads the effect type nibble from the voice structure and branches to the matching effect routine, writing the period to the audio hardware.
- `tube_position_clamp_and_output`: Reads word at offset 16 of the object, clamps its low twelve bits between 113 and 856, preserves the high nibble, and stores the result at offset 6 of the caller's structure.
- `tube_target_lookup_by_position`: Scans a 74-byte table row selected by byte 18 for the first entry above the masked position, stores the found target at offset 24 and sets direction flag at offset 22.
- `tube_move_toward_target_and_snap`: Moves the position at offset 16 toward the target at offset 24 by the step in byte 23, snaps on arrival, then re-looks-up the table entry and writes it to offset 6.
- `tube_control_byte_dispatch`: Reads control bytes at offsets in the a6 structure, scales and clamps values, updates words at offsets 6 and 8 of a5, and branches to sub-handlers based on the control byte.
- `dispatch_control_nibble_commands`: Calls a setup routine, then switches on the low nibble of 2(a6) and the high nibble of 3(a6) to branch to command handlers, including toggling the CIAA_PRA fire bit and updating bytes 30(a6) and 31(a6).
- `store_low_nibble_to_byte_18`: Masks the low nibble of the byte at 3(a6) and writes it to byte 18(a6), then returns to the caller.
- `advance_tube_scroll_on_beam_overflow`: Reads high nibble of byte at offset 31 of the object, adds a table value to the byte at offset 35, and when bit 7 sets, clears it and advances the scroll position at offset 36 to the value at offset 10 if it passed the limit, storing it through a5.
- `wait_three_vertical_position_changes`: Reads VHPOSR into d1 and compares it against VHPOSR repeatedly, looping up to four times until the vertical position changes, then returns.
- `set_dma_audio_enable_flag`: Tests d0; if nonzero sets bit 0 of the flag word at 7652 and writes 1 to DMACON, otherwise clears bits 1-3 of that flag and writes 1 to DMACON, then returns.
- `toggle_disk_dma_by_d0`: Tests d0 byte; if nonzero sets bit 1 in the word at 7652 and writes 2 to DMACON, otherwise clears that bit and writes 2 to DMACON.
- `toggle_audio_dma_by_d0`: Tests d0 byte; if nonzero sets bit 2 in the word at 7652 and writes 4 to DMACON, otherwise clears that bit and writes 4 to DMACON.
- `toggle_bitplane_dma_by_d0`: Tests d0 byte; if nonzero sets bit 3 in the word at 7652 and writes 8 to DMACON, otherwise clears that bit and writes 8 to DMACON.
- `player_one_input_and_script_advance`: Reads player one joystick and fire from CIA and JOY registers, updates movement and script pointers, and advances the player one script byte stream.
- `player_two_input_and_script_advance`: Reads player two joystick and fire from CIA and JOY registers, updates movement and script pointers, and advances the player two script byte stream.
- `script_command_dispatch`: Dispatches script command codes in d1 to set flags, load script pointers, update score and speed, and call subroutines for player movement.
- `wait_until_fire_button_released`: Reads the CIAA port A fire button bit and branches back to itself while bit 6 stays set, then returns once the button is released.
- `select_animation_frame_from_input_bits`: Tests bits of d0 and the current d1 value to choose and return a new d1 frame index, returning early if a global flag is set.
- `update_tube_warrior_state_machine`: Checks global flags and the a0 state bytes, compares d1 against many values, sets d1 and a0 fields, and calls two routines to swap pointers and update state.
- `tube_warrior_state_timer_update`: Compares d1 to 1; if not equal clears byte at offset 2 of a0 and returns, otherwise checks and increments a counter byte at offset 3 of a0 against two global flags, clearing d1 on completion.
- `tube_warrior_spawn_slot_scan`: Checks global flags, clears a pending slot, sets CIAA control register bit, reads CIAA timer low byte into d0, then walks an eight-byte-stride table comparing accumulated counts and horizontal distance to pick a spawn entry, returning d1 and d2.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `zero_filled_data_area_2`: Single zero word that decodes as ori.b #0,d0; this address is a zero-filled data area referenced by other code, not executable instructions.
- `zero_filled_data_word_at_1590`: This single zero word decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by address, not executable code.
- `zero_fill_word_at_1594`: This single zero word decodes as ori.b with immediate zero on d0; it is zero-filled data referenced by address, not executable code.
- `zero_fill_word_at_1598`: One zero word at this offset decodes as ori.b zero to d0; the bytes are padding data pointed to by absolute references, not a real routine.
- `zero_fill_data_word_at_159c`: A single zero word that decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by address, not executable code.
- `zero_fill_data_word_at_15a0`: One zero word in the same zero-filled data area, decoded as ori.b immediate zero to d0; referenced by address from four callers, not real code.
- `zero_fill_data_word_at_15a4`: Another zero word of the zero-filled data region, decoding as ori.b immediate zero to d0; used as an address and handler pointer, not executable.
- `zero_fill_word_at_15a8`: A lone zero word here decodes as ori.b immediate zero on d0; it is part of a zero-filled data area reached through handler pointers, not code.
- `zero_word_data_at_15ac`: One zero word at this offset, decoding as ori.b immediate zero to d0; the bytes are a zero-filled data area pointed to by absolute references.
- `zero_word_data_at_15b0`: One zero word referenced by absolute addi, cmpi and move operands plus a hunk data pointer; the bytes are filler data, not a real instruction.
- `zero_word_data_at_15b4`: A lone zero word reached through an absolute move operand and a data handler pointer; it is padding data rather than runnable code.
- `zero_filled_data_word_at_15b8`: One zero word taken as an address by two callers; the bytes are a zero-filled data area, not a real instruction sequence.
- `zero_filled_data_word_at_15bc`: A lone zero word in a zero-filled data region, referenced by address and handler pointers; it is data, not executable code.
- `zero_filled_data_word_at_15c0`: A single zero word that decodes as ori.b with immediate zero to d0; it is data, not executable code, referenced by address.
- `zero_fill_word_at_15c4`: This single zero word decodes as an ori.b immediate instruction but is really zero-filled data referenced by address from six callers.
- `zero_fill_word_at_15c8`: This lone zero word is data, not code, sitting in the zero-filled table and reached by address from three callers.
- `zero_filled_data_words_at_15cc`: Two zero words that decode as ori.b immediate zero to d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_15d4`: A single zero word decoding as ori.b immediate zero to d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_words_at_15d8`: Two zero words decoding as ori.b immediate zero to d0; this is zero-filled data referenced by address, not executable code.
- `write_audio_channel_registers`: Computes the audio channel register base from the channel index, stores pointer, period, volume and length into the four channel registers, then enables that channel's DMA bit in DMACON.
- `enable_audio_dma_channel`: Reads the channel number argument, builds a single DMA enable bit for that channel and writes it to DMACON to turn on that audio channel.
- `write_audio_channel_length`: Computes the audio channel register base from the channel index and writes the length value into the channel's length register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_65`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_66`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_67`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_68`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_69`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_70`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_71`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_72`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_73`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_74`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_75`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_76`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_77`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_78`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_79`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_80`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_81`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_82`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_83`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_84`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_85`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_86`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_87`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_88`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_89`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_90`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_91`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_92`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_93`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_94`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_95`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_96`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_97`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_98`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_99`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_100`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_101`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_102`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_103`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_104`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_105`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_106`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_107`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_108`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_109`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_110`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_111`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_112`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_113`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_114`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_115`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_116`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_117`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_118`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_119`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_120`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_121`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_122`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_123`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_124`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_125`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_126`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_127`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_128`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_129`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_130`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_131`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_132`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_133`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_134`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_135`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_136`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_137`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_138`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_139`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_140`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_141`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_142`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_143`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_144`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_145`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_146`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_147`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_148`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_149`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_150`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_151`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_152`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_153`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_154`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_155`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_156`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_157`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_158`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_159`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_160`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_161`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_162`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_163`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_164`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_165`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_166`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_167`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_168`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_169`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_170`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_171`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_172`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_173`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_174`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_175`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_176`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_177`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_178`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_179`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_180`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_181`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_182`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_183`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_184`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_185`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_186`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_187`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_188`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_189`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_190`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_191`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_192`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_193`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_194`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_195`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_196`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_197`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_198`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_199`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_200`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_201`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_202`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_203`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_204`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_205`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_206`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_207`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_208`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_209`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_210`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_211`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_212`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_213`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_214`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_215`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_216`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_217`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_218`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_219`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_220`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_221`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_222`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_223`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_224`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_225`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_226`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_227`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_228`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_229`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_230`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_231`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_232`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_233`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_234`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_235`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_236`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_237`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_238`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_239`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_240`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_241`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_242`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_243`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_244`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
