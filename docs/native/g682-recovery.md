# Game 682 (Deuteros v1.10 0958): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00682). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4138489 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 47 of 47 units identical (bytes and relocations), 40 raw-word lines |
| Functions named and described | pass, 216 of 216 named (0 author symbols), 216 of 216 described (27 by rule, 189 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1002 of 1003 (99.90%) |
| Recovered without load address (counted in proof 1, flagged) | 108544 bytes, 7 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3945 of 5300 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 241818 |
| code | 21066 |
| data | 3770000 |
| padding | 133 |
| unused | 105472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 512 | 44 |
| Disk.1:bootblock | 1 | 31 | 1 |
| logged disk load from Disk.1 | 1 | 592 | 30 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 3362 | 114 |
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
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |
| Disk.1 bytes 497664-526336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 566272-636928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 23552-29184 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 95232-98816 (code never loaded by a reference run) | 1 | 772 | 7 |

## Cases handled

- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: custom-format (non-DOS) disk, 1584 non-empty blocks: matched against track-loaded segments
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: custom-format (non-DOS) disk, 1697 non-empty blocks: matched against track-loaded segments
- Deuteros_v1.10_0958/Deuteros_v1.10_0958.hdf: hard-disk image (AmigaDOS OFS, 4448 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree Deuteros_v1.10_0958_hdf, which is recovered file by file; no block outside the filesystem holds data
- extended reference run ignored: it does not re-execute 1070 of the 1084 PCs the canonical run logged (15 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- 38 logged WHDLoad disk loads recovered as units built from the disk image bytes
- prefix run f150 ignored: it logged 1 PCs the canonical run never executed (not a prefix of it)
- prefix run f300 ignored: it logged 1 PCs the canonical run never executed (not a prefix of it)
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: bytes 497664-526336 look like code no run loaded: recovered position-independently (load address unknown)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: bytes 566272-636928 look like code no run loaded: recovered position-independently (load address unknown)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: bytes 23552-29184 look like code no run loaded: recovered position-independently (load address unknown)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: bytes 95232-98816 look like code no run loaded: recovered position-independently (load address unknown)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: 40 logged loads cover 358400 bytes of the image
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 2-21 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.76)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 154-170 (bytes 78848-87552) outside the filesystem hold data, not code (opcode density 0.046, entropy 5.85)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 172-194 (bytes 88064-99840) outside the filesystem hold data, not code (opcode density 0.043, entropy 6.23)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 198-219 (bytes 101376-112640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.72)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 605-873 (bytes 309760-447488) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.42)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 875-880 (bytes 448000-451072) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.23)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.41)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 931-931 (bytes 476672-477184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 934-937 (bytes 478208-480256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 951-970 (bytes 486912-497152) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.48)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 1089-1104 (bytes 557568-565760) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.33)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 1245-1746 (bytes 637440-894464) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.63)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.1: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.54)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: 2 logged loads cover 9216 bytes of the image
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 2-24 (bytes 1024-12800) outside the filesystem hold data, not code (opcode density 0.036, entropy 4.93)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 26-27 (bytes 13312-14336) outside the filesystem hold data, not code (opcode density 0.057, entropy 4.82)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 33-45 (bytes 16896-23552) outside the filesystem hold data, not code (opcode density 0.052, entropy 5.59)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 60-76 (bytes 30720-39424) outside the filesystem hold data, not code (opcode density 0.041, entropy 4.80)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 88-111 (bytes 45056-57344) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.74)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 132-148 (bytes 67584-76288) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.91)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 150-182 (bytes 76800-93696) outside the filesystem hold data, not code (opcode density 0.054, entropy 4.93)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 193-275 (bytes 98816-141312) outside the filesystem hold data, not code (opcode density 0.015, entropy 5.78)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 277-441 (bytes 141824-226304) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.06)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 451-1488 (bytes 230912-762368) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.47)
- Deuteros_v1.10_0958/Deuteros_v1.10_0958_hdf/Deuteros/Disk.2: blocks 1496-1759 (bytes 765952-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.01)
- 3 never-loaded region(s), 104960 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_disk_interrupt_handler`: Calls three routines through a base pointer, compares the returned word against two constants, and depending on the result writes jump instructions and a routine address into fixed memory locations before storing a code pointer into a global and calling another routine.
- `wait_for_fire_button`: Polls the CIAA port register, looping while bit 6 is set and bit 7 is clear, then waits for bit 0 to go low and high, repeating up to 200 times before returning.
- `handle_trackdisk_io_request`: Processes an I/O request structure: for a read command it zeroes a buffer, for a write command it searches a table for a matching entry and calls a routine, and for other commands it calls a function pointer; it also updates a global length and builds a jump stub sequence.
- `build_jump_stub_sequence`: Writes a fixed sequence of opcode words and addresses into a global area to form jump and return instructions referencing another routine, then returns a byte with the top bit set; the same sequence is written whether or not a global equals four.
- `build_jump_stub_table`: Writes a sequence of opcode words and addresses into a global area, forming jump and return instructions that reference another routine.
- `store_return_opcode`: Writes the value of a return instruction opcode into a global variable.
- `store_jump_opcode`: Writes a jump instruction opcode and the address of a global variable into consecutive global variables.
- `set_global_to_two`: Stores the value 2 into a global variable.
- `set_global_to_three`: Stores the value 3 into a global variable.
- `negate_field_and_call_vectors`: Negates a 16-bit field at an offset from an address register, stores a code pointer and a register value into globals, then calls three function pointers from offsets of the address register and performs an indirect jump through a table.
- `jump_through_unrecovered_table`: Performs an indirect jump through a jump table that could not be recovered.
- `jump_through_second_table`: Performs an indirect jump through a second jump table that could not be recovered.
- `clear_bits_in_register`: Reads a hardware register and returns its value with bits 4 through 6 cleared.
- `store_stack_word_to_global`: Copies a word from the stack into a global variable and touches the interrupt request register.
- `return_zero_constant`: Returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: Sets bit 1 in the byte at offset 14 of the structure pointed to by an address register, then returns.
- `empty_routine_2`: Empty routine that only returns to its caller.
- `duplicate_structure_with_allocations`: Allocates a new structure, copies a word count and pairs of pointers from the source, allocating memory for each non-null pointer, and frees the new structure on failure.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `return_immediately`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `find_node_by_string`: The routine iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `store_register_to_global`: The routine copies the value from a data register into a global variable.
- `copy_global_and_jump`: The routine copies a global variable into another global variable and then performs an indirect jump through an unrecovered jump table.
- `copy_byte_through_pointer`: The routine copies a byte from an offset of a pointer argument into an offset of a frame-relative destination.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: The routine compares a global variable to a specific constant value.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `check_collision_flags`: XORs a global state word with a value from a linked list node and checks for specific bit flags.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a7, then returns.
- `copy_word_if_valid`: Checks if two pointer arguments are non-null.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with two register bytes using bitwise logic, storing the computed byte through a pointer offset and returning it ORed with a byte from another pointer offset.
- `clear_four_words`: Writes the value -1 to four consecutive 32-bit words ending at an offset in a structure pointed to by a register.
- `calculate_table_offset`: Reads three values from a global table, divides one by fifty, multiplies another by a constant and adds the third, then combines the results and stores the computed value into a global variable.
- `select_status_byte`: Clears a byte at offset 31 of a structure, then based on flags and a character field either calls a function pointer, sets the byte to a constant, or stores a character derived from a table; finally calls another routine and returns the byte.
- `flip_bits_and_negate_result`: Negates a 16-bit value at an offset in a pointer and flips the high bit of a byte in a structure, returning a value based on the state of that bit before it was flipped.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location, negates the zero test result, and stores the resulting byte into a location pointed to by a pointer.
- `return_immediately_2`: The routine consists of only a return instruction; it performs no work and returns to its caller.
- `return_immediately_3`: The routine consists of only a return instruction; it performs no data operations and simply returns to its caller.
- `toggle_flag_and_count`: Clears bit 1 of a byte at offset 64 of a structure, increments a counter if the bit was set, then tests and clears a bit in a global byte and conditionally sets the flag and increments the counter again.
- `set_bit_in_byte_array`: Disables the exec kernel, calls another routine to obtain an index, stores a byte from a register through a pointer offset by that index, sets a bit in a byte pointed to by a pointer using the index, then calls another routine and enables the exec kernel.
- `return_immediately_4`: The routine consists of three instructions that perform no data manipulation and simply return to the caller.
- `store_byte_through_pointer`: Reads a byte through an address register and writes the same byte back to that address, effectively a no-op self-store.

### Disk.1:bootblock

- `send_trackdisk_commands_and_clear_state`: Writes command words and a zero byte into an IO request structure and invokes the exec vector three times, with a busy delay loop between the first and second calls; then stores the request pointer and zeroes several global variables.

### logged disk load from Disk.1

- `initialize_trackdisk_and_display_3`: Calls routines to open trackdisk.device, initialize display structures and buffers, then issues I/O requests and closes the device and removes the display.
- `open_trackdisk_and_create_port`: Calls exec to open trackdisk.device, fills in a message port structure with a name and flags, adds the port, finds a task, and if the lookup fails it loops forever; otherwise it sets a field to all ones and clears a byte.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `reset_request_and_do_io`: Clears the length field of the global IO request, sets its command word to nine and its flag byte to zero, then invokes exec DoIO on that request.
- `chunk_large_transfer_and_issue_io`: While the incoming length exceeds a fixed chunk size, subtracts that size, calls another routine with the adjusted values and advances two registers; then clears a length field, sets a command word and flag byte in a global IO request and calls through a function pointer.
- `reset_io_request_and_dispatch`: Clears the length field of a global IO request, sets its command word to nine and its flag byte to zero, then calls through a function pointer from a global variable.
- `fill_request_from_registers_and_do_io`: Sets the global IO request command word to a fixed value and copies three incoming register values into its length and address fields, clears a flag byte, then invokes exec DoIO.
- `initialize_trackdisk_and_display_2`: Calls routines to open trackdisk.device, initialize display structures and buffers, then issues I/O requests and closes the device and removes the port.
- `return_constant_pair`: Returns a fixed pair of constant values in the data registers without touching memory or calling anything.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `initialize_two_display_structures`: Calls several routines through a table of function pointers, then fills two parallel structures with fixed field values (dimensions, pointers to zeroed buffers, and four buffer pointers spaced apart) and clears a block of words.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `init_two_bitmap_structures`: Calls a library routine to obtain a base pointer, then initializes two parallel structures: each gets a header, a zeroed table of twenty words, four plane pointers spaced by a fixed stride, and dimensions of 320 by 200, before invoking further library routines to set them up.
- `initialize_two_display_structures_2`: Calls into a library base to obtain a base pointer, then fills two parallel display/copper-style structures with zeroed fields, screen dimensions, bitplane pointers, and color/control values, invoking library routines to set up and enable each structure.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_two_display_structures_3`: Calls a routine through a pointer and stores the result, then fills two parallel display/bitmap structures with dimensions, pointers to four bitplane buffers, and zeroed fields, invoking several routines through the same pointer table between the two setups.
- `initialize_display_structures`: Calls several routines through a table of function pointers, then fills two parallel display/copper-style structures with fixed field values, clears a block of words, and stores four pointers spaced by a fixed stride into each structure.
- `initialize_display_structures_2`: Calls a routine through a global pointer, then initializes two parallel display/copper-like structures in memory: it sets up base pointers, dimensions, and clears a block of words, then invokes further routines through the same pointer to configure and enable them.
- `init_display_buffers`: Calls several routines through a global function-pointer table, then initializes two parallel structures of display/bitmap parameters (dimensions, pointers, and zeroed fields) and registers them via further table calls.
- `initialize_two_display_bitmaps`: Opens graphics.library and allocates chip memory, then fills two parallel display/bitmap structures with screen dimensions, bitplane pointers, and zeroed tables, and registers them via graphics library calls.
- `fill_display_structures`: Issues two IO requests with fixed command and address fields, then copies four interleaved words from a source table into four parallel destination arrays, and finally calls a function pointer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 228 into d1.
- `store_pointer_from_low_nibble`: Computes a pointer from the low nibble of the input word shifted left three bits and stores it into a global variable.
- `store_pointer_from_low_nibble_2`: Computes a pointer from the low nibble of the input word shifted left three bits and stores it into a different global variable.
- `parse_byte_stream_with_commands`: Walks a byte stream, dispatching on each byte: bytes below a threshold are handled by another routine, a specific byte is skipped, two bytes advance the pointer by two, and two other values update a pair of globals by adding a fixed offset; stops at a zero byte.
- `parse_stream_with_commands`: Walks a byte stream, dispatching on each byte: bytes below a threshold are handled by another routine, a specific byte is skipped, two bytes are handled by routines that store pointers from their low nibbles, and two other bytes adjust global variables before advancing.
- `compute_offset_from_bytes`: Reads two bytes from a structure, clamps the second to a maximum of 48, and computes a pointer by adding the first byte plus the clamped value times 160 to a base address.
- `blit_masked_rows_to_bitmap`: Reads bytes from a table indexed by the input byte minus a constant, and for each of eight rows writes four masked bytes into a destination bitmap using two source pointers, advancing the destination pointer by a fixed stride per row and incrementing a global pointer afterward.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_trackdisk_and_display`: Calls exec AvailMem twice, opens trackdisk.device, initializes display structures, clears a large global area, and then loops forever if a call returns zero.

### logged disk load from Disk.1

- `empty_routine_3`: Contains a single instruction and is called by one unnamed routine; no other behavior is evident.
- `open_graphics_library`: Calls exec OpenLibrary for graphics.library, stores the returned base in a global, and if successful increments another global; otherwise loops forever.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_four_times`: Calls the same library vector at a fixed negative offset from a library base four times in succession.
- `call_library_vector_and_clear_global`: Calls a library vector through a stub and then clears a global variable to zero.
- `advance_index_from_table`: Adds a table entry indexed by the current global value to that value plus a constant, storing the result back into the global variable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 136 into d2.
- `empty_global_self_assign`: Contains a single instruction that assigns a global variable to itself, leaving the data unchanged.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d7.
- `self_assign_global_noop`: Reads a global variable and writes the same value back to it, leaving memory unchanged, then returns the byte passed in the data register.
- `return_byte_in_d0`: Returns the byte passed in the low byte of the data register unchanged; the only other operation is a self-assignment of a global variable to itself.
- `call_routine`: Calls another routine and returns.
- `jump_to_absolute_address`: Transfers control to an absolute address by calling through a fixed target.
- `tiny_ori_5`: Small routine of 1 instruction that merges 176 into d2.
- `call_parse_stream_routine`: Calls another routine that walks a byte stream, invoking handlers for bytes above a threshold and dedicated handlers for specific bytes.
- `compute_table_pointer_from_low_nibble`: Takes the low nibble of the input word, shifts it left by three, adds a fixed base, and stores the resulting pointer in a global variable.
- `compute_table_pointer_from_low_nibble_2`: Takes the low nibble of the input word, shifts it left by three, and adds it to a fixed base to form a pointer stored in a global variable.
- `interpret_control_byte_stream`: Reads bytes from a pointer, dispatching on control codes to call other routines and adjust the pointer, and updates a global variable based on code values.
- `parse_stream_with_bitmap_blits_2`: Walks a byte stream, calling routines for bytes above a threshold (twice when the high bit is set), handling specific bytes with dedicated routines, and adjusting global variables for two other bytes before advancing.
- `parse_stream_with_bitmap_blits`: Walks a byte stream, calling routines for bytes above a threshold (twice when the high bit is set), handling a specific byte, advancing by two for two command bytes, and for two other values updating a global by adding a fixed offset or by a shifted sum; stops at a zero byte.
- `set_global_and_call`: Stores the input word ORed with a constant into a global location, then calls another routine.
- `accumulate_offset_from_structure_bytes`: Reads two bytes from a structure pointed to by an uninitialized address register, caps the second at forty-eight, multiplies it by one hundred sixty, adds the first byte and the current global value, and stores the sum back into the global variable.
- `call_byte_returning_routine`: Calls another routine and returns the value it produces in the data register.
- `blit_masked_bytes_into_bitmap`: Uses a byte from a table to combine two source bitmaps with a mask and writes the result into a destination bitmap across a small block, then doubles a global variable.
- `empty_routine`: Contains no instructions and returns immediately.
- `initialize_bitmap_structures_at_offset`: Calls a routine to initialize two bitmap structures, passing a pointer derived from an address register plus two.
- `repeat_blit_until_counter_zero`: Repeatedly calls a bitmap-blitting routine and loops until a returned word value becomes zero.
- `execute_self_and_return`: Calls itself once and then returns the value stored in a register.
- `poll_status_and_dispatch_2`: Loops reading a global status byte, calling another routine each pass, and branching on individual bits of the status; depending on the bit, calls further routines.
- `initialize_vectors_and_call_handler`: Writes fixed values into several global locations and then calls a routine through a pointer derived from a global variable.
- `store_interrupt_server_table`: Writes a small set of fixed values into consecutive global words and longs, then calls another routine through a pointer derived from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `open_trackdisk_and_setup_port`: Calls library functions to allocate a port, find a task, and open the trackdisk device, then initializes fields in the resulting structure and loops forever if opening fails.
- `issue_doio_with_command_nine`: Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine.
- `issue_doio_with_command_8004`: Sets the command field of a global structure to a fixed value and a flag byte to zero, then calls the I/O routine.
- `issue_doio_with_command_five`: Sets the command field of a global structure to five and a flag byte to zero, then calls the I/O routine.
- `issue_chunked_io_requests`: Loops over a length, issuing I/O requests in chunks of a fixed size, setting fields in a global structure and calling the I/O routine for each chunk.
- `issue_doio_and_copy_words`: Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer.
- `issue_io_and_load_pointer`: Calls two routines that issue I/O requests, then loads a longword from a fixed offset in a global structure into a global variable.
- `copy_table_entry_and_call`: Computes a table index from the low word of the data register, copies a longword and a word from that table entry into two global variables, then calls a routine through a library vector with the address register minus one as argument.
- `blit_masked_tile_to_bitplanes`: Reads a command word from a structure and, for the masked-copy path, ORs/ANDs four interleaved bitplane words from a source buffer into a destination bitplane buffer using a mask, advancing through rows and planes.
- `decompress_rle_bitmap`: Decodes a run-length-encoded bitmap stream into a destination buffer, expanding literal, repeated-byte, and repeated-word runs while wrapping the destination pointer at a fixed boundary and advancing row by row.
- `blit_run_length_encoded_tile`: Decodes a run-length encoded byte stream into a planar bitplane buffer, expanding literal, repeated-byte, and word-copy runs while advancing through rows of a fixed stride and wrapping at the buffer end.
- `relocate_and_process_object_table`: Walks a table of pointers, adds a base offset to each non-null entry, copies the resulting object records into a working array, then iterates over those records applying per-record state changes (countdowns, counters, and conditional calls to another routine) before a final pass over the working array.
- `relocate_and_process_table`: Rebases a table of pointers by adding a fixed base to each nonzero entry, copies the resulting records into a working buffer, then walks the records applying per-record state changes (countdowns, counters, and conditional calls to another routine) before a final pass over the buffer.
- `relocate_and_process_table_2`: Relocates a table of pointers by adding a base offset and stores the results into a destination table, then iterates over entries copying fields and dispatching on a type byte to decrement counters, reset entries, or call another routine.
- `process_entity_table`: Copies a table of pointers into a working array, then iterates over entries updating per-entry state fields (counters, timers, and type-dependent branches) and invoking other routines; finally compacts the entries and calls further routines.
- `relocate_and_process_object_table_2`: Walks a table of pointers, adds a base offset to each non-null entry, stores the results into a second table, then copies each entry's fields into a working array and iterates over that array, decrementing counters and invoking another routine based on per-entry type codes.
- `relocate_and_process_table_3`: Rebases a table of pointers by adding a fixed base offset, copies entries into a working array, then iterates over the entries dispatching on a type byte to decrement counters, reload timers, and recursively invoke itself for certain types before a final pass.
- `relocate_and_process_table_4`: Relocates a table of pointers by adding a base offset, copies entries into a working array, then iterates over the entries dispatching on a type byte to decrement counters, reset fields, or invoke another routine; finally compacts the list and calls a helper.
- `initialize_and_run_entity_tables`: Rebases a table of pointers by adding a fixed base, copies the resulting records into a working buffer, then iterates over the entries updating per-entry counters, timers and type-dependent state, recursively invoking itself for certain entry types.
- `update_active_object_slots`: Walks a fixed-size table of object slots, decrementing counters and invoking per-slot handlers based on a type byte, then runs a second pass over the same table that counts active slots and dispatches each to one of two routines.
- `interpret_bytecode_script`: Interprets a byte-coded script stream: reads a type byte and its operands, dispatching on opcodes to store values into a state structure, adjust counters, jump within the stream, call helper routines, and invoke a library function; returns when an end or blocking opcode is reached.
- `clear_buffer_alternating`: Increments a global counter, selects one of two buffer pointers based on the counter's low bit, stores that pointer in a global variable, then writes zero to four consecutive longwords per iteration for two thousand iterations, clearing the selected buffer.
- `call_library_and_wait_vblank`: Calls a routine through a pointer offset from a global pointer, then polls a global variable until its bit 32 becomes set, waiting on a hardware status flag.
- `initialize_system_and_loop_forever`: Calls SuperState, opens the graphics library, allocates a port and opens the trackdisk device, relocates and processes object tables, initializes entity tables, updates active object slots, and then enters an infinite loop that repeatedly polls a global byte, tests its bits, and recurses.
- `recursive_loop_until_forever`: If a global variable is nonzero, recursively calls itself twice, then calls another routine and loops forever.
- `wait_for_potgor_bit`: Stores a word from a register into two global variables, calls a routine through a table indexed by that word, and then loops until bit 2 of a global variable is set, returning the value of another global variable.
- `poll_status_and_dispatch`: Loops reading a global status byte, calling another routine each pass, and branching on individual bits of the status; depending on the bit tests it either clears the status, calls further routines, or falls into an endless wait loop.
- `poll_global_and_recurse`: Reads a global byte variable and, while it exceeds one, repeatedly calls another routine and tests bits of the variable, recursing into itself and calling a second routine before looping forever.
- `close_device_and_remport`: Sets a global variable to five, then loops calling a routine and a library vector until a returned value matches a constant; afterwards it calls two library vectors through a base pointer, stores that base into two globals, and polls a global bit while waiting.
- `clear_2000_longwords`: Clears a block of memory by writing zero to 2000 consecutive longwords starting at a global pointer.
- `wait_and_decrement_volume_table`: Waits for a hardware status bit, then decrements each of 16 words in a table by one if it is above a threshold, calls a routine twice, and repeats with a decreasing counter.
- `decrement_nibbles_until_zero`: Waits for a status bit in a global variable, then walks a table of words, subtracting one from each nibble that is nonzero (with borrow-like handling of the high byte and low nibble), and calls another routine twice per pass while decrementing the global variable by eight until it reaches zero.
- `wait_and_decrement_nibbles`: Waits for a status bit in a global variable, then walks a table of words, subtracting one from each nibble that is nonzero (with borrow-like), and calls a library routine twice per iteration while decrementing a counter.
- `set_flag_and_call_helpers`: Sets bit 1 of a global variable, writes a constant to another global, then calls a helper that relocates and processes a table of pointers and a helper that waits on a status bit and decrements table nibbles.
- `initialize_audio_and_scan_table_3`: Stores pointers derived from an incoming base pointer into globals, clears a series of global variables and sets one to a small constant, then scans a word table to find the maximum value while counting down, and finally calls another routine and stores a constant pointer.
- `process_table_entries`: Iterates over a table of entries, skipping zero entries, and for each nonzero entry updates fields based on a running value and a constant, possibly resetting a field to a fixed address.
- `silence_audio_channels`: Writes zero to the volume registers of all four audio channels and enables DMA for those channels.
- `conditional_initialize_display`: If a global flag bit is set, calls another routine; then performs display initialization including setting up structures, calling library functions, and storing values.
- `update_audio_channels_2`: Updates audio channel registers (pointer, length, period, volume) for channels 0-3 based on flags, enables DMA channels, and computes scaled values.
- `wait_and_call_sequence_2`: Waits for a nonzero value at an offset from a global pointer, then computes an address from a table and calls several routines in sequence before returning.
- `wait_and_call_sequence`: Waits for a nonzero value at an offset from a global pointer, then computes an address from a table and calls several routines in sequence before clearing a global variable.
- `add_indexed_byte_to_global_10`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global, and stores the result back.
- `store_audio_channel_fields_2`: Computes a structure pointer from a data register and a global base, then stores fields from that structure and a data register into global variables.
- `store_audio_channel_fields`: Computes a structure pointer from a data register and a global base, then stores fields from that structure and a data register into global variables, with identical paths for several register values.
- `store_audio_channel1_registers`: Depending on a data register value, copies fields from a structure pointed to by an address register and a word from a data register into global variables, then clears a global.
- `store_audio_channel3_registers`: Copies fields from a structure pointed to by an address register and a word from a data register into global variables, with a separate path when a data register equals three, then clears a global variable.
- `store_audio_registers`: Stores values from a structure into global variables and writes to audio hardware registers for channel 3 (length, period, volume, and high bits of pointer).
- `dispatch_control_word_bits`: Examines bits of a data register and, depending on their value, stores a byte, negates it, sets or clears bits in a global variable, or computes an address from the register and stores it.
- `set_flag_from_register_value`: Compares a word from a data register against three constants and either stores another register word into a global variable, clears bit 1 of a global variable, or sets bit 1 of a global variable.
- `set_flag_from_register`: Checks a register value and sets or clears bit 1 of a global variable based on whether the value equals specific constants.
- `set_flag_if_register_matches`: If a register equals a specific value, sets bit 1 of a global variable.
- `compute_indexed_address`: Computes an address from a word argument and a base pointer, stores it in a global variable, and sets another global variable to a constant.
- `store_negated_word_to_global`: Negates a word from a data register and stores the result into a global variable, with identical paths for several register values.
- `store_word_to_global_3`: Stores a word from a data register into a global variable, with separate paths when another data register equals two or three.
- `store_word_conditionally`: Stores a word from a data register into a global variable, with a separate path when another data register equals three.
- `store_word_to_global`: Stores a word from a register into a global variable.
- `store_word_to_global_4`: Stores a word from a data register into a global variable, with identical paths for several register values.
- `store_word_to_other_global`: Stores a word from a data register into a different global variable, with separate paths when another data register equals two or three.
- `store_word_conditionally_2`: Stores a word from a data register into a different global variable, with a separate path when another data register equals three.
- `store_word_to_global_2`: Copies a word from a register into a global variable.
- `update_audio_channels`: Updates audio channel registers (pointer, length, period, volume) for channels 0-3 based on flags, enables DMA channels, and computes scaled values from global variables.
- `add_indexed_byte_to_global`: Reads a byte from a table indexed by a global variable, adds it to that global variable, and returns the result with some bits masked.
- `add_indexed_byte_to_global_2`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global's low word.
- `add_indexed_byte_to_global_3`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global variable, and stores the result back.
- `add_indexed_byte_to_global_4`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global variable, and stores the result back while returning the low byte of the sum.
- `add_byte_to_global`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global variable, and stores the result back.
- `compute_byte_from_global`: Reads a byte from a fixed memory location indexed by the low bits of a global variable, masks it, adds it to the global, and returns the result.
- `add_indexed_byte_to_global_5`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global variable, and returns the low byte of the result while updating the global.
- `tiny_ori_6`: Small routine of 1 instruction that merges 0 into d5.
- `add_indexed_byte_to_global_6`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global.
- `add_masked_byte_to_global`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global, and stores the result back while returning the low byte of the sum.
- `add_byte_to_global_word`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global variable, and returns the low byte of the sum.
- `add_indexed_byte_to_global_7`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global variable, and stores the result back.
- `add_indexed_byte_to_global_8`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global while returning the low byte of the sum.
- `add_masked_table_byte_to_global`: Reads a byte from a table indexed by the low bits of a global variable, masks off one bit of it, adds it to that global variable, and stores the result back.
- `add_indexed_byte_to_global_9`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global while returning the low byte of the sum.
- `copy_table_entries`: Copies words from a table into fixed memory locations, repeating the copy for several entries and setting a global variable to a constant.
- `copy_selected_entries`: Selects a table entry based on a word argument, then for each set bit in a byte argument copies four words from that entry into fixed memory locations and sets corresponding bits in a global variable.
- `select_entry_and_store_fields_2`: If a global flag is clear, uses a word argument times fourteen as an offset into a table to fetch an entry; if the entry's first longword is negative it follows the second longword as a pointer.
- `select_entry_and_store_fields`: Uses a word argument times fourteen as an offset into a table to fetch an entry; if the first longword is negative it follows the second longword as a pointer, then conditionally copies a longword from offset ten of that entry into four separate global locations according to bits of a byte argument, returning the first longword.
- `update_dma_control_state_machine`: Processes a bit-flagged control word, updating counters, pointers and a small state block, and writes DMA control bits (with the enable bit set) to the custom chip register.
- `unrecovered_data_control_flow`: The disassembly for this entry point consists of data that the decompiler cannot decode as valid instructions, so no behavior is recovered; it is reached only as an address taken by another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_and_set_bits`: Clears a specific bit within a byte using a bitmask and sets two specific bits in an adjacent byte.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `single_instruction_stub`: A one-instruction routine entered from outside static flow; no decompiled body is available, so only its single-instruction size is known.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_return`: Empty routine that immediately returns without doing anything.
- `return_immediately_5`: The routine is empty and returns immediately without touching any data.
- `return_immediately_6`: The routine is empty: it performs no operations and simply returns to its caller.
- `return_immediately_7`: An empty routine that performs no operations and simply returns to its caller.
- `return_immediately_8`: The routine is empty and returns immediately without touching any data or registers.
- `return_immediately_9`: This routine is empty: it performs no operations and simply returns to its caller.
- `empty_stub_routine`: The routine is empty and immediately returns to its caller.

