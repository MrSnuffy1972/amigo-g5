# Game 1034 (stardust (demo) (1993)(bloodhouse)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01034). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 41 of 41 named (0 author symbols), 41 of 41 described (2 by rule, 38 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 37 of 37 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 846 of 846 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 441907 |
| code | 2558 |
| data | 442920 |
| padding | 1447 |
| unused | 12288 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Stardust (demo) (1993)(Bloodhouse).adf:Echo | 2 | 117 | 2 |
| Stardust (demo) (1993)(Bloodhouse).adf:PAL | 1 | 45 | 1 |
| Stardust (demo) (1993)(Bloodhouse).adf:SDPREV | 3 | 333 | 18 |
| Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV | 4 | 333 | 19 |
| Stardust (demo) (1993)(Bloodhouse).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- stardust (demo) (1993)(bloodhouse)/Stardust (demo) (1993)(Bloodhouse).adf:SDPREV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- stardust (demo) (1993)(bloodhouse)/Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_environment` (SDPREV): Calls several library routines to check a structure, sets a global pointer to a specific routine, clears four hardware registers with the value 32767, and either jumps to a routine or calls two library functions before returning. Kind: disk hardware/integrity check. Pass path: The branch following the call that continues normal system initialization/loading..
- `setup_and_jump_to_handler` (TUNNELPREV): Performs library calls to inspect a structure, stores a specific routine address into a global variable, writes 32767 to four hardware registers, and jumps to a routine or calls two library functions. Kind: disk hardware/track validation routine. Pass path: The branch taken when the hardware check (ADKCON/disk controller interaction) succeeds, typically leading to the main game/application data loading..

## Functions


### Stardust (demo) (1993)(Bloodhouse).adf:Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another location.

### Stardust (demo) (1993)(Bloodhouse).adf:PAL

- `check_ecs_hardware`: Opens the dos and graphics libraries and checks the system chip configuration.

### Stardust (demo) (1993)(Bloodhouse).adf:SDPREV

- `initialize_system_environment`: Calls several library routines to check a structure, sets a global pointer to a specific routine, clears four hardware registers with the value 32767, and either jumps to a routine or calls two library functions before returning.
- `copy_memory_block`: Compares two addresses and, if the first is not greater than the second, copies a byte from the first to the second and branches based on a comparison with a constant.
- `copy_and_trap`: Copies a byte from a memory location to a destination address and calls a trap if the source address is not zero.
- `initialize_system_tables`: Sets the processor status register, calculates several memory addresses, calls routines to populate tables, and finally writes a value to a hardware register before performing an indirect jump.
- `apply_amiga_hunk_relocations_2`: Iterates through a data structure to copy memory blocks and perform relocations by adding a base address to specific offsets if the hunk type matches AmigaDOS relocation codes.
- `process_amiga_hunk_data`: Iterates through a data structure to copy memory blocks and perform relocations by adding values to specific offsets if the hunk type matches certain AmigaDOS constants.
- `process_hunk_data`: Iterates through a data structure, copying blocks of memory and performing additions to specific offsets if a value matches a specific constant, then continues based on a loop counter in d2.
- `fill_zero_data`: Performs four logical OR operations on d0 with zero.
- `initialize_and_populate_data_tables`: Clears a sixteen-word table at an offset from a pointer in a6, then calls several routines to populate various tables.
- `decrement_counter_and_shift_register`: Decrements a 16-bit value in a register, and if the result is not zero, performs a logical shift right on another register before branching to another routine; otherwise, it branches to a different location.
- `fetch_interpolated_value`: Performs bitwise manipulations on d6 and d0, then enters a loop that compares a value in d1 against a value from a table to return a word from a table at an offset determined by a calculation.
- `process_bit_rotation`: Performs a loop that rotates d6 and d7 based on d1, pulls a word from a table pointed to by a0, and returns a bitwise AND of that word with d0.
- `fill_tables_with_processed_data`: Calls another routine to process data, then uses two loops to store the resulting words into two different destination tables, updating a pointer and a counter in each loop.
- `fill_accumulated_code_tables`: Sets the first table entry to zero, then runs fifteen steps that accumulate counts from a source array into running sums.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_dma_tables`: Populates a series of memory locations with incrementing values derived from d0, then writes a pointer to a table into a hardware register and sets specific values in other hardware registers.
- `trigger_system_trap`: Loads a constant into a register and calls a trap.
- `transfer_handler_data`: Checks a bit in a global variable, clears the status register, copies a longword from a routine address to a destination, and performs an indirect jump if a comparison succeeds.

### Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV

- `setup_and_jump_to_handler`: Performs library calls to inspect a structure, stores a specific routine address into a global variable, writes 32767 to four hardware registers, and jumps to a routine or calls two library functions.
- `copy_memory_block_2`: Copies bytes from a source address in a global variable to a destination address in another global variable until the source address reaches a specific limit, then returns.
- `copy_byte_and_trap`: Copies a byte from a memory location to a destination address calculated by adding an offset to a value from a PC-relative table, then calls a system trap after writing a value to a global variable.
- `setup_memory_tables`: Sets the processor status register, calculates memory addresses, calls routines to clear and populate several tables, and performs an indirect jump.
- `apply_amiga_hunk_relocations_3`: Iterates through a list of memory hunks to copy data blocks and apply additive relocations to specific memory locations based on hunk type codes.
- `process_amiga_hunk_relocations`: Iterates through a list of memory hunks, copying data blocks and applying additive relocations to specific memory locations based on hunk type codes.
- `apply_amiga_hunk_relocations`: Iterates through a list of data structures, copying blocks of memory to a destination and performing additive relocations if a specific hunk type is encountered.
- `clears_data_register_zero`: Performs four consecutive logical OR operations with zero on data register zero.
- `populate_data_structures`: Calculates several memory addresses from a source structure, clears a 16-word table, and performs multiple calls to populate various tables and decode symbol offsets using loops.
- `decrement_and_shift_loop_counter`: Clears register d1, decrements a 16-bit counter in d7, and if the counter is not zero, shifts register d6 left by one bit and branches back to the start of the routine; otherwise it returns.
- `search_bitstream_table`: Performs a bit-by-bit search through a stream of data pointed to by a register, using a loop that runs up to sixteen times.
- `fetch_table_entry_with_bitmask`: Calculates an index using bitwise shifts and arithmetic on registers, then performs a bitwise AND between a value from a memory table and a word from a pointer in a0 before returning.
- `populate_dual_tables_from_lookup`: Calls a lookup routine to fetch values into two separate destination tables, using a calculated sum of the first set of fetched values to determine the iteration count for the second set.
- `accumulate_symbol_decoder_offsets`: Clears a 16-bit value at the start of a destination table and then executes a loop fifteen times that reads values from a source array to calculate and store running sums into two separate tables at the current destination pointer.
- `zero_d0_byte`: Performs a logical OR operation on d0 with zero.
- `configure_dma_registers`: Stores the high and low words of d0 into two consecutive memory locations, then writes a pointer to a table into a hardware register, clears another hardware register, and sets a third hardware register to a constant value.
- `clear_memory_block`: Clears sixteen consecutive 16-bit words in a memory block starting at a location derived from the program counter using a loop of clear and increment instructions.
- `trigger_system_trap_2`: Loads the address of a local constant into a register, moves it to a specific memory location, and executes a system trap.
- `copy_single_word_and_jump`: Checks a bit in a global structure, disables interrupts, and copies a single longword from a source address to a destination address before jumping to a location stored at that destination address.

### Stardust (demo) (1993)(Bloodhouse).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
