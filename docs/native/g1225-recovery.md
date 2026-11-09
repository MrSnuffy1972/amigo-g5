# Game 1225 (LordsOfTheRisingSun v1.3b 0133): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01225). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3904233 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 715 of 715 named (0 author symbols), 715 of 715 described (263 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4305 of 4305 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 972257 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10104 of 10104 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 534099 |
| code | 37646 |
| data | 3332432 |
| padding | 56 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 1090 | 72 |
| main | 84 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 304 | 243 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 83 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2886 | 75 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5547 | 301 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| RisingSun.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| Tiles.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| Tiles6.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- LordsOfTheRisingSun_v1.3b_0133/LordsOfTheRisingSun_v1.3b_0133.hdf: hard-disk image (AmigaDOS OFS, 4096 blocks): its filesystem parses and all 15 files in it are byte-identical to the extracted tree LordsOfTheRisingSun_v1.3b_0133_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 213 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 83 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 879 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2287 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- LordsOfTheRisingSun_v1.3b_0133/LordsOfTheRisingSun_v1.3b_0133_hdf/LordsOfTheRisingSun/data/RisingSun.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LordsOfTheRisingSun_v1.3b_0133/LordsOfTheRisingSun_v1.3b_0133_hdf/LordsOfTheRisingSun/data/Tiles.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LordsOfTheRisingSun_v1.3b_0133/LordsOfTheRisingSun_v1.3b_0133_hdf/LordsOfTheRisingSun/data/Tiles6.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LordsOfTheRisingSun_v1.3b_0133/LordsOfTheRisingSun_v1.3b_0133_hdf/LordsOfTheRisingSun/data/system-configuration: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- LordsOfTheRisingSun_v1.3b_0133/LordsOfTheRisingSun_v1.3b_0133_hdf/LordsOfTheRisingSun/data/main: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 972257 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_minus_one_at_entry`: Loads the immediate value minus one into d0 and returns immediately, providing a constant return value to the program entry caller.
- `store_base_and_init_library_vectors`: Saves the incoming a0 base pointer to memory, keeps it in a5, calls library offsets 96, 52 and 100, stores a callback pointer, then jumps through the base pointer's second vector.
- `set_two_base_offsets_and_jump`: Loads the base pointer from memory, adds an address to form a4, stores a4 at offsets 6 and 18 of a5, adds an address, calls a routine, then jumps to a5.
- `set_a3_to_80000_and_jump_a5`: Loads the immediate value an address into address register a3, then jumps to the address held in a5 without saving a return address.
- `read_and_mask_d0_flags`: Loads a longword from program-counter-relative memory into d0, clears bits 4 through 6 with an and-immediate, and returns that masked value.
- `test_intreqr_and_check_stack_bit5`: Reads the interrupt request register at the custom chip base, tests bit 5 of a byte on the stack, and returns without changing other registers.
- `acknowledge_interrupt_request`: Reads the word on top of the stack and writes it to the INTREQ hardware register, then advances the stack pointer by two and returns.
- `push_a3_and_return_zero_in_d0`: Allocates eight bytes on the stack, saves a3 and the return address, loads a3 with a nearby code address, sets d0 to zero and returns.
- `set_bit1_in_a1_offset_and_return`: Pops d1 from the stack, pushes a nearby code address, restores d1, sets bit 1 of the byte at offset fourteen from a1, and returns.
- `load_custom_chip_base_and_branch`: Loads the custom chip register base address into a0 and branches to another routine that continues setup.
- `store_d0_to_scratch_and_load_ciab`: Stores the d0 value into a scratch longword near the program counter, then loads the CIAB base address into a6 and returns.
- `copy_scratch_to_cop2lch_and_jump`: Copies the stored scratch longword into the COP2LCH hardware register, then jumps indirectly through a0 to the next routine.
- `copy_a0_offset6_byte_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 2048 from a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_pc_relative_value_for_4_or_1`: Loads the longword at a fixed pc-relative location, compares it against two constants, and returns 4 if it matches either, otherwise 1 in d0.
- `store_minus_two_and_advance_a3_pointers`: Writes the value minus two to the a3 buffer twice, advances a3, updates pointers at a4 offset 132 and a2 offsets 50 and 54, clears a4 offset 386, and skips the return address by adding fourteen to the stack pointer.
- `copy_two_pointer_fields_to_a3`: Reads two pointers from offsets 4 and 8 of a1; if non-null, copies the longword at offset 4 of each target into offsets 50 and 54 of a3, then pops 14 bytes and returns.
- `return_minus_one_if_d0_nonzero`: Decrements d0, sets the byte to all ones if the result is nonzero, sign-extends it to a longword in d0 and returns that value.
- `compare_rotated_not_d0_against_pc_bytes`: Clears d4, complements and rotates d0, compares it with two bytes near the program counter, and on no match loads an address into a1 and stores the vertical position register byte there.
- `push_minus_one_and_branch_to_330`: Pushes the immediate value minus one as a longword onto the stack, then branches unconditionally to the code at offset 330.
- `restore_registers_and_disable_interrupts`: Tests d0, and if zero restores saved registers from the stack, writes to INTENA to disable interrupts, then rearranges stack words and returns via a computed address.
- `compute_time_from_clock_data_into_a2`: Reads three longwords from the structure pointed to by the pc-relative pointer, multiplies the first by 1440, adds the second, scales by 60, adds the third divided by 50, and stores the result at offset 198 of a2.
- `disk_menu_select_action`: Reads disk index from a3 offset 67, clears a1 offset 31, and depending on a3 flag bit and a1 state either sets a status byte, calls a helper, or invokes disk driver jump table entries at offsets 40 and 56, returning the status byte in d0.
- `disk_flag_toggle_data_table`: Bytes decoded as code: negates a word at a1 offset 736b, toggles bit 7 of a3 offset 65, and returns a flag; the instruction stream is data referenced by a lea, not executable logic.
- `disk_index_bit_test_data_table`: Bytes decoded as code: tests a bit of the byte at pc-relative 42e using a3 offset 67, stores a zero or all-ones longword at a1 offset 32, and returns; this is data, not executable code.
- `push_pc_relative_address_plus_29e30`: Pushes the longword at the program counter offset twenty onto the stack, adds the immediate value an address to it, and returns, yielding a computed absolute address.
- `push_pc_relative_address_plus_296e8`: Pushes the longword at the program counter offset twenty onto the stack, adds the immediate value an address to it, and returns, yielding a computed absolute address.
- `clear_bit_and_toggle_flag_40`: Clears a bit indexed by byte at offset 67 of a3 in a global bitmap, then sets or clears bit one of the byte at offset 64 of a3 and increments the longword at offset 294, calling the address helper.
- `set_d1_10001_and_branch`: Loads the immediate value 10001 into d1 and branches unconditionally to the shared routine at offset ec0.
- `read_byte_at_a0_offset_d2`: Tests d2 for negative, and if non-negative loads the byte at a0 plus d2 into d3, copies d3 to d1 and returns.
- `set_d1_280_and_branch`: Loads the immediate value 280 into d1 and branches unconditionally to the shared routine at offset ec0.
- `clear_a1_offset_c_and_store_a4_global`: Clears the longword at offset 12 of a1, loads 12 into d4, then stores a4 into the global at the pc-relative address and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `compare_a4_to_global_then_dispatch`: Compares a4 with the stored global; if equal it branches to the bsr path, otherwise jumps through the pointer at offset minus 12 of a5.
- `set_pc_relative_pointer_and_return`: Loads a program-counter-relative address into a3, stores a3 into memory at offset minus 342 from a3, then loads another pc-relative address into a3 and copies it to d1 before returning.
- `open_dos_library_and_alloc_name_buffer`: Copies a name string into a newly allocated exec memory block, opens dos.library, allocates a 44-byte structure, fills its fields from the library base and links it into a list before returning the structure in d0.
- `open_whdload_device_and_init_structures`: Allocates 68 bytes of chip memory, sets bit 4 in its flags, stores a callback pointer, opens whdload.device, builds a message port with two reply pointers, allocates a jump table with a shifted entry address, and enqueues a request.
- `open_dos_library_and_jump_to_entry`: Loads a dos.library name string, calls the library open vector through a6, then takes the returned base pointer, reads the entry at offset 22 and jumps to it.
- `open_dos_port_and_dispatch_message_loop`: Opens the DOS library, creates a message port, sends a DOS packet, then repeatedly waits on the port, fetches messages and dispatches them by comparing the message type against a jump table.
- `compute_shifted_values_and_branch`: Calls two routines to fill d0 and d1 from d7, loads d2 from offset 28 of a4, calls a third routine, shifts d0 right by two and branches depending on whether the result is zero.
- `call_two_subs_return_ff`: Calls the routine at c40, moves d7 into d0, calls the routine at cf6, sets d0 to minus one, then branches to the routine at 724.
- `read_file_chunk_into_buffer`: Advances the file position by the requested length, reads that many bytes into the buffer at offset 24 of a4, and handles the wrap-around buffer at offset 260 of the file structure.
- `write_buffer_chunk_to_file`: Adds the requested length to the file position, grows the high-water mark, and copies bytes from the buffer at offset 24 of a4 into the wrap-around buffer at offset 260 of the file structure.
- `call_library_with_d6_d7_and_a0_offset`: Copies d6 to d0 and d7 to d1, loads a0 from the longword at offset 4 of a0, moves d5 to a1, calls the function at offset 56 of a2, then loads d0 from offset 28 of a4 and branches away.
- `set_d2_minus_two_and_branch`: Loads the immediate value minus two into d2 and branches forward to another routine, setting up d2 before that code runs.
- `probe_and_store_result_at_d7_offset_36`: Sets d2 to minus one, calls several subroutines passing d7 in d0 and d1, tests the returned d0, and on success stores it at offset 36 of d7 before returning minus one in d0.
- `seek_and_play_sample_via_audio`: Calls three helper routines with d7 to locate a sample, then if found sets a0 to it, clears a1 and d0, calls the audio device through a2 and a6, and branches back.
- `stop_channel_and_restart_audio`: Loads the audio structure from a4, checks a flag at offset 256, calls the audio routine through a2 with a0 and a1, then calls a helper and returns minus one.
- `adjust_audio_volume_with_bounds`: Reads the audio structure from a4, adds a delta to the current volume depending on the sign flag, compares against the maximum, and returns the old volume or an error.
- `get_a4_offset_20_shifted_left_2`: Loads the longword at offset 20 from the a4 base register into d7, shifts d7 left by two bits, and returns.
- `get_a4_offset_24_shifted_left_2`: Loads the longword at offset 24 from the a4 base register into d7, shifts d7 left by two bits, and returns.
- `get_a4_offset_28_shifted_left_2`: Loads the longword at offset 28 from the a4 base register into d7, shifts d7 left by two bits, and returns.
- `shift_string_left_and_store_length`: Copies bytes from a0 forward one position, shifting the string left, then writes the byte count into the byte before the new start and returns.
- `open_file_and_build_io_request`: Opens a file via dos.library, fills an io request block with mode, length and saved registers, and returns the request pointer or zero on failure.
- `close_file_and_free_io_request`: Closes the file handle stored in the io request, frees the request block via dos.library, then closes the saved handle and returns.
- `build_allocated_path_from_dir_and_name`: Allocates memory via the dos library call sized from the directory string length plus the name length, then copies the directory, a slash and the name into it, collapsing duplicate slashes and trimming trailing slashes, returning the buffer or an error code.
- `verify_whdboot_files_present`: Compares two embedded strings against the WHDBoot.exe and WHDBoot.ss names using the string compare routine, returning zero when both match and a nonzero value otherwise.
- `compare_strings_until_null`: Saves a0 on the stack, compares bytes at a0 and a1 incrementing both pointers until a mismatch or a null byte in a0, restores a0 and returns with the zero flag reflecting the comparison.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_pc_relative_pointer_plus_32`: Pushes the longword stored at the program-counter-relative location onto the stack, adds 32 to that pushed value, and returns it to the caller.
- `open_libraries_and_launch_main`: Opens the RisingSun libraries via exec, calls three setup routines, allocates memory, invokes the main entry with adjusted stack argument, closes the library and jumps to the exit vector.
- `verify_signatures_and_dispatch_sections`: Checks the RisingSun signature at offset 24, relocates a1 by fixed amounts and calls the dispatch vector at offset 100 for each section, then tests further signature words and branches to setup or return.
- `skip_d7_pointer_chain_d2_times`: Loads a1 from d7, decrements d2 and, while it stays non-negative, follows the pointer at (a1) and doubles it twice, then adds four to a1 and returns.
- `init_patch_slots_and_call_three_setups`: Saves all registers, zeroes d4 through d7, writes four immediate values into four pc-relative slots, then calls three setup routines before restoring registers and returning.
- `zero_filled_data_slot`: Contains a single zero word that decodes as ori.b with immediate zero; this is zero-filled data referenced by address, not executable code.
- `zero_fill_data_word_at_2096`: A single zero word that decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by address, not executable code.
- `zero_fill_data_word_at_209a`: Another zero word decoding as ori.b with immediate zero to d0; this is zero-filled data pointed to by four callers, not a real routine.
- `zero_fill_data_words_at_209e`: Two words of zero-filled data, decoding as ori.b with immediates zero and seven to d0; referenced by address, not executable code.
- `delta_decode_bitplane_stream`: Walks a run-length encoded bitplane stream, adding accumulated deltas to words in memory and advancing through plane pointers until the count reaches zero, then returns.
- `apply_delta_commands_to_planes`: Iterates over plane command lists, calls the delta decoder for each entry and adds returned offsets into the plane data at indexed positions, returning zero.
- `follow_plane_pointer_chain`: Starting from the saved pointer at a6 minus four, follows a chain of pointers d0 times, each dereferenced and multiplied by four, then returns the final address plus four in a0.
- `copy_counted_words_then_zero_fill`: Reads a count from offset 2156 of a6, copies that many longwords from the table at a6 plus the pc-relative offset into the a6 buffer, then zero-fills the remaining longwords up to the copied end.

