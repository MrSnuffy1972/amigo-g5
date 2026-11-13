# Game 186 (International Soccer Challenge): code recovery summary

## Functions (part 2 of 2)

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)
(continued)

- `copy_byte_and_dispatch`: Copies the byte at the pointer in a1 to a byte field of the structure in a3, advancing the pointer, then jumps to a dispatch routine.
- `update_structure_bytes`: Stores a byte from a pointer into an offset of 1 of a structure in a0, copies two bytes from a1 into offsets 44 and 46 of the structure in a0, clears a byte at offset 45 of the structure in a0, and branches to another routine.
- `set_status_flag_2`: Sets the sign flag based on a byte in a0 and branches to another routine.
- `copy_byte_to_structure`: Copies the byte at the pointer in a1 to a byte field of the structure in a0, advancing the pointer, then jumps to a dispatch routine.
- `initialize_structure_values`: Copies two bytes from a1 into offsets 8 and 9 of a structure in a0, sets a word at offset 10 of the structure in a0 to zero, and branches to another routine.
- `extend_and_store_byte`: Copies a byte from a1 to d0, sign-extends d0 to a word, stores the word in a structure at offset 1512 using a3, and branches to another routine.
- `copy_byte_to_structure_2`: Copies a byte from a1 to offset 1519 of a structure in a3, then copies a byte from a PC-relative address into offset 1520 of the structure in a3, and branches to another routine.
- `process_chain_entries`: Walks a chain of structures stored in a table, copying each entry's fields into a destination structure four times, then stores the final chain pointer and increments a counter, but only when the flag at offset 95 of the structure is zero.
- `process_dma_chain_setup`: Sets bit 1 of a global variable, zeroes one of four bytes depending on the value in d1, and copies that value into a byte at offset 45c82.
- `enable_dma_channels`: Writes the values 1 and 124 to the custom DMA and interrupt control registers at offsets 164 and 166, then calls spin_short_delay_loop to burn a short amount of time before returning.
- `spin_short_delay_loop`: Decrements a 16-bit counter from 512 down to -1 in a loop that branches back to the earlier decrement, then returns, producing a short busy-wait delay.
- `disable_dma_channels`: Writes to the DMA control register with bit 15 set to disable DMA channels, then returns after twelve instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_blitter`: Clears several RAM regions and initializes multiple lookup tables, then sets the bitplane pointer registers and waits for the blitter to finish.
- `configure_audio_channels`: Checks a global variable and, when it is nonzero, writes the value 24596 to three audio channel registers derived from the pointer in a0, then restores the stack and returns.
- `dispatch_table_handlers`: Walks a table of ten byte entries, calling a handler for each entry whose value is 0 or 1, returning 2, 3, or 4 when the entry is 2, 3, or 4, and returning the entry value when the loop counter reaches zero.
- `draw_string_glyphs_loop`: Stores the two input coordinates into a global variable, then loops over the characters of a null-terminated string, writing each character's eight-row font glyphs to a screen table and advancing the column position per character.
- `draw_text_glyphs`: Reads characters from a null-terminated string and writes their eight-row font glyphs to a screen table, advancing the column position per character and looping once per byte until the terminator is reached.
- `step_counter_toward_joystick_value`: Reads a joystick direction value from the fire-button and disk status port and steps a stored counter up or down toward it, updating a state word across several calls.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `read_keyboard_status`: Reads the keyboard serial register after disabling interrupts, inverts and rotates the value, and stores it into a global variable only when bit 7 of the result is clear, otherwise it returns.
- `fill_gradient_table_rows`: Fills a table in nested loops, writing a stepped byte sequence across each row and advancing the increment as it goes, then null-terminating the rows.
- `increment_table_by_flag`: Selects a byte based on a global flag, then loops over a table adding that byte to values at spaced offsets and toggling the flag each iteration before returning.
- `wait_and_configure_port`: Waits in a loop until bit 14 of the byte at offset 2 of the structure in a6 becomes clear, then stores the values an address, 0, an address and 9876 at offsets 84, 100, 64 and 88 of that structure, and finishes by decrementing register d0 from 256 until it underflows to -1.
- `set_bitplane_bits_from_lookup`: Walks a nine-by-thirty-two grid of source bytes and uses two lookup tables to find the byte and bit to set in each of two bitplane-style buffers for every cell.
- `initialise_lookup_tables`: Subtracts 4112 from each of seven consecutive words in a global table while calling a routine for each, then fills 256 entries with multiples of 40 and 320 word pairs holding a right-shifted index and its inverted low three bits.
- `process_wait_structure`: Decrements a counter that resets to 8 when it reaches 0, skips past empty slots in a table, then loops eight times copying a value and adding it to a field while calling a wait routine before returning.
- `store_structure_fields_wait`: Reads a value from a structure field and computes an address from another field, then waits until a status bit clears before storing several words and long words into fixed offsets of a structure.
- `wait_and_configure_port_2`: Waits in a loop until bit 14 of the byte at offset 2 of the structure in a6 becomes clear, then stores the values an address, 0, an address and 4052 at offsets 84, 100, 64 and 88 of that structure, then returns.
- `fill_sample_pointer_table`: Fills a table of integers with a repeating pattern of two values that change by an address each iteration, then clears a byte table at address an address by writing zeros to every 40th pair of consecutive bytes, and returns.
- `clear_sample_ram`: Points register a0 at address an address, then writes zeros to every 40th pair of consecutive bytes starting there, repeating the store 37 times, and returns.
- `advance_pointer_list_flag`: Toggles a flag bit and, when the previous value was set and a counter is nonzero, reads a pointer from a list, stores it, and resets it to a default when the next entry is negative, otherwise it returns.
- `fill_nibble_packed_lookup_table`: Fills 320 table entries, each storing the index's high part shifted left by one and a packed word built from the index's high and low nibbles combined with a fixed constant.
- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `dispatch_audio_channels`: Increments a global counter and, when it wraps, calls the audio dispatch routine four times, runs two short delay loops, reloads eight audio channel values from memory, and then repeatedly adds 16 to a value until it reaches 1024.
- `handle_audio_dispatch`: Touches the eight audio channel registers and DMACON, calls mask_and_compare_word and dispatch_structure_byte_handler, and contains a loop that branches back to an earlier instruction before returning.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `dispatch_structure_byte_handler`: Reads the low nibble of a byte at offset 2 of the structure in a6 and dispatches on it: for 0 it looks up a value in a table using a global variable modulo 3, for 1 it decrements a 12-bit counter at offset 16 with clamping, for 3 it moves a byte to offset 23 and adjusts a counter at offset 24, and for 4 it indexes a table at offset an address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_11`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.
- `manipulate_data_registers`: Copies a byte from d0 to d2, performs logical OR and exclusive OR operations on d0 with zero, and returns.

