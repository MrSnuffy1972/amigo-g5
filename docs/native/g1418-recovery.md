# Game 1418 (empire soccer 94 (1994)(graftgold)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01418). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 40 of 40 named (21 author symbols), 40 of 40 described (2 by rule, 36 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 333 of 333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1034 of 1401 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 44033 |
| code | 4728 |
| data | 412229 |
| padding | 2370 |
| unused | 437760 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Echo | 2 | 117 | 2 |
| Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Empiresoccer | 1 | 417 | 15 |
| Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Galahad | 1 | 18 | 2 |
| Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Fairlight | 1 | 839 | 20 |
| Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- empire soccer 94 (1994)(graftgold)[cr flt]/Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Empiresoccer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_resources` (Empiresoccer): Opens and closes libraries, allocates memory, sets up hardware registers including DMA and interrupts, and performs a series of memory operations and loops before jumping to a routine via a stack-based call. Kind: disk hardware/track check. Pass path: The loop checking `sVar3 != 0` must complete its iterations (iVar6 reaching 0) to avoid the infinite loop, and the subsequent bitwise checks on `u1__sub_01E8` (likely a disk/hardware structure) must satisfy the specific hardware signature r.
- `LBC0001C6` (Fairlight): Saves the system state, replaces two entries in a system variable structure with local handlers, and calls the saved routines. Kind: disk hardware/sync check. Pass path: the instruction following the call (sf.b an address(a5)).

## Functions


### Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 into register d0 and then branches to another routine.

### Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Empiresoccer

- `initialize_system_resources`: Opens and closes libraries, allocates memory, sets up hardware registers including DMA and interrupts, and performs a series of memory operations and loops before jumping to a routine via a stack-based call.
- `set_bitmask_range_bits`: Sets one bit in a global word for each index from a starting value up to a computed end, then returns the input register value.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `fill_zero_data`: Performs four consecutive operations that logically result in no change to register d0.
- `process_data_and_jump`: Writes values to a table, calls a routine to process data, copies a block of bytes, and performs a loop to add values to a table before jumping to an address stored in a memory location.
- `fill_zero_data_long`: Performs twelve consecutive operations that logically result in no change to register d0.
- `build_copper_list_entry_table`: Builds a table of four entries, storing a pointer, an offset and a length for each, then runs a second pass that calls a routine for entries whose flag bit is clear and sets a global pointer.
- `copy_bytes_handling_overlap`: Copies a block of bytes from one address to another, moving forward when the destination is below the source and backward otherwise, so overlapping blocks are handled.
- `unpack_tagged_bitstream_rows`: Checks for a specific magic longword, copies a header block, and unpacks bit-coded runs of bytes into a buffer.
- `decrement_and_shift`: Decrements a byte in a register and, if not zero, performs a logical shift right on another register before branching.
- `search_bit_encoded_table`: Performs a bitwise search through a table using a value derived from a register and a pointer, returning a word from a table based on a calculated index.
- `mask_low_bits_of_value`: Returns the input word masked to the low bits given by a count in a register, using one shifted left by that count minus one as themask.
- `prepare_table_accumulation_parameters`: Sets a register to 9, loads two pointers from the stack frame into registers, calls build_accumulated_limit_table, then sets a register to 4 and loads two different pointers from the stack frame into registers.
- `build_accumulated_limit_table`: Calls another routine to populate a table with values, then calculates accumulated sums and offsets into a second structure using a loop.
- `build_code_limit_table`: Copies a run of words from a routine into a table, then derives accumulated code limits and offsets from a source array and stores them in the structure.

### Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Galahad

- `copy_stub_to_low_memory`: Entry point that copies a block of 32 words from the routine named EMPIRE into low memory at address 224, then returns.
- `EMPIRE`: Writes two values into a table at an offset from a pointer in a0 and then jumps to the address stored in that pointer.

### Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:Fairlight

- `LBC000000`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `SAVE_SYS`: Saves the system variables, replaces two system vector entries with local handler routines, and calls the saved-state routine.
- `GETVBR`: Reads the vector base register through the exec Supervisor call when the CPU-type flag is set, and stores the result at the address held in a register.
- `SUPERVISOR`: Checks a bit in the CIAA_PRA register and, if set, enters a loop that manages text scrolling, building text, and fading text in and out using various subroutines and a wait routine.
- `LBC000078`: Toggles a flag word and, each time it returns to zero, advances a counter and writes a value from a global word into three entries spaced eight words apart.
- `LBC00009A`: Toggles a flag word and, when it reaches zero, decrements a counter and writes a value from a global word into three entries spaced eight words apart, but only while the counter stays non-negative.
- `LBC0000BE`: Calls a routine 32 times, then iterates over a table of 32 words, splitting each word into four-bit nibbles and storing them with their negated values into a destination structure.
- `LBC000118`: Scans the four nibbles of a 16-bit word and keeps the largest one in a register.
- `LBC00012E`: Decrements a counter in a structure and, if zero, iterates through a table to perform bitwise operations and updates a second table based on bitmask comparisons.
- `LBC00017E`: Decrements values in a table by checking specific bit patterns in each word and subtracting 1, 16, or 256 depending on the highest set bit, then repeats this for a specified number of entries.
- `LBC0001C6`: Saves the system state, replaces two entries in a system variable structure with local handlers, and calls the saved routines.
- `LBC0002BE`: Fills four pointer entries from a base address with a fixed stride, each addressing a display plane register, then writes 80 words of increasing values, each followed by a zero word, into a second table.
- `LBC00030A`: Writes 32767 to two offsets of a structure in A6, loads three values from a table into a structure in A2, and writes bytes to two specific memory locations before branching.
- `LBL000352`: Writes the constant 255 to one variable and 32 to a global variable, which the listed facts associate with the interrupt request register.
- `LBC000366`: Writes the constant 255 to a global variable and 8192 to another global variable to configure the CIA interrupt control state.
- `LBL000548`: Clears a memory area and then parses a string to copy font data into a bitmap buffer, using newline and control characters to manage line spacing and positioning.
- `BUILD_TXT`: Clears a bitmap area with zeros, then iterates through a source string to copy character data from a font table into the bitmap, handling line breaks and character offsets.
- `FADE_TEXTIN`: Repeatedly waits for the display frame and stores the returned value at the address in A0 until the value reaches 4095.
- `FADE_TEXTOUT`: Repeatedly waits for the display frame and stores the returned value while it stays at or above 273, then returns the difference from 273.
- `WAIT_TOF`: Clears a frame flag byte and spins until that byte becomes non-zero.

### Empire Soccer 94 (1994)(Graftgold)[cr FLT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
