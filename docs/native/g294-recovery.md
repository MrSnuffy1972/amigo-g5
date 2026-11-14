# Game 294 (gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00294). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902560 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 20 raw-word lines |
| Functions named and described | pass, 77 of 77 named (0 author symbols), 77 of 77 described (4 by rule, 73 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 198 of 198 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 6656 bytes, 51 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1492 of 1494 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 5952 |
| data | 833841 |
| padding | 815 |
| unused | 61952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf:bootblock | 1 | 39 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 329 | 24 |
| Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf bytes 856064-862720 (code never loaded by a reference run) | 1 | 1126 | 51 |

## Cases handled

- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: custom-format (non-DOS) disk, 1637 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 192 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: bytes 856064-862720 look like code no run loaded: recovered position-independently (load address unknown)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: 1 logged loads cover 6656 bytes of the image
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 2-8 (bytes 1024-4608) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.51)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 11-454 (bytes 5632-232960) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.97)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 457-470 (bytes 233984-241152) outside the filesystem hold data, not code (opcode density 0.033, entropy 4.16)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 510-1018 (bytes 261120-521728) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 1020-1501 (bytes 522240-769024) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.90)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 1505-1664 (bytes 770560-852480) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.04)
- gauntlet ii (1989)(u. s. gold)[cr qtx][t +4 subway - dream team]/Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf: blocks 1686-1693 (bytes 863232-867328) outside the filesystem hold data, not code (opcode density 0.059, entropy 6.18)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf:bootblock

