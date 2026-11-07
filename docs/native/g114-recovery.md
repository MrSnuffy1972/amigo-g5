# Game 114 (ThunderStrike v1.0 0406): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00114). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926891 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 68 of 68 named (0 author symbols), 68 of 68 described (6 by rule, 43 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 624 of 624 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 8704 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 838 of 838 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 273849 |
| code | 2884 |
| data | 644513 |
| padding | 13 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Thunderstrike.Slave | 1 | 137 | 13 |
| Disk.1:bootblock | 1 | 8 | 2 |
| logged disk load from Disk.1 | 1 | 691 | 51 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| Disk.1 bytes 281600-290304 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: custom-format (non-DOS) disk, 1733 non-empty blocks: matched against track-loaded segments
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: bytes 281600-290304 look like code no run loaded: recovered position-independently (load address unknown)
- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: 3 logged loads cover 289280 bytes of the image
- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: blocks 330-549 (bytes 168960-281600) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.77)
- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: blocks 567-945 (bytes 290304-484352) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.55)
- ThunderStrike_v1.0_0406/ThunderStrike/Disk.1: blocks 1186-1759 (bytes 607232-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.50)
- 1 never-loaded region(s), 8704 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_channel_from_data` (Disk.1 +an address loaded at an address (157696 bytes)): Loads a value from a data stream into a global variable and processes the stream to update channel parameters. Kind: checksum/data integrity trap. Pass path: the loop completes and returns to the caller via the return instruction at the end of the function.

## Functions


### Thunderstrike.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_memory_table`: Iterates through a memory range to search for specific patterns and writes values and function pointers to a table.
- `wait_for_bit_and_store`: Polls a bit in a global variable until it matches a specific value, then stores a register value into two consecutive memory locations.
- `check_hardware_status`: Polls hardware registers and checks bits in a global variable before potentially entering an infinite loop.
- `enter_infinite_loop`: Performs an infinite loop that never returns.
- `store_and_poll_bit`: Stores a word into a memory location using a pointer and then polls a bit in a global variable.
- `store_byte_and_poll_blitter`: Stores a byte from a data register into a memory location pointed to by an offset from a base register, then enters a loop that waits for a specific bit in a global variable to clear.
- `store_word_and_wait_blitter`: Stores a 16-bit value from a data register into a memory location pointed to by an address register.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `update_obfuscated_value`: Performs arithmetic and bitwise operations on a global variable based on a carry flag.
- `update_and_call_function`: Performs a bitwise OR operation on a longword at a memory location and then calls a function through a pointer.
- `call_function_via_pointer`: Calls a function through a pointer stored in a table.
- `return_without_action_2`: The routine takes its entry, makes no calls, reads or writes no data, and returns to its caller.

### Disk.1:bootblock

- `execute_boot_sequence`: A single instruction routine that calls another routine.
- `execute_seven_instructions`: A routine consisting of seven instructions that calls another routine.

### logged disk load from Disk.1