### main

- `initialize_stack_and_registers`: Branches to a later instruction, saves the current stack pointer to the stack, loads a new stack pointer, saves all registers to a new stack frame, restores a value from the stack to a register, copies a value from the stack to the new stack frame, loads a constant into a register, pushes a new stack pointer, restores a value from the stack to a global variable, and executes an illegal instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_one`: Jumps directly to a fixed address in the runtime segment, transferring control to another routine without saving any registers.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_two`: Jumps directly to a different fixed address in the runtime segment, handing execution to another routine without touching the stack.
- `jump_to_shared_runtime_entry_one`: Performs an unconditional jump to a shared runtime entry point, transferring execution without saving registers or returning to this location.
- `jump_to_shared_runtime_entry_two`: Performs an unconditional jump to a second shared runtime entry point, passing control onward without touching registers or the stack.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry_three`: Performs an unconditional jump to a third shared runtime entry point, handing off execution with no register or memory changes.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_shared_runtime_entry`: Transfers control unconditionally to another routine in the runtime segment by loading its longword address into the program counter.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
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
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_external_runtime_entry`: Transfers control unconditionally to a different routine in the runtime segment, entered from outside the static flow.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_51`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_52`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_53`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_54`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_55`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_56`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_57`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_58`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_59`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_60`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_61`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_62`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_63`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_64`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_65`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_66`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_67`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_68`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_69`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_70`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_71`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_72`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_73`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_74`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_75`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_76`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_77`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_78`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_79`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_80`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_81`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_82`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_83`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_84`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_85`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_86`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_87`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_88`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_89`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_90`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_91`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_92`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_93`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_94`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_95`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_96`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_97`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_98`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_99`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_100`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_101`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_102`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_103`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_104`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_105`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_106`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_107`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_108`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_109`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_110`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_111`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_112`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_113`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_114`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_115`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_116`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_117`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_118`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_119`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_120`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_121`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_122`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_123`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_124`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_125`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_126`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_127`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_128`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_129`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_130`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_131`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_132`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_133`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_134`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_135`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_136`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_137`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_138`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_139`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_140`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_141`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_142`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_143`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_144`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_145`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_146`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_147`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_148`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_149`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_150`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_151`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_152`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_153`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_154`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_155`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_156`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_157`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_158`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_159`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_160`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_161`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_162`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_163`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_164`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_165`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_166`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_167`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_168`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_169`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_170`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_171`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_172`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_173`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_174`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_175`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_176`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_177`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_178`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_179`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_180`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_181`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_182`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_183`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_184`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_185`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_186`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_187`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_188`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_189`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_190`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_191`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_192`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_193`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_194`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_195`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_196`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_197`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_198`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_199`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_200`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_201`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_202`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_2f_into_d0_and_branch`: Loads the immediate value 47 into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_e7_into_d0_and_branch`: Loads the immediate value 231 into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_ed_into_d0_and_branch`: Loads the immediate value 237 into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_to_2_and_branch`: Loads the immediate value 2 into data register d0, then branches unconditionally to the shared code at the target address.
- `set_d0_to_minus_eleven_and_branch`: Loads the sign-extended immediate value -11 (f5) into data register d0, then branches unconditionally to the shared code at the target address.
- `set_d0_to_minus_ten_and_branch`: Loads the sign-extended immediate value -10 (f6) into data register d0, then branches unconditionally to the shared code at the target address.
- `load_minus_eight_into_d0_and_branch`: Loads the immediate value minus eight into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_sixty_six_into_d0_and_branch`: Loads the immediate value sixty-six into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_sixty_five_into_d0_and_branch`: Loads the immediate value sixty-five into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_minus6_and_branch`: Loads the immediate value minus six into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_minus3_and_branch`: Loads the immediate value minus three into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_93_and_branch`: Loads the immediate value ninety-three into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_ff_and_branch_to_4cee`: Loads the immediate value 255 into data register d0 and then branches unconditionally to the code at the fixed target address.
- `nop_then_branch_to_4cea`: Executes a single no-operation instruction and then branches unconditionally to the fixed target address, changing no registers or memory.
- `increment_a6_offset_20_word`: Adds one to the 16-bit word at offset 32 from address register a6, then copies a6 into d0 and returns to the caller.
- `decrement_counter_and_return_zero`: Decrements the 16-bit counter at offset 32 of a6, then sets d0 to zero and returns to the caller.
- `undecodable_bytes_203`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_204`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_205`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_206`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_207`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_208`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_209`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_string_pointer_and_branch`: Subtracts 8 from the stack pointer, pushes the address of a string constant, then branches forward to shared code.
- `push_string_and_setup_frame`: Subtracts 8 from a7, pushes a string address, branches to shared code, then stores a5 and sets up a new a5 frame before returning.
- `pop_a5_and_return`: Pops a5 from the stack and returns immediately to the caller, restoring the saved frame pointer.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_all_game_init_functions`: Calls a long sequence of game routines in order, then jumps to a final routine instead of returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_tile_flag_set_action_code`: Saves a4, loads a data pointer, calls two routines, then reads a tile flag at offset 29 and sets action code 29 unless already 28.
- `dispatch_sound_command_by_index`: Calls a setup routine, then uses d2 as an index into a table at a3 to jump to a command handler, stores a word from a4 into a global, and conditionally calls four routines based on a counter value before restoring a4 and returning.
- `save_a4_and_jump_to_shared_body`: Pushes a4 onto the stack, loads a4 from a nearby constant, opens an a5 frame, then branches unconditionally into another routine's body.
- `append_sprite_entry_to_buffer`: Saves a4, opens an a5 frame, indexes a buffer by a global counter, stores a word from the stack frame, computes a relative position from a pointer table, increments the counter and wraps it at 150 before branching onward.
- `update_scanline_offset_counter`: Computes a negated word difference between a5 offset 14 and a2 offset 964, stores it at a3+2, increments a word counter, and resets it with a flag at 150.
- `process_input_events_loop`: Pops events from the input queue, clears three flags, and dispatches on event codes to set direction, speed, and toggle globals until the queue is empty.
- `check_player_hit_against_rect_list`: Calls the input event processor, reads player coordinates, scans a rectangle list for a containing box, and updates the selected index and display.
- `init_sprite_and_center_text`: Writes 1 to the byte at a3, optionally calls a graphics routine with fixed arguments, clears word at offset 6 of a3, measures a string via a library call and stores 160 minus half its width at offset 4, then queues a text draw.
- `clear_flag_unlink_and_return_to_caller`: Clears the byte flag at a fixed global address, releases the a5 frame, pops a4 and a0 from the stack, discards six more stack bytes and jumps to the saved return address in a0.
- `advance_selection_and_draw_menu_entry`: When the timer word is negative, either sets it to a large value, advances a selection index modulo ten, positions and draws a sprite and text entry from tables, or on first entry plays a sound and sets the timer to 26.
- `reset_state_and_copy_word_then_call`: Saves a4, loads a base pointer, clears a byte flag, writes minus one to a word, copies one word to another, calls a subroutine, then restores a4 and returns.
- `advance_cursor_and_store_tile_coords`: Compares two cursor words, advances the column index wrapping at nine, advances the row index when it matches, calls a library routine with a table pointer, and stores three words into a six-byte-stride table indexed by the column.
- `draw_rotated_sprite_ring`: Computes sprite width and height from arguments, centers negative coordinates, then repeatedly calls the blit routine to draw sprites around a rectangular ring, advancing the position and sprite index each step.
- `clear_six_global_bytes_after_call`: Saves a4, loads the a4 base pointer, calls a subroutine, then clears six global bytes to zero before restoring a4 and returning.
- `update_ship_slot_weapon_and_target_data`: Scans 52 weapon slots and 19 ship records, matching indices against the passed parameters, updating byte fields at offsets 16, 25, 26, 27, 28, 30 and 43, calling two subroutines and a library function.
- `build_and_register_formation_buffer`: Computes a position from global counters, fills a stack buffer, calls a library routine with the buffer and a global destination, stores the destination pointer globally and returns.
- `format_text_into_columns`: Parses a format string with percent escapes, copies characters into a global buffer, calls text drawing routines to render each column, and advances through the string until the column count is exhausted.
- `setup_screen_and_draw_intro`: Calls subroutines to configure display, draws formatted text, clears a global, copies ten longwords from a table to the stack, sets a palette, and computes a value from a global word times twenty-five.
- `release_display_resource`: Pushes a global pointer and a zero word, calls a library routine to free or close that resource, then restores the frame and returns.
- `create_text_object_and_measure`: Saves a4, links a frame, calls a setup routine, copies the caller's argument into a text object, sets its flag and clears its word, measures the string via a library call, stores the centered vertical offset, then queues the object for drawing.
- `draw_status_panels_and_clear_screen`: Checks a global counter, calls the setup routine, creates a text object, copies three 40-byte structures to the stack and passes them to a graphics routine, optionally clears a hardware register, then restores state and returns.
- `build_menu_entries_from_table`: Walks a table of word entries, calls a library routine for each entry to build a menu item, stores the returned pointer in a table, advances the index and returns when the count is exhausted.
- `free_ten_global_slots`: Loops over ten longword slots in a global table, and for each non-null entry pushes it and calls the external free routine, then clears the slot.
- `clamp_d0_to_upper_bound`: Loads the word at offset twelve of the a5 frame into d2, compares it with the word at offset sixteen, and if the first is smaller returns the second word in d0.
- `clamp_d0_to_lower_bound`: Loads the word at offset twelve of the a5 frame into d2, compares it with the word at offset fourteen, and if the first is larger returns the second word in d0.
- `load_word_at_a5_offset_c_into_d0`: Reads the 16-bit word at offset 12 from the a5 frame into d0, then branches to the shared epilogue at the next routine.
- `unwind_frame_and_jump_to_saved_return`: Unlinks the a5 frame, pops a4 and a0 from the stack, skips six bytes of stack, then jumps through a0 to the saved return address.
- `compare_loop_index_with_limit`: Saves a4, loads a4 from a pc-relative longword, opens an a5 frame, reads word at offset 14 into d2 and compares it with the word at offset 12, branching if d2 is greater or equal.
- `return_lower_bound_word`: Loads the word at offset 12 from the a5 frame into d0 and branches onward, returning that bound value to the caller.
- `unwind_frame_and_jump_to_saved_return_2`: Unlinks the a5 frame, pops a4 and a0 from the stack, skips four bytes, and jumps through a0 to the saved return address.
- `set_flag_and_call_allocator_for_pointer`: Saves a4, loads a global base into a4, sets a byte flag, and if the caller's pointer is null calls the allocator and stores the result there.
- `check_flag_then_call_script`: Saves a4, loads the global base pointer, tests a byte flag at offset -15; if zero it calls the routine at the absolute address with the argument from the stack frame, then restores a4 and a0 and jumps back.
- `scan_entries_and_mark_expired`: Iterates a table of word entries at offset 28 of the passed structure, calls a time routine, compares each entry's masked timestamp against the current time, and writes a byte flag when an entry is older than the threshold.
- `lookup_byte_from_two_tables`: Uses the word argument times 18 to index a table and read a byte at offset 8; if not 255, uses that byte times 64 to index a second table and read a byte at offset 27, returning it in d0.
- `unlk_restore_a4_a0_skip_two_jump`: Unlinks the a5 frame, pops a4 and a0 from the stack, skips two bytes on the stack, then jumps to the address in a0.
- `scan_slots_for_active_entry_and_select`: Scans up to fifty-two table entries of eighteen bytes, and for entries of type four or five with a nonzero flag byte, calls a routine with selector three and stores the matching index in the global word.
- `copy_selected_entry_fields_to_buffer`: Walks a list of word indices, calls a lookup for each, copies four-byte fields from the entry table into a buffer at offset sixty-four, and calls a copy routine for paired values.
- `play_sound_entries_for_record`: Walks up to two sound entries of a record, skipping negative ones, and calls the sound trigger with each valid entry index and the global sound bank.
- `refresh_record_sound_and_display`: Calls a setup routine, updates the current record's sound slot, redraws two record entries through the display routine, then restores the record and triggers a delay.
- `load_and_activate_record`: Validates a record index, copies its sound entries, sets the active record and sound bank, configures display and input, then runs the record's update loop.
- `spawn_unit_from_slot_and_link_owner`: Reads slot index from stack, copies unit record fields, calls two helpers to allocate and register, clears a flag on the previous owner and sets it on the new one, then scans all slots to update matching entries.
- `retire_unit_and_advance_scan_index`: Checks the unit's timer at offset 44, and if negative sets its state to 20 and copies a global value into its field, then increments the loop counter and returns via the saved frame.
- `link_frame_and_jump_to_shared_exit`: Creates an empty a5 stack frame and branches unconditionally to a shared exit path, so it performs no work of its own.
- `decrement_counters_and_run_callback_list`: Decrements two word counters, reloads a six-frame countdown minus a delay, then walks a linked list calling each node's second longword as a function and returns zero.
- `install_frame_callback_if_absent`: Checks the callback slot at offset 18 of the global structure; if empty, sets byte 8 to 2, stores the frame routine pointer there and registers the structure with the system call.
- `prepend_node_to_callback_list`: If the node's second longword is zero, links the node into the global list head, storing the old head and the given function pointer, then pops arguments and jumps to the return address.
- `unlink_node_from_global_list`: Sets up an a5 frame, loads the argument node pointer, and if its next field is non-null compares the global list head with the node, replacing the head with the node's next pointer before returning.
- `search_list_for_matching_node`: Walks the global list starting at the stored head, comparing each node pointer against the argument node, and branches back to continue until a match or null is found.
- `splice_node_into_list_and_return`: Stores the node's next pointer into the previous node's link field, clears the node's next field, then releases the a5 frame and jumps back to the caller.
- `release_sound_channel_and_reset_state`: If the channel object at the fixed pointer has a nonzero field at offset 18, calls the sound routine with command 5 and that object, then clears the field; otherwise sets the current object pointer, command word and timer word, and if byte 530 equals 50 sets a flag and adjusts the command and timer.
- `save_a4_and_jump_to_4112`: Pushes a4 onto the stack, loads a4 from a program-counter-relative longword, opens an a5 stack frame, then branches unconditionally to the code at 4112.
- `find_first_marked_list_node`: Walks a linked list from a0, skipping nodes whose type byte is not 1 or whose word at offset 6 is not 54 or 55 or whose flags word lacks bit 6, and returns in d0 the first skipped node, or zero if none.
- `init_ui_state_and_jump`: Saves a4, loads the global base pointer, opens a stack frame, compares a field against a global and either sets a word or branches to the main setup code.
- `build_and_draw_menu_screen`: Sets up a menu screen, copies tile data, calls graphics and text routines, opens and closes a screen, then restores the frame and returns.
- `initialize_game_state_tables`: Clears many state bytes and words in the global base, copies screen dimensions, checks a flag and fills name and stat tables using random values.
- `call_playback_with_107_and_saved_ptr`: Saves a4, loads the global base pointer, pushes the saved pointer at offset -104 and the constants 107 and 1, then calls the external routine and restores a4.
- `setup_scene_and_open_playback_window`: Saves display and sprite registers, reads the vertical position register, opens a screen and window, copies structures, then initializes playback and scene objects through many external calls.
- `restore_registers_and_close_playback`: Restores saved display and sprite register values, closes the playback window and screen, frees the six stored objects, then restores the saved pointer and returns.
- `save_a4_frame_and_return`: Pushes a4, loads a4 from a pc-relative longword, opens and immediately closes an a5 frame, restores a4 from the stack and returns.
- `save_a4_load_pointer_and_return`: Pushes a4, loads a4 from a different nearby constant longword, creates an empty a5 frame, then pops a4 back and returns to the caller.
- `save_a4_set_frame_and_jump_away`: Pushes a4, loads a4 from a nearby constant longword, opens an empty a5 frame, then branches unconditionally to code elsewhere instead of returning.
- `clear_pending_flag_and_reset_state`: Saves a4, loads the base pointer from the nearby constant, and if the global flag byte is set, writes one and clears a word at offsets from the base, then clears the flag byte.
- `advance_frame_counter_and_call_draw`: Clears a flag byte, pushes the object pointer and frame counter onto the stack, calls the draw routine, increments the counter at offset -14 from a4, wraps it to 11 past 31 and sets the flag byte.
- `run_clear_then_install_callback`: Saves a4, loads the base pointer, calls the clear routine, and if the byte at offset minus fifteen from the base is nonzero, stores the address of a nearby routine into the global callback pointer.
- `init_tile_row_and_column_counters`: Saves a4, loads the game state pointer, calls the setup routine, then clears the row counter and begins scanning rows up to twenty while computing a clamped column span.
- `draw_tile_run_and_advance_counters`: Computes the column count minus one, calls the tile blit routine with row and column arguments, then advances row and column counters and installs the next callback.
- `copy_tile_row_words_to_screen_buffer`: If a flag byte is clear, copies sixteen words from a tile graphics source into the screen buffer at an offset derived from two state words, then calls the blit routine.
- `increment_counter_and_set_audio_bit`: Saves a4, loads the counter base, increments the word counter at offset minus four if below ten, calls the shared update routine, then sets bit 82 in the word pointed to by the audio structure at offset 1492.
- `decrement_counter_and_call_update`: Saves a4, loads the counter base, decrements the word counter at offset minus four when it is nonzero, calls the shared update routine, then restores a4 and returns.
- `init_level_audio_and_state`: Queries four audio channels, calls setup routines, installs the increment and decrement callbacks, initializes level state words, optionally configures channels, and sets the level-start flag.
- `save_a4_set_base_and_jump`: Pushes a4, loads a4 from a pc-relative longword, opens an a5 frame, then branches away to another routine.
- `conditional_audio_setup_and_cleanup`: Saves a4 and sets a4 from a pc-relative pointer, tests a global byte and, if set, calls several routines with a4-relative and global arguments, storing returned bytes, then clears a byte and returns.
- `save_a4_set_base_and_jump_alt`: Pushes a4, loads a4 from a different pc-relative longword, opens an a5 frame, then branches to another routine.
- `draw_two_grid_cells_and_play_sound`: Saves a4, loads globals, then calls library routines to draw two grid cells indexed by the stack arguments plus offset and to trigger a sound effect before restoring registers and returning.
- `set_a4_frame_and_jump_forward`: Pushes a4, loads a4 from a nearby constant, opens an a5 stack frame, then branches unconditionally to another routine without returning here.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `divide_d3_by_constant_and_return`: Divides d3 by the immediate value 9807, then releases the a5 frame, pops a4 and returns to the caller.
- `unlink_frame_restore_a4_return`: Unlinks the a5 stack frame, pops the saved a4 register from the stack, and returns to the caller.
- `save_a4_setup_frame_branch_away`: Pushes a4, loads a4 from a program-counter-relative longword, links an empty a5 frame, then branches away; the trailing unlink and restore are unreachable.
- `save_a4_link_frame_branch_to_1536`: Pushes a4, loads a4 from a program-counter-relative longword, links an empty a5 frame, then branches to another routine without returning here.
- `find_matching_sprite_entries`: Scans up to nineteen sprite records comparing type byte and two position words against the record indexed by the argument, storing matching indices into a table and terminating it with nineteen.
- `set_a4_frame_and_jump`: Pushes a4, loads the a4 frame pointer from the nearby constant, opens an empty a5 frame and branches away to another routine.
- `reset_entity_offsets_and_notify`: Calls three library routines, clears the words at offsets ten and twelve of two objects from the entity table, then posts a message with code three hundred forty-five.
- `save_a4_set_base_and_jump_2`: Pushes a4 onto the stack, loads a4 from a program-counter-relative longword, opens an a5 frame with zero size, then branches to another routine.
- `copy_indexed_longword_and_call`: Saves a4, loads a4 from a pc-relative longword, opens an a5 frame, reads a word global, shifts it left six bits, uses it to index a table and copies a longword to a global, then calls a routine with two word arguments.
- `save_a4_load_base_and_branch_2`: Pushes a4, loads the base pointer from the nearby constant into a4, opens an a5 stack frame, then branches onward to another routine without returning here.
- `play_sound_effect_189_via_library`: Saves a4, loads a4 from a nearby constant, builds a stack frame, fetches a pointer from absolute memory, pushes a word from offset 2056 plus two word arguments, calls a library routine, then restores a4 and returns.
- `save_a4_and_jump_to_shared_exit`: Pushes a4 onto the stack, loads a4 from a nearby constant, creates an a5 stack frame, then branches unconditionally to another routine's code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_a4_and_return_from_frame`: Releases the a5 stack frame, pops the saved a4 register from the stack and returns to the caller.
- `save_a4_set_a4_from_pc_bra`: Pushes a4 onto the stack, loads a4 from a program-counter-relative longword, opens an a5 frame, then branches away to another address.
- `save_a4_load_a4_pc_bra_425e`: Pushes a4 onto the stack, loads a4 from a program-counter-relative longword, opens an a5 frame and branches to the target at offset 425e.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `indirect_jump_via_a3_d2_table`: Reads a word from a3 plus d2 into d2, calls the code pointer at a3 plus d2, then pops a4 and returns.
- `unlink_a5_restore_a4_return`: Unlinks the a5 stack frame, pops a4 from the stack and returns to the caller.
- `save_a4_load_base_and_branch`: Pushes a4 onto the stack, loads a4 from a pc-relative longword, opens an empty a5 frame and branches away to another routine.
- `update_weapon_sprite_positions`: Saves a4, loads the player data base, calls a routine, then tests flags and writes sprite coordinates and a tile value into the object structures.
- `position_sprite_from_actor_offsets`: Reads actor fields at offsets 12 and 14, scales them, writes screen coordinates to two sprite objects at offsets 10 and 12, and optionally spawns a projectile via jsr.
- `decrement_counter_clear_flag_branch`: Subtracts 3 from the word at offset -2 of a4, clears the byte at offset -19 of a4, then branches to another routine.
- `copy_sprite_index_and_reset_state`: Adds 53 to the word at -4 of a5, stores it at -8, clears a flag, copies a sprite index into a table slot, and resets state if the slot is empty.
- `update_ship_weapon_and_hud_positions`: Increments a counter, reads a ship table entry, computes two screen coordinates from a2 and a3 tables, stores them into the ship object, then rebuilds a 22-entry pointer list and calls display routines to redraw the HUD.
- `report_error_and_unwind_stack_frame`: Pushes a3 and an error code with a global pointer, calls the error reporter, then pops a3, releases the a5 frame, restores a4 and a0, discards six stack bytes and jumps to the saved return address.
- `save_a4_set_global_base_and_jump`: Pushes a4, loads a4 from a program-counter-relative longword, opens an empty a5 stack frame, then branches unconditionally to another routine.
- `store_sprite_frame_and_position`: Saves a4, links a5, copies a 12-byte sprite record from the table at the global pointer into the slot indexed by the second argument, then writes two words at offsets 10 and 12 of that slot and jumps to the caller's return address.
- `branch_to_shared_237a_body`: Saves a4, loads the a4 global base, opens an a5 frame and branches to the shared body at the later address; the trailing unlink and return are unreachable.
- `restore_a4_and_return_via_trampoline`: Saves a4 on the stack, loads a4 from a nearby constant, sets up an a5 frame, branches to another routine, then unlinks and restores a4 before returning.
- `save_a4_set_frame_jump_2c60`: Pushes a4, loads a4 from the small data base, opens an a5 stack frame, then branches to the code at 2c60; the trailing unlk, a4 restore and rts are unreachable.
- `save_a4_set_frame_jump_2d66`: Pushes a4, loads a4 from the small data base, opens an a5 stack frame, then branches to the code at 2d66; the trailing unlk, a4 restore and rts are unreachable.
- `save_a4_set_frame_jump_2ee6`: Pushes a4, loads a4 from the small data base, opens an a5 stack frame, then branches to the code at 2ee6; the trailing unlk, a4 restore and rts are unreachable.
- `save_a4_load_pc_relative_a4_and_branch`: Pushes a4 onto the stack, loads a4 from a pc-relative longword, creates an a5 stack frame, then branches to another routine.
- `store_word_to_global_and_test_sign`: Saves a4, loads a4 pc-relative, opens an a5 frame, copies a word from the caller's frame into a global, tests it and branches on its sign.
- `restore_frame_and_jump_or_return`: Unlinks the a5 frame, pops a4 and a return address, skips two bytes, jumps through a0; a second path unlinks, pops a4 and returns.
- `save_a4_load_3690_and_jump_377a`: Pushes a4 onto the stack, loads a4 from the longword at the program counter offset 3690, creates an empty a5 frame, then branches to the code at 377a.
- `check_flag_and_write_palette_registers`: Calls a check routine with arguments 107 and 1, and if it returns nonzero writes values 86 and 20 into four hardware registers pointed to by a3, then calls the palette routine with selector 137.
- `set_sprite_registers_and_trigger_sound`: Writes values into four sprite control registers via a3 offsets, then pushes arguments and calls three library routines before storing a word to a global and returning.
- `branch_to_391e`: Branches unconditionally to the code at the target address, transferring execution without touching any registers or memory.
- `branch_to_shared_exit_code`: Unconditionally branches forward to a shared block of code, transferring execution without touching any registers or memory.
- `unsigned_32bit_multiply_d0_d1`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, returns the low 32-bit result in d0, and preserves d2 and d3.
- `signed_32bit_divide_with_16bit_divisor`: Divides the 32-bit value in d0 by the low word of d1, using a 16-step shift-subtract loop when the divisor exceeds 65535 and hardware divu otherwise, returning quotient in d0 and remainder in d1.
- `open_libraries_and_load_raw_level_data`: Opens dos, graphics, intuition, layers and mathffp libraries, reads the RAW level file through dos, copies its text into a buffer and stores the parsed level pointer in the game structure before closing everything.
- `store_a4_field_into_pc_relative_global`: Loads the address of a nearby data area into a0 and copies the longword at offset minus twenty of a4 into offset minus four of that area, then returns.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `store_a4_minus16_to_pc_relative_global`: Loads the address just before this code into a0, then copies the longword at offset minus sixteen from a4 into the longword four bytes before that address, and returns.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `tail_call_library_offset_minus66`: Loads the caller's argument pointer from the stack into a0, fetches the library base from a pc-relative global into a6, shifts the return address down one slot, and jumps to the library entry at offset minus sixty-six.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `tail_call_library_offset_minus96`: Loads the caller's argument pointer from the stack into a0, fetches the library base from a pc-relative global into a6, shifts the return address down one slot, and jumps to the library entry at offset minus ninety-six.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `call_library_offset_378_with_one_arg`: Loads the first stack argument into a0, fetches the library base from pc-relative memory into a6, shifts the return address down over the argument slot, pops four bytes and jumps to library offset minus 378.
- `call_library_offset_162_with_three_args`: Loads the third stack argument into a0, the second word into d0 and the first word into d1, fetches the library base into a6, moves the return address over the three argument slots, pops eight bytes and jumps to library offset minus 162.
- `call_library_offset_198_with_one_arg`: Loads the first stack argument into a0, fetches the library base from pc-relative memory into a6, moves the return address over the argument slot, pops four bytes and jumps to library offset minus 198.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `jump_library_offset_minus_246`: Loads the return address into a0, fetches the library base from pc-relative memory into a6, moves the caller's return address down one slot, advances the stack by four and jumps through offset minus 246 of the library vector table.
- `jump_library_offset_minus_252`: Loads the return address into a0, gets the library base from pc-relative memory into a6, shifts the return address into the argument slot, adds four to the stack pointer and jumps via library vector offset minus 252.
- `call_library_offset_minus_282`: Loads a0 from the caller's stack, zeroes d0, copies the byte argument into d0, loads the library base from pc-relative memory into a6, shifts the return address down and jumps to library offset minus 282.
- `call_library_vector_minus_204`: Loads the return address from the stack into a0, fetches the library base from pc-relative memory into a6, moves the caller's return address down one slot, advances the stack by four bytes and jumps through the library vector at offset minus 204.
- `call_library_vector_minus_456`: Loads the return address from the stack into a0, fetches the library base from pc-relative memory into a6, shifts the return address down one stack slot, adds four to the stack pointer and jumps through the library vector at offset minus 456.
- `call_library_vector_minus_270_with_args`: Loads two pointer arguments and four word arguments from the stack into a0, a1, d0 through d3, fetches the library base into a6, moves the return address up the stack, adjusts the stack pointer and jumps through the vector at offset minus 270.
- `store_a4_minus20_to_global_4230`: Loads the address of the global at offset 4230 into a0, then copies the longword at a4 minus 20 into that global and returns.
- `call_library_offset_minus_492`: Moves the two word arguments from the stack into d0 and d1, loads the library base from pc-relative memory into a6, shifts the return address down and jumps to library offset minus 492.
- `call_library_offset_minus_498`: Loads a0 from the stack, moves the two word arguments into d0 and d1, loads the library base from pc-relative memory into a6, shifts the return address down and jumps to library offset minus 498.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `exec_addport_thunk`: Loads the first stack argument into a1, loads the ExecBase pointer from address 4 into a6, shifts the return address down one slot, then jumps to the AddPort vector in the library table.
- `exec_findport_thunk`: Loads the first stack argument into a1, loads the ExecBase pointer from address 4 into a6, shifts the return address down one slot, then jumps to the FindPort vector in the library table.
- `exec_getmsg_thunk`: Loads the first stack argument into a0, loads the ExecBase pointer from address 4 into a6, shifts the return address down one slot, then jumps to the GetMsg vector in the library table.
- `exec_remport_trampoline`: Loads the first stack argument into a1, loads the exec library base from address 4 into a6, shifts the return address down one slot, then jumps to the RemPort vector at offset -360 in the library.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `exec_adddevice_trampoline`: Loads the first stack argument into a1, loads the exec library base from address 4 into a6, shifts the return address down one slot, then jumps to the AddDevice vector at offset -432 in the library.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `exec_remdevice_trampoline`: Loads the first stack argument into a1, loads the exec library base from address 4 into a6, shifts the return address down one slot, then jumps to the RemDevice vector at offset -438 in the library.
- `exec_doio_tail_call`: Loads the io request pointer from the stack into a1, loads the exec library base into a6, shifts the return address down, and jumps to the DoIO vector.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `exec_addmemlist_tail_call`: Moves five stack arguments into d0, d1, d2, a0 and a1, loads the exec base into a6, adjusts the stack, and jumps to the AddMemList vector.
- `exec_allocmem_tail_call`: Moves the byte size and memory flags from the stack into d0 and d1, loads the exec base into a6, adjusts the stack, and jumps to the AllocMem vector.
- `exec_availmem_library_trampoline`: Loads the exec library base from address 4, shifts the return address and argument on the stack, then jumps to the AvailMem vector at offset -216 from the library base.
- `call_copy_mem`: Library stub: loads its arguments and the exec library base, calls CopyMem and returns its result to the caller.
- `exec_freemem_library_trampoline`: Loads the exec library base from address 4, moves the memory pointer and size arguments up the stack, then jumps to the FreeMem vector at offset -210 from the library base.
- `store_a4_minus20_to_pc_relative_slot`: Loads the address of a nearby data slot into a0 and copies the longword at offset -20 from a4 into offset -4 of that slot, then returns.
- `jump_abs_exec_lib_offset`: Loads the return address from the stack into a0, loads the library base from a pc-relative longword into a6, moves the caller's return address down one slot, advances the stack pointer by four and jumps to an offset in the library vector table.
- `call_lib_with_five_stack_args`: Loads a0 from the stack, loads d0 through d3 from successive stack words, loads the library base into a6, moves the return address down, adjusts the stack pointer and jumps to a library vector offset.
- `save_a4_load_global_frame_then_branch`: Pushes a4 onto the stack, loads a4 from a pc-relative longword, creates an empty a5 stack frame and branches to another routine; it does not return here.
- `refill_buffer_via_allocator`: Saves a4, loads the global context pointer, calls the buffer routine with size 512 and two context fields, and on failure invokes the stored callback at offset -12 before storing the new size and pointer.
- `dispatch_bytecode_stream_ops`: Allocates a buffer if none given, then repeatedly reads a byte from the context stream, indexes a jump table by the low nibble and calls the selected operation while the remaining count is positive.
- `fetch_next_bytecode_operand`: Advances the context stream pointer, refills the buffer when the count is exhausted, reads the next byte, and if bit 7 is set extracts a length from bits 4-6 and masks the low nibble.
- `set_d6_offset_and_clear_byte`: Loads the longword at offset minus eight from a5, adds two, stores it in d6, clears the byte at offset minus forty from a4 and returns.
- `advance_stream_pointer_and_fetch_byte`: Adds two to the a5 longword into d6, increments the a4 stream index, refills the buffer via the helper when empty, decrements the count and copies the next byte to offset minus forty from a4.
- `copy_byte_and_fetch_two_stream_bytes`: Copies the byte at offset minus five from a5 to minus six, then twice advances the a4 stream index, refilling via the helper when the count runs out, storing fetched bytes at a5 minus five and a4 minus forty, and sets d6 to the a5 longword plus eighteen.
- `read_three_bytes_advance_stream`: Copies byte at a5-5 to a5-7, then reads three bytes from the stream pointer at a4-22 into a5-6, a5-5 and a4-40, refilling via the stream routine when the count at a4-26 drops below one, and sets d6 to the value at a5-8 plus 4114.
- `parse_two_or_four_byte_record`: Clears a4-42, reads a leading byte into a5-5, then either reads two bytes into a4-40 and a4-39 setting a4-44 to 1, or reads four bytes into a4-40 through a4-37 setting a4-44 to 3, and computes d6 from a5-8.
- `copy_min_of_three_counts_to_buffer`: Advances the stream pointer, clamps d6 to the minimum of a5+20 and a4-26, copies that many bytes from a4-22 to a5-4, then subtracts the count from a5+20, d6, a4-26 and adds it to a5-4 and a4-22.
- `fill_buffer_with_byte_from_a4`: Clamps d6 to the count at offset 20 of a5, subtracts it from that count, then writes the byte at -40(a4) repeatedly to the buffer pointer at -4(a5), advancing the pointer and returning.
- `copy_ring_buffer_bytes_to_output`: Clamps d6 to the count at 20(a5), then copies bytes from the circular buffer at -40(a4) indexed by the word at -42(a4), wrapping at -44(a4), to the pointer at -4(a5) until d6 is exhausted.
- `walk_list_clear_and_release_entries`: Stores the list pointer at a4-8, then for the entry index at a5+14 checks the flag byte, releases the entry's resource through the free routine, clears its pointer, and advances to the next flagged entry until the list end.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_indexed_jump_table_entry`: Decrements the counter at a5-4 and returns, then loads the jump table base, clamps the index in d2 to 19, and jumps through the table entry; the remaining instructions free an entry and unwind the frame.
- `load_bolt_resource_from_disk`: Allocates memory, opens the file named bolt, reads a header and data into a buffer, verifies the bolt signature, stores the file handle in the a4 global and returns the buffer pointer in d0.
- `free_bolt_entries_and_buffer`: Takes a pointer from the caller, frees each flagged twelve-byte entry in the bolt table, releases the table buffer and clears the caller's pointer.
- `save_a4_load_table_and_enter_58c2`: Pushes a4 onto the stack, loads a4 from a pc-relative table pointer, opens an a5 frame with a large negative local area, then branches to code at 58c2.
- `append_pointer_to_sorted_list`: Checks the word at the passed pointer for the sentinel value, increments a counter, allocates a list with the count times four bytes, stores the pointer at the indexed slot and returns.
- `fetch_record_field_and_dispatch_by_byte`: Computes a record address from a table plus a twelve-byte stride, stores a masked long field into the caller's structure, loads a byte index and branches to a jump table.
- `allocate_node_with_data_pointer`: Pushes a3, calls the allocator with the object field at offset minus 530 and two zero arguments, stores the returned pointer and sets its second field to point four bytes past itself.
- `allocate_node_with_header_size`: Pushes a3, calls the allocator with the object field at offset minus 530, zero and size two, stores the returned pointer and sets its first field to point twelve bytes past itself.
- `build_and_fill_allocated_buffer_table`: Allocates a small structure and a 40-byte buffer, fills the buffer with bytes from a mask, allocates per-bit sub-buffers, writes minus-one bytes into each, then frees the buffer and clears the pointer.
- `allocate_and_fill_sound_buffer`: Reads a word flag into a local, picks a size of 54 or 30, allocates a buffer, stores the flag at its start, fills three sub-buffers via a copy routine, and frees the buffer on failure.
- `allocate_and_copy_sound_chunks`: Reads a flag byte, allocates a buffer of size two or the count from the object at offset minus 530, then copies successive words from the source into the buffer while more than one word remains.
- `allocate_buffer_for_object_count`: Loads the object pointer from the caller frame, reads its count field at offset minus 530, and allocates a buffer of that size, storing the result at offset 8 of the frame.
- `store_allocated_block_to_object`: Pushes a3, loads the object pointer from offset 8 of a5, allocates a block via the called routine with two zero arguments, then stores the returned pointer at offset 8 of that object and returns.
- `build_and_link_sprite_from_descriptor`: Reads a descriptor byte from a3, allocates and fills a sprite structure with dimensions, flags and linked-list offsets, then frees or clears the object pointer and returns.
- `copy_saved_pointer_to_global`: Loads the address of a nearby global into a0 and copies the longword at offset minus 8 of a4 into the global at offset minus 4 of a0, then returns.
- `call_library_offset_minus_36_with_one_arg`: Loads the first stack argument into d1, fetches the library base from pc-relative memory into a6, shifts the return address down over the argument, pops four bytes and jumps to library vector at offset minus 36.
- `jump_library_offset_minus_120`: Loads the return address into d1, fetches the library base from pc-relative memory into a6, moves the caller's return address down one slot, pops four bytes and jumps to offset minus 120 of the library vector table.
- `jump_library_offset_minus_30`: Loads two stack arguments into d1 and d2, fetches the library base into a6, shifts the return address up two slots, pops eight bytes and jumps to offset minus 30 of the library vector table.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `negate_d0_after_library_call`: Loads two arguments from the stack into d1 and d2, calls a library function through a6 at offset minus 78, negates the low byte of d0, then pops a0 and returns via jmp.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `save_a4_setup_frame_jump_forward`: Pushes a4, loads a4 from a pc-relative longword, creates an a5 stack frame, then branches forward to another routine.
- `iterate_object_list_call_renderers`: Saves a4, checks a flag byte, calls a setup routine, then walks a linked list of objects adding their offsets and calling render routines with object fields pushed on the stack.
- `dispatch_object_by_flag_bits`: Pushes a3 and object fields plus a5 offsets onto the stack, calls the object dispatch routine, then tests flag bits 1 and 3 in the object word and calls a second dispatch routine when bit 3 is clear.
- `dispatch_object_then_copy_next_pointer`: Pushes a3 and object fields plus a5 offsets, calls the object dispatch routine, then copies the longword at offset 22 of a3 into a5 offset 20 and branches back to the caller loop.
- `conditional_cleanup_and_indirect_jump`: Tests a byte at a4 offset minus 9, optionally calls a cleanup routine, then unlinks a5, restores a4 and a0, adjusts the stack and jumps through a0.
- `scan_entity_list_build_active_array`: Walks the caller's entity pointer array up to the count at offset minus six of the global structure, stores non-null entries into a local array, and adjusts each entry's flag word bits.
- `set_entity_flag_bit5_and_skip`: Sets bit five of the flag word pointed to by a3 and branches forward to the shared exit, marking the entity as processed.
- `clear_bit9_set_bit5_and_skip`: Reads the flag word at a3, tests bit nine, clears bit nine and sets bit five, then branches to the shared exit.
- `dispatch_object_with_alt_callback`: Tests flag bits in the word at a3; if bit 5 clear it decrements a counter, otherwise when bit 0 set and bit 3 clear it pushes object fields and calls the alternate callback, then restores a3 and branches back.
- `dispatch_object_with_main_callback`: Pushes object fields from a3 plus the a5 context onto the stack, calls the main callback, restores a3 from the stack and branches back to the shared loop.
- `decrement_counter_and_scan_object_list`: Decrements the counter at -804(a5), increments the index at -2(a5), compares it against the saved count, loads the object pointer from the list at -802(a5) and tests its flag bits to choose a branch.
- `set_flag_bit7_clear_bit6`: Clears bit 6 and sets bit 7 of the word at address a3, then branches to another routine.
- `clear_bits_and_dispatch_two_vectors`: Clears bits in the word at a3, tests bits 1 and 3, and depending on them pushes a3 fields plus a5 or a4 pointers and calls the dispatch routine twice.
- `dispatch_two_entity_records`: Pushes fields from the a3 record and a5 pointer onto the stack, calls the shared dispatcher twice, once with the a5 field and once with the a4 field, then restores a3 and branches back.
- `dispatch_active_entity_events`: Iterates the entity pointer list at a5 offset, and for each entity with flag bit 0 set calls the update routine with its fields, then stores its position words into per-slot arrays; entities with bit 3 set are passed to a second routine.
- `finish_entity_pass_and_restore_frame`: Pushes entity fields and the a5 argument, calls the entity update routine, advances the loop counter, toggles the byte at a4 offset when enabled, optionally calls another routine, then unwinds the a5 frame and returns.
- `set_a4_from_pc_and_jump_forward`: Saves a4 on the stack, loads a4 from the longword at the program counter relative location, opens an empty a5 stack frame and branches forward to another routine.
- `pop_object_and_clear_flags`: Saves a4, loads the object base from a pc-relative pointer, opens a stack frame, and if the flag byte at offset 16 is set clears the object's flag word and pops the next object pointer from the list into the frame slot.
- `clear_object_pointer_and_branch`: Clears the longword at offset minus four from a4 and branches unconditionally to the shared continuation at the end of the caller's routine.
- `apply_object_velocity_and_flags`: Sets bit 1 of the object's flag byte, subtracts the object's speed words from the frame's position fields, and when bit 0 is set adds acceleration to the object's velocity, clamping it and updating position.
- `clamp_velocity_and_position_offsets`: Adds acceleration at offset 46 to velocity at 30, clamps it to zero, integrates into position at 12, then adds gravity at 42 to vertical velocity at 34 and clamps it to zero.
- `apply_gravity_and_spawn_effect_object`: Adds gravity at offset 50 to vertical velocity at 34, clamps it, integrates into position at 16, and if the object has a pending effect, calls the effect spawner and links the new object into the active list.
- `link_object_into_active_slot`: Reads the active slot index from a5 offset -402, scales it by four, loads the slot pointer from the table at a5 offset -400, and stores the object's next pointer at offset 8 of that slot.
- `copy_a3_field_to_a4_and_clear_flags`: Copies the longword at offset 4 of a3 to four bytes before a4, clears bits 0 and 1 of the byte at offset 2 of a3, then branches to another routine.
- `advance_a3_counter_and_call_73b0`: Sets bit 0 at offset 2 of a3, and if the word at offset 22 is nonzero and the word at offset 20 is zero, calls the routine at 73b0 with a3 and that count, storing the result plus one at offset 20.
- `reset_a3_counter_and_dispatch_substate`: Copies the word at offset 22 of a3 to offset 20 and decrements it, optionally calls a table routine indexed by the byte at offset 3, adds offsets 12 and 16 of a3 to words at offsets 26 and 24 of a5, and calls another routine with a3 and those values.
- `append_entity_to_active_list`: Increments the global counter, stores the a3 record pointer into the a5 list, copies a5 offsets into the record, and links the a3 field into the a4 list before branching back.
- `unwind_frame_restore_a4_a0_and_jump`: Unlinks the a5 frame, pops a4 and a0 from the stack, discards twenty bytes of arguments, and jumps to the popped a0 address; the trailing instructions after the jump are unreachable.
- `store_a4_minus8_into_pc_relative_slot`: Loads the address of the nearby data slot into a0 and copies the longword at -8(a4) into the slot at -4(a0), then returns.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `restore_d1_and_jump_library_vector`: Loads the caller's return address into d1, loads the library base from a pc-relative pointer into a6, moves the return address down one slot, pops four bytes and jumps through the library vector at offset minus 198.
- `store_a4_minus20_into_pc_relative_slot`: Loads the address of a nearby pc-relative slot into a0 and copies the longword at offset minus 20 from a4 into the slot four bytes before it, then returns.
- `restore_registers_and_jump_library_vector`: Reloads a0 through a2 and d0 through d7 from the stack frame, loads the library base from a pc-relative pointer into a6, moves the return address into the frame, pops the frame and jumps through the library vector at offset minus 30.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `store_custom_chip_base_pointer`: Opens an a5 stack frame, writes the custom chip register base address into a longword global, then releases the frame and returns.
- `copy_a4_minus20_to_pc_relative_global`: Loads a program-counter-relative address into a0 and copies the longword at offset minus twenty from a4 into the global four bytes before that address, then returns.
- `clear_bit3_in_caller_frame_word_and_jump`: Loads the caller return address from the stack, clears bit three of the word at offset thirty-two in that frame, pops the address and jumps to it after skipping four bytes.
- `call_library_offset_minus_246_with_args`: Loads a1 from the stack, d0 and d1 from stack words, fetches the library base from pc-relative memory into a6, shifts the return address down and jumps through offset minus 246 of the library vector table.
- `call_library_offset_minus_240_with_args`: Loads a1 from the stack, d0 and d1 from stack words, loads the library base into a6, moves the return address into the argument slot, advances the stack and jumps through offset minus 240 of the library vector table.
- `forward_args_to_library_vector`: Loads arguments from the stack into a1 and d0 through d3, fetches the library base from pc-relative memory into a6, shifts the return address and jumps through the library vector.
- `execute_indirect_jump`: Loads a pointer from an offset of the stack into a register, loads a word from an offset of the stack into a register, loads a base address into a register, copies a value from the stack to an offset of a pointer, increments the stack pointer, and jumps to an address calculated from a register and a constant offset.
- `store_pointer_to_global_data_block`: Opens a stack frame, branches forward, loads the address of a data block into a3 and stores it in a global longword, then returns.
- `open_frame_and_branch_forward`: Creates an a5 stack frame with no local space and branches unconditionally to another routine without returning here.
- `create_and_init_34_byte_object`: Allocates a 34-byte object, fills its fields from the caller's arguments, registers it via a library call, and returns the object pointer in d0 or zero on failure.
- `release_object_and_free_34_bytes`: If the object's field at offset 10 is set, calls a release routine, then marks the object closed, clears its field at offset 20, frees its byte field value and frees the 34-byte block.
- `jump_to_shared_741e_entry`: Sets up an a5 stack frame and branches unconditionally to the shared code at offset 741e, which performs the actual work.
- `scale_global_counter_by_divisions`: Loads the global counter, adds one, divides by a large constant, stores the quotient back, then divides the stored value and the stack argument by ten thousand using the math helpers.
- `save_a4_load_pc_relative_and_enter_frame`: Pushes a4 onto the stack, loads a4 from a program-counter-relative longword, opens an a5 stack frame and branches away to another routine.
- `setup_blitter_and_loop_over_entries`: Saves a4, links a frame, calls a subroutine, reads DMACONR bit 6, programs blitter registers at offsets in a3, then loops comparing a counter against a byte from a2.
- `copy_sprite_offsets_to_channel_struct`: Checks a bit in d2 and the DMACONR busy bit, then copies two word offsets and two longword sprite pointers plus a word from a4/a5/a2 tables into fields at offsets 80, 84, 88, 100 and 102 of the a3 structure.
- `clear_channel_and_load_sprite_pointers`: Zeroes words at offsets 100 and 102 of a3, loads two longword sprite pointers from a2 and a5 tables into offsets 80 and 84, computes a word from a2 into offset 88, increments the a5 counter and returns via a jump.
- `setup_copper_list_from_dimensions`: Computes scaled dimensions from stack arguments, builds a control word from a masked value, loads a3 and a2 pointers, tests DMACONR and a stack longword, then writes a copper control word and a word to offsets 64 and 116 of a3.
- `blit_columns_with_dmacon_wait`: Configures blitter registers from a5 parameters, waits on DMACONR bit 6, then blits each column offsetting the destination pointer by the row stride.
- `compute_blit_stride_and_offset`: Multiplies two a5 words, doubles the product into a long stride, subtracts twice the width from the destination pointer, and derives a nibble offset from the sign of the third word.
- `compute_shift_masks_and_blit_rows`: Derives shift count and masks from the low nibble of a5 word, sets blitter registers, then loops over rows advancing the destination by the computed stride.
- `scale_shift_by_negated_angle`: Decrements the longword at offset 12 of a5, loads the word at offset -26 of a5 into d2, negates it, sets d4 to all ones, and if d2 is non-negative negates it and shifts d4 right by d2 before branching.
- `shift_mask_by_positive_angle`: Shifts d4 left by d2, reloads the word at offset -26 of a5 into d2, and if d2 is negative negates it and shifts d4 right by d2 before branching.
- `compute_complement_angle_and_mask`: Shifts d4 left by d2, stores d4 at offset -24 of a5, sets d2 to sixteen minus the word at offset -26 of a5, stores that back, then builds a mask in d2 from the new angle and branches.
- `scale_d2_by_d4_sign_adjust`: Shifts d2 left by d4 bits, loads a negative word from a5 offset into d4, negates it, tests it, negates again if negative, then shifts d2 right by d4 and branches.
- `setup_blitter_copper_list_and_loop`: Stores shifted d2, loads copper pointer and source, waits on DMACONR bit 6, writes blitter and copper registers, calls a multiply routine, then loops writing sprite entries and advancing the source pointer.
- `init_sprite_scale_and_bounds`: Saves a4, loads a4 from pc-relative data, opens a stack frame, loads a3 and a2 pointers, computes scaled offsets from arguments into locals, sets sentinel words, and branches on the vertical argument sign.
- `init_velocity_and_clear_vertical_flag`: Clears the word at offset -50 from a5, computes a doubled negative product of words at offsets 16 and 24 into d2, stores it at -16 from a5 and adds it to the longword at offset 30 from a5, then branches to the shared continuation.
- `clamp_horizontal_delta_to_limit`: Subtracts the word at offset 16 from a5 from the word at offset 2 of a2, stores the result at -42 from a5, and if it exceeds the word at offset 22 from a5 replaces it with that limit before branching onward.
- `test_clamped_delta_and_branch`: Tests the word at offset -42 from a5 and branches to the continuation when it is positive, otherwise jumps to the alternate path; it performs no other memory or register changes.
- `scale_a2_word_and_test_vertical_speed`: Reads a word from (a2), shifts it left by three into d2 and stores it at offset -44 from a5, then tests the word at offset 18 from a5 and branches based on its sign.
- `negate_and_scale_velocity_fields`: Negates a word at offset 18 in the a5 frame, derives shifted values, updates several frame fields, and branches to another routine when the computed value is nonzero.
- `build_shift_mask_from_negated_delta`: Negates the word at offset -38 from a5 into d2, then builds a mask in d4 by shifting all ones right by d2 when d2 is positive, negating d2 first if negative.
- `scale_d4_and_set_blit_modulos`: Shifts d4 left by d2 bits, stores it at offset -26 from a5, computes sixteen minus the word at -38 into -36, writes -16 to -52 and branches onward.
- `compute_remaining_span_and_branch`: Masks the word at -52 with fifteen into -36, computes a span from the word at 18 plus -46 minus -44 into -38, then branches based on its sign and a comparison.
- `round_span_to_blocks_and_adjust_offsets`: Rounds the word at -38 up to a multiple of sixteen into -40, subtracts it from 24, doubles it, subtracts from the long at -4, adds to -30, masks -38 and branches.
- `negate_shift_count_to_d2`: Loads d2 with all ones, reads the signed word at offset -38 from a5 into d4, and if negative negates d4 then shifts d2 right by d4, then branches on.
- `compute_sprite_geometry_and_dma_setup`: Scales d2 by d4, derives sprite position and size words from a2 and a5 offsets, waits on DMACONR bit 6, writes sprite control words into a3, and advances the animation pointer.
- `write_sprite_control_words_loop`: Sets sprite control word in a3 from a5 offset, clears a counter, then loops writing position and pointer longs into a3 while waiting on DMACONR bit 6, then unwinds the stack frame.
- `save_a4_and_jump_to_7e12`: Pushes a4 onto the stack, loads a4 from the longword at the pc-relative address, opens an empty a5 frame, then branches to another routine.
- `pack_three_bytes_into_word_d0`: Saves a4, loads a4 from pc-relative memory, opens an a5 frame, then combines bytes at offsets 16, 14 and 12 from a5 into d0 and branches onward.
- `restore_registers_and_jump_a0`: Releases the a5 frame, pops a4 and a0 from the stack, skips six bytes of arguments, and jumps to the address in a0.
- `nested_loop_call_with_three_args`: Saves a4, loads the global base into a4, links a frame, then loops comparing a counter against a limit and calls the inner routine with four stack arguments until the counter overflows.
- `nibble_compare_and_dispatch_loop`: Saves a4, loads the global base, links a frame, then loops over rows extracting nibbles from two tables, adjusting values, calling a routine with four arguments, and finally invoking a stored callback or freeing memory.
- `load_global_base_and_jump`: Saves a4 on the stack, loads the global base pointer into a4, creates an empty a5 frame, then branches away to another routine.
- `release_stream_and_buffers`: Closes the stream via its vtable, then frees the two allocated buffers with sizes 490 and 1032 and clears their pointers.
- `open_stream_and_start_playback`: Allocates two buffers, opens a stream, sets up its structure, starts playback, and returns whether the stream pointer is still set.
- `branch_to_playback_entry`: Unconditionally branches to another routine, transferring control without touching registers or memory itself.
- `branch_to_shared_code_at_9352`: Unconditionally branches to the code at offset 9352, transferring execution there without touching any registers or memory.
- `reset_flag_and_scan_four_pointers`: If the word at offset 10 of a6 is nonzero, clears it, then walks four longword pointers at offset 38 of a6, calling a subroutine for each nonzero entry and counting up in d0.
- `parse_four_script_streams_into_state`: Saves all registers, then for four entries at offset 198 of a6 clears state fields and walks word streams pointed to by a0, decoding opcodes into counters and flags until each stream ends.
- `increment_d0_store_to_a1_offset`: Adds one to the low byte of d0, writes that byte to offset 16 of a1, then branches to another routine.
- `advance_pointers_count_four_and_return`: Adds four to a0 and a1, increments d7, compares d7 with four and branches back if not equal, otherwise restores all registers and returns.
- `copy_state_and_init_four_slots`: Saves registers, copies eight longs from a6 offsets 198 to 230, copies field 12 to 16, then for four slots clears and initializes fields, optionally calling a routine and reading a table entry into offset 150.
- `update_audio_channels_and_tempo`: Decrements the tick counter at offset 30 of the sound structure, recomputes tempo from the note table indexed by the current pattern position, writes timer values to the CIA chips, then walks four channel structures updating period, volume and effect state.
- `audio_mix_voices_and_update_dmacon`: Loops over four voice structures at offset 62 of a6, calling each voice's start and mix routines through pointers, accumulating active voice bits in d6 and writing the result to DMACON at a fixed address.
- `set_voice_volume_and_enable_dma`: Clears d0, sets bit d7 in it, writes d0 to DMACON, then computes the AUDxVOL address from d7 shifted left four and stores volume 2 there before returning.
- `allocate_sound_channel_for_sample`: Saves all registers, scans the four channel slots for a free one matching the requested priority, links the sample pointer into the chosen slot and marks it active, returning the channel index or minus one.
- `clear_flag_and_repeat_three_times`: Pushes d0, clears the word at offset 10 of a6, then calls the slot-clearing routine three times via dbra before restoring d0 and returning.
- `clear_slot_and_mark_pending`: Saves d0 and a1, computes the slot record at offset 62 of a6 from the index in d0, clears its first byte, and sets it to two when its second byte equals one.
- `clear_slot_and_retry_three_times`: Saves d0 on the stack, clears the word at offset 10 of a6, then calls a routine three times via dbra, restoring d0 and returning.
- `reset_object_state_and_call_vtable`: Indexes a 22-byte object table from a6 using d0, clears its state byte, and if active calls the object's first vtable entry before resetting the flags.
- `compute_scaled_ratio_from_d0_d1`: Clears word at offset 60 of a6, and if d0 is nonzero computes a scaled ratio from a6 word 0 and d1, storing results at offsets 56 and 58.
- `store_d1_to_a6_offset_zero`: Writes the low word of d1 to offset zero of the structure pointed to by a6, then branches backward to another routine.
- `advance_scroll_position_and_init_audio`: Updates a6 scroll position from d1 and d2, writes the high byte to offset zero, and on first entry clears the a6 structure, sets up the a0 buffer, and programs the CIAA timers and control registers.
- `link_frame_and_jump_to_996e`: Creates an empty a5 stack frame with link and then branches unconditionally to another routine, transferring control without returning here.
- `update_animation_slots_and_tiles`: Iterates four animation slots, decrements frame counters, advances frames, calls the frame update routine, and writes tile and duration values into the tile buffer at offset 160.
- `allocate_and_init_animation_record`: Allocates 20 bytes via the memory allocator, then stores the byte, word and long parameters into fields of the new record and returns its pointer in d0.
- `return_popping_frame_and_extra_args`: Unlinks the a5 frame, pops the return address into a0, discards 14 bytes of arguments from the stack and jumps to the return address.
- `init_loop_counter_and_test_bitmask`: Sets up an a5 stack frame with a two-byte local, clears that local to zero, compares it with three and branches away if greater, otherwise loads the local into d2 and the byte argument at offset eight into d4, then compares d2 with seven and clears d2 if above.
- `test_bit_and_dispatch_table_entry`: Tests bit d2 of d4 and branches away if clear, otherwise reloads the local counter into d2, shifts it left twice, loads the table base into a3, checks the longword at that table offset and calls the routine at the fixed address if non-zero.
- `increment_counter_and_return_via_stack`: Adds one to the two-byte local counter, branches back if no carry, then releases the a5 frame, pops the return address into a0, discards two bytes of stack and jumps to that address.
- `activate_slots_from_bitmask`: Loops over four slot indices, tests each bit of the byte argument, and for set bits clears slot fields, sets an active flag, updates a bitmask word and clears a pointer table entry.
- `deactivate_slot_matching_pointer`: Scans four pointer table entries for one equal to the argument, and on a match pushes a single-bit mask and calls the deactivation routine for that slot.
- `activate_slot_matching_pointer`: Scans four pointer table entries for one equal to the argument, and on a match pushes a single-bit mask and calls the activation routine for that slot.
- `init_four_channel_sample_headers`: Calls a setup routine, then loops over four channel indices, testing bits in the first byte of each sample descriptor and writing period, length and pointer fields into the audio channel table.
- `install_sample_with_optional_loop`: For the current channel index, tests a bit in the descriptor byte, clears loop fields when bit five is set, stores the sample start pointer plus two and sets the period from the sample length.
- `set_looped_sample_period_and_pointer`: Computes the channel table offset, stores half the sample length as period, sets the channel enable bit, records the descriptor pointer in the channel array and advances the index.
- `open_audio_device_and_start_player`: Sets up an a5 frame, calls the audio init routine with value 15, opens the audio device with allocation size 4 and flags, stores the returned handle, then queues the player start callback.
- `play_sound_effect_15_with_params`: Opens a stack frame, pushes sound id 15 and calls the sound routine, then pushes a stored long value, 4 and a stored pointer before calling two library routines, then returns.
- `test_d0_bit_zero`: Tests bit 0 of register d0 and sets the condition codes accordingly, then falls through into the following routine.
- `zero_word_data_at_9986`: A single zero word that decodes as ori.b with immediate zero to d0; it is data referenced by address, not executable code.
- `zero_word_data_at_998a`: Another single zero word decoding as ori.b with immediate zero to d0; a zero-filled data slot referenced by address, not code.
- `btst_d0_d0_data_word`: A single btst.l d0,d0 instruction that tests bit zero of d0 and sets the zero flag; referenced by address as a small data or flag word.
- `mask_d0_and_enter_frame`: Masks d0 with the constant value an address, opens an a5 stack frame with zero local space, then branches away to another routine.
- `free_sprite_slots_and_release_block`: Walks two rows of sprite slot pointers in the structure at a3, frees each non-null entry through the release routine, then frees the block itself and returns.
- `allocate_and_fill_sprite_block`: Allocates a 88-byte block, fills its sprite slot arrays from the given coordinates, builds a sprite descriptor and registers it, returning the new block or zero.
- `dispatch_object_via_156_offset`: Sets up an a5 frame, saves the object pointer, calls a routine with it, then tail-jumps through the pointer stored at offset 342 in the object.
- `toggle_object_state_and_update_pointers`: Reads the object's byte at offset 86, computes a 40-byte record pointer, stores it at offset 4 of the object's sub-structure, calls two routines, flips that byte, and stores the new record pointer at offset 88.
- `empty_frame_then_branch_to_9de4`: Creates an empty a5 stack frame and immediately branches to another routine, performing no other work itself.
- `append_byte_to_output_buffer`: Stores the byte argument into the buffer pointed to by the long at a fixed global, increments that pointer by one, then pops the return address and jumps back.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `exec_addintserver_stack_trampoline`: Loads the word and pointer arguments from the stack, fetches the Exec library base from address 4, shifts the return address and jumps to the AddIntServer vector.
- `exec_cause_stack_trampoline`: Loads the pointer argument from the stack, fetches the Exec library base from address 4, moves the return address down and jumps to the Cause vector.
- `call_disable`: Wrapper that calls the exec library function Disable and returns its result to the caller.
- `call_enable`: Wrapper that calls the exec library function Enable and returns its result to the caller.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `link_frame_then_branch_to_9ee4`: Creates an a5 stack frame with zero local space, then branches unconditionally to another routine at a fixed offset, transferring control without returning here.
- `set_audio_control_word_8020`: Loads the global pointer at a fixed long address into a3 and writes the immediate word 8020 to offset 154 of that structure, then releases the a5 frame and returns.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `call_add_task`: Library stub: loads its arguments and the exec library base, calls AddTask and returns its result to the caller.
- `tail_call_alloc_signal_via_exec`: Reads a word argument from the stack, sign-extends it into d0, loads the exec library base from address 4, shifts the return address down two bytes and jumps to AllocSignal.
- `alloc_trap_via_exec_library`: Reads a 16-bit size from the caller's stack argument, sign-extends it into d0, loads the exec library base from address 4, shifts the return address down two bytes and jumps to the AllocTrap vector.
- `find_task_via_exec_library`: Loads the task pointer argument from the caller's stack into a1, loads the exec library base from address 4, moves the return address down four bytes and jumps to the FindTask vector.
- `free_signal_via_exec_library`: Reads a 16-bit signal number from the caller's stack argument, sign-extends it into d0, loads the exec library base from address 4, shifts the return address down two bytes and jumps to the FreeSignal vector.
- `store_a4_minus20_to_pc_relative_global`: Loads the address of a nearby data word into a0, then copies the longword at offset minus twenty from a4 into the longword four bytes before that address, and returns.
- `tail_call_library_offset_via_a6`: Loads the caller's first argument pointer into a1, loads the library base from a pc-relative global into a6, shifts the return address down one slot, then jumps to the library entry at offset minus four hundred eighty from a6.
- `link_frame_then_branch_to_a532`: Creates a stack frame by pushing a5 and allocating no locals, then branches unconditionally to the code at a532 without returning here.
- `advance_animation_script_frames`: Walks up to four active object slots, steps each scripted entry's frame counter, and on wrap either resets, clears linked entries, or calls the frame-draw routine with packed nibble arguments.
- `release_object_slot_by_pointer`: Scans the four object slot pointers and clears any slot whose stored pointer equals the argument passed on the stack, then returns through a popped return address.
- `reset_audio_state_and_init_buffer`: Sets a flag byte to one, clears four consecutive long globals, then pushes a buffer address and a pc-relative pointer to a library routine before returning.
- `call_global_init_and_return`: Sets up an a5 stack frame, pushes the address of a global variable, calls a subroutine with it, then releases the frame and returns.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `frame_then_jump_to_a646`: Creates an a5 stack frame with no local space and then branches unconditionally to another routine, never returning here.
- `allocate_object_and_set_fields`: Checks the second argument, allocates an object via a subroutine with size and type, stores a byte and two fields from the arguments, then returns the object pointer.
- `call_unnamed_with_0x30_and_arg`: Pushes the caller's first argument and the constant 48 onto the stack, calls an unnamed routine, then unwinds the frame and jumps back to the caller.
- `init_object_fields_and_dispatch`: If the passed pointer is non-null, sets byte 8 to 255 and longs at offsets 20 and 24 to -1, then calls an unnamed routine with the word at offset 18.
- `load_a4_from_pc_and_branch`: Saves a4 on the stack, loads a4 from a pc-relative longword, opens an a5 frame and branches to another routine.
- `build_and_submit_sprite_descriptor`: Builds a 32-byte descriptor on the stack from the a4 config block and the three incoming arguments, sets status flags, then passes its pointer to the submit routine.
- `push_a4_set_base_and_jump`: Saves a4, loads the module base pointer from the pc-relative long at a786 into a4, opens an empty a5 frame and branches to the shared code at a83c.
- `build_and_submit_effect_descriptor`: Copies seven incoming arguments and fields from the a4 config block into a 48-byte stack descriptor, sets a type word based on one argument, then passes it to the submit routine.
- `save_a4_load_global_and_return`: Pushes a4, loads a4 from the longword at the pc-relative address, opens and immediately closes an a5 frame, restores a4 and returns.
- `store_a4_minus16_to_global`: Loads a0 with a pc-relative address and copies the longword at offset minus sixteen from a4 into the longword just before that address, then returns.
- `store_a4_minus20_to_global`: Loads a0 with a pc-relative address and copies the longword at offset minus twenty from a4 into the longword just before that address, then returns.
- `a4_frame_trampoline_to_b8a0`: Saves a4 on the stack, loads a4 from a nearby constant, opens an a5 frame, then branches to the routine at b8a0 before restoring a4 and returning.
- `a4_frame_trampoline_to_bdd8`: Pushes a4, loads a4 from a nearby constant, creates an a5 stack frame and branches to the routine at bdd8 without restoring a4 itself.
- `setup_ui_and_decrement_counter`: Saves a4 and a5 frame, loads globals, calls three subroutines to set up display and sound, clears a flag, stores pointers, and decrements a word counter unless it is zero.
- `refresh_selected_unit_panel`: Clears a display region, draws the selected unit's sprite, copies panel coordinates, prints a label, and updates the unit's status display before returning.
- `empty_stub_restore_a4_and_return`: Saves a4, loads a4 from a pc-relative long, opens and immediately closes an a5 frame, restores a4 and returns; it does nothing else.
- `empty_stub_restore_a4_and_return_alt`: Saves a4, loads a4 from a pc-relative long, opens and immediately closes an a5 frame, restores a4 and returns; it performs no other work.
- `store_a4_minus20_to_be38_global`: Loads the address of the nearby global slot into a0 and copies the longword at offset minus twenty from a4 into that slot, then returns.
- `tail_call_library_offset_minus366`: Moves the first stack argument into a1, loads the library base from the global slot into a6, shifts the return address down one slot, pops four bytes and jumps to library offset minus 366.
- `tail_call_library_offset_minus378`: Loads the second stack argument into a1, the word arguments into d0 and d1, loads the library base into a6, moves the return address down two slots, pops eight bytes and jumps to library offset minus 378.
- `tail_jump_after_frame_link`: Links a5 to a new stack frame with zero local space, then branches unconditionally to another routine without returning here.
- `call_with_count_and_byte_arg`: Pushes the longword at offset 8 of the frame, then the word 10000 and the byte 255, calls a routine, unlinks a5 and jumps back through the saved return address.
- `call_two_routines_with_frame_args`: Pushes the longword at offset 12 and the words at offsets 10 and 8, calls one routine, pushes the longword at offset 12 again, calls a second routine, then unlinks and jumps back.
- `link_frame_and_jump_to_c212`: Creates an a5 stack frame with zero local space, then branches unconditionally to the code at the fixed target address, transferring control without returning here.
- `copy_two_entries_and_accumulate_deltas`: Loops over two table entries, copies words from a source pointer into a working table, then computes byte differences between two tables and adds them into a per-entry accumulator array.
- `wrap_delta_and_advance_entry_counters`: Wraps a signed delta past 127 by subtracting 255, adds it to a table word, advances the inner and outer counters, and on completion stores a new source pointer and table constants.
- `save_a4_set_global_a4_and_return`: Pushes a4, loads a4 from a program-counter-relative longword, opens and immediately closes an a5 frame, restores a4 from the stack and returns.
- `save_a4_load_second_global_a4_and_return`: Pushes a4, loads a4 from a different program-counter-relative longword, opens and closes an a5 frame, pops a4 and returns to the caller.
- `copy_a4_minus20_to_global_longword`: Loads the address of a nearby longword into a0, copies the longword at offset minus twenty from a4 into that global, then returns.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `jump_through_a6_offset_minus_126`: Loads the return address from the stack into a0, loads the library base from a pc-relative longword into a6, moves the caller's return address down four bytes, pops four bytes and jumps through offset minus 126 of a6.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `exec_addhead_tailcall_thunk`: Loads two stack arguments into a0 and a1, loads the exec library base from absolute address four into a6, shifts the return address down eight bytes, pops eight bytes and jumps through offset minus 240 of a6 to call AddHead.
- `init_three_longword_struct_and_jump`: Takes a pointer argument, stores the pointer at its first longword, adds four to that value, clears the second longword, stores the pointer at the third longword, then pops the return address and jumps to it.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_vector_with_two_args`: Loads the word at offset 8 of the stack into d0, loads the return address into a6, shifts the return address down over the argument, pops six bytes and jumps through the library vector at offset minus 18 of a6.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `add_resource_via_exec_vector`: Loads the argument at offset 4 of the stack into a1, loads the exec library base from address 4, moves the return address down over the argument, pops four bytes and jumps through the AddResource vector at offset minus 486 of a6.
- `open_resource_via_exec_vector`: Loads the argument at offset 4 of the stack into a1, loads the exec library base from address 4, moves the return address down over the argument, pops four bytes and jumps through the OpenResource vector at offset minus 498 of a6.
- `link_frame_then_jump_to_shared_body`: Pushes a5 and allocates an empty stack frame with link, then branches unconditionally to another routine's body at a fixed offset.
- `write_8100_to_audio_register_offset`: Loads a fixed absolute pointer into a3, writes the immediate value 33024 to offset 150 of that structure, then releases the frame and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_22`: Empty routine: returns to the caller immediately without doing anything.
