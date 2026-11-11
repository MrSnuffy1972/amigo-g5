# Game 1810 (d-generation (1991)(mindscape)(disk 1 of 2)[cr flt][h eternal]): code recovery summary

## Functions (part 2 of 2)

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)
(continued)

- `decrement_and_reset_table`: Decrements a countdown at a fixed offset in a structure and, when it reaches zero, resets it to four and walks 32 words, subtracting one from each nonzero 4-bit field in the high, middle and low positions.
- `initialize_dma_and_audio_channels`: Writes the DMA and interrupt control registers, the CIA B port registers and the audio-channel registers, then waits for the vertical beam position counter to reach a threshold.
- `build_register_value_table`: Fills a table with four groups of paired words holding incrementing register-style values and the high and low halves of a fixed address, then writes 80 words of incrementing values interleaved with zeros, and stores a pointer and a counter into a structure.
- `restore_dma_and_interrupt_state`: Disables DMA and interrupt enables through the custom registers, restores saved values from globals, sets two global flags, then re-enable DMA by writing a fixed value.
- `copy_structure_members`: Copies a 4-byte value from one location in a structure to another and stores a 2-byte value into a different structure offset.
- `set_interrupt_request_register`: Writes the constant 255 to one variable and 32 to a global variable associated with the interrupt request register.
- `set_interrupt_control_register`: Writes the constant 255 to one variable and 8192 to a global variable associated with the CIA interrupt control state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_game_entry`: Calls a routine to decode animation frames, calls a routine to initialize the game state, and returns zero.
- `execute_library_sequence`: Calls a routine, then calls a library function to open a library, calls another routine, calls a library function with a pointer and a size, calls a third routine, and finally calls a library function to close a library.
- `initialize_memory_and_call_library`: Calls two library functions, sets a certain memory range to zero, and checks if certain memory values match expected constants before calling a third library function and a fourth routine.