- `calculate_scaled_values`: Stores two values into global variables and performs division on shifted versions of input registers.
- `calculate_scaled_values_alt`: Stores two values into global variables and performs division using values from a table.
- `store_word_values`: Stores two 16-bit values from registers into global variables.
- `store_three_values`: Stores three specific values into three different global variables.
- `update_button_status`: Checks specific bits in a register and a global variable to conditionally set various global bytes to a specific value.
- `update_conditional_flag`: Checks a bit in a register and a byte value to conditionally set a global byte to one.
- `calculate_scaled_differences`: Calculates differences between a global variable and a constant, then performs scaled arithmetic and stores the results in multiple global variables.
- `copy_word_to_global`: Stores a word from a register into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 78 into d0.
- `empty_routine_sub_9c1c`: An empty routine that performs no operations.
- `nop_placeholder`: This routine is empty and performs no operations.
- `perform_no_operation`: This routine is empty and performs no executable instructions.
- `empty_routine`: This routine contains no executable instructions and performs no operations.
- `set_byte_on_match`: Compares a byte against several global values and sets a byte at a calculated offset to one if a match is found.
- `calculate_bitmask`: Performs bitwise operations on a global variable to generate a new word value.
- `update_game_state`: Modifies several global variables and memory locations based on conditional checks and table lookups.
- `store_scaled_values`: Stores two values into global variables after performing division using a table.
- `mask_and_store_word`: Applies a bitmask to a word and stores the result in two separate global word locations.
- `process_byte_table`: A two instruction routine that calls another routine.
- `initialize_table_processing`: Sets specific global values and then iterates through a byte table to update global counters and values.
- `process_byte_table_2`: Iterates through a table of bytes, performing bitwise operations and updating global values based on the content of the table.
- `process_and_distribute_bytes`: Reads bytes from a table using an offset, performs bitwise operations, and writes the results to multiple global locations based on a bitmask.
- `tiny_or_and_asr`: Small routine of 3 instructions that merges d0 into d0, then masks d0 with d0, then shifts d0 right by 8.
- `modify_memory_blocks`: Performs bitwise AND and OR operations on four different locations in a memory table using values from registers.
- `update_audio_state`: Manages internal counters and flags to update hardware-related state, including calls to other subroutines.
- `step_channel_countdown_table`: Decrements a per-channel countdown byte; when it reaches zero it reads the next duration from a table, ending the step if that value is zero.
- `update_channel_envelope_step`: Advances one channel's envelope and modulation state: it calls the countdown routine, scales a table value by a volume byte, applies optional sign and offset adjustments under flag bits, and stores the result as the channel's output word.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d3.
- `reset_audio_channel_state`: Clears five per-channel state bytes, then writes two values to a shared control word, first all bits set and then a low nibble, which the facts show as audio DMA and ADKCON register writes.
- `read_channel_sequence_bytes`: Reads bytes from a channel's sequence stream until one exceeds the threshold.
- `store_channel_header_and_parse`: Stores the first byte of a sequence stream into a song-level field and then runs the same sequence byte loop that ends by saving the stream position.
- `update_channel_status`: Sets specific bits in a control structure and processes a data stream to update channel parameters.
- `update_channel_status_minimal`: Sets a bit in a control structure and processes a data stream to update channel parameters.
- `store_channel_byte_and_parse`: Stores the first byte of a sequence into a channel field and then runs the same sequence byte loop that ends by saving the stream position.
- `init_pitch_slide_and_parse`: Stores a step value and a count into channel fields, clears the accumulator, and sets the slide flag, then parses the remaining sequence bytes.
- `init_channel_oscillator_and_parse`: Stores two header bytes as oscillator parameters, one used as a phase counter and one doubled as its range, sets the oscillator-enable flag, and then parses the remaining sequence bytes.
- `store_channel_param_and_parse`: Stores the first sequence byte into a channel parameter field and then parses the rest of the sequence stream until the step ends.
- `update_channel_status_extended`: Sets multiple bits in a control structure and processes a data stream to update channel parameters.
- `clear_channel_status`: Clears a control structure and processes a data stream to update channel parameters.
- `initialize_channel_from_data`: Loads a value from a data stream into a global variable and processes the stream to update channel parameters.
- `tick_channel_sequence_timer`: Counts down the channel's step timer; while it runs, a ramp flag moves a level byte up or down.
- `store_wave_index_and_parse`: Stores the first sequence byte as a wave table index for the channel and then parses the rest of the stream.
- `store_song_byte_and_parse`: Stores the first sequence byte into a song-level field and then parses the rest of the sequence stream until the step ends.
- `repeat_sequence_or_continue`: Decrements the channel's repeat counter; while it stays non-zero, parsing restarts from the saved loop pointer, otherwise it continues from the current position.
- `set_channel_pointer`: Stores a byte and a pointer into a control structure before processing a data stream to update channel parameters.
- `set_channel_fixed_value`: Stores a constant byte and a pointer into a control structure before processing a data stream to update channel parameters.
- `store_song_param_and_parse`: Stores the first sequence byte into a song-level parameter field and then parses the rest of the sequence stream until the step ends.
- `reset_step_and_save_position`: When the channel's hold flag is clear, it zeroes the channel's extra counter, sets its start count and writes the channel's output byte.
- `load_table_entry_into_channel`: Looks up a table entry at the given index, saves the index in the channel's table position field, stores the looked-up value as the channel's current level, and sets its counter to one.
- `update_channel_sample_pointer_length`: For a channel whose start delay is running, it counts the delay down, clamps a negative offset to zero on expiry, and writes the sample pointer and word length into the channel's audio hardware slot, then writes a DMA control value.
- `reset_and_load_channel_table`: Clears the channel state table of sixteen records, marks two global bytes with a full-set sentinel value, then loads four channel start entries from a table selected by the argument, setting each channel's pointer, length and sentinel fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_without_action`: A single return instruction with no other work, so it is an empty routine in the runtime segment with no observable effect.
