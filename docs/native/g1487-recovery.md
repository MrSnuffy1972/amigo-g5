# Game 1487 (deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01487). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912736 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 90 of 90 named (0 author symbols), 90 of 90 described (11 by rule, 78 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 618 of 618 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 107648 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1727 of 1727 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3240 |
| code | 11510 |
| data | 748373 |
| padding | 109 |
| unused | 149504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf:bootblock | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 351 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1244 | 65 |
| logged disk load from Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf | 1 | 90 | 5 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 25088-27136 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 93184-93696 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 94208-94720 (code never loaded by a reference run) | 1 | 13 | 1 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 486912-523776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 564224-631808 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 12160-12288 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: custom-format (non-DOS) disk, 1466 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 248 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 272 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: 72 executed PCs whose RAM was overwritten before any snapshot match its bytes 11264-12160: recovered as a unit run at an address
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: bytes 25088-27136 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: bytes 93184-93696 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: bytes 94208-94720 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: bytes 486912-523776 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: bytes 564224-631808 look like code no run loaded: recovered position-independently (load address unknown)
- Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 12160-12288 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 128 bytes decode)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: 7 logged loads cover 108416 bytes of the image
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: 9 track-loaded blocks found in recovered runtime segments and classified through them
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 2-21 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.88)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 24-26 (bytes 12288-13824) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.55)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 30-45 (bytes 15360-23552) outside the filesystem hold data, not code (opcode density 0.042, entropy 4.57)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 47-47 (bytes 24064-24576) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.53)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 177-180 (bytes 90624-92672) outside the filesystem hold data, not code (opcode density 0.041, entropy 4.68)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 188-188 (bytes 96256-96768) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.92)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 192-192 (bytes 98304-98816) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.18)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 195-360 (bytes 99840-184832) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.87)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 362-599 (bytes 185344-307200) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.35)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 601-616 (bytes 307712-315904) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.00)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 619-620 (bytes 316928-317952) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.44)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 624-629 (bytes 319488-322560) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.36)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 631-667 (bytes 323072-342016) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.67)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 670-673 (bytes 343040-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.57)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 687-728 (bytes 351744-373248) outside the filesystem hold data, not code (opcode density 0.048, entropy 5.22)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 730-880 (bytes 373760-451072) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.54)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.41)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 931-931 (bytes 476672-477184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.50)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 934-937 (bytes 478208-480256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 1085-1100 (bytes 555520-563712) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.20)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 1237-1739 (bytes 633344-890880) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.66)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr black monks]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.54)
- 5 never-loaded region(s), 107136 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `issue_doio_with_command_nine` (runtime segment an address): Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine. Kind: trackdisk.device command check (CMD_WRITE/CMD_READ with specific parameters). Pass path: the branch following the indirect function call (the return from the device driver).
- `call_do_i_o` (runtime segment an address): Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller. Kind: track/sector verification or disk I/O trap. Pass path: the instruction following the indirect call (the return address on the stack).
- `issue_doio_and_copy_words` (runtime segment an address): Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer. Kind: disk/track integrity check and data copy. Pass path: the branch where the loop executes (the 'if' block containing the word-copying loop).

## Functions


### Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf:bootblock

