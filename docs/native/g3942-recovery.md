# Game 3942 (HollywoodPokerPro v1.2 2505): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03942). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3679281 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 58 of 58 named (0 author symbols), 58 of 58 described (17 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 214932 bytes, 57 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1466 of 1611 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1146875 |
| code | 8146 |
| data | 2524211 |
| padding | 49 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| hollydat (raw code file, load address unknown) | 1 | 0 | 0 |
| ongame.dat (raw code file, load address unknown) | 1 | 946 | 36 |
| titlesong (raw code file, load address unknown) | 1 | 663 | 21 |

## Cases handled

- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505.hdf: hard-disk image (AmigaDOS OFS, 4032 blocks): its filesystem parses and all 33 files in it are byte-identical to the extracted tree HollywoodPokerPro_v1.2_2505_hdf, which is recovered file by file; no block outside the filesystem holds data
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/birg1: high-entropy data file (7.63 bits/byte), classified as packed data
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/ines3: high-entropy data file (7.64 bits/byte), classified as packed data
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/ines4: high-entropy data file (7.68 bits/byte), classified as packed data
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/ines5: high-entropy data file (7.71 bits/byte), classified as packed data
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/hollydat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/ongame.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- HollywoodPokerPro_v1.2_2505/HollywoodPokerPro_v1.2_2505_hdf/HollywoodPokerPro/data/titlesong: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 129938 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### ongame.dat (raw code file, load address unknown)

- `frag_bra`: Short code fragment that jumps to another point in the code (2 times in a row).
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `setup_coprocessor_2`: Performs conditional jumps based on hardware register states, calls several initialization routines, handles keyboard input from a serial register, processes input values through bit operations, and updates global variables based on comparisons.
- `initialize_audio_channels`: Clears audio channel registers and sets initial values for all four audio channels, then stores a pointer value in a global variable.
- `check_audio_condition`: Compares a memory location with zero and branches if equal, decrements another memory location, checks if it reaches zero, loads a value from memory, and calls other routines based on conditions.
- `update_audio_state_1`: Calls another routine multiple times with different register values, each call updates audio state variables based on table lookups and bit operations.
- `update_audio_state_2`: Updates audio state variables based on table lookups and bit operations, then branches to another routine for further processing.
- `copy_audio_data`: Clears two global variables, calculates an offset into a data table using a counter, copies six longwords from the table to another location, increments the counter, compares it with a limit, and resets it when exceeded.
- `copy_audio_data_2`: Sets several global variables to specific values, calculates an offset into a data table using a counter, copies six 16-bit values from the table to global variables, and updates counter values.
- `frag_cmpi_beq_bset_bsr_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then sets bit 1 of a fixed memory address, then calls another routine (2 times in a row), then returns.
- `process_audio_commands`: Processes a sequence of audio commands by calling another routine multiple times with incrementing parameters, then returns immediately.
- `handle_audio_state`: Processes audio state transitions based on a value at a pointer, updates audio channel parameters, and decrements a counter.
- `frag_lea_move_ori_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to d0, then merges 33280 into d0, then copies d0 to a fixed memory address, then returns.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to a fixed memory address, then returns.
- `manage_audio_state`: Manages audio state transitions by reading from a table, performing bitwise operations on the read value, clearing specific memory locations, and branching to different handlers based on the result.
- `configure_audio_channel_2`: Configures an audio channel by storing values to global variables, clearing memory locations, performing comparisons with table entries, and updating multiple global variables through subroutine calls.
- `select_audio_parameter`: Iterates through a table to find the maximum value, updates a global variable with the result, and modifies another global variable based on the selection.
- `setup_audio_pointers`: Calculates memory addresses for audio data tables and stores pointers to these locations in global arrays.
- `configure_audio_channel`: Initializes audio channel settings by copying values from a table, performs arithmetic operations on parameters, and updates multiple global variables.
- `frag_move_bsr_rts`: Short code fragment that copies a fixed memory address to a fixed memory address (4 times in a row), then calls another routine (4 times in a row), then returns.
- `manage_audio_state_machine`: Implements a state machine for audio processing that updates counters, handles conditional branches, and modifies global variables based on comparison results.
- `manage_audio_state_machine_2`: Implements a second state machine for audio processing with similar functionality to the first, updating counters and handling conditional logic.
- `manage_audio_state_machine_3`: Implements a third state machine for audio processing that updates counters and handles conditional branches in a pattern similar to the others.
- `update_global_counter`: Updates a global variable based on conditional logic, potentially toggling between states and performing arithmetic operations.
- `process_character_data`: Processes character input data by copying bytes from a source buffer to a destination, updates counters, and handles special control characters.
- `output_character_data_2`: Saves registers to the stack, performs left shift operations on a value, calls another routine multiple times with shifted values, restores registers from the stack, and returns a combined register value.
- `process_character_data_2`: Processes character input by extracting a value from a lookup table, performs bit manipulation operations, saves stack pointers to memory locations, and calls multiple routines with different parameters.
- `output_character_data`: Outputs character data by calling a routine multiple times with different parameters, then increments a global counter before returning.
- `output_character_sequence`: Outputs a sequence of character data by calling routines with specific parameters, handling special control characters, and updating a global counter after each operation.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then copies 16384 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies an address to a fixed memory address, then copies 49168 to a fixed memory address, then returns.
- `enable_interrupts`: Enables interrupts by setting bit 15 of the interrupt enable register and copies a global variable's value to a memory location.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `frag_movea_move_subi_jsr_rts`: Short code fragment that copies a fixed memory address to a6, then copies the memory at an offset from a0 to d0, then subtracts 1 from d0, then copies the memory at a0 to a1, then calls another routine, then returns.
- `copy_memory_block`: Copies data from one memory region to another in reverse order, decrementing a counter until it reaches zero, then stores a global variable's address to a hardware register.
- `frag_move_bsr_lea_movea_rts`: Short code fragment that copies 160 to a fixed memory address, then calls another routine, then loads the address of a fixed memory address into a0, then calls another routine, then copies a fixed memory address to a0, then copies a fixed memory address to the memory at an offset from a0 (2 times in a row), then returns.
- `setup_coprocessor`: Initializes coprocessor registers by opening a library, copying data from a structure to hardware registers, and calling routines to set up audio channels.