### International Soccer Challenge.adf:Intro.Pdx#unpacked

- `initialize_game_environment`: Clears a large memory range, initializes several lookup tables, and sets up hardware registers including the copper list, interrupts, and DMA. It enters a main loop that updates sprite positions, handles input from the joystick and mouse, manages a frame counter, and calls various subroutines for graphics and audio updates until a specific exit condition is met.
- `write_constant_to_memory`: Tests a word in a global variable, and if it is zero, skips a block that pushes a0 to the stack, loads a pointer from the stack, writes the word 6004 to three different offsets of that pointer, restores a0, subtracts a1 from a1, loads a pointer into a6, and returns.
- `update_global_values`: Iterates through a table of words, calling another routine for each entry, and writes specific constant values to a global variable based on the value read from the table.
- `draw_text_string`: Stores two words into global variables, then loops through a null-terminated string to copy eight bytes from a table to a destination calculated from a base address, an offset, and a multiplier.
- `draw_text_glyphs_from_position`: Calculates a destination address based on a base address, an offset, and a multiplier, then loops through a null-terminated string to copy eight bytes per character from a font table to the destination, advancing the destination by 40 bytes each iteration, and increments a global offset before returning.
- `update_state_and_generate_table`: Cycles through a three-state counter and updates a global index based on a value read from a structure in a6.
- `read_keyboard_serial_data`: Clears a byte at a hardware register, clears d0, sets bit 6 of a control register, reads a byte from a serial data register into d0, clears the serial data register, clears bit 6 of the control register, performs a bitwise NOT and a rotate on d0, and if bit 7 of d0 is set, branches to another routine, otherwise it stores the result in a global variable and returns.
- `populate_data_table`: Performs a nested loop to calculate values using multiplication and addition, then writes a sequence of bytes and words to a memory range starting at 6e000.
- `increment_memory_bytes`: Checks a global word, then enters a loop that adds either 1 or -1 to specific byte offsets within a memory range starting at 6e000.
- `initialize_hardware_structure`: Waits for a bit to be set in a structure at offset 2 of a6, then writes several longwords and a word to a structure at a6.
- `update_bitmaps`: Iterates through nested loops to read bytes from two source tables and set specific bits in a destination memory range starting at 7b000.
- `initialise_lookup_tables_2`: Subtracts 4112 from seven consecutive words in a global table while calling another routine for each, then fills 256 entries at a memory location with multiples of 40, and finally fills 319 entries at another memory location with word pairs consisting of a right-shifted index and its inverted low three bits.
- `update_graphics_parameters`: Decrements a global word and, if it reaches zero, resets it and increments a pointer; it then loops through a sequence to write values to a table and calls another routine to set up graphics parameters.
- `setup_graphics_parameters`: Calculates two values using table lookups and writes a set of longwords and a word to a structure at a6.
- `clear_graphics_structure`: Waits for a bit to be set in a structure at offset 2 of a6, then writes a longword, a longword, and a word to a structure at a6.
- `fill_large_table`: Fills a large memory area with a pattern of longwords generated through nested loops and then writes a series of specific longwords to a table.
- `clear_memory_blocks`: Enters a loop that clears a byte and a byte at offset 39 from a memory address starting at 7f810.
- `pop_short_from_list`: Toggles the lowest bit of a global variable and, if certain conditions are met, moves a word from a list to a global variable and increments the list pointer.
- `generate_color_palette`: Runs a loop 320 times to calculate and store two 32-bit words at a time into a memory range starting at 78000.
- `build_pointer_table`: Walks a byte table to build a pointer table, clears several global variables, and resets certain counters or registers.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_sequence_and_hardware`: Increments a byte in a structure and, if it reaches a limit, resets it and calls another routine four times to update the four audio channels with parameters from a table before updating the DMA control register and audio length registers.
- `update_audio_and_hardware_state`: Loads a structure from a pointer, uses a nibble-based index to fetch sound parameters from a table, and updates audio registers and DMA control.
- `mask_and_compare_word_2`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_structure_fields_2`: Reads a command byte from an offset in a structure to perform various arithmetic or logical operations on a 16-bit value and a byte at other offsets within the same structure.

