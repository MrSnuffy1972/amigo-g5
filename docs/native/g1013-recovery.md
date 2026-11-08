# Game 1013 (seconds out (1988)(tynesoft)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01013). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903552 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 249 of 249 named (0 author symbols), 249 of 249 described (24 by rule, 225 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 478 of 478 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1913 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3200 of 3259 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 530018 |
| code | 15770 |
| data | 68115 |
| padding | 7537 |
| unused | 282112 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Seconds Out (1988)(TyneSoft).adf:.prg | 1 | 3236 | 242 |
| Seconds Out (1988)(TyneSoft).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 6 |
| Seconds Out (1988)(TyneSoft).adf:.tos (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- seconds out (1988)(tynesoft)/Seconds Out (1988)(TyneSoft).adf:crowd: high-entropy data file (7.60 bits/byte), classified as packed data
- 11 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- seconds out (1988)(tynesoft)/Seconds Out (1988)(TyneSoft).adf:.tos: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 1913 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Seconds Out (1988)(TyneSoft).adf:.prg

- `save_stack_and_call_routines`: Saves the current stack pointer into a global variable, then calls two other routines in sequence before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_table_data`: Copies 3999 words from the source buffer in a0 into four separate destination tables, then copies 7999 long words from a global table back into the source buffer in a0.
- `iterate_callbacks_in_loop`: Clears a counter, then loops through a table of entries by index until it finds a negative marker, loading each entry's fields into registers and calling a routine for every valid entry before returning.
- `zero_padding_word`: Contains a single zero word that decodes as an instruction with no effect and is referenced by address by other routines.
- `init_and_run_game_loop`: Splits the value an address into two bytes and stores them at offsets 1002 and 1003 of a global structure, then loads a series of data-table addresses and copies each into a destination table.
- `reset_state_and_run_game_loop`: Clears a set of global flag words and loads two blitter source pointers, then repeatedly runs the main game loop while decrementing two pending-pointer counters and inserting a short delay whenever a counter is still positive.
- `run_game_loop_dispatch`: Runs a repeating game loop that calls several update routines, checks global flags and a byte at offset 34 of the data segment, and dispatches based on a counter that wraps past one.
- `save_and_check_flags`: Pushes a register onto the stack, tests a byte and jumps to another routine when zero, otherwise clears a word, calls two routines, and when a counter is at least four and a flag is set calls two more routines before jumping to a shared label.
- `double_and_write_palette`: Reads a word from a data table at a0, doubles it, and writes the doubled word to both the COLOR00 palette register and a second table, repeating this for ten iterations.
- `clear_palette_colors`: Clears ten consecutive 32-bit color registers in the color palette and then returns.
- `store_high_byte_and_clear`: Extracts the high byte of d1 into d2, clears a byte and a word in a table, stores d2 at one offset, then stores the high byte of d1 at another offset and clears a flag.
- `store_high_byte_when_below`: Compares a word in a table with another word and, if the first is not greater, extracts the high byte of d1 to store it at two table offsets and clear a flag.
- `test_byte_then_return`: Tests the byte at offset 45ac of the data segment and then returns without modifying any data.
- `load_pointer_and_call`: Builds an index from d0, loads a pointer from a table using it, adjusts the index, stores the loaded pointer into a global variable, then calls a routine before returning.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d7, then merges 0 into d7.
- `store_zero_byte`: Contains a single byte of zeroed data that another routine references by address.
- `load_pointer_and_call_other`: Builds an index from d0, loads a pointer from a table using it, adjusts the index, stores the loaded pointer into a global variable, then calls a different routine before returning.
- `increment_counters_and_call`: Increments a global counter and sets a flag word to a maximum value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `zero_padding_words`: Contains ten consecutive zero words that decode as instructions with no effect and are referenced by address by other routines.
- `setup_blitter_parameters`: Loads a table address into a0, derives a pointer by adding an offset to a global value into a1, sets d0 to a constant, calls a routine, then returns.
- `setup_blitter_copy`: Loads a source pointer, adds offset 2400 to it, stores the constant 70 in a blitter control word at offset 4 of the structure in a6, then hands the pointers and mode to another routine.
- `copy_longword_blocks`: Copies 1000 long words from a source table into two separate destination tables until the first destination reaches the given address, then returns.
- `copy_word_blocks`: Copies 140 words from a source table into two destination tables, including copies at offsets 8000, 16000 and 24000, then returns.
- `toggle_and_adjust_value`: Sets bit 2 of a byte pointed to by a0, stores a constant, masks a word, and when a flag is zero increments a counter and toggles the word, otherwise clears the counter and a flag, then adjusts a value and returns.
- `store_value_and_dispatch`: Stores the value 169 at offset 6580 of a global structure and clears a counter, then loads a pointer from a table indexed by that counter and jumps to it, returning early when the pointer is the sentinel value.
- `update_counter_and_store`: Sets a global word to five, adds a masked copy of another global into it, and when the result is four or more stores the original value back while clearing a counter at offset 12 of the structure in a4, otherwise it returns.
- `dispatch_from_counter`: Sets a global word to two, increments a counter, derives a shifted index from it, and jumps to a dispatch table; if the entry matches a sentinel it returns, otherwise it sets several fields and jumps to the target.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `wait_and_adjust_counter`: Waits until a global counter reaches zero, then calls a routine and adjusts a second global counter by a table value, clamping it and dispatching to a handler.
- `adjust_and_dispatch_counter`: Adjusts a global counter by a passed value, clamping it to a range and decrementing a slot counter, then calls a handler and dispatches to a routine from a table indexed by the counter.
- `dispatch_from_loaded_counter`: Reads two global words, adds one to another while clearing it, loads a counter, derives a shifted index, and jumps to a dispatch table that either returns or sets fields and jumps to the target.
- `init_table_state_a`: Copies a word from offset 56 of a table to a global variable, clears two other variables, stores the value 14 and the value 65535, then copies a word from offset 126 of the table.
- `init_table_state_b`: Copies a word from offset 46 of a table to a global variable, clears two other variables, stores the value 18 and the value 65535, then copies a word from offset 130 of the table.
- `init_table_state_c`: Copies a word from offset 56 of a table to a global variable, clears two other variables, stores the value 15 and the value 65535, then copies a word from offset 128 of the table.
- `init_counter_state`: Clears a global variable, stores the value 13 and the value 2 into two variables, then stores the value 105 into a fourth variable before returning.
- `store_state_constants`: Writes the decimal constants 1, 13, 2 and 105 into the 16-bit values at offsets 10, 8, 12 and 0 of the structure referenced by the zeroed data area, then returns.
- `set_state_value_one`: Loads the address of the zeroed data area into a register and stores the decimal constant 1 into its 16-bit value at offset 10, then returns.
- `set_state_value_two`: Loads the address of the zeroed data area into a register and stores the decimal constant 2 into its 16-bit value at offset 10, then returns.
- `set_state_value_three`: Loads the address of the zeroed data area into a register and stores the decimal constant 3 into its 16-bit value at offset 10, then returns.
- `reset_state_to_nine`: Clears the 16-bit values at offsets 10 and 12 of the zeroed data area, stores the decimal constant 9 into the value at offset 8, then returns.
- `reset_state_to_ten`: Clears the 16-bit value at offset 10 of the zeroed data area, stores the decimal constant 10 into the value at offset 8 and the constant 1 into the value at offset 12, then returns.
- `set_state_to_nine`: Stores the decimal constant 1 into the 16-bit value at offset 10 of the zeroed data area, the constant 9 into the value at offset 8, and clears the value at offset 12, then returns.
- `set_state_to_ten`: Stores the decimal constant 1 into the 16-bit value at offset 10 of the zeroed data area, the constant 10 into the value at offset 8, and the constant 1 into the value at offset 12, then returns.
- `dispatch_subtask_by_counter`: Decrements a 16-bit counter and, when it reaches zero, dispatches a sub-task by reading a pointer from a table using a shifted counter value, then adjusts a status byte based on flag bits.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 1 into d0, then merges 0 into d1.
- `index_table_by_counter`: Reads a counter from a global word, doubles it, and uses it as a shifted index into a table to load a word into another global word, then increments a second counter that wraps to zero when it passes one and advances the first counter, which wraps to zero when it reaches four.
- `advance_counter_and_dispatch`: Reads a word from a table using a shifted counter as index, increments the counter, and when it reaches six adjusts a value based on flag bits, sets a flag word, and jumps to a handler.
- `update_counter_and_dispatch`: Increments a global counter, and when it reaches six stores a masked copy of another global into a flag word, sets a byte at offset 4 of a structure in a6, and jumps to a handler.
- `advance_subtask_counter_and_dispatch`: Tests a global flag and, when set, decrements a 16-bit counter that resets to four, then advances a wrapping counter that indexes a table and calls a routine.
- `adjust_and_set_flags`: Reads a global word, subtracts three when bit 3 is clear and the value exceeds three, adds three when bit 2 is clear and the value is below 221, then sets or clears two flag words at offsets 22 and 113 of the structure in a4 based on bit 1.
- `dispatch_with_counter_check`: Clears a register, sets a global word to four, adds a counter to another while clearing it, and jumps to a dispatch table that branches on a sentinel before setting fields and jumping to the target.
- `count_down_register`: Stores the byte 243 at offset 0 of a hardware register, then repeatedly decrements a counter while storing it there until it reaches 44, after which it stores the byte 255 there and writes the value an address into the register at offset 8.
- `count_up_register`: Stores the byte 44 at offset 0 of a hardware register, writes the value an address into the register at offset 8, then repeatedly increments a counter while storing it there until it reaches 244, after which it stores the byte 255 there and writes the value an address into the register at offset 8.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_palette_from_buffer_2`: Waits for a frame counter to reach zero, then copies sixteen 16-bit palette entries from a data area into the color register block, doubling.
- `copy_color_table`: Tests a flag word and returns without action when set, otherwise loops sixteen times copying a doubled word to the color register at offset 624 of the custom area and the original word to a table, then returns.
- `update_palette_from_buffer`: Waits for a frame counter to reach zero, then copies sixteen 16-bit palette entries from a data area into the color register block, doubling each value, and finally jumps to a handler routine.
- `loop_five_copy_bytes`: Loops five times, each time reading a byte from a table via a zero-based index and passing the masked value to another routine.
- `copy_byte_rows`: Copies 8 bytes from a source table into two destination tables at four offsets, advances the source and destination pointers, and repeats this for 7 iterations before returning.
- `copy_blitter_blocks`: Copies a block from address an address to address an address and then from address an address to address an address by calling the blitter helper twice with the same mode value.
- `adjust_table_byte`: Adds a byte from a register to a table entry, wraps the result below 10 when it reaches 14, decrements a counter, and repeats the process five times.
- `subtract_string_bytes`: Subtracts 101 from every byte of two null-terminated strings and skips over a run of forty-six zero words referenced by address.
- `loop_string_subtract_65`: Walks a null-terminated string through a pointer, and for each non-zero byte subtracts 65 from it before handing the result to another routine.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d0.
- `copy_triple_blocks`: Copies four words from a source block into four destination blocks at offsets 0, 32768, 65536 and 98304, advancing all pointers, and repeats the copy nine times.
- `copy_triple_blocks_loop`: Copies four words from a source block into two destination blocks at offsets 0, 8000, 16000 and 24000, advancing all three pointers, and repeats this eight times before decrementing an outer counter.
- `write_zero_byte`: This routine is a single instruction that writes a zero byte to the memory location pointed to by register d0.
- `write_zero_byte_2`: This routine is a single instruction that writes a zero byte to the memory location pointed to by register a0, then returns.
- `copy_byte_from_offset`: Copies one byte from the location at offset 4696 in a2 to the byte at the global variable a fixed address, then returns.
- `copy_blocks_in_loops`: Computes a destination address from a table and offset, then copies words from a source block into a destination block in four aligned positions.
- `shift_byte_blocks`: Saves two source pointers, stores a mode word, then shifts bytes bit by bit from two source blocks into a destination block eight times.
- `shift_byte_into_word`: Shifts a byte from a source block left into a destination block bit by bit, carrying the overflow across a 16-bit word, and repeats this for several bytes and outer groups.
- `lookup_and_jump`: Looks up a pointer in a table using a shifted index and checks whether it matches a sentinel value.
- `is_zero_data_word`: Is a lone zero word that decodes as ori.b #0, d0 and does nothing, so this range is a zero-filled data area referenced by address rather than working code.
- `validate_and_dispatch_action`: Checks a flag word and a counter, and when the counter is out of range returns immediately.
- `update_and_configure_audio`: Increments a counter, clears several fields, subtracts a table word from another, stores a table word, calls two routines, and conditionally calls a third before configuring audio and returning.
- `increment_and_dispatch`: Compares a global counter against a threshold and returns early if it is too large.
- `enable_audio_channel`: Stores the constant 608 in a hardware register, enables an audio channel, then sets a flag at offset 9494 of the structure in a6.
- `is_data_table_words`: Is a run of twenty-nine zero words that decode as ori.b #0, d0 and do nothing, so this range is a data table referenced by address rather than working code.
- `is_data_table_words_2`: Is a run of forty-six zero words that decode as ori.b #0, d0 and do nothing, so this range is a data table referenced by address rather than working code.
- `copy_indexed_word_block`: Looks up a pointer in a table indexed by a doubled register value, then copies pairs of words from the pointed-to block into a destination block eight times.
- `clears_byte_via_pointer`: It writes a zero byte to the memory location pointed to by a0, clearing that byte, then returns.
- `test_and_clear_word`: Tests a word at offset 10304 and jumps away when it is negative, otherwise clears that word and returns.
- `clamp_and_clear_word_block`: Clamps a counter at 63, then clears a run of words and fills a partial word with shifted bits before clearing the remainder of the block.
- `copy_struct_triple`: Copies four longwords from a0 to a1 at offsets 0, 4, 8000, 16000 and 24000, swapping the two pointers, and repeats this for a total of three outer iterations, then returns.
- `clamp_and_fill_word_block`: Clamps a counter between 0 and 63, then fills a run of words with ones, sets a partial word with shifted bits, and clears the remaining words.
- `reset_and_load_state`: Stores several constants into global variables and clears a number of flag words, then copies two words from a table into other globals and returns.
- `store_zero_byte_2`: It writes a byte value of zero through a pointer in a6 to the memory location it points to, then returns.
- `noop_instruction`: The routine consists solely of the single instruction ori.b #0, d0, which leaves the contents of register d0 unchanged, so the routine does nothing.
- `store_byte_via_pointer`: Writes the low byte of the data register d0 to the byte pointed to by the address register a0, then returns.
- `manage_state_counter`: Tests a flag word and returns when set, otherwise adds a counter to another, stores a constant, clears several words, increments a counter, loads a word from a table indexed by a global value, and returns.
- `increment_and_dispatch_audio`: Increments a counter and masks it to a range, sets a flag when it wraps, tests a bit to call a routine, then builds an index, loads two table words, and jumps to a dispatch target.
- `write_zero_byte_3`: This routine is a single instruction that writes a byte value of zero to the memory location addressed by the register pair, and it does not return to its caller.
- `decrement_counters`: Decrements three counters, resetting the first two to 9 and 5 when they underflow, then clears a flag word and a byte before jumping to a shared exit.
- `loop_four_copy_bytes`: Loops four times, each time reading a byte from a table via a zero-based index and passing the masked value to another routine.
- `loop_and_configure_audio`: Configures audio, sets a counter to one, clears two fields, then loops storing table words, calling a routine, toggling a flag, and incrementing the counter until it reaches a threshold before a final delay loop.
- `clamp_pointer_fill_words`: Clamps a pointer value at 70000, then computes block offsets from three counters and calls another routine for each combination before filling a set of words with ones.
- `and_blocks_iteratively`: Bitwise-ANDs eight 16-bit words from a source block pointed to by a0 into the corresponding words of a destination block pointed to by a1, then advances both pointers by 40 and repeats for the 16 iterations counted in d1.
- `clear_counters_store_values`: Clears a series of 16-bit counters in global variables and stores the values 110 and 102 into two of them before returning.
- `clear_and_set_globals`: Clears the 16-bit word at offset 6462 of a global variable and sets the 16-bit word at offset 6464 to one, then returns.
- `loop_until_counter_reaches`: Clears a flag word and returns immediately if it is already clear.
- `copy_blocks_iteratively_2`: Copies eight 32-bit words and eight 16-bit words from the source table at offset an address to the destination table at offset an address, advancing both pointers by 10 each time, then returns.
- `load_address_into_a0`: Loads the address of a data area at offset an address into a0 and returns.
- `copy_blocks_iteratively`: Copies eight blocks from a source pointed to by a0 into a destination table at offset an address, moving 32-bit and 16-bit words at matching offsets, then advances both pointers by 40 and repeats for the 28 iterations counted in d0.
- `empty_data_area`: This routine is empty, consisting only of a zero-filled data word that other code refers to by address.
- `zeroed_data_area`: It is a zero-filled data area that other routines reference by address, containing no instructions of its own.
- `write_zero_byte_4`: It writes a zero byte to the byte addressed by the register in a0, then returns.
- `index_table_slot`: Stores 100 in a global word, sets two other global words to 3 and 16, increments a counter, and loads a pointer from a table indexed by that counter into a global address slot, returning early when the counter reaches 4.
- `process_table_dispatch`: Decrements a 16-bit counter in a global variable and returns early unless it is zero and several other global flags are clear, then it compares a global value against a table of thresholds and, if it falls inside a range, toggles a flag byte and clears another variable, otherwise it selects a case from a dispatch table based on a counter and a state value.
- `store_zero_byte_3`: Writes a zero byte to the memory location pointed to by a0.
- `clear_global_word`: Clears the 16-bit word at offset 12504 of a global variable, then returns.
- `dispatch_indexed_handler`: Reads a 16-bit index from a global variable and branches to one of several handlers based on its value, loading a mode constant and index into registers.
- `test_indexed_word`: Loads the address of a data table into a register, tests a word at an offset from that address using a register index, and branches away when the tested word is zero, otherwise storing two 16-bit constants into two registers.
- `compare_indexed_sum`: Adds a table word indexed by d0 to a global value and compares the two running sums against a second global value, returning early when the sums fall outside the range.
- `update_and_process_pointers`: Increments a counter, clears several fields, subtracts a table word, calls a routine, conditionally calls others, then reads a pointer list and advances an index when a sentinel is reached.
- `decrement_and_jump`: Tests a global counter and returns early if it is zero.
- `enable_audio_flag`: Enables an audio channel only when a flag word at offset 11736 is clear, then sets a flag at offset 11752 and returns.
- `clears_register_d0_low_byte`: This routine is a single instruction that clears the low byte of the data register d0, then returns.
- `empty_routine`: This routine is empty, consisting of a single instruction that performs no data movement or control flow.
- `store_immediates_pointers`: It stores a 32-bit immediate value into the stack pointer, then writes immediate values into the locations pointed to by a5, a0, a1, a7 and a4 while advancing or decrementing those pointers, and finally returns control.
- `dispatch_and_wrap_counter`: Tests a flag and returns early when set, otherwise loads two table words, calls a dispatch routine, increments a counter, and wraps it before configuring audio and returning.
- `clear_when_zero`: Tests a global word and returns without clearing it when nonzero, otherwise clears three global words and sets the tested word to 65535, then returns.
- `noop_or_d0`: It is a single instruction that performs a no-op byte OR on register d0 with an immediate value of zero, leaving the register unchanged.
- `advance_and_check_state_flags`: Tests a global flag and, when set, advances a wrapping counter, clears several state words, and looks up a value in a table indexed by another counter, then returns.
- `noop_d0_read`: It contains a single instruction that performs a no-op read on the 16-bit register d0, leaving the value unchanged.
- `noop_byte_write`: Performs a no-op byte write of zero to the location pointed to by d0, leaving the data unchanged.
- `clears_byte_at_pointer`: It writes a zero byte to the memory location pointed to by a0, clearing that byte.
- `advance_counters_then_jump`: Increments a global counter that wraps to zero when it passes one, then decrements another global value and increments a third counter that wraps when it reaches three, storing constants and clearing several locations before jumping to another routine.
- `process_flag_bit`: Stores the constant 5 into a global word and decrements a global counter when it exceeds 3.
- `store_16bit_immediates`: It stores 16-bit immediate values into the locations pointed to by a7, a6, a4, a2 and d4, advancing the first three pointers as it goes, and then returns.
- `empty_routine_2`: It is an empty routine that does nothing.
- `increment_counters_and_set_flags`: Increments a global word at offset 6 and stores the maximum 16-bit value at offset 8, then calls another routine that also increments a global counter.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `copy_blocks_pointers`: It copies words from the block pointed to by a5 to the block pointed to by a6, writing fixed offsets of 8000, 16000 and 24000 plus a running sequence, repeating four copies per outer pass.
- `copy_blocks_loop`: Initializes a counter to one, then loops sixty-three times copying words from a source table to a destination table at offsets 8000, 16000 and 24000 plus a running offset, calling a routine before each pass, and finally stores 16384 into a data area and returns.
- `copy_blocks_loop_offset`: Stores an address into a data register, then loops sixty-three times copying words from a source table to a destination table at offsets 8000, 16000 and 24000 plus a running offset, calling a routine before each pass, and finally stores 16384 into a data area and returns.
- `decrement_counter_loop`: It decrements a counter and branches back to the start of the routine while the counter is not zero, otherwise it returns.
- `load_area_call`: Loads the address of a data area into a6, calls a routine, then loads the address of a second data area into a6, then falls through into the next routine.
- `clear_four_word_blocks`: Clears four 16-bit words at offsets 0, 4000, 8000 and 12000 of the structure in a6, repeating the four clears five times, then advances a6 by 40 and returns.
- `call_with_registers`: Stores a constant into a hardware register at offset 740, calls a routine, stores another constant into the same register, calls the routine again, then returns.
- `dispatch_from_loaded_pointers`: Stores the value 2 at offset 184 of the structure in a6, checks a sentinel word at offset 446, then loads two pointers from a data area and calls them in sequence.
- `dispatch_from_loaded_pointers_b`: Stores the value 2 at offset 184 of the structure in a6, checks a sentinel word at offset 450, then loads two pointers from a data area and calls them in sequence.
- `contains_zero_word`: Contains a single zero word that decodes as ori.b #0, d0 and is otherwise empty, referenced by other code by address.
- `copy_byte_through_pointers`: Reads a byte from the memory location pointed to by a0 and stores it through the pointer in a1, then returns.
- `noop_d0`: Executes a single ori.b #0, d0 instruction, which is a no-op that leaves the 32-bit register d0 unchanged.
- `store_byte_via_pointer_2`: Reads a byte from the memory location pointed to by a0 and stores it through the pointer in a1.
- `zero_memory_pointer`: Writes a zero byte to the memory location pointed to by a0, using the pointer stored in a global variable.
- `build_code_and_copy_blocks`: Builds a character code from a table entry and stores it into three global bytes.
- `dispatch_from_loaded_pointers_c`: Calls another routine, stores the value 3 at offset 1000 and the value 5 at offset 184 of the structure in a6, checks a sentinel word at offset 454, then loads two pointers from a data area and calls them in sequence.
- `clear_four_areas`: Stores a fixed address in a global word, then clears one word at each of four data areas eight times per outer pass, advancing each pointer by 13 words, repeating for 21 outer passes, then returns.
- `call_and_clear_globals`: Calls a routine, initializes a data register to zero, calls another routine, clears a global word, clears another global word, and falls through into the next routine.
- `dispatch_from_value`: Loads a value into a register, calls a routine, clears a word, tests a byte, and when nonzero calls another routine, otherwise returns.
- `load_pointer_and_dispatch`: Loads a pointer from a table indexed by a global counter, and when the pointer is the sentinel value resets the counter and jumps back to the start, otherwise it stores the value 6 at offset 184 of the structure in a6, calls another routine, increments the counter, and toggles bit 0 of a status byte.
- `dispatch_from_table`: Builds a table index from d0, loads a pointer from a table into a global slot and a byte from a second entry into d1, stores that byte in two global byte locations, clears a global word, and returns unless that word is nonzero and greater than 1.
- `clear_longwords_loop`: Loads a pointer from a global slot into a0, then clears eight consecutive longwords at that address, repeating for 1000 iterations, then returns.
- `process_and_mask_words`: Processes two 32-bit words per iteration by combining and inverting them into masks, applies the masks to a destination table at offsets 0 and 8, decrements a counter, and when it reaches zero resets counters, advances pointers, and repeats until an outer counter is exhausted.
- `clear_and_jmp`: Clears a global word, then jumps to a routine that processes data using the incoming register and pointer values.
- `copy_four_byte_blocks`: Indexes a table at offset 22796 using the low byte of d0, then copies four bytes at offsets 0, 8000, 16000 and 24000 to matching offsets in another block, repeating the four copies six times and advancing both pointers by 40 each time.
- `setup_a6_a4_a5_pointers`: Loads the address at offset 6918a into a6 and copies it to a4, then loads the address at offset 3946 into a5, with no return and no call.
- `index_and_call_routine`: Reads a byte from a source table advancing the pointer, returns if it is 254, returns to a routine if it is 255, subtracts 65 from it, calls a routine, increments a register, and jumps back to reload and call the same routine again.
- `jump_via_a6`: Loads a6 from d6 and jumps to a routine, then advances a4 by 280 words, reloads a6, and jumps to the same routine again.
- `setup_a6_a4_a5_pointers_2`: Loads the address at offset 691a4 into a6 and copies it to a4, then loads the address at offset 39a2 into a5, with no return and no call.
- `decode_buffer_byte`: Reads a byte from a buffer and compares it against several marker values.
- `call_routine_loop`: Calls a routine, increments a register, and jumps back to a routine that reads bytes from a buffer, returning when it finds the byte 254 and repeating while it finds the byte 255.
- `jump_via_a6_offset`: Loads a6 from d6 and jumps to a routine, then advances a4 by 320 words, reloads a6, and jumps to the same routine again.
- `compare_five_byte_blocks`: Compares five bytes from a table at offset 1640 with five bytes from the block in a1, clearing a 16-bit word at a global variable when the table byte is smaller or when all five bytes are equal, and setting that word to -1 when a table byte is larger.
- `tiny_move`: Small routine of 1 instruction that copies 7 to d7.
- `copy_block_conditional`: Computes an index from the byte in d7 by multiplying it by eleven, adding five, and masking to sixteen bits, then calls a routine that compares five bytes from a table at offset 1640 with five bytes from the block in a1.
- `update_state_range`: Returns immediately unless a flag is clear and a global state word is between 2 and 5.
- `noop_data_area`: It is a single instruction that performs a no-op on a data register, so this range is a zero-filled data area referenced by address rather than executable code.
- `read_indexed_table_word`: Tests a byte at offset 2 and branches away when it is nonzero, otherwise loads a word from a table indexed by a global counter, increments that counter by two, and reads a value from the same table.
- `dispatch_by_counter_value`: Checks a global counter, and when it equals 5 increments it and returns, and when it equals 6 clears a pointer and returns, otherwise it increments the counter, stores the value 2 at offset 184 of the structure in a6, checks a sentinel word, and calls two pointers from a data area.
- `clear_memory_and_poll`: Clears a 32000-byte block of global memory, stores a base address and a pointer value into global words, calls another routine, then polls a byte in a loop until it reaches a marker or a status bit is set.
- `clear_and_loop_tables`: Calls a routine, then loops eight times clearing words at offsets 8000, 16000 and 24000 of two tables and advancing both pointers, then stores a constant, clears several words, and returns.
- `clear_and_copy_structures`: Clears sixteen long words at fixed offsets within a structure, then copies four words from a source table into four destination locations using two incrementing pointers inside a doubly nested loop.
- `store_zero_byte_4`: Writes a zero byte to the memory location pointed to by a0, then returns.
- `compute_index_and_dispatch`: Stores the value 2 at offset 184 of the structure in a6 and calls another routine, then computes an index from a global counter, stores it, and when the indexed word is negative resets the counter before calling two more routines.
- `setup_blitter_pointers`: Loads a pointer into a register, loads a pointer from a global variable into another register and advances it by 3008, sets a register to thirty, calls a routine, and returns.
- `load_constant_d6`: It loads the decimal constant 3200 into a data register and then returns immediately.
- `copy_string_loop`: It copies bytes from the source pointer to the destination pointer, decrementing a counter in a register until it reaches zero or a zero byte is found.
- `store_byte_via_pointer_3`: Writes the low byte of the 16-bit value in d0 through the pointer in a0, then returns.
- `noop_data_area_2`: It is a run of zero words that decode as no-ops on a data register, so this range is a zero-filled data area referenced by address rather than executable code.
- `store_indexed_from_sequence`: Walks a sequence of word triples until it finds one whose first word is negative two, and for each entry whose masked value equals two hundred and thirty-two it stores the second word into a table indexed by the low nibble of the first word.
- `no_op_instruction`: Does nothing, containing only a single instruction that has no effect on the processor state.
- `noop_byte_or_chain`: Performs seven consecutive no-op byte OR operations on register d0, each of which leaves the register unchanged, and then falls through into the following routine.
- `open_library_twice`: It opens a library and stores the returned handle in a global variable, then opens the same library again and stores that handle in another global variable, then returns.
- `run_setup_sequence`: Calls a routine and, when its flag is clear, calls several other routines in sequence, stores the returned value into a global word, and then returns.
- `save_and_set_interrupts`: Saves three interrupt vectors into global variables and installs new ones, clears the DMA enable register, sets the copper control and interrupt mask registers, copies nine words of saved registers into a stack, and points the DMA channel 0 base address register at it.
- `does_nothing`: It does nothing and returns immediately.
- `clears_byte_offset6`: Writes zero to the byte at offset 6 of the structure in a6, then returns.
- `clears_byte_a6_offset6`: Clears the byte at offset 6 of the structure in a6 and returns.
- `reads_clears_global_byte`: Reads a byte from a global variable at offset 6, clears that variable, and returns the original byte value.
- `read_keyboard_store_byte`: Saves all registers, reads a byte from the keyboard serial register, calls a routine, stores the byte 76 into the keyboard serial register, restores all registers, pushes a pointer from a global variable, and returns.
- `manages_dma_and_irq`: Sets bit 7 of the DMA enable register, decrements a global counter, and when that counter is non-negative and a flag at offset 12 is zero it sets the blitter request register and the IRQ enable register, then returns.
- `handle_interrupt_flags`: Saves registers, reads two interrupt mask registers at offsets 28 and 30 of the custom area, and when bit 5 is set increments a counter and stores a base address plus offsets 8000, 16000 and 24000 into three consecutive custom registers, then restores registers and returns.
- `tiny_ori_4`: Small routine of 1 instruction that merges 7 into d0.
- `tiny_or`: Small routine of 1 instruction that merges d0 into d0.
- `loops_when_value_zero`: Clears a 16-bit value at offset 6, tests it, and if it is zero it returns otherwise it loops forever.
- `configures_display_registers`: Writes a sprite pointer register, three display positioning registers, and a DMA control register, then stores a value in a global variable and returns.
- `store_doubled_value_registers`: Masks the incoming value in d1 to four bits, doubles it, and stores the value from d0 into a table at that offset, then shifts d0 left by one, adds two hundred and twenty‑eight to the offset, and stores the shifted value into the custom register area before restoring all registers and returning.
- `waits_for_flag_clear`: Polls bit 6 of a byte at offset 2 in the structure in a6 and spins in a loop while that bit stays set, then returns.
- `reads_joystick_buttons`: Reads the joystick port, inspects several button bits to build a 4-bit value, and stores that value in a global variable at offset 10, then returns.
- `transform_and_store_byte`: Rotates the byte in d0 right by one, inverts it, and masks it to eight bits, then stores it at offset seven of a data area.
- `clears_table_byte_indexed`: Clears a global variable at offset 6, masks an incoming index, and clears the byte at that offset in a table pointed to by a5, then returns.
- `clears_three_bytes`: Clears three consecutive bytes in a data area and returns.
- `execute_noops`: It performs twelve consecutive no-op byte operations on a register, each writing a zero value.
- `save_registers_and_call_routine`: Stores the values from registers d0, d1, a0 and a6 into four consecutive global variables at offsets 4656, 46A8, 46AC and 46B0, then calls another routine and, if that routine returns nonzero, calls it two more times.
- `save_context_and_call_routines`: Saves the a6 base, a0, d1, and d0 into global storage, then calls a routine and returns zero if it reports failure.
- `store_zero_byte_5`: Writes a zero byte through the pointer in a0.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `store_zero_byte_6`: Writes a zero byte to the memory location pointed to by register d0.
- `store_zero_byte_7`: Writes a zero byte to the memory location pointed to by d0 and then returns.
- `write_zero_byte_5`: It writes a zero byte to the memory location pointed to by d0 and then returns.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_custom_registers`: Clears four consecutive custom-chip control registers starting at offset 168 and stepping by 16, repeating the store three times, then writes a value to offset 158 and a byte to offset 150 before returning.
- `store_pointers_call_routine`: Loads a pointer from a global variable, reads a pointer from a table, stores both pointers into a structure, calls a routine, then repeats this sequence four more times using successive table pointers and an accumulating offset, and finally returns.
- `load_table_pointer_and_call`: Loads the value 4 into a data register, increments it to 5 if a byte in a global variable is greater than 4, then reads a 32-bit pointer from a table indexed by that value and calls the routine at that address.
- `compute_index_call_pointer`: Computes an index from registers, stores a value into a global variable, sets several global words, loads a pointer from a table indexed by the computed value, calls a routine that waits for a flag to clear, calls another routine, invokes the loaded pointer, and returns.
- `configure_blitter_loop`: Saves the a6 base, clears several blitter control words, derives a mode value from d0, and stores the a0 and a1 pointers with that mode into blitter registers inside a loop that runs four times before restoring the base and returning.
- `store_blitter_pointers`: Saves the a6 base, advances both pointers by eight thousand bytes, and stores them along with a fixed value and the value from d3 into blitter registers before restoring the base and returning, so it contains one call and no loop.
- `store_struct_from_registers`: Stores values from registers a0, a1 and d1 into a structure in a6 at offsets 66 through 100, writes the constant an address at offset 80, clears the words at offsets 66 and 102, then returns the contents of a global variable shifted left by 12.
- `init_struct_and_clear`: Writes the value 65535 into the words at offsets 68 and 70 of a structure in a6, stores pointers from registers a0 and a2 at offsets 80 and 84, copies register d1 into the words at offsets 100 and 102, clears the word at offset 66, writes 2544 into the word at offset 64, and returns.
- `save_registers_and_store_values`: Saves registers on the stack and calls three routines, then stores a shifted value into a structure at offset 66 and a modified value at offset 64, and calls another routine three more times.
- `store_incremented_pointers`: Loads a base address into a0 and a1, adds 8000 to both, stores a1 into a structure at offset 76, stores the constant an address at offset 80, and stores the incremented a0 into offsets 84 and 72 before returning.
- `store_pointers_in_loop`: Clears a global counter and loads a base address, then stores a word from a register into a structure at offset 88.
- `clears_global_word`: Clears the 16-bit value stored at a global variable and returns.
- `store_registers_and_loop`: Stores the 32-bit values in a4 and a5 into the structure in a6 at offsets 76 and 80, then decrements d7 and loops back to the store while d7 is nonzero, and otherwise stores d5 into d7 and returns.
- `save_registers_and_call`: Pushes all registers on the stack, calls a routine, then restores them.
- `init_timer_fields_and_call`: Initializes a block of timer fields in the structure in a6 by writing base values and incremented offsets into 32-bit counters, sets several word fields, combines d2 and d3 into a value stored at offset 88, calls a routine, then updates the counters and clears fields before returning.
- `compute_table_index_pointer`: Computes an index from d1 and d0 by combining shifts and masks, loads it into a0, then adjusts a0 by a value from d0 and adds the address of a table, leaving the computed pointer in a0 before returning.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d7.
- `manage_audio_counter`: Saves registers, references the custom registers via a6, checks a global flag and a counter, clamps or resets that counter based on its sign and value, writes a mode word and the counter into custom registers, clears the flag, then restores registers and returns.
- `clears_memory_via_pointer`: It writes a byte of value zero through a pointer in a register, clearing the memory location it points to.
- `configure_audio_channel`: Validates d0 against a range, then when enabled sets a flag, scales d0 into an index, loads pointers from two tables, derives values from them, programs several custom audio registers, and clears the flag before returning.
- `tiny_subi`: Small routine of 1 instruction that subtracts 0 from d0.
- `clear_and_set_bit_6`: Clears bit 6 of the 8-bit value in d0 and sets bit 6 of the byte pointed to by a0, then decrements d0 again before returning.

### Seconds Out (1988)(TyneSoft).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `save_and_reset_byte`: Saves the low byte of d0, clears d0 and a global variable if the byte at offset 17 of the data segment is nonzero, then returns the saved byte.
- `push_pointer_and_return`: Pushes a 32-bit table pointer onto the stack and returns immediately.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