### titlesong (raw code file, load address unknown)

- `initialize_audio_channels_2`: Clears specific memory locations, copies data from memory to audio hardware registers, sets initial values for audio channels, clears channel length registers, and returns immediately.
- `update_sequence_state_2`: Calls another routine multiple times with different register values, each call updates sequence state variables based on table lookups and bitwise operations.
- `update_sequence_state`: Updates sequence state variables based on table lookups and bitwise operations, then performs conditional execution based on state values.
- `initialize_audio_setup_2`: Clears a global variable, loads multiple longword values from a table into global variables, increments the counter, and performs a conditional check on the counter value.
- `frag_cmpi_beq_bset_bsr_rts_2`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then sets bit 1 of a fixed memory address, then calls another routine (2 times in a row), then returns.
- `process_audio_parameters`: Processes audio parameters by calling another routine multiple times with incrementing parameters, then returns immediately.
- `update_audio_state`: Updates audio playback state based on input conditions, modifying values in memory and handling different cases through conditional branches.
- `frag_lea_move_ori_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to d0, then merges 33280 into d0, then copies d0 to a fixed memory address, then returns.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to a fixed memory address, then returns.
- `manage_sequence_state`: Manages sequence state transitions by reading from a table, performing bitwise operations on the read value, clearing specific memory locations, and branching to different handlers based on the result.
- `initialize_audio_setup`: Initializes audio setup by storing values to global variables, clearing memory locations, performing comparisons with table entries, and updating multiple global variables through subroutine calls.
- `update_sequence_value`: Updates a sequence value by iterating through a table, comparing values, and modifying global variables based on comparisons.
- `store_audio_pointers`: Calculates a memory address using a value from a register, stores that address in a table, and stores another value from a global variable into a second table.
- `initialize_audio_channel`: Initializes an audio channel by copying data from tables, performing arithmetic operations, and setting up various audio parameters in hardware registers.
- `frag_move_bsr_rts_2`: Short code fragment that copies a fixed memory address to a fixed memory address (4 times in a row), then calls another routine (4 times in a row), then returns.
- `update_sequence_counter`: Updates a sequence counter based on conditional logic, modifying values in memory and handling different states through branching instructions.
- `update_sequence_counter_2`: Updates a second sequence counter with similar logic to the previous routine, modifying memory values and handling conditional branches for state management.
- `update_sequence_counter_3`: Updates a third sequence counter following the same pattern as the other counter routines, modifying memory locations and managing state through conditional execution.
- `update_global_counter_2`: Updates a global counter variable with arithmetic operations and bitwise logic, handling different states through conditional branches and setting specific bits in the counter.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `frag_movea_move_subi_jsr_rts_2`: Short code fragment that copies a fixed memory address to a6, then copies the memory at an offset from a0 to d0, then subtracts 1 from d0, then copies the memory at a0 to a1, then calls another routine, then returns.
