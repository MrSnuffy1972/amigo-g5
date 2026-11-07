# Game 2348 (king's quest iv - the perils of rosella (1990)(sierra)[cr pdx](disk 1 of 4)): code recovery summary

## Functions (part 2 of 2)

### King's Quest IV - The Perils of Rosella (1990)(Sierra)[cr PDX](Disk 1 of 4).adf:AMIGASND.DRV
(continued)

- `update_audio_parameters`: Iterates through a list of values to update a table, performing subtractions and conditional increments based on comparison results.
- `write_audio_register_value`: The routine writes a scaled value to a hardware audio register.
- `set_audio_channel_volume`: The routine writes a computed value to a custom hardware register.
- `clear_sound_flags`: The routine checks a byte in a structure pointed to by A4 for four individual bits.
- `initialize_sound_channel_state`: The routine initializes a sound channel structure pointed to by a pointer in a0 using a state flag pointed to by a1.
- `copy_sound_params`: The routine copies sound parameters from a source structure pointed to by a pointer into a destination structure pointed to by another pointer.
- `copy_sound_driver_fields`: The routine copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields at fixed offsets within a destination structure pointed to by a0.
- `set_sound_driver_state`: The routine updates fields in a sound driver structure pointed to by a register using values from a short array pointed to by another register.
- `set_sound_channel_parameters`: The routine copies a set of parameters from a table pointed to by a pointer into fields of a structure pointed to by a pointer, and sets two fields to a constant.
- `copy_sound_header_fields`: The routine copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes constant 16-bit values into two other fields of the destination structure.
- `set_sound_driver_state_2`: The routine updates a sound driver control block pointed to by a register using a state flag pointed to by another register.
- `reset_sound_channel_state`: The routine checks a flag in a short table pointed to by a pointer.
- `copy_sound_parameters`: The routine copies a long word and two word values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes constant word values into two other fields of the destination structure.
- `initialize_sound_driver_state_2`: The routine initializes or updates a sound driver control block pointed to by a register.
- `copy_sound_parameters_2`: The routine copies sound parameters from a source structure to a destination structure.
- `copy_sound_params_2`: The routine copies a 32-bit value and two 16-bit values from a source structure pointed to by a register into specific offsets of a destination structure pointed to by another register.
- `execute_cleanup_callbacks`: Calls several function pointers stored in a structure and then calls a routine that takes a single argument.
- `restore_audio_state`: The routine saves a two-byte value from an object, calls another routine, and if the result is not three it sets a flag byte, repeatedly calls another routine until a two-byte field stops changing, clears the flag, restores the saved two-byte value, and calls another routine.
- `update_sound_driver_state`: The routine updates per-channel driver fields from an input byte pair, writes a derived value into a table entry, sets flags, and conditionally calls another routine.
- `store_audio_driver_parameters`: The routine reads two bytes from a pointer and stores one of them into a driver structure field, then updates a table entry based on an index derived from a register.
- `configure_audio_hardware`: Calculates an offset into a table to load values into a destination structure and updates a bitmask in a global variable.
- `set_flag_and_write_byte`: The routine sets a bit in a byte located via a pointer and writes a 16-bit value from a register to a table entry, then calls another routine.
- `copy_sound_index_entry`: The routine copies a byte from a table indexed by an offset into a destination byte in a structure and writes a constant marker into the source byte.
- `compare_and_copy_byte`: The routine compares a byte pointed to by a pointer with a byte from a table indexed by a word offset.
- `return_constant_value`: The routine returns a constant value without performing any other operations.
- `initialize_audio_channels`: The routine sets up four audio channel control blocks in RAM, copies a small table from a global variable into the first block, clears flags and initializes fields for each channel, then installs interrupt handlers for the four audio channels via Exec SetIntVector and AllocMem.
- `set_sound_channel_state`: The routine updates a sound channel entry in a table using an index from the input and a global state flag.
- `initialize_sound_channel`: The routine initializes a sound channel entry in a global table and sets up related per-channel state bytes in a global structure.
- `clear_sound_channel`: The routine clears a sound channel entry selected by an index in D0 and initializes its fields from a global state block and a secondary pointer.
- `initialize_sound_driver_state_3`: The routine writes constants and values read from an uninitialized pointer into fields of a structure pointed to by a0, clamping one field to a maximum.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)[cr PDX](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_20`: The routine contains a single instruction and performs no observable operation.
- `set_two_word_flags_2`: The routine writes the value 1 to two consecutive words of a global variable by using an offset from a pointer in register A4, then returns the value in D0.
- `set_two_word_flags_3`: The routine writes a constant value into two separate locations in a global variable structure using an uninitialized pointer base.
- `set_two_word_flags_4`: The routine writes the value one to two adjacent global word variables and returns the incoming register value unchanged.
- `set_flag_and_return_mask`: The routine writes a constant to a global variable and returns a constant mask.
- `set_flag_and_return_mask_2`: The routine stores a constant value into a global variable and returns a constant bit mask.
- `set_two_word_flags_5`: The routine writes the value one to two consecutive word locations in a global variable area and returns the value in D0.
- `set_flag_and_return_mask_3`: The routine writes a constant to a global variable and returns a constant mask.
- `set_flag_and_return_mask_4`: The routine stores a constant byte value into a global variable via a pointer derived from a register and then returns the bitwise OR of an input byte with a constant mask.
- `set_flag_and_return_mask_5`: The routine stores a constant into a global variable and returns a constant mask.
- `set_flag_and_return_mask_6`: The routine stores a constant byte value into a global variable and returns a byte formed by OR-ing an input byte with a constant mask.
- `set_flag_and_return_mask_7`: The routine stores a constant into a global variable and returns a constant mask.
- `set_two_word_flags_6`: The routine writes the constant 1 to two word-sized locations derived from a base register offset, then returns the value originally in D0.
- `set_flag_and_return_masked_byte`: The routine writes a constant to a global variable and returns a byte from a register masked with a constant.
- `set_two_word_flags_7`: The routine writes the value one to two consecutive word-sized locations in memory referenced through an uninitialized pointer.
- `return_constant_address`: The routine returns a constant address value without performing any operations on registers or memory.
- `return_constant_address_2`: The routine returns a constant address.
- `unreached_routine`: The routine is a single-instruction block that is never reached by the program.
- `empty_routine_21`: The routine contains a single instruction and performs no observable work.
- `empty_routine_22`: The routine contains a single instruction and performs no observable work.
- `execute_single_instruction`: The routine contains a single instruction and has no body.
- `unreached_routine_2`: The routine contains a single instruction and is never reached by the program.
- `empty_routine_23`: The routine is empty; it contains a single instruction and performs no operations, makes no calls, and does not access registers or memory.
- `unreachable_stub`: The routine consists of a single instruction and is never called by any other code in the segment.
- `store_byte_via_pointer`: The routine stores a byte from a register through a pointer.
- `unreached_gap_code`: The routine is a one-instruction fragment that lies in a gap between other code.
- `store_byte_from_register`: The routine stores a byte from a register through a pointer.
- `describe_unreached_routine`: The routine contains a single instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_two_word_flags_8`: The routine stores the value one into two separate word-sized global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_address_3`: The routine returns a constant address value.
