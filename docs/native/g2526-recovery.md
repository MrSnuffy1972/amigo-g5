# Game 2526 (bloodwych (1989)(imageworks)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02526). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905194 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 58 of 58 named (0 author symbols), 58 of 58 described (6 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 539 of 541 (99.63%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 868 of 871 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 392623 |
| code | 3114 |
| data | 58049 |
| padding | 1872 |
| unused | 449536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| TEST | 1 | 166 | 9 |
| BLOODWYCH | 1 | 18 | 2 |
| Bloodwych (1989)(ImageWorks)[b].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 160 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 146 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 120 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 207 | 14 |

## Cases handled

- runtime segment an address (play-rs-m1500-unpk00): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 160 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 75 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 94 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 180 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 598 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_dma_registers` (first-run image an address): Writes several values to the custom hardware registers at offset a fixed address and calls another routine. Kind: disk hardware/sync check. Pass path: sub_01F0 (beq.b an address).
- `modify_cia_register` (first-run image an address): Clears a bit in a byte at a specific address using a value from a global variable, then performs a conditional check on a hardware register to set register d0 to zero. Kind: disk hardware/timing check. Pass path: bra.b an address.
- `update_hardware_byte` (first-run image an address): Modifies a byte at a specific address by applying a bitmask and clearing a bit based on a value from a global variable, then performs a loop. Kind: hardware-based bitmask/status check. Pass path: the branch where the bitmask is updated/applied without triggering a failure state.

## Functions


### TEST

- `initialise_state_with_delay`: Program entry that calls several routines to set up state, runs a nested countdown loop over a short counter as a delay, then writes a constant into a global variable and returns.
- `decode_packed_rows_into_buffer`: Doubles sixteen short values in a global array, then decodes a packed byte stream into 200 rows with a 40-byte row stride, copying literal byte runs whose lengths come from the stream and skipping a special marker value.
- `loop_mask_color_words`: Repeats sixteen times: calls another routine sixty-four times, masks each of sixteen words from a global array down to twelve bits, and calls two more routines.
- `copy_to_color_registers`: Copies sixteen words from a global array into the colour registers starting at COLOR00, then writes zero to the following sixteen register words.
- `busy_wait_countdown_loop`: Runs a countdown loop over a 16-bit value until it wraps, a short busy-wait with no other work.
- `increment_nibble_byte`: Reads a byte from the global array at an index derived from the D6 register, selects a nibble by the parity of that register, and adds one or sixteen to the stored byte when the nibble comparison differs.
- `loop_mask_color_words_2`: The routine loops sixteen times, each time calling a routine sixty-four times, then masks sixteen words from a global variable to 4095 and calls two other routines.
- `decrement_nibble_byte`: Reads a byte from the global array at an index derived from the D6 register, selects the high or low nibble by its parity, and subtracts one or sixteen from the stored byte when that nibble is nonzero.
- `configure_custom_chipset`: Writes a sequence of constants into the global array, builds bitplane pointers spaced 8000 bytes apart, and copies table entries into a list of colour words.

### BLOODWYCH

- `copy_handler_to_low_memory`: Program entry that copies its own routine into low memory at a fixed address, writes the disable-all mask to the interrupt enable register, and then issues trap zero.
- `copy_block_to_low_memory`: Copies a large byte block from the source pointer into low memory at a fixed address, then writes the disable-all mask to a global and a bit to a computed address before ending in bad instruction data.

### Bloodwych (1989)(ImageWorks)[b].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_functions_twice`: Calls a library routine twice with specific values in d1 and then enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_nested_loops`: Calls two different routines and then executes nested loops that manipulate a global variable.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, lea.l) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_state_byte_after_calls`: Calls routines, then stores the value 2 into a byte at offset 78 of the structure addressed by A5 and returns.
- `conditional_call_and_function`: The routine checks a byte value from a register and if it is greater than or equal to zero it calls another routine and then calls a function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Calls another routine, executes two nested loops that decrement counters, calls another routine, saves registers to the stack, writes a value to a hardware register, and returns.
- `expand_and_copy_data`: Copies sixteen words from one table to another while doubling their values, then performs a complex nested loop to expand and copy bytes from a source table to a destination table based on sign-extended values.
- `process_color_tables`: Runs three nested loops that call another routine, mask values in a table at a0, and call two other routines.
- `copy_and_clear_colors`: Copies sixteen words from a table to the color registers and then clears the following sixteen words in the color registers.
- `fill_loop_d1`: Performs a loop two hundred and fifty-six times using register d1.
- `update_table_value`: Fetches two bytes from tables at offsets determined by a shifted value in d6, compares their lower nibbles, and if they differ, adds either 1 or 16 to a byte in a table at offset 0 of a0.
- `initialize_palette_and_color_data`: Executes a nested loop structure where an inner loop runs sixty-four times calling a routine, followed by a loop that masks the lower twelve bits of sixteen words from a table, and finally calls two routines while an outer loop runs sixteen times.
- `decrement_table_value`: Fetches a byte from a table at offset 0 of a0 using an index derived from d6, and if the lower nibble of that byte is non-zero, subtracts either 1 or 16 from the byte at that table location.
- `setup_display_and_colors`: Calls another routine, writes several values to hardware registers, fills a table at a0 with values from a second table using a loop, and finally writes a value to a hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_record_from_table`: Updates a state record addressed by A5, decrementing a countdown byte and resetting fields when it reaches zero, then loads a word from a lookup table indexed by a computed value and stores it in the record.
- `toggle_and_decrement_counters`: Toggles a parity value and, on one phase, decrements two global counters and a three-entry table of words, clamping them at zero, before calling routines.
- `swap_plane_pointer_pair`: Toggles a stored pointer between two buffer addresses 32000 bytes apart, then writes four pointer words into a table at a fixed address, spaced 8000 bytes apart.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d6.
- `merge_masked_bitplane_words`: Combines four bitplane words spaced 4000 bytes apart with source words from a second buffer, adding source bits only where all four planes are zero.
- `copy_8000_longwords`: Copies 8000 longwords from the second buffer to the first with a counted loop, and was reached only by decoding gap bytes as code.
- `clear_8000_longwords`: Clears 8000 longwords at the buffer addressed by a global pointer, writing zero to each in a counted loop.
- `clear_8000_longwords_2`: Clears 8000 longwords at the other buffer pointer using a counted loop of zero writes.
- `copy_words_to_color_registers`: Copies thirty-two words from a global array into the colour registers starting at COLOR00 in one counted loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scan_byte_stream_calling_handlers`: Walks a byte stream through the A6 address register, calling one unnamed routine for each nonnegative byte and another routine for negative bytes until the loop ends.
- `dispatch_command_byte`: Dispatches on a command byte in the D0 register: one command clears 8000 longwords at a fixed low address when a routine returns nonzero, two commands store the next stream byte into a global, and another calls an unnamed routine.
- `save_registers_to_memory`: The routine stores the current values of several data and address registers into a series of consecutive global memory locations.
- `store_cacr_and_stack`: The routine stores the value of a hardware register into a global variable and then populates a series of memory locations with values from a stack or other registers.
- `xor_word_with_swapped_complement_2`: When the flag byte has its top bit clear, the routine saves the word at the address and XORs it with the bitwise complement of the preceding word with i.
- `xor_word_with_swapped_complement`: When the flag byte has its top bit clear, saves the word at the address and XORs it with the bitwise complement of the preceding word with its halves swapped; otherwise it restores the saved word.
- `apply_word_scramble_from_saved`: Restores the previously saved word at the stored address, then saves the word at the given address and XORs it with the complement of the preceding word with its halves swapped.
- `wait_and_copy_data`: The routine waits for a bit in a hardware register to be set, performs a countdown loop, and then copies data from one memory location to another based on a value in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_jump_table`: Runs a loop eight times that calculates an address by adding a constant to a value loaded from a table, stores the result in a table at offset eight, and calls two unnamed routines.
- `execute_conditional_logic`: Sets a global variable to zero, calls another routine, and if the return value is zero, calls another routine and increments a value through a pointer.
- `check_nested_conditions`: Calls another routine and, if certain conditions are met, calls another routine and then a third routine, returning a concatenated result of the last call.
- `loop_until_bit_match`: Calls another routine and enters a loop that calls another routine until a bitwise comparison between a value and a memory location matches a specific pattern.
- `rotate_and_test_bits`: Performs a loop sixteen times that swaps the high and low words of register d0, shifts the value left, and sets specific bits based on previous bit states.
- `configure_dma_registers`: Writes several values to the custom hardware registers at offset a fixed address and calls another routine.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (illegal) and then falls through into the code that follows, with no return of its own.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_branch_sequence`: Sets d1 to 255 and clears bit 7, calls another routine, loads a long value into d0, calls another routine, and branches to another location.
- `modify_cia_register`: Clears a bit in a byte at a specific address using a value from a global variable, then performs a conditional check on a hardware register to set register d0 to zero.
- `update_global_table`: Calls another routine, and if a bitwise check on d5 and the result of another routine passes, calls another routine and writes a word to a table at an offset calculated from a value in d0.
- `clear_table_entry`: Checks a bit in a hardware register and, if set, clears a word at an offset in a table and updates a global variable via a loop.
- `update_hardware_byte`: Modifies a byte at a specific address by applying a bitmask and clearing a bit based on a value from a global variable, then performs a loop.
- `shift_and_decrement_d0`: Performs a loop that shifts the value in d0 right by five bits and decrements it until it reaches zero.