### MUSIC.BIN (raw code file, load address unknown)

- `save_restore_call_return_2`: Saves the caller's registers, calls a routine, restores the saved registers, and returns without doing anything else.
- `init_audio_channels`: Initializes the audio subsystem by clearing music globals, enabling the four audio channels, writing control values to the audio hardware registers, and resetting one of four one-byte channel fields.
- `initialize_linked_structures`: Sets a flag bit in a global variable, reads a byte from a table indexed by a computed offset, then initializes four linked structures by filling their fields with pointers and constants before returning.
- `clear_music_globals`: Clears a series of global variables and sets a global variable to fifteen, then returns.
- `set_audio_flags`: Sets four global variables to one, enabling the four audio channels.
- `enable_audio_channels`: Checks a global flag and, when it is set, builds a channel-enable value by setting four bits and stores it in the audio control register, then sets a follow-up flag and returns.
- `set_register_values`: Writes the value 3 to the two byte registers at offsets 1519 and 1520, then returns; the trailing branch and final instruction are not a loop and do not call anything.
- `advance_music_channels`: Decrements a global counter and, when it reaches zero, clears a block of song-related fields, then iterates over a table of channel entries decrementing their timers and advancing their data pointers until a channel's timer expires.
- `advance_table_entries_seven_times`: When the flag byte at offset 947 of the structure in a3 is zero, it walks a table of 16-bit entries seven times, copying each entry's first field forward by six bytes and storing its shifted value and a division result into adjacent fields, then records the advanced pointer and clears the flag; otherwise it returns.
- `store_index_and_clear_field`: Sets bit 1 of a global variable, loads 255 into it, then clears one of four one-byte fields at offsets 1485, 1493, 1501 and 1509 depending on whether the incoming data byte is zero, one, two or three, stores that byte and the other incoming byte into globals, and returns.
- `spin_short_delay_loop_2`: Counts a local variable down from five hundred and twelve to zero in a loop and returns.
- `step_channel_envelope_and_slope`: It decrements the envelope counter at offset 9 and, when it reaches zero, stores a stored value and clears a flag; otherwise it applies a shift and a pitch slope to the channel period.
