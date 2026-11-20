# Game 3921 (HighSteel v1.0 1397): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03921). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (918924 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (5 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 45984 bytes, 18 functions |
| Independent decoder cross-check (Ghidra against capstone) | 944 of 1182 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 309005 |
| code | 4028 |
| data | 605886 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 6 | 2 |
| opal9 | 1 | 2 | 1 |
| mainprog (raw code file, load address unknown) | 1 | 1174 | 18 |

## Cases handled

- HighSteel_v1.0_1397/HighSteel_v1.0_1397.hdf: hard-disk image (AmigaDOS OFS, 1088 blocks): its filesystem parses and all 53 files in it are byte-identical to the extracted tree HighSteel_v1.0_1397_hdf, which is recovered file by file; no block outside the filesystem holds data
- HighSteel_v1.0_1397/HighSteel_v1.0_1397_hdf/HighSteel/data/mainprog: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `empty_routine`: the routine performs no observable work and ends without modifying registers or memory.

### opal9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### mainprog (raw code file, load address unknown)

- `initialize_system`: Initializes system hardware by clearing memory locations, setting interrupt and DMA control registers, and configuring custom hardware registers.
- `handle_input_and_update`: Handles input from hardware registers, processes bit operations, updates a global variable, and calls another routine to update an input buffer.
- `frag_btst_beq_not_and_bra`: Short code fragment that tests bit 7 of d0, then branches if the last result was zero, then inverts d3, then masks d4 with d3, then jumps to another point in the code.
- `frag_or_bclr_move_movem_rte`: Short code fragment that merges d3 into d4, then clears bit 6 of the memory at an offset from a0, then copies d4 to a fixed memory address, then restores saved registers from the stack, then copies 8 to the memory at an offset from a6, then returns from the exception.
- `update_state_counter`: Updates a global state counter and a circular buffer index, then clears a specific bit in a hardware register.
- `process_velocity_values`: Processes velocity values by reading from memory locations, performing arithmetic operations, clamping results between -128 and 127, and storing the result back to memory.
- `initialize_global_data`: Initializes global data structures by clearing memory locations and setting a word value from another location.
- `setup_display_registers`: Configures display-related hardware registers with predefined values and calls another routine to initialize rendering parameters.
- `generate_and_store_lookup_table`: Generates lookup table values by performing arithmetic operations on data from memory locations, stores results in memory, and copies the data to another location.
- `copy_lookup_data`: Copies 16 words from a source location to a destination in memory when a specific byte matches a given value, otherwise returns immediately.
- `setup_render_parameters`: Calculates and sets up rendering parameters including coordinates, clears hardware registers, and initializes multiple data structures with specific values.
- `calculate_and_store_coordinates`: Calculates coordinate values based on input parameters, stores them in memory, and sets up rendering data for multiple iterations.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (lea.l, adda.w, ror.w) and then falls through into the code that follows, with no return of its own.
- `configure_rendering_context_2`: Configures rendering context by setting up hardware registers, initializing multiple data structures with specific values, and performing loop operations for rendering setup.
- `configure_rendering_context`: Configures rendering context by setting up pointers, clearing hardware registers, and initializing multiple data structures with specific values.
- `setup_rendering_parameters`: Sets up rendering parameters by configuring pointers, clearing hardware registers, and initializing multiple data structures with specific values.
- `initialize_rendering_params`: Sets up rendering parameters by writing specific values to hardware register locations and storing a pointer in a global variable.
- `configure_rendering_context_2_2`: Initializes rendering context by setting up hardware registers and data structures with specific values, performing loop iterations to configure multiple entries.
