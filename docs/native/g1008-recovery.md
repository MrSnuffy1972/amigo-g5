# Game 1008 (turbo trax (1995)(arcane entertainment)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01008). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2707184 top-level bytes in 149 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 28 of 28 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 360 of 360 named (0 author symbols), 360 of 360 described (17 by rule, 339 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 648 of 648 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13558 of 126101 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 568171 |
| code | 24582 |
| data | 2041591 |
| padding | 26248 |
| unused | 46592 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Echo | 2 | 117 | 2 |
| Copy | 2 | 1671 | 5 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe | 2 | 267 | 8 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad | 2 | 267 | 8 |
| Logo.PAK#unpacked | 1 | 180 | 11 |
| Shopper.PAK#unpacked | 1 | 1544 | 29 |
| DiskSwop.PAK#unpacked | 1 | 114 | 5 |
| TrackSel.PAK#unpacked | 1 | 15807 | 47 |
| Stat.PAK#unpacked | 1 | 2212 | 23 |
| Options.PAK#unpacked | 1 | 33537 | 104 |
| assign | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| Skip | 2 | 117 | 2 |
| Execute | 1 | 888 | 12 |
| Copy | 2 | 1671 | 5 |
| MakeDir | 1 | 112 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 2 | 404 | 8 |
| LinkUp.PAK#unpacked | 1 | 15009 | 25 |
| ppmore | 2 | 233 | 8 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| Copy | 2 | 1671 | 5 |
| EndPart.PAK#unpacked | 1 | 49285 | 21 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 503 | 19 |

## Cases handled

- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy5.PAK: BK: ByteKiller-packed, unpacked with ancient (22952 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy3.PAK: BK: ByteKiller-packed, unpacked with ancient (22248 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy1.PAK: BK: ByteKiller-packed, unpacked with ancient (29316 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/2indycarSP.PAK: BK: ByteKiller-packed, unpacked with ancient (24636 -> 34816 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/2indyblocks.PAK: BK: ByteKiller-packed, unpacked with ancient (112420 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy4.PAK: BK: ByteKiller-packed, unpacked with ancient (24636 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/3indycarBB.PAK: BK: ByteKiller-packed, unpacked with ancient (30928 -> 40960 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy2.PAK: BK: ByteKiller-packed, unpacked with ancient (20324 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Speech.PAK: BK: ByteKiller-packed, unpacked with ancient (26456 -> 30592 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Logo.PAK: BK: ByteKiller-packed, unpacked with ancient (25152 -> 65544 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Shopper.PAK: BK: ByteKiller-packed, unpacked with ancient (60988 -> 83232 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/DiskSwop.PAK: BK: ByteKiller-packed, unpacked with ancient (8456 -> 52260 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/TrackSel.PAK: BK: ByteKiller-packed, unpacked with ancient (43660 -> 151444 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Stat.PAK: BK: ByteKiller-packed, unpacked with ancient (35980 -> 85132 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Options.PAK: BK: ByteKiller-packed, unpacked with ancient (66356 -> 229484 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:pak/LinkUp.PAK: BK: ByteKiller-packed, unpacked with ancient (19968 -> 97264 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/2woodblocks.PAK: BK: ByteKiller-packed, unpacked with ancient (123052 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood4.PAK: BK: ByteKiller-packed, unpacked with ancient (16696 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood2.PAK: BK: ByteKiller-packed, unpacked with ancient (23272 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/3jeepBB.PAK: BK: ByteKiller-packed, unpacked with ancient (59460 -> 81920 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood5.PAK: BK: ByteKiller-packed, unpacked with ancient (16856 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/2jeepSP.PAK: BK: ByteKiller-packed, unpacked with ancient (49304 -> 69632 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood3.PAK: BK: ByteKiller-packed, unpacked with ancient (21108 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood1.PAK: BK: ByteKiller-packed, unpacked with ancient (19312 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow4.PAK: BK: ByteKiller-packed, unpacked with ancient (20320 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/34x4BB.PAK: BK: ByteKiller-packed, unpacked with ancient (61252 -> 81920 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow3.PAK: BK: ByteKiller-packed, unpacked with ancient (18000 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow1.PAK: BK: ByteKiller-packed, unpacked with ancient (25324 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/2snowblocks.PAK: BK: ByteKiller-packed, unpacked with ancient (105096 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow5.PAK: BK: ByteKiller-packed, unpacked with ancient (18920 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/24x4SP.PAK: BK: ByteKiller-packed, unpacked with ancient (51016 -> 69632 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow2.PAK: BK: ByteKiller-packed, unpacked with ancient (17508 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand5.PAK: BK: ByteKiller-packed, unpacked with ancient (22508 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand3.PAK: BK: ByteKiller-packed, unpacked with ancient (27804 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand1.PAK: BK: ByteKiller-packed, unpacked with ancient (23956 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/2dunebuggySP.PAK: BK: ByteKiller-packed, unpacked with ancient (22808 -> 34816 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/2sandblocks.PAK: BK: ByteKiller-packed, unpacked with ancient (109744 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand4.PAK: BK: ByteKiller-packed, unpacked with ancient (17552 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/3dunebuggyBB.PAK: BK: ByteKiller-packed, unpacked with ancient (27588 -> 40960 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand2.PAK: BK: ByteKiller-packed, unpacked with ancient (21752 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city5.PAK: BK: ByteKiller-packed, unpacked with ancient (11096 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city3.PAK: BK: ByteKiller-packed, unpacked with ancient (14452 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/3sportsBB.PAK: BK: ByteKiller-packed, unpacked with ancient (26760 -> 40960 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city2.PAK: BK: ByteKiller-packed, unpacked with ancient (12600 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city4.PAK: BK: ByteKiller-packed, unpacked with ancient (12296 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/2sportsSP.PAK: BK: ByteKiller-packed, unpacked with ancient (22592 -> 34816 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city1.PAK: BK: ByteKiller-packed, unpacked with ancient (13796 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/2cityblocks.PAK: BK: ByteKiller-packed, unpacked with ancient (97984 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:pak/EndPart.PAK: BK: ByteKiller-packed, unpacked with ancient (28780 -> 274704 bytes)
- runtime segment an address: 244 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:c/ppmore: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_interrupts` (runtime segment an address): Configures CIA interrupt control, sets up DMA and custom hardware registers, and populates a table with pointers and constant values. Kind: disk hardware/interrupt configuration for protection. Pass path: the code continues to the next instruction after the bsr.w an address call.

## Functions


### Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies the contents of D1 into D0 and then performs a short branch.

### Copy

- `copy_files_to_destination`: Iterates through a list of files and copies them to a specified directory.
- `call_two_callbacks_with_flag`: If D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `call_function_table`: The routine receives a count in D0 and a pointer to a table in A0.
- `check_bitmask_range`: The routine tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning a flag indicating the result.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe

- `process_amiga_hunk_relocations`: Parses a data stream containing AmigaDOS hunk types to perform memory allocation, data relocation, and bitstream decompression.
- `decrement_and_loop`: Decrements a 16-bit value in D1 and loops until the value reaches zero, otherwise it returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_bitstream_data_2`: Processes a bitstream by reading a value from a pointer in a0, calculating an offset, and performing a loop to copy or move bytes into a destination buffer pointed to by a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `decompress_bitstream_data`: Performs bitwise rotations on a value retrieved from a table and bitwise-ORs the result into a destination memory range.
- `execute_trap_instruction`: Moves a 16-bit word from an offset of 3085 in A2 to D4, performs a bitwise OR on D1 with 112, calls a trap, and performs a bitwise OR on D1 with the result of the trap.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad

- `parse_amiga_hunk_data_2`: Parses a data stream containing AmigaDOS hunk type codes to perform memory allocation and relocation.
- `traverse_linked_list`: Decrements a counter in a loop and performs pointer arithmetic on a linked list, calling two different routines and writing a value to a global variable based on a pointer comparison.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_variable_length_bitstream`: Reads a value from a pointer in a0, calculates a destination address and a count based on that value, and then enters a loop to write bytes to memory at a3.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_register_to_register`: Subtracts the value of d4 from d0.
- `decompress_bitstream_data_3`: Processes a bitstream by reading a longword from a pointer in a0, performing bitwise rotations and shifts, and writing the resulting bits into a destination buffer.
- `execute_trap_instruction_2`: Moves a word from an offset of a2 into d4, performs a bitwise OR on d0 with a constant, calls a trap, and performs a bitwise OR on d1 with d0.

### Logo.PAK#unpacked

- `jump_to_address_2`: Performs a jump to an address stored in a longword at offset 6.
- `execute_conditional_subroutine_sequence`: Clears a word in a global variable and executes a sequence of three subroutine calls, where each call is preceded by loading a value into a register and followed by a check of the global variable to determine if further subroutine calls should be executed.
- `wait_for_input_sequence`: Polls a hardware register in a loop until specific bit patterns are detected, then modifies a global variable.
- `adjust_table_nibbles`: Iterates through two tables and modifies values in the second table by applying incremental adjustments based on differences found in the high nibble, middle nibble, and low nibble of the corresponding word pairs.
- `populate_pointer_table_2`: Fills a table with pointers to a global variable at fixed intervals and waits for a specific bit pattern in a memory location.
- `loop_subroutine_calls`: Repeatedly calls another routine until a specific condition is met.
- `populate_pointer_table`: Populates a table with 4 entries by storing a pointer and its high word into a global variable structure, incrementing the source pointer by 5120 and the destination pointer by 8 for each iteration.
- `initialize_pointer_table`: Populates a table with memory addresses and sets two global word values before entering a loop of subroutine calls.
- `loop_input_sequence_processing`: Repeatedly executes a sequence of input polling and data nibble adjustment routines until a specific condition is met.
- `fill_address_list`: Writes a series of memory addresses into a table with a specific stride and updates two global variables with constant values.
- `process_input_nibble_sequence`: Repeatedly calls wait_for_input_sequence and adjust_table_nibbles using two different sets of parameters until the value in register d1 is zero.

### Shopper.PAK#unpacked

- `jump_to_external_routine`: Performs a long jump to an external address.
- `initialize_and_store_registers`: Increments several register values, stores them into global memory locations, and performs a sequence of subroutine calls.
- `clears_data_register_zero`: Performs four consecutive logical OR operations on register d0 with a zero constant.
- `rotate_global_bit`: Performs a bitwise rotation on the complement of a byte read from a hardware register and writes several bytes to hardware registers.
- `initialize_state_structure`: Sets several word and long values in a structure and assigns a function pointer based on a register value.
- `clears_data_register_d0`: Performs four consecutive logical OR operations on register d0 using a zero byte constant.
- `update_input_state_2`: Reads a word from a hardware register and performs bitwise operations to update bytes and shorts within a structure, potentially incrementing a counter in the structure if a calculated value matches a stored value.
- `initialize_memory_tables_2`: Populates a table with incrementing long values split into word components and clears a subsequent block of memory.
- `decompress_data_block`: Iterates through a table to perform bit-shifting and extraction to populate a destination buffer from a source table.
- `branch_to_offset`: Loads a longword from a global variable into a register, adds a constant to a pointer, and branches to another routine.
- `process_pixel_data`: Performs a complex loop involving division and bitwise operations to write processed data into a memory buffer.
- `wait_for_status_and_check_flag`: Polls a memory location until a specific bitmask is matched, then executes a series of subroutines and checks a global flag and a character value.
- `copy_memory_block`: Copies thirty-two bytes from a source pointer to a destination pointer using a loop.
- `configure_render_parameters`: Populates a structure with several constants, a pointer to a global variable, and an integer value.
- `execute_repeated_routines`: Calls another routine multiple times in two separate loops based on values retrieved from a global table.
- `initialize_hardware_structure`: Initializes a large memory structure with specific constants and pointers, including setting up two source and destination addresses.
- `check_status_and_increment`: Uses values from a table to load a pointer, then checks a global variable and a value in a memory location, potentially resetting or incrementing a value at a specific offset.
- `setup_bitstream_render`: Initializes several global variables and a structure with specific values and a function pointer.
- `finalize_bitstream_render`: Writes several values to a structure using global variables and then clears those globals.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `performs_logical_or_on_data_register`: Performs a logical OR operation on the least significant byte of a data register using a zero constant.
- `populate_data_structure_via_loop`: Checks a bit in a structure at offset 2, then performs a loop five times to write various 16-bit and 32-bit values to a structure at offset 40, including a calculated value based on a register and a bit-shifted value at offset 58.
- `null_data_padding`: This routine consists of a sequence of instructions that perform no functional operations and serves as a data table of zeroed bytes.
- `initialize_data_structure`: Sets several fields in a memory block and executes a loop to populate a table using bitwise operations and an incrementing pointer.
- `update_state_logic`: Modifies a global index based on comparisons between two values and increments a value stored in a table.
- `wait_for_bit_pattern_2`: Enters a loop that polls a specific offset within a memory block until two different bit patterns are detected sequentially.
- `adjust_table_values_3`: Iterates through two tables and modifies the values in the second table by adding or subtracting increments based on differences in nibbles and bits.
- `execute_repetitive_subroutines`: Repeatedly calls two specific subroutines within a loop until a condition based on a register value is met.
- `execute_repeated_subroutines`: Repeatedly calls two separate subroutines in a loop until a specific byte value in a register becomes zero.

### DiskSwop.PAK#unpacked

- `jump_to_entry_point`: Performs an unconditional jump to a location specified by a longword at a fixed offset from the current instruction pointer.
- `store_and_call_routines`: Stores a word from d0 into a global variable and calls two other routines.
- `adjust_table_values_4`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `initialize_data_tables`: Iterates through nested loops to populate multiple memory tables with specific byte patterns and then stores a pointer to a global variable into a structure.
- `execute_loop_routine`: Repeatedly calls another routine until a specific status flag is cleared.

### TrackSel.PAK#unpacked

- `transfer_and_rotate_global_values`: Stores values from registers into global variables, calls store_and_rotate_values, writes two 16-bit values to a structure at offsets 1168 and 1172 using a pointer in a5, and returns a 16-bit value.
- `contains_null_data_table`: This routine consists of a sequence of instructions that perform no functional operations, as they consist of zeroing data registers with immediate values.
- `store_zero_bytes_to_data_register`: Stores zero values into a data register using a sequence of instructions that are interpreted as data.
- `store_zero_bytes_to_data_registers`: Performs a sequence of operations that load zero into various data registers using immediate values.
- `store_zero_bytes_to_a_table`: Stores a sequence of zero bytes into a data table using a series of instructions that appear to be data interpreted as code.
- `store_zero_bytes_to_data_registers_2`: Stores zero values into data registers using immediate operand instructions.
- `contains_null_data_table_2`: This routine consists of a sequence of instructions that perform no functional operations, as they consist of zero-valued data words interpreted as instructions.
- `store_zeroed_data_words`: Stores a sequence of zeroed 16-bit words into a data memory area.
- `store_zero_bytes_to_a_table_2`: Stores zero bytes into a sequence of memory locations using a series of immediate operations that appear as instructions.
- `store_zeroed_bytes_in_a_table`: Stores zeroed bytes into a data table using a series of instructions that appear as zeroed data words.
- `store_zero_bytes_in_a_table`: Stores a sequence of zero bytes into a data table using the ORI instruction with a zero immediate value.
- `execute_repeated_calls_2`: Calls another routine a fixed number of times in succession before returning.
- `insert_string_constants`: Writes five word constants into a buffer at increments of two words and calls another routine between each write.
- `format_string_buffer`: Copies a string into a buffer and then calls another routine multiple times to insert specific characters at fixed positions.
- `convert_nibble_to_ascii`: Splits a byte from d0 into two nibbles and stores them as ASCII characters in the memory location pointed to by a3.
- `store_and_rotate_values`: Stores several values into global variables and then copies a subset of those values to a different memory location before returning a value.
- `performs_nothing`: performs an or operation with zero on d0.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `zero_d0_byte`: Performs a logical OR operation on the least significant byte of register d0 with zero.
- `rotate_global_bit_2`: Performs a bitwise rotation on the complement of a byte from the keyboard serial register, writes several bytes of 255 to the keyboard serial register and a global variable, and writes 0 to a global variable.
- `initialize_track_state_structure`: Sets several word and long values in a structure and assigns a function pointer based on a register value.
- `empty_data_block`: Contains four zero words that perform logical OR operations on register d0.
- `clear_and_populate_tables`: Zeros out a block of memory, populates a table with incrementing values, and waits for a hardware register bit pattern before storing a pointer to a global variable.
- `process_data_structure`: Waits for a hardware register condition, toggles a byte, and then copies and transforms data from a source structure into a destination table.
- `expand_bit_packed_data`: Iterates through a source buffer to expand bit-packed byte values into a larger destination table.
- `empty_data_block_2`: Contains a series of zero words that perform logical OR operations on register d0.
- `update_status_counters`: Updates a structure based on bitwise operations and hardware register values, incrementing specific counters or resetting them based on comparison results.
- `update_status_counters_2`: Updates bytes and shorts in a structure based on the value of a global variable, including incrementing or decrementing counters with wrap-around logic.
- `adjust_table_values_5`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `execute_recursive_calls`: Performs a sequence of five consecutive recursive calls to itself.
- `fill_memory_blocks`: Fills multiple memory blocks with the value 255 using a nested loop structure.
- `copy_expanded_data`: Uses an index from a global variable to locate a source pointer and copies four longwords from that location to a destination buffer.
- `empty_data_block_3`: Contains five zero words that perform logical OR operations on register d0.
- `perform_recursive_calls`: This routine executes a single recursive call to itself followed by five subsequent calls to itself before returning.
- `execute_recursive_call`: Calculates several addresses based on a value in d0 and calls another routine.
- `initialize_data_structures_5`: Populates a series of memory offsets with specific constant values and pointers within a loop.
- `load_table_entry`: Retrieves a value from a table based on an offset and uses it to configure a memory structure.
- `populate_data_structure_2`: Writes several constant values and a calculated offset into a memory structure, then stores a calculated function pointer at a specific offset within that structure.
- `wait_for_register_toggle`: Loops until a hardware register toggles between two specific bit patterns a set number of times.
- `loop_subroutine_calls_2`: Calls adjust_table_values_5 twice with a count of 2, then calls wait_for_register_toggle with a count of 32 using two tables as arguments, repeating this sequence until the return value in register d1 is zero.
- `loop_subroutine_calls_3`: Calls adjust_table_values_5 twice with different parameters and then calls wait_for_register_toggle, repeating this sequence until the return value in d1 is zero.
- `pack_status_word`: Calculates a byte by adding a constant to an input byte and checks for overflow and specific bit states.
- `store_routine_results`: Calls another routine eight times and stores the returned values into a table.
- `store_routine_results_2`: Executes another routine eight times and stores the returned values into a table, then performs a swap of two global variables.
- `run_double_calls`: Executes another routine twice in a loop until a register is zero and returns a value.
- `copy_table_entry`: Calculates an offset into a table using a byte from a register and copies sixteen bytes from that location to a destination.
- `execute_complex_polling_loop`: Executes a series of calls and enters a loop that checks a byte at offset 1166 of a structure in a5 and 1167 of a5, performing further calls and checking a bit in a hardware register.

### Stat.PAK#unpacked

- `parse_string_input`: Reads characters from a sequence of subroutine calls and writes them into a buffer pointed to by a global variable, handling numeric digits, specific character mappings from a table, and space characters.
- `parse_ascii_input_stream`: Processes a stream of input bytes to construct an ASCII string in a memory buffer.
- `empty_data_block_4`: Contains a single zero word that performs a logical OR operation on register d0.
- `perform_zero_operation`: Performs a bitwise OR operation with zero on register d0.
- `zero_d0_register`: Performs a logical OR operation on the data register zero with zero.
- `rotate_complement_byte`: Reads a byte from a CIAA serial register, performs a bitwise NOT and a one-bit left rotation, writes 97 to a CIAA control register, writes 255 to the CIAA serial register nine times, writes 0 to a CIAA control register, and returns.
- `populate_and_mask_data`: Populates a table with pointers to a global variable and performs complex bitwise masking and data streaming operations using a source table.
- `copy_data_block_2`: Copies multiple blocks of data from a source table to a global variable using nested loops.
- `copy_data_conditionally`: Copies data from a source table to a global variable, using a conditional branch to determine the copy method.
- `update_memory_pattern`: Increments a global variable and periodically writes a specific sequence of bytes to a calculated memory location.
- `copy_memory_block_2`: Copies a block of thirty-two bytes from a source pointer in a0 to a destination starting at a global variable in a1.
- `poll_sequential_bit_patterns`: Enters a loop that polls a specific offset within a memory block until two different bit patterns are detected sequentially, repeating this process for a fixed number of iterations determined by a value in register d7.
- `search_data_table`: Iterates through a structured data list to find a matching byte within a specific range and calculates a value based on table offsets.
- `parse_data_structure`: Iterates through a data stream to find specific byte markers and updates a global variable based on the results.
- `calculate_offset_data`: Calculates several values based on global variables and offsets, then writes these values into a structure, potentially within a loop.
- `calculate_and_store_offsets`: Performs arithmetic on input values to calculate offsets and stores the results into a structure.
- `process_data_loop`: Increments a 16-bit value at an offset calculated from a global variable and a counter, then enters a loop that processes bytes from a pointer and calls another routine.
- `process_data_stream`: Iterates through a data stream, loading bytes into global variables and performing arithmetic operations based on the results of a subroutine call.
- `call_adjust_nibbles`: Calls another routine to adjust nibble values in a table.
- `wait_and_adjust_nibbles`: Waits for a bit pattern in memory and then calls a routine to adjust nibble values.
- `perform_zero_operation_2`: Performs a bitwise OR operation with zero on register d0 sixteen times.
- `setup_adjust_table_values_6`: Loads two table addresses into registers a0 and a1, sets a word value of sixteen in register d0, and calls adjust_table_values_6.
- `adjust_table_values_6`: Iterates through two tables and modifies the values in the second table by adding or subtracting increments based on differences in nibbles and bits.

### Options.PAK#unpacked

- `initialize_and_loop_options_structure`: Stores two pointers into global variables, calls another routine, and enters a loop that repeatedly calls another routine with different values at offset 6 of a structure, updating offsets 0, 4, 6, and 18 based on the results of those calls and the values read from the structure.
- `perform_zero_operation_3`: Performs a bitwise OR operation with zero on register d0 many times.
- `store_zero_data_table`: Contains a sequence of zero words that act as a data table.
- `store_zeroed_data_table`: Writes a sequence of zeroed bytes into a data block starting at the routine's address.
- `clear_memory_and_call`: Fills a memory range with zeros and executes a sequence of function calls through pointers if certain conditions are met.
- `store_zero_data_table_2`: Contains eleven zero words that act as a data table.
- `execute_conditional_pointers`: Checks a bit in a global structure and, if set, executes two functions via pointers stored within that structure.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_function`: Checks a value in a structure and, if non-zero, calls a function pointer stored in a global table.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `close_library_function`: Checks a value in a structure and, if non-zero, calls a function pointer from a global table to close a resource.
- `poll_linked_list_nodes`: Iterates through a linked list by checking a pointer at a specific offset within a structure.
- `append_node_to_linked_list`: Initializes a new three-word structure using values from registers and appends it to the end of a linked list pointed to by a structure at offset 40 of a5.
- `store_pointer_to_structure`: Stores the contents of a register into a structure at offset 40.
- `remove_linked_list_node`: Searches a linked list for a matching value and removes that node from the list.
- `allocate_and_insert_node`: Calls a memory allocation routine via a pointer and, if successful, calls another routine.
- `free_memory_on_condition`: Calls another routine and, if the result is non-zero, calls a memory free routine via a pointer.
- `assign_allocated_pointer`: Calls a memory allocation routine via a pointer and stores the resulting pointer into a location pointed to by a register.
- `clear_and_call_function`: Clears two words at a memory location pointed to by a register, calls another routine, and then calls a function pointer from a global variable if the previous routine returned a non-zero value.
- `store_function_result`: Calls a function pointer from a table and stores the returned value into a memory location offset from a register.
- `zero_and_call_function`: Stores zero into a memory location offset from a register, then calls a function pointer from a table.
- `conditionally_call_function`: Checks if three different values are non-zero and then calls a function pointer from a table.
- `conditionally_call_function_v2`: Checks if two values are non-zero and then calls a function pointer from a table.
- `execute_conditional_sequence`: Calls two function pointers from a table based on the return value of a first function call, and returns a value that may be modified by the results.
- `allocate_and_initialize_structure`: Clears a block of memory in a loop, calls a subroutine to allocate memory, and then conditionally populates a structure with the result.
- `update_structure_values`: Writes a longword and a word to a structure pointed to by a5, calls another routine, and then either calls another routine or sets d0 to a constant.
- `execute_conditional_updates`: Copies a value from a structure into a register, calls two routines, and if a comparison fails, adds a constant to a register and returns.
- `execute_conditional_sequence_2`: Clears a memory range using a loop, calls a routine, and if the result is non-negative, updates a structure and calls two more routines.
- `compare_and_execute_subroutines`: Loads a pointer and a longword from a structure in a4 into a1 and d0, calls two subroutines, and then calls another routine if the longword in d0 does not match the value at offset 4 of the structure.
- `verify_double_function_call`: Calls a function and, if the result is non-zero, calls two more functions and compares the second result against a register value before returning a concatenated result.
- `iterate_string_and_call`: Initializes a global variable via a function pointer if it is zero, iterates through a null-terminated string, and then executes a function pointer.
- `execute_conditional_loop_2`: Stores two pointers into global variables, performs a series of subroutine calls with varying parameters, and updates specific offsets within a structure based on a loop condition.
- `store_zero_data_table_3`: Contains a single zero word that acts as a data table.
- `initialize_memory_blocks`: Zeroes out a block of memory, populates an integer table with incrementing values, fills a short array with incrementing values, and updates several pointers and status registers.
- `apply_relative_offsets_to_multiple_lists`: Calls another routine five times, each time passing a different address in a register.
- `add_base_address_to_table_2`: Iterates through a list of values in memory, adding the starting address of the list to each individual element until a negative value is encountered.
- `apply_relative_offsets_to_two_lists`: Calls another routine twice, each time passing a different address in a register.
- `apply_relative_offsets_2`: Iterates through a list of shorts and adds the base address of the list to specific longword entries at fixed intervals of six bytes.
- `process_structured_data`: Waits for a specific bitmask in a register, toggles a byte, and then populates a series of memory locations using values from a source structure.
- `execute_multiple_subroutines`: Calls four different routines in a repeating sequence three times.
- `fill_memory_with_ones_2`: Fills sixteen bytes of memory starting from a pointer derived from a global structure with the value negative one.
- `decrement_and_recurse`: Calls itself recursively and decrements a word value at a specific offset until it becomes negative or a byte in a table reaches zero.
- `initialize_data_structures_3`: Sets several pointers and zero-initializes two distinct blocks of memory using a loop.
- `update_state_and_clear_buffer`: Selects a source value based on a flag, stores it and two pointers into a table, and clears a memory block through a pointer.
- `update_sprite_attributes`: Performs bitwise rotations on a value and writes the result into a memory block via a pointer.
- `update_sprite_attribute_list`: Calculates and writes new coordinate and attribute data into a structure, likely for sprite or object management.
- `render_bitplane_data`: Iterates through a structured data table to extract bitmask values and write them to a memory buffer.
- `process_bitfield_logic`: Evaluates bits within a global variable to calculate a value based on specific bit combinations, updates a global counter if the result matches a stored value, and writes the constant 1000 to an offset of 16 from a pointer held in a register.
- `read_joystick_input`: Reads the state of the joystick port and CIAA fire buttons to update internal status variables.
- `update_input_state`: Copies a byte from one memory location to another and retrieves a word from a table based on a masked value.
- `adjust_table_values_7`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 253 into d0, then merges 4 into (a0)+.
- `invoke_structured_data_processing`: Calls a routine that processes structured data based on bitmask validation and memory population, then pushes a longword from offset 1e of a structure in a6 onto the stack.
- `check_table_value`: Calls two routines, accesses a pointer from a global table, calls a third routine, and then compares a word from that pointer to a value in a5.
- `perform_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `increment_table_index`: Calls two routines, accesses a pointer from a global table, calls a third routine, and then increments a word at that pointer before comparing it to a constant.
- `compare_and_return_status`: Calls three routines, accesses a pointer from a global table, calls a fourth routine, and then compares a word from that pointer to a value in a5.
- `decrement_word_at_pointer`: Subtracts four from a 16-bit word at the address held in a pointer.
- `verify_table_limit`: Calls three routines, accesses a pointer from a global table, calls a fourth routine, and then compares a word at that pointer to a constant.
- `increment_word_at_pointer`: Adds four to a 16-bit word at the address held in a pointer.
- `update_attribute_tables`: Iterates through nested tables to perform bitwise rotations and updates memory blocks with calculated values based on pointer offsets and register contents.
- `update_counter_logic`: Calls a routine, accesses a pointer from a global table, modifies a word at an offset of that pointer based on a value in a5, and calls another routine or pushes a value to the stack.
- `restore_registers`: Restores the contents of registers d0 through d7 and a0 through a6 from the stack and returns.
- `check_and_call_handler`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine with a0 pointing to a table at 7fe, a1 pointing to a table at 16 from a4, and d0 set to 32.
- `check_and_call_handler_2`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine with a0 pointing to a table at 7be, a1 pointing to a table at 16 from a4, and d0 set to 32.
- `check_and_call_twice`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine twice with a0 and a1 pointing to specific tables and d0 set to 32.
- `check_and_call_twice_2`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine twice with a0 pointing to a table at 7be and a1 pointing to a table at 73a, and d0 set to 32.
- `apply_conditional_writes`: Checks a byte at an offset from a register, and if it is non-zero, compares a word from a structure to zero and performs writes to a structure if the comparison matches.
- `set_conditional_values_2`: Compares a byte in d0 to 1; if it is not equal, it returns, otherwise it writes 1 to a word at offset 18 from a5 and stores a byte from a0 at offset 24 from a0.
- `set_conditional_values`: If a byte passed in a register equals two, it writes three to a word at an offset from a pointer and writes a byte to another offset from a pointer.
- `process_conditional_byte`: Checks a byte at offset 9 of a structure in a5; if it is not zero, it compares a word from offset 4 of a structure in a0 to zero, and if it matches, stores d0 at offset 25 of a0, updates a global pointer, and calls another routine.
- `update_byte_on_match_2`: Compares a byte in d0 to one, and if it matches, stores d0 at offset 25 of a structure in a0, updates a global pointer, calls another routine, and stores the byte at offset 25 of a0 again.
- `update_byte_on_match`: Compares a byte in d0 to two, and if it matches, stores d0 at offset 25 of a structure in a0, updates a global pointer, calls another routine, and stores the byte at offset 25 of a0 again.
- `set_byte_on_match`: If a byte passed in a register equals three, it writes a byte to a specific offset from a pointer.
- `calculate_coordinate_differences`: Calculates absolute differences between two sets of values stored in global tables, then stores the results in several other global variables.
- `compare_and_add_values`: Loads two words from a table, compares them to two other words, and if they match, adds a word from a third table to d0 if certain conditions are met.
- `perform_subtraction_and_branch`: Adds a word from a table at 378a4 to d1, subtracts a word from a table at 3789e from d3, and branches to another routine.
- `branch_if_non_zero`: Tests a 16-bit word in d2 and, if it is not zero, adds a longword from a table at offset 378a4 to d1 before branching to another location; otherwise it returns.
- `reset_option_settings`: Adds a word from a table at 378a2 to d0, adds a word from a table at 378a0 to d3, stores d3 in a global variable, and copies d0 and d1 to a global table within a loop.
- `zero_data_entry`: Contains a single instruction that performs a logical OR with zero on a data register.
- `zero_d0_register_2`: Performs a logical OR operation on register d0 with zero.
- `clear_register_d0`: Performs a bitwise OR on register d0 with zero.
- `or_byte_zero_to_d0`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `clears_data_register_zero_2`: Performs a logical OR operation on the least significant byte of data register zero using a zero constant.
- `poll_and_clear_globals`: Clears several global variables and enters nested loops that poll other routines until specific global values match or a byte is set to a specific value.
- `store_global_pointer`: Stores a longword from a global variable into a memory location at an offset of 10 from a pointer retrieved from a structure at offset 20 of another global pointer, then calls another routine.
- `poll_and_increment_buffer`: Polls a memory location for specific bit patterns in a loop and increments two counters.
- `copy_memory_block_3`: Copies a block of data from one memory location to another using a loop.
- `poll_and_increment_offset`: Iteratively checks a memory location for a bitmask and updates a counter and an offset value.
- `clear_memory_block_2`: Fills two memory blocks with zeros using a loop.
- `swap_and_poll_buffer`: Clears a byte at offset 24 of a table, enters a loop that swaps a value between two global variables, and continues as long as a byte in a table remains zero.
- `update_conditional_word_2`: Tests a byte at an offset from a pointer, and if it is zero, writes a word to another offset of that pointer and a byte to a location from a register.
- `update_conditional_word`: Compares a byte in d0 to 1 and, if it is not equal, returns; otherwise it writes the word 2 to an offset of c from a5 and stores a byte from a0 at offset 24 of a0.
- `update_data_structure_3`: Compares a byte in d0 to 2; if it is not equal, it returns, otherwise it writes 3 to a word at an offset from a5 and stores a byte from a0 at offset 24 from a0.
- `update_data_structure`: If a byte in a register equals three, it writes a word to one offset and a byte to another offset in memory.
- `update_data_structure_conditional`: Checks if a byte at offset 9 from a5 is non-zero, then checks if a word at offset 4 from a0 is zero; if both are true, it writes 2 to a word at offset 14 from a5 and stores a byte from a0 at offset 24 from a0.
- `update_data_structure_2`: If a byte in a register equals one, it writes a word to one offset and a byte to another offset in memory.
- `verify_conditional_logic`: Checks a byte at offset 9 of a structure in a5, and if it is zero, it sets a byte at offset 24 of a structure in a0, performs several calls, and updates a pointer at offset 0 of a structure in a1.
- `verify_double_function_call_2`: Checks if a byte in a register is one, stores a byte at an offset from a pointer, and calls two subroutines with a value from a register and a constant.
- `set_byte_value`: If a byte in a register equals two, it writes a byte to a specific offset in memory.
- `clear_data_d0_register`: Performs a logical OR operation on the least significant byte of the d0 register with zero.
- `wait_for_vblank`: Polls the vertical position register for two specific scanline values within a loop, then writes two values to the DMA control and interrupt enable registers after checking a bit in the status register.
- `wait_for_vblank_and_set_registers`: Polls the vertical position register for specific values through a loop, then modifies two words in a global structure by applying bitwise masks and setting specific bits, and finally checks a bit in a hardware register before completing the second modification.

### assign

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header of sequential words into a buffer.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.

### Echo

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, then writes a header containing sequential words into a buffer.
- `transfer_register_and_branch_2`: Moves the contents of register d1 into register d0 and then performs a relative branch to another routine.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `transfer_register_and_branch_3`: Moves the contents of data register 1 into data register 0 and branches to another routine.

### Execute

- `initialize_command_execution_environment`: Allocates and initializes a data structure for command processing by populating it with function pointers and clearing internal buffers.
- `process_command_state_machine`: Evaluates a command structure by executing various function pointers based on a status code.
- `process_input_buffer`: Iterates through a buffer of input characters, comparing them against specific boundary values and updating a data structure.
- `validate_single_character_input`: Checks if a string length is exactly one; if so, it retrieves a byte from a table at offset 120 of a structure and returns it, otherwise it performs a series of function calls and returns a specific error code.
- `check_single_char_string_length`: Calculates the length of a null-terminated string and, if the length is exactly one, returns a byte from a table at offset 120 of a structure passed via a pointer in a3.
- `execute_callback_pair`: Extracts a pointer from offset 4 of a structure, calls a routine at an offset within that pointer, and then calls a second routine using the original structure and a pointer located at offset 16.
- `execute_callback_pair_2`: Extracts a pointer from offset 4 of a structure and calls several different routines through pointers found at various offsets within that pointer, then calls another routine using a value from a register and a pointer from a structure offset.
- `execute_offset_callbacks`: Calls two different routines using pointers located at specific offsets within a provided structure.
- `store_registers_to_pointer`: Copies the contents of D1 through D7, A1 through A6, and the value from the stack into successive entries of the pointer passed in A0, then returns.
- `normalize_param_and_copy_word`: Ensures the second argument is non-zero and then copies the first word of the structure pointed to by the first argument into a word located at an offset within that structure, returning a combined value formed from the adjusted second argument and a word from the structure.
- `normalize_signed_operands`: Negates the values in D0 and D1 if they are negative to ensure they are positive before calling another routine, then restores the original sign of the result based on the input signs.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend in D0 by a 32-bit divisor in D1 using a normalized fixed-point method, returning a 64-bit result with the quotient in the high 32 bits and the remainder in the low 32 bits.

### Copy

- `perform_file_copy_operations`: Iterates through a list of files identified by a pattern, checks for their existence, and attempts to copy each one to a target directory.
- `execute_conditional_library_calls`: If the value in d0 is non-zero, it calls a routine via a pointer at offset -126 of the library base in a1, then calls a routine at offset -828 of the same library base.
- `call_function_table_2`: Receives a count in D0 and a pointer to a table in A0, then for each iteration calls a function from the table at offset -948 of the library base using a pointer derived from the table and an incrementing index.
- `check_bitmask_range_2`: Tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning one if the value is within the range and the bitmask check passes, otherwise it returns zero.
- `compare_strings_case_insensitive_2`: Compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal, and returns the signed difference between the first non-matching characters, or zero if the strings are equal up to the null terminator.

### MakeDir

- `set_error_and_cleanup`: Initializes an error code and, if a library base is present, it clears a small stack buffer, opens a DOS library context, and iterates over a null-terminated list of strings to attempt an operation, updating an error flag and status code before closing the context and releasing resources.

### EndIf

- `clear_d0_register`: Clears the D0 register and returns.

### If

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_d1_to_d0`: Moves the value from D1 into D0 and then branches to another routine.

### IconX

- `run_command_with_template_options`: Executes a command using a template of options by iterating through a list of arguments, performing various library operations including string copying, memory allocation, and window management, and handles errors or delays based on the command's execution status.
- `call_library_entry_point`: Calls one Amiga library function through the library base passed as an argument, then returns.
- `build_linked_entry_list`: Allocates a memory block through an Amiga library call, stores a count derived from register D0 in it, and walks a linked list of entries, allocating a node for each entry with a non-zero field and copying a value into it.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `copy_c_string`: Advances to the terminating zero of the string at A0, then copies the bytes from A1 onto its end, including the zero.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.

### LinkUp.PAK#unpacked

- `jump_to_address_3`: Performs a long jump to a specific address.
- `update_status_flags_and_store_registers`: Sets a byte in a global variable to 255 if a byte in a data register is 0, sets another byte in a global variable to 255 if the register byte is 1, and writes several 16-bit values from data registers to a series of global variables before calling another routine twice.
- `write_constants_to_memory`: Sets a global byte to 255 if a byte in a data register is 0, sets another global byte to 255 if that same register contains 1, writes three 16-bit values from data registers to global memory offsets, and calls another routine twice.
- `initialize_global_state`: Sets several global variables based on the value of a byte in a register and stores multiple values into a global memory range.
- `initialize_memory_tables`: Fills a global memory range with zeros, populates a table with incrementing integer values, and fills another structure with incrementing short values.
- `add_base_addresses`: Calls a routine twice to add the contents of two different memory addresses to a table.
- `add_base_address_to_table`: Iterates through a list of values in memory, adding the starting address of the list to each individual element.
- `apply_relative_offsets_3`: Calls a routine to add the address at a specific memory location to a list of values.
- `apply_relative_offsets`: Iterates through a list of shorts and adds the base address of the list to specific longword entries at fixed intervals.
- `process_data_table`: Waits for a specific bitmask in a memory location, toggles a byte, and then populates a table with values derived from a source structure.
- `initialize_data_structures_4`: Stores a memory address into a location at offset 0 of a structure and calls four different routines three times each.
- `fill_memory_with_ones`: Fills a block of eight longwords with all bits set to one.
- `initialize_data_structures`: Clears several memory blocks and sets up multiple pointers and counters within a structure.
- `update_state_pointers`: Updates a set of pointers and clears a block of memory based on a conditional check of a byte value.
- `rotate_and_merge_data`: Performs bitwise rotations on a value retrieved from a table and bitwise-ORs the result into a destination memory range.
- `update_structure_list_data`: Calculates updated values for a series of fields within a structure based on arithmetic operations on input registers and table lookups.
- `decode_compressed_sprite_data`: Iterates through a structured table to extract and decompress bitstream data into a destination buffer.
- `adjust_table_values`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `execute_repeated_calls`: Decrements a value provided in a register and executes another routine repeatedly until the counter reaches zero.
- `execute_conditional_loop`: Repeatedly calls another routine twice per iteration until a specific byte value becomes zero.
- `invoke_nibble_adjustment`: This routine acts as a wrapper that calls a procedure to modify nibble bits within a second array based on comparisons with a first array.
- `toggle_and_loop_global`: Performs a series of subroutine calls and decrements a local counter in a loop, bitwise inverting a global variable when the counter reaches zero.
- `process_table_data`: Retrieves a value from a table using an offset and calls another routine with parameters derived from a pointer.
- `loop_and_modify_pointers`: Executes a loop that decrements a counter and modifies two global pointers by fixed offsets while calling several subroutines.
- `parse_and_store_data`: Converts character data into numeric values, stores them in a table, and performs a loop to process data based on the resulting values.

### ppmore

- `parse_amiga_hunk_data`: Parses a data structure containing AmigaDOS hunk types, performing memory relocations and copying data blocks based on hunk codes.
- `decrement_counter_loop`: Decrements a 16-bit counter in a register and loops until it reaches zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `expand_compressed_bitstream`: Reads a length value from a pointer in a0, calculates a destination offset, and enters a loop that extracts bit-packed data into a buffer pointed to by a3.
- `check_shifted_flag_3`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word`: Subtracts the value in register d4 from the value in register d0.
- `extract_bitstream_and_shift`: Extracts bits from a source pointed to by a0 and shifts them into a destination register using a bit count in d4, looping d0 times and updating a flag based on the extracted bits.
- `execute_trap_and_or`: Moves a word from an offset of a2 into d4, calls a trap, and performs a logical OR of d2 into d0.

### Echo

- `find_max_and_prepare_buffer_6`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Moves the value from register d1 into register d0 and branches to another location.

### Lab

- `clear_d0_and_return`: Returns immediately after clearing register d0.

### Copy

- `iterate_and_copy_files`: Iterates through a file list obtained from a directory, performing file operations including opening, reading, and writing data to a destination.
- `call_two_callbacks_with_flag_2`: If the value in D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `execute_function_table_entries`: Iterates a specified number of times, calling a function from a table located at a fixed offset within the library base for each entry.
- `validate_range_and_bitmask`: Checks if the value in register d0 is between 200 and 231 inclusive, and verifies that a specific bit within a bitmask is set based on the offset from 200.
- `compare_strings_case_insensitive_3`: Compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal.

### EndPart.PAK#unpacked

- `jump_to_address`: Performs a long jump to an address stored in a register.
- `execute_subroutines`: Calls two subroutines and then returns.
- `wait_and_assign_pointer`: Loops until a specific bit pattern is detected in a memory location, then writes a constant word to a structure and assigns a global variable address to another structure.
- `poll_hardware_bit`: Calls three other routines and then loops until bit 7 of the CIAA_PRA register is cleared.
- `populate_data_structure`: Waits for a specific bit pattern in a value at an offset of a structure in A6, then swaps two longwords in a table and fills a second table with incrementing values split into high and low words.
- `populate_calculated_table`: Fills a table with values derived from a combination of loop counters and results from another routine.
- `find_minimum_bit_index`: Iterates through a list of words to find the lowest bit index set in each word and tracks the minimum index found across the entire list.
- `update_global_counter`: Increments a global variable, performs conditional checks, and returns a value derived from a calculation involving that variable.
- `tiny_ori_2`: Small routine of 1 instruction that merges 10 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d0.
- `clear_memory_block`: Calculates a memory address by adding a value from a global variable to a base address stored in another global variable, then clears a large contiguous range of memory starting at an offset from that address using multiple block moves.
- `copy_data_block`: Copies twenty-two words from a calculated memory location to another location.
- `initialize_structure_fields`: Writes several constant values, pointers, and calculated offsets into a memory structure.
- `sum_table_values`: Iterates through a data stream to extract values and accumulate their sum into a local variable.
- `populate_data_list`: Repeatedly writes a series of pointers and associated values into a structure at a calculated offset.
- `loop_and_store_word`: Repeatedly calls another routine and stores a word from a structure into a register until a condition is met.
- `wait_for_bit_pattern`: Loops until a specific bit pattern is detected within a word at a memory location.
- `adjust_table_values_2`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `save_sys`: Saves the system variables, replaces two system vector entries with local handler routines, and calls the saved-state routine.
- `get_vbr`: Reads the vector base register through the exec Supervisor call when the CPU-type flag is set, and stores the result at the address held in a register.
- `execute_frame_logic`: Checks a bit in CIAA_PRA, modifies a structure in a5, performs several loops involving subroutine calls and data manipulation, and finally restores the status register and returns from interrupt.
- `toggle_and_increment_field`: Toggles bit 0 of a word at offset 14 of a structure, and if the result is zero, increments a value at offset 18 and writes a constant to three consecutive words starting at offset 168 of the stack.
- `toggle_and_decrement_field`: Toggles bit 0 of a word at offset 16 of a structure, and if the result is zero and the value at offset 18 is non-negative, decrements that value and writes a constant to three consecutive words starting at offset 168 of the stack.
- `lbc0000_be`: Calls a routine 32 times, then iterates over a table of 32 words, splitting each word into four-bit nibbles and storing them with their negated values into a destination structure.
- `find_max_nibble`: Scans the four nibbles of a 16-bit word and keeps the largest one in a register.
- `update_nested_tables`: Decrements a value at offset 12 of a structure, and if it reaches zero, sets it to four and performs a nested loop to update values in a table based on bitwise comparisons.
- `decrement_table_values`: Decrements values in a table by checking specific bit patterns in each word and subtracting 1, 16, or 256 depending on the highest set bit, then repeats this for a specified number of entries.
- `configure_hardware_and_interrupts`: Configures CIA interrupt control, sets up DMA and custom hardware registers, and populates a table with pointers and constant values.
- `initialize_data_structures_2`: Populates several memory areas with structured data including incrementing word values and specific constants, then stores the resulting pointers in a structure at offset 128 of a6.
- `set_hardware_registers`: Writes values to specific memory locations and performs a branch to another routine.
- `write_interrupt_register`: Writes the constant 32 to the interrupt request register and returns.
- `configure_cia_interrupt_control`: Writes the constant 255 to a global variable and 8192 to another global variable to configure the CIA interrupt control state.
- `parse_input_string`: Processes a buffer of characters by iterating through lines and copying data from a lookup table into a destination buffer based on character values.
- `store_modified_values`: Calls another routine three times, stores the result in a table, and then enters a loop that adds a constant to a value from a stack-stored word until a comparison fails.
- `store_subtracted_values`: Calls another routine three times, stores the result in a table, and then enters a loop that subtracts a constant from a value from a stack-stored word until a comparison fails.
- `wait_for_frame_flag`: Spins in a loop until bit 0 of a byte at offset 20 of a5 becomes non-zero.