- `bootblock_setup_dma`: Loads a list of addresses into a register, copies six of them into a table in a loop, and then jumps to a fixed address where it configures the DMA channels.
- `wait_vblank_configure_dma`: Waits for the vertical-blank counter to reach specific values while decrementing a timer, then programs four DMA channel descriptors in a structure via a6, clears a region of memory, and finally disables all DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_display`: Initializes the system by opening a library, clearing a memory region, starting the graphics and font systems, computing sizes from stored addresses, and then configuring display and DMA registers before returning.
- `fill_routine_table`: Stores the addresses of thirteen routines into consecutive longword slots in a table, then returns.
- `set_flag_bits`: Sets specific bits in three global variables using bitwise OR operations with immediate values.
- `set_more_bits_in_d7`: Sets bits 142, 192, 222, 252, 26 and 34 of the byte register d7 in sequence, then returns.
- `decrement_and_format_counter`: Reads a 16-bit counter at offset 374 of the structure in a5 and, when it is nonzero, decrements that counter and stores the previous value divided and modulo by ten as two ASCII digits at offsets -2 and -1 of a table, then returns.
- `clear_and_store_bytes_r4`: Clears the 16-bit value at offset 376 of the structure in a5 and stores the three bytes 32, 78 and 79 at offsets -3, -2 and -1 of a table, then returns.
- `clear_and_store_bytes_r5`: Clears the 16-bit value at offset 378 of the structure in a5 and stores the three bytes 32, 78 and 79 at offsets -3, -2 and -1 of a table, then returns.
- `clear_and_store_bytes_r6`: Clears the 16-bit value at offset 380 of the structure in a5 and stores the three bytes 32, 78 and 79 at offsets -3, -2 and -1 of a table, then returns.
- `store_one_at_offset_382`: Stores the value one into the 16-bit value at offset 382 of the structure in a5, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_format_counter`: Reads a 16-bit counter at offset 374 of the structure in a5, and when it is not 98 increments that counter and stores the value plus two divided and modulo by ten as two ASCII digits at offsets -2 and -1 of a table, then returns.
- `store_one_and_bytes_r9`: Stores the value one into the 16-bit value at offset 376 of the structure in a5 and stores the three bytes 89, 69 and 83 at offsets -3, -2 and -1 of a table, then returns.
- `store_one_and_bytes_r10`: Stores the value one into the 16-bit value at offset 378 of the structure in a5 and stores the three bytes 89, 69 and 83 at offsets -3, -2 and -1 of a table, then returns.
- `store_byte_sequence`: Stores the value 1 at offset 380 of the structure in a5, then writes the bytes 89, 69 and 83 into the three bytes preceding the pointer in a0, and returns.
- `set_flag_offset_382`: Stores the value 1 at offset 382 of the structure in a5, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_on_button`: Decrements a 16-bit counter at offset 894 of the structure in a5 and returns when it is nonzero, otherwise checks a fire button in the joystick port and dispatches to a routine selected from a jumptable by a scaled difference of two counters.
- `adjust_button_state`: Reads the vertical blank counter and the current button state, then adjusts the stored button state by the change since the last frame while keeping it non-negative and clamped to the previous value, and finally adds the button delta divided by 40 to the value at offset 1730.
- `update_table_entries`: Clears a flag at offset 384, then loops over ten entries of a table writing the value 4095 to two fields of each and copying the stored value into those fields when the entry index matches a counter, adds an increment to a running value and negates it when it reaches 3584 or 3806.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `store_scaled_block_bytes`: Reads a byte from a pointer in a1 and advances that pointer by that value, then loops a number of times equal to 40 minus the byte.
- `setup_blitter_and_increment`: Initializes a set of blitter registers with fixed source, mask, destination and size values around a wait call, then increments a counter in a structure and returns either the counter value or a value from another routine when the counter is divisible by eight.
- `copy_block_data`: Loops over a number of blocks copying 48 bytes from a source table into a destination area, scaling each block by subtracting 32 from a source index and advancing the pointers by fixed amounts each iteration.

### Gauntlet II (1989)(U. S. Gold)[cr QTX][t +4 Subway - Dream Team].adf bytes 856064-862720 (code never loaded by a reference run)

- `set_pointer_and_trap`: Sets a global variable to its own address and issues a processor trap.
- `install_interrupts_and_config`: Sets up the stack, installs interrupt vectors, and configures custom and CIA registers.
- `pack_joystick_and_buttons`: Returns immediately when a global counter does not match a stored value, otherwise reads the fire buttons and movement ports and packs them into a status structure, storing the four joystick directions and button states at fixed offsets.
- `pack_button_state`: Combines bits from the input register to build a status byte, setting bits according to the fire buttons and the transitions between states, and sets the high bit when the corresponding input line is clear, then stores the result in a global variable.
- `combine_word_difference`: Computes a difference between the high and low words of a table entry minus 2, stores the result in a global variable, restores the saved registers, and returns the original input value.
- `xor_word_pairs_into_memory`: Reads a word from a fixed address and XORs it into a destination word, then reads another pair of words from nearby fixed addresses and XORs each into a second destination word, then returns.
- `xor_state_words_with_keys`: XORs two state words with one key word and two more with a second word, stores one result into a structure, and calls another routine before returning.
- `set_bits_in_registers`: Applies a series of immediate OR operations to registers d0, d6 and d7, setting specific bits in each, and then falls through into the following routine.
- `update_palette_entries`: Clears 32 consecutive 16-bit palette entries in a color table when the branch is taken, otherwise loads a value from a source, doubles it, adds 273, masks the low 12 bits, and stores the result into each palette entry in a loop.
- `init_display_if_flag_clear`: Checks a flag in a custom register and returns immediately when it is set.
- `call_routines_in_sequence`: Calls a routine, then sets up register pairs and calls another routine several times with different source and destination addresses.
- `call_four_routines_only`: Sets a value in a register and calls a routine, then sets up register pairs and calls another routine.
- `call_routines_then_jump`: Sets a value in a register and calls a routine, then calls another routine directly.
- `call_three_routines_only_2`: Calls three other routines in sequence and returns, with no other code.
- `load_table_and_call_routines`: Loads a jump-table address into a register and calls a routine, then returns.
- `call_three_routines_only`: Calls another routine, then a second routine, then a third routine in sequence, and returns.
- `step_palette_toward_target`: Copies sixteen words from a source area into a destination area, then calls a helper three times while stepping each four-bit component of sixteen colour words one unit toward a target value, adjusting each word up or down as needed.
- `decompress_byte_stream`: Decompresses data by looping over regions, copying bytes when a value is positive and filling repeated bytes when it is negative, advancing pointers accordingly, and returns.
- `advance_pointers_in_loops`: Decrements a counter and returns early if it stays non-negative, otherwise advances one pointer forward and another backward while two nested loops run, then returns.
- `configure_blitter_and_audio`: Stores an address into a global word, calls two routines, then loops nine times calling eight routines, and finally calls one more routine before returning.
- `configure_blitter_in_loops`: Configures blitter registers with a fixed mask value, then loops nine times calling four routines, and finally reconfigures the mask and calls those routines again before returning.
- `swap_eight_word_pairs`: Loops eight times swapping a pair of words between two regions, advancing one pointer by four bytes and the other by two bytes per pass, then returns.
- `set_bits_d7_then_noop`: It sets bits in register d7 by OR-ing 64, 128, and 192 into it, then ORs 0 into the same register, with no branch or call.
- `start_copper_list_dma`: It loads the custom-register base address into a6, stores an address into the copper list location register, clears the copper jump register, and writes 33664 into the DMA control register to enable DMA channels, then returns.
- `wait_then_start_copper_dma`: Calls a routine repeatedly until a stored word equals 1793, then points the copper list register at a fixed list, clears a strobe register, and enables master, bitplane and copper DMA before returning.
- `copy_table_words_with_wrap`: It advances a table index by two and resets it to zero when the referenced entry is negative, then copies 98 words from the table into a destination array eight bytes apart, wrapping the source back to its start whenever it reaches a negative entry, and finally restores the saved registers and returns.
- `setup_blitter_masks_modulos`: It writes 32832 into the DMA control register to enable DMA channels, sets the first and last word masks to 65535, and zeroes the blitter modulo and control registers, then returns.
- `queue_four_blit_parameter_sets`: Computes a size from a register, sets the DMA control register and blitter pointers, then fills four sets of blitter registers with increasing destination addresses and calls a blit routine for each before clearing the DMA control register.
- `setup_blitter_masks_modulos_2`: It writes 32832 into the DMA control register to enable DMA channels, sets the first and last word masks to 65535, and stores 28 into the blitter modulo registers, then returns.
- `queue_four_fixed_blit_sets`: Sets the DMA control register and blitter pointers, then fills four sets of blitter registers with fixed destination addresses and a constant size, calling a blit routine for each set before clearing the DMA control register.
- `setup_blitter_masks_modulos_3`: It writes 32832 into the DMA control register to enable DMA channels, sets the first and last word masks to 65535, and stores 34 into the blitter modulo registers, then returns.
- `queue_four_blits_clamped`: Returns unless a register holds a value between 39 and 199, then computes a size from that value, sets the DMA control register and blitter pointers, and fills four sets of blitter registers with increasing destination addresses before calling a blit routine for each and clearing the DMA control register.
- `set_two_audio_channel_registers`: It stores the sample pointer an address, a length of 60944, a volume of 64, and a period of 729 into two channel blocks sixteen bytes apart, one block for each of two channels, then returns.
- `start_audio_channel_on_index`: When the index equals 128 it enables audio channel zero via a DMA control write and an interrupt-enable write that set bit 15, and when it equals 122 it enables channel one the same way, setting a flag byte for channel zero, and it returns without otherwise doing anything.
- `copy_segmented_byte_data`: Reads a table entry containing a pointer, a length and an offset, then copies that many bytes from a source area into a destination area in a loop.
- `configure_disk_after_key`: Waits for a key record whose top 16 bits match a value, then configures the disk and audio hardware by writing the disk length and pointer registers, setting the audio and DMA control registers, and calling two routines before returning.
- `configure_disk_after_key_2`: Waits for a key record whose top 16 bits match a value, then configures the disk and audio hardware by writing the disk length and pointer registers, and finally calls the audio routines.
- `process_linked_key_records`: Walks a linked list of records, each holding a 32-bit key and a 16-bit value, and for every record whose key matches the top 16 bits of a value read from a table at a fixed address, stores the key, a count of 15, and the next key at the following record.
- `initialize_disk_drive_ports`: Writes fixed values to the DMA control register and three custom registers, stores 255 into the byte at offset 768 of the CIAB port and 3 into the byte at offset 512 of the CIAA port, then calls three routines and returns.
- `wait_for_interrupt_bit`: Waits in a busy loop until bit 2 of the interrupt request register is set, then writes 2 to the interrupt request register at offset 156 and returns the bit's value.
- `process_input_byte`: Toggles bit 2 of the CIA-B port based on bit 0 of the input byte, then adjusts a bit in that port depending on whether a computed difference is negative, pulses bit 0 in a nested delay loop, and finally stores the input byte into a global byte.
- `step_track_counter`: Increments a global counter and, while it stays below 160, pulses a CIA port bit with a fixed delay on even steps and clears it on odd steps, otherwise clears a direction bit and calls another routine.
- `pulse_line_while_flag`: Sets bit 1 of the CIA-B port, then pulses bit 0 in a fixed delay loop while a flag bit in a global word stays set, and finally clears that bit and resets the global word to zero.
- `toggle_ciab_bits`: Sets bits 3 and 7 of the CIA-B port, clears bit 3, and sets bit 3 again, with no loop or call in the instructions.
- `pulse_ciab_bit4`: Sets bits 4 and 7 of the CIA-B port, clears bit 4, and sets bit 4 again, with no loop or call in the instructions.
- `clear_drive_control_bits`: Clears bit 3 of the byte at offset 0 of the structure referenced by a4, then clears bit 7 and bit 3 of that same byte, then calls another routine, and returns; it then clears bit 4 of that byte, clears bit 7 and bit 3 again, calls the same routine, and returns.
- `call_routine_nine_times`: Calls a routine nine times in a loop that decrements a counter until it reaches zero, then returns.
- `wait_for_ciab_bit`: Waits in a busy loop until bit 4 of the CIA-B port is clear, then waits until that bit is set, and returns.
- `wait_for_table_marker`: Loops over words in a table until it finds one equal to 17545, then calls two routines, stores their results into a global word, and returns.
- `scan_table_for_marker`: Calls two routines, then scans words from a table at offset 34452 until it finds one equal to 62506, incrementing a counter as it goes, and returns.
- `setup_disk_transfer`: Sets up a disk transfer by loading a source address and a modified value into the disk and audio registers, then calls two routines before returning.