### Disk.2 bytes 95232-98816 (code never loaded by a reference run)

- `initialize_audio_and_scan_table_2`: Sets up audio volume and DMA registers, clears many global variables, scans a table to find a maximum, then calls another routine and stores the result.
- `initialize_audio_and_scan_table`: Sets up audio volume and DMA registers, clears many global variables, scans a table to find a maximum, then calls another routine and stores a constant.
- `relocate_pointer_table_entries`: Iterates over a table of entries, skipping zero entries, and for each non-zero entry replaces a pointer with a base value, adjusts another field by the difference, optionally sets a fixed value, and advances the base by a scaled word field.
- `advance_counter_and_dispatch_audio`: Increments a global counter, and when it reaches a limit resets it and walks a table of pointers until a stored value exceeds the counter, then adjusts a pointer by an offset; afterwards adds a constant to a global and calls several other routines before clearing a global.
- `copy_audio_channel_entry`: Uses a nibble from the data register to index a table of sixteen-byte entries and copies fields from the selected entry into global variables, branching on a selector value but performing the same copy for every selector.
- `apply_control_code_to_global`: Examines a control field in a register and, depending on its value, stores a byte, negates a byte, stores a constant, adjusts a global by a computed offset, or sets and clears bits in a global variable.
- `update_audio_dma_channels`: Shifts a global value left four times, then for each of four channels whose bit is set clears that bit and loads pointer and length values from a table into global variables, finally enabling the corresponding DMA channel bits.