- `issue_timed_io_request_sequence`: Boot block entry point that issues three Exec library requests, filling in request fields between them and running a fixed countdown loop of fifty thousand steps before the second request.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_global_value`: Calls a routine through a pointer in a6 with an offset of negative 552, and if the return value in d0 is non-zero, stores it in a global variable and increments a 16-bit value at a global address.
- `close_library_loop`: Decrements a value from a global variable and calls a routine through a pointer in a loop, using the decremented value as a counter.
- `initialize_and_copy_data_structures`: Initializes two sets of data structures by setting base addresses, calculating offset pointers, and performing a loop to copy 19 words from one memory location to another.
- `call_library_four_times`: Calls a routine through a pointer offset from a global variable four times, passing a constant value in d0 and clearing d1 each time.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d1.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `initialize_data_structures`: Sets d0 to 1, stores it in d4, calls a routine, and then sets a global word to zero.
- `clear_memory_block`: Fills a block of memory starting at a pointer from a global variable with zeros using a loop.
- `execute_self_and_return`: Calls another routine and then returns.
- `poll_status_and_dispatch`: Loops reading a global status byte, calling another routine each pass, and branching on individual bits of the status; depending on the bit, calls further routines.
- `setup_interrupt_server`: Sets bytes at offsets 8 and 9 of a structure at a fixed address, stores two longwords at offsets 14 and 18 of that structure, and calls a routine via a pointer from a global variable.
- `open_trackdisk_and_create_port`: Calls exec to open trackdisk.device, fills in a message port structure with a name and flags, adds the port, finds a task, and if the lookup fails it loops forever; otherwise it sets a field to all ones and clears a byte.
- `issue_doio_with_command_nine`: Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `process_io_requests`: Compares d0 to 5632, and if it is greater, subtracts 5632 from d0, calls another routine with d0 set to 5632, adds 5632 to d1 and d2, and then checks d7 to decide whether to return or continue with further logic involving another call to the same routine.
- `issue_doio_and_copy_words`: Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_entity_list_and_update_states`: Iterates through a list of pointers to calculate relative addresses, populates a secondary table with copied structure data, and then loops through that table to update various counters and state flags based on specific byte values.
- `process_bytecode_instruction_stream`: Iterates through a table of structures, performing operations based on an opcode: decrementing a value at offset 8, checking a counter against a global value and calling another routine, or manipulating a 32-bit value to update an offset at offset 4 and a value at offset 14.
- `parse_bytecode_stream`: Iterates through a stream of words in a1, using the first word of each entry as an opcode to perform various data manipulations on a structure in a0.
- `update_table_and_clear_memory`: Increments a global counter and, if bit 0 is set, updates a global pointer and fills a large memory area with zeros using a loop.
- `call_library_and_wait_vblank`: Calls a routine through a pointer offset from a global pointer, then polls a global variable until its bit 5 becomes set, waiting on a hardware status flag.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_registers`: Sets bits 0 of registers d0, d1, d3, d4, d5, and d6.
- `set_color_register`: Sets the value of a hardware color register to 511 and branches to another routine.
- `reset_hardware_and_sync_state`: Resets several custom chip registers including color and DMA control, updates pointers in a table, and enters a loop that monitors hardware bits and global flags to manage state transitions and call various subroutines.
- `manage_audio_and_logic`: Calls a routine, checks a global pointer, and if not zero, calls another routine; it then compares a global longword to itself and, if they differ, calls two more routines before incrementing a value from a global word and branching.
- `wait_for_potgor_bit`: Stores a word from a register into two global variables, calls a routine through a table indexed by that word, and then loops until bit 2 of a global variable is set, returning the value of another global variable.
- `tiny_ori_4`: Small routine of 1 instruction that merges 4 into d0.
- `update_global_status`: Sets bit 2 of d0, loads a word from a global variable into d0, and if d0 is not 2, branches; otherwise it sets a global word to 1 and branches to another routine.
- `manage_device_and_memory`: Calls a routine, and if the return value is not a specific constant, calls another routine, performs a memory operation via a library call, calls a routine with a pointer, and checks bit 6 of a hardware register.
- `clear_2000_longwords`: Clears a block of memory by writing zero to 2000 consecutive longwords starting at a global pointer.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d0.
- `set_ciaa_bit`: Sets bit 1 of a hardware register, loads a pointer from a global variable into a register, and calls another routine.
- `initialize_audio_and_scan_table`: Stores pointers derived from an incoming base pointer into globals, clears a series of global variables and sets one to a small constant, then scans a word table to find the maximum value while counting down, and finally calls another routine and stores a constant pointer.
- `mu_calcins`: Iterates over a table of entries, skipping zero entries, and for each nonzero entry updates fields based on a running value and a constant, possibly resetting a field to a fixed address.
- `silence_audio_channels`: Writes zero to the volume registers of all four audio channels and enables DMA for those channels.
- `check_audio_counter`: Checks bit 5 of a hardware register, and if set, calls another routine to increment a global word and compares it to a global variable before jumping to a fixed address.
- `increment_audio_counter`: Calls another routine, increments a global word, and compares it to a value in another global variable; if the value is greater than or equal, it loops back to the start of the routine.
- `update_voice_pointers`: Increments a global counter in a loop until it reaches 65, then updates a global pointer by calculating an offset from a table and a value stored in another global variable.
- `update_global_pointers`: Clears a longword in a global variable, moves a longword from one global variable to another, adds a constant to the second global variable, and branches to another routine.
- `play_four_voices`: Loads four voice entries from a table, advances the table pointer by 16, calls two routines for each entry to set parameters, and finally writes d3 to the DMA controller.
- `check_high_word`: Copies d0 to d5, masks d0 to check if bits 12 through 15 are set, and if they are not zero, it calls another routine; otherwise it returns.
- `configure_audio_channel`: Manipulates register d0 to calculate an offset, then if d2 is 1, it clears a hardware register, updates several global pointers and values, and sets hardware registers for audio playback using a pointer from a table.
- `configure_audio_channel_one`: If d2 equals 2, it clears a hardware register, stores a pointer from a6 into a global variable, sets hardware registers for sample pointer, length, and period using values from a6 and d1, and sets bit 1 of d3.
- `configure_audio_channel_three`: If d2 equals 3, it clears a hardware register, stores a pointer from a6 into a global variable, sets hardware registers for sample pointer, length, and period using values from a6 and d1, and sets bit 2 of d3.
- `mu_conti3`: Clears AUD3LCH, stores a6 as the channel 3 sample pointer, writes the sample pointer, length and period from a6 and d1 to the channel 3 hardware and shadow registers, clears the channel 3 volume shadow, sets bit 3 of d3 and returns.
- `dispatch_parameter_value`: Masks d5 and d6 to determine a category, then branches to different routines or stores d5 into various global variables based on the value of d6.
- `store_shadow_word`: If d6 does not equal 800, it returns; otherwise it stores a word from d5 into a global variable.
- `check_register_and_clear_bit`: Compares d6 to a constant and, if equal, clears bit 1 of a hardware register; otherwise it returns.
- `set_flag_if_register_matches`: If a register equals a specific value, sets bit 1 of a global variable.
- `calculate_offset_and_store`: Clears d4, copies d5 to d4, performs arithmetic on d4, stores the result in a global variable, calculates a new address using d4 and a global pointer, stores that address in another global variable, and branches to another routine.
- `negate_and_store_word`: Negates a word in d5, compares d2 to 1, and if they are not equal, it stores the negated word into a global variable.
- `store_conditional_word`: Compares register d2 to 2; if they are not equal, it returns, otherwise it stores a word from d5 into a global variable.
- `store_shadow_word_alt`: If d2 does not equal 3, it returns; otherwise it stores a word from d5 into a global variable.
- `store_d5_to_shadow`: Stores a word from d5 into a global variable and returns.
- `negate_and_store_word_alt`: Compares d2 to 1; if they are not equal, it returns, otherwise it stores a word from d5 into a global variable.
- `update_global_word_on_match`: Compares a 16-bit value in register d2 to 2; if the value is not 2, it branches to another routine, otherwise it stores a 16-bit word from register d5 into a global variable.
- `compare_and_update_word`: Compares a 16-bit value in a register to 3; if the values are not equal, it returns, otherwise it stores a 16-bit word from a register into a global variable.
- `update_global_word`: Stores a 16-bit word from a register into a global variable and returns.
- `mu_everyvert`: Increments frequency or slide-rate values for audio channels and resets channel-specific registers or pointers if certain flags are set.
- `do_nothing`: Performs an operation that clears the low byte of register d0.
- `do_nothing_2`: Performs an operation that clears the low byte of register d0.
- `do_nothing_3`: Performs five consecutive operations that clear the low byte of register d0.
- `clear_data_byte`: Performs a logical OR operation on register d0 with zero.
- `zero_d0_register`: Performs a logical OR operation on the d0 register with zero.
- `clear_d0_register`: Performs a logical OR on register d0 with zero.
- `clear_d0_register_2`: Performs a logical OR on register d0 with zero.
- `tiny_ori_6`: Small routine of 1 instruction that merges 0 into d5.
- `clears_data_byte`: Sets the lowest byte of register d0 to zero.
- `clear_data_byte_2`: Sets the least significant byte of data register zero to zero.
- `clears_data_byte_2`: Performs a logical OR operation on register d0 with zero.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clears_data_byte_3`: Performs a logical OR operation on a byte in register d0 with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `add_indexed_byte_to_global`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global while returning the low byte of the sum.
- `prepare_copy_selection`: Clears d0, stores it as a byte in a global variable, sets d1 to a constant, and branches to another routine.
- `copy_selected_entries`: Selects a table entry based on a word argument, then for each set bit in a byte argument copies four words from that entry into fixed memory locations and sets corresponding bits in a global variable.
- `test_and_branch_on_global_longword`: Tests a longword in a global variable and branches to another routine if the value is zero; otherwise it returns.
- `select_entry_and_store_fields`: Uses a word argument times fourteen as an offset into a table to fetch an entry; if the first longword is negative it follows the second longword as a pointer, then conditionally copies a longword from offset ten of that entry into four separate global locations according to bits of a byte argument, returning the first longword.
- `update_dma_control_state_machine`: Processes a bit-flagged control word to update counters, pointers, and a state block, then writes DMA control bits to the custom chip register.

### logged disk load from Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf

- `switch_privilege_and_open_trackdisk`: Switches the CPU between supervisor and user privilege states by calling exec.SuperState and exec.UserState.
- `open_trackdisk_device_2`: It opens the trackdisk.device by calling exec.AddPort, exec.FindTask and exec.OpenDevice, and it is entered from outside the static control flow by one unnamed routine.
- `perform_io_request_2`: It performs an I/O request by calling exec.DoIO, and it is entered from outside the static control flow by one unnamed routine.
- `manage_trackdisk_io`: It is a 46-instruction routine that opens the trackdisk.device and performs I/O requests by calling exec.CloseDevice, exec.DoIO and exec.RemPort.
- `run_trivial_sequence`: It is a four-instruction routine that is entered from outside the static control flow, but its exact operations are not recoverable from the available information.

### Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr Black Monks].adf bytes 94208-94720 (code never loaded by a reference run)

- `test_pot_button`: Sets a global variable to zero, calls another routine, and if the global variable is non-zero, calls that same routine again with a different value in d1.
