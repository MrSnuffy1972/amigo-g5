# Game 1175 (FinalFight v1.2 1329): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01175). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2655869 top-level bytes in 134 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 67 raw-word lines |
| Functions named and described | pass, 277 of 277 named (0 author symbols), 277 of 277 described (11 by rule, 264 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2953 of 2957 (99.86%) |
| Recovered without load address (counted in proof 1, flagged) | 66820 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7566 of 7568 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 89745 |
| code | 25372 |
| data | 2540733 |
| padding | 19 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 278 | 27 |
| final | 2 | 230 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 62 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 57 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6776 | 221 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| level1.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| level2.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| level3.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| level5.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| level6.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- FinalFight_v1.2_1329/FinalFight_v1.2_1329.hdf: hard-disk image (AmigaDOS OFS, 2944 blocks): its filesystem parses and all 77 files in it are byte-identical to the extracted tree FinalFight_v1.2_1329_hdf, which is recovered file by file; no block outside the filesystem holds data
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/axl.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17196 -> 33600 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/axl2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17036 -> 33600 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/bigboss.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (18680 -> 34928 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/bill.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (21856 -> 38870 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/bill2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (20468 -> 38870 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/bred.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (15676 -> 28422 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/bred2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (14988 -> 28678 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/bred3.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (16008 -> 28550 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/burntguy.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (2744 -> 4876 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/chandelier.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (4688 -> 6908 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/chars.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (7732 -> 9390 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/cody.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (37776 -> 68972 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/cop.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (26668 -> 41304 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/dontusedrugs.iff: high-entropy data file (7.68 bits/byte), classified as packed data
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/fflogo.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (9972 -> 13506 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/flames.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (3880 -> 4870 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/giant.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17324 -> 37092 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/giant2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17812 -> 34890 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/glass.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6644 -> 12550 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/guy.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (23204 -> 46450 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/guys.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (13244 -> 17766 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/guys_ring.iff: high-entropy data file (7.90 bits/byte), classified as packed data
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/gymnast.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (18364 -> 40350 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/haggar.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (23696 -> 49310 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/haggarpics.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/heads.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (7696 -> 8626 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/j.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (12192 -> 22100 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/j2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (12440 -> 22236 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/knifeguy.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17744 -> 33564 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/knifeguy2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (16616 -> 33564 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/l8mask.bin: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (2240 -> 6400 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level1.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (5328 -> 6660 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level1.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (31740 -> 61184 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level2.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (8188 -> 10756 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level2.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (28592 -> 61440 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level3.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6768 -> 8708 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level3.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (36556 -> 58368 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level4.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6472 -> 7684 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level4.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (27376 -> 46848 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level5.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (12040 -> 18436 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level5.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (33660 -> 54912 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level6.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (15000 -> 22260 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level6.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (30996 -> 57472 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level7.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (16804 -> 32128 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level8.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6660 -> 16000 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/maptrain.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (13760 -> 17062 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/mod.mit: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (132768 -> 180488 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/rasta.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (24080 -> 39182 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/swordsman.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (19276 -> 34380 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/townmap.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (10168 -> 13694 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/trainstrap.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (2564 -> 2672 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/tv.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (24152 -> 31964 bytes)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/weapons.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (4380 -> 6646 bytes)
- 10 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (f150): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 29 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2491 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 108 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level1.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level2.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level3.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level5.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/level6.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- FinalFight_v1.2_1329/FinalFight_v1.2_1329_hdf/FinalFight/FinalFightData/final: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 5 never-loaded region(s), 66820 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `hash_and_load_resource_table` (runtime segment an address): Computes a checksum of a string to generate a hash and searches for a matching entry in a data block. Kind: resource table hashing and disk hardware check. Pass path: DAT_00001af8 = 0; return 0;.
- `validate_and_load_disk_data` (runtime segment an address): Verifies the integrity of a data block using a checksum and marker validation before updating a global table with the processed data. Kind: disk track/sector validation and checksum check. Pass path: d0 == 0.
- `update_conditional_pointer` (runtime segment an address): Checks a bit in a byte and, if set, calls a function via a pointer to update a global variable with a value from the stack. Kind: disk hardware/track check. Pass path: the branch following the call where the condition is met (e.g., the bne.b or beq.b logic depending on the specific flag state).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `process_amiga_hunk_relocations`: Parses a data structure containing AmigaDOS hunk type codes to perform memory relocation and address patching.
- `return_constant_value`: Returns a constant longword value.
- `initialize_system_parameters`: Sets several global values, updates a memory location via a pointer loop, and copies a block of data.
- `modify_memory_via_loop`: Writes values to custom hardware registers and performs a loop to add a constant to a memory location pointed to by a table, followed by a memory copy loop.
- `copy_memory_and_register_function`: Copies a block of memory from a source to a destination using a loop.
- `copy_memory_block_3`: Copies a block of data from one memory location to another using a loop and sets two global variables.
- `initialize_function_pointer_table`: Sets multiple global function pointers to a specific routine and clears a block of memory using a loop.
- `initialize_jump_table_and_clear_memory`: Initializes several global variables and a table with specific values and then clears a large block of memory by iterating through it in four-byte increments.
- `clear_memory_block`: Clears a four-byte word and a two-byte word at an offset from a pointer provided in a register.
- `return_register_value`: Returns the value held in a data register.
- `return_uninitialized_register`: This routine is empty and returns the current value held in a data register.
- `return_uninitialized_register_2`: The routine returns the current value held in a data register without performing any operations.
- `return_register_value_2`: This routine is empty and simply returns the current value held in a data register.
- `return_uninitialized_register_3`: This routine returns the current value of a data register without performing any operations.
- `return_uninitialized_register_4`: This routine returns the current value held in a data register without performing any operations.
- `return_uninitialized_register_5`: The routine returns the current value of a data register without performing any operations.
- `traverse_linked_list`: Iterates through a linked list of structures to find the minimum value in a specific field while performing bitwise operations on a byte.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `find_minimum_value_2`: Iterates through a null-terminated list of pointers to find the smallest positive unsigned short value stored at a specific offset within each element.
- `find_minimum_value`: Iterates through a null-terminated list of pointers to find the smallest positive unsigned short value stored at a specific offset within each pointed-to structure.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `return_uninitialized_register_6`: returns the current value held in a data register.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_3`: Performs a return from a subroutine.
- `return_register_value_3`: This routine returns the current value held in a data register to the caller.
- `update_global_variable`: calls a function via a pointer in a table and adds the returned value to a global variable before resetting that global variable to zero.

### final

- `parse_amigados_hunks`: Parses a memory block containing AmigaDOS hunk types to perform relocation and memory management.
- `decrement_register_to_zero`: Decrements a 16-bit register value until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `rotate_data_blocks`: Performs bitwise rotations and shifts on data while moving blocks of memory based on a loop counter and bit flags.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_rotation`: Performs a series of bitwise shifts and rotations on data based on a variable number of bits specified by a mask.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_amiga_hunk_list`: Iterates through a list of memory hunks to perform relocation and memory allocation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_memory_table`: Adds a constant value to a series of integer elements in a table pointed to by a list of pointers.
- `store_register_to_pointers`: Stores a byte from a data register into a memory location pointed to by one pointer, and clears a second memory location pointed to by another pointer.
- `update_global_counters`: Increments values in a list of memory locations and copies a small block of code to a destination.
- `copy_and_clear_memory`: Copies a block of memory using a direction-optimized loop and then fills a large memory range with zeros.
- `clear_memory_block_2`: copies a block of memory using a loop and then clears a large range of memory by writing zeros to it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_byte`: Loads a value from a global variable and subsequently stores the value of all ones into that same global variable.
- `modify_global_variable`: Performs a bitwise OR operation on a global variable with itself shifted left by four bits, then clears the global variable.
- `write_word_to_pointer`: Loads a value from a global variable and writes a specific word to the memory location pointed to by that value.
- `store_word_at_pointer`: Writes a specific word to the memory location stored in a global variable.
- `update_blitter_sprite_tables`: Iterates through a nested table structure to copy data from a source buffer into a destination memory area.
- `assign_constant_value`: Stores a specific word value into a global variable.
- `parse_sprite_attribute_table`: Iterates through a data table to extract sprite attributes and updates a target structure with coordinate and attribute data.
- `update_display_parameters_2`: Updates multiple fields in a structure using values from a global table and a pointer, including several repetitive writes of constant values to specific offsets.
- `update_display_parameters`: Updates a series of memory locations with values derived from a global table and a pointer.
- `reset_global_flag`: Checks the value of a global variable and resets it to zero or one based on specific comparison conditions.
- `calculate_display_coordinates`: Calculates two coordinate values based on input registers and a global offset.
- `process_bitfield_data`: Manipulates bits from input registers to write specific values into a structure based on conditional checks and a loop.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `copy_memory_blocks`: Performs a nested loop to copy data from a source table to a destination buffer.
- `initialize_structure_fields_2`: Waits for a specific bit in a structure to clear before writing several constant values to various offsets within that structure.
- `initialize_display_structure`: Sets specific constant values in a structure and then calls another routine three times.
- `store_register_values_2`: Writes constant values to a structure, calls another routine three times, and then stores two 32-bit values from registers into the structure.
- `store_register_values`: Stores two 32-bit values from registers into a structure and writes a constant value to a third offset within that structure.
- `initialize_and_fill_structure_buffer`: Waits for a specific bit flag in a structure to clear before initializing several fields and performing a bulk memory fill of a buffer using a repeating sequence of values from a local stack or register set.
- `fill_memory_with_pattern`: This routine waits for a specific bit flag in a structure to clear before initializing several fields in that structure.
- `initialize_data_structure`: Clears several fields in a structure using zero and sets four specific fields to a constant value of 300.
- `update_bitmask_from_hardware`: Performs a bitwise AND operation on two values and a constant, stores the result in two locations, and updates a global variable by ORing it with a shifted version of the result.
- `process_attribute_table`: Iterates through a table pointed to by a global variable, performing complex calculations to update structure members, bitmasks, and pointers based on values found in the table.
- `set_structure_parameters`: Waits for a specific bit in a structure to clear, then populates several fields in that structure with specific constants and register values.
- `update_global_byte`: Checks a bit in a byte and updates a global variable accordingly.
- `do_nothing`: This routine is empty and performs no operations.
- `initialize_data_structures`: Writes several constant values to offsets within a memory block pointed to by a register.
- `configure_hardware_parameters`: Waits for a bit in a structure to clear before writing a large constant, a small constant, and two other values into the structure.
- `initialize_hardware_and_tables`: Writes values to various hardware registers and populates a memory table with a specific address through a loop.
- `tiny_ori`: Small routine of 1 instruction that merges 252 into d7.
- `fill_coordinate_table`: Populates a table with a repeating pattern of incrementing values and specific constants using a loop.
- `fill_table_with_offset`: Writes a sequence of values into a table where the second and fourth words of each entry are derived from an input value that increments by 44 each iteration.
- `initialize_zero_table`: Fills a table with alternating values, where one word is an incrementing constant and the next word is zero.
- `clamp_table_values`: Iterates through a table and updates its contents by combining nibbles from a source table, ensuring the resulting values are clamped within specific ranges.
- `initialize_runtime_data_tables`: Populates several memory tables with interleaved incrementing values, constants, and data from a source table.
- `populate_data_table`: Iterates through a loop to interleave values from a source table into a destination table using a specific incrementing pattern.
- `configure_hardware_registers`: Writes specific values to several hardware registers, including CIA and interrupt control registers, and a memory location.
- `wait_for_keyboard_bit`: Sets a global variable and waits in a loop until a specific bit is set in a value read from a hardware register.
- `modify_bit_in_byte`: Sets or clears a specific bit within a byte at a memory offset calculated from a provided value.
- `clear_bit_in_byte`: Clears a specific bit within a byte at a given memory offset using a bitmask derived from a register.
- `update_global_from_table`: Compares a global variable against a value in a table and updates the global variable accordingly.
- `reset_global_on_match`: Compares a byte at a calculated memory offset with a global variable and sets that global variable to zero if they match.
- `call_subroutine_and_store`: Calls another routine and then stores a constant value into a global variable.
- `clear_and_initialize_tables`: Clears a large block of memory by zeroing out consecutive longwords and words, then populates a table with pointers to itself.
- `fill_memory_block`: Fills a block of memory with a repeating sequence of values derived from registers and a pointer.
- `zero_memory_block_2`: Clears a specified block of memory by writing zero values to consecutive longwords and words.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `copy_null_terminated_strings`: Iterates through a list of pointers, treating them as pairs, and copies null-terminated strings from one memory location to another.
- `execute_recursive_calls`: Performs two consecutive calls to another routine.
- `execute_callback_chain`: Checks a global pointer and recursively calls the function it points to if it is not zero.
- `initialize_local_variables`: Assigns several constant values to a global variable and specific memory locations.
- `initialize_structure_fields`: Checks if a value in a table is non-negative and, if so, resets several fields within that structure and a separate memory location.
- `process_entity_collision_and_movement`: Iterates through a list of entities to check for spatial overlaps and collision flags.
- `update_entity_timer_and_state`: Decrements a timer field within a structure and resets it to a default value if it reaches zero or becomes negative.
- `process_entity_list`: Iterates through a list of structures to perform complex spatial and attribute comparisons between elements.
- `process_complex_list`: Iterates through a list of structures, performing various comparisons and arithmetic operations on their members.
- `reset_structure_data`: Clears several fields within a structure and resets a bit in a byte at a specific offset.
- `search_list_elements_2`: Iterates through a list of data to find an entry that matches specific bitmask, range, and pointer-based conditions.
- `search_list_elements`: Iterates through a list of data to find elements that satisfy specific range and bitmask conditions.
- `clear_conditional_table_entries`: Iterates through a table and sets an entry to zero if it is non-zero and a specific bit in a corresponding offset is set.
- `clear_memory_table`: Fills a series of memory locations with zeros using a fixed increment.
- `zero_memory_block`: Fills a large block of memory with zeros by iterating through a table in increments.
- `read_global_variable`: Performs redundant reads of a global variable.
- `sort_and_process_table`: Filters a table based on range and bitwise criteria, sorts the resulting entries using a bubble sort, and performs memory operations on the sorted data.
- `preserve_and_clear_variable`: Saves a global variable, calls another routine to potentially update a memory location, and then restores the original variable value.
- `set_structure_values`: Assigns zero and a constant value to specific offsets within a structure while preserving one existing value.
- `update_object_kinematics_and_bounds`: Calculates an object's new position and velocity by applying directional modifiers and decrementing internal counters.
- `update_object_properties`: Modifies an object's position, velocity, and attribute flags using values from a table and a global structure.
- `reset_entity_data`: Clears several fields in a structure pointed to by a global offset and copies values from another structure into it.
- `clear_structure_bit`: Calls another routine and then clears a specific bit in a byte at a given offset within a structure.
- `populate_data_table_2`: Iterates through a loop to copy values between two pointers and updates specific offsets in a structure with calculated values.
- `process_data_loop`: Iterates through a loop to update specific offsets in a structure using values from other routines and performs a multi-word copy between memory locations.
- `update_structure_fields`: Clears specific fields within a structure and updates a value at a fixed offset based on a global variable and a bitwise calculation.
- `nop_padding_sequence`: This routine is empty and consists of instructions that perform no operation.
- `parse_data_table`: Iterates through a table of 16-bit values, executing indirect calls for negative values and updating a structure with data from the table.
- `search_table_elements`: Iterates through a table of pointers to check specific bit patterns and values, performing a bitwise calculation if certain conditions are met.
- `search_nested_data_tables`: Iterates through a series of nested tables by performing indexed lookups and searching for specific values.
- `update_status_flags`: Iterates through a list of pointers to clear a specific bit in a byte at an offset from each pointer, then iterates through another list to set a bit based on a condition.
- `parse_data_stream`: Iterates through a data stream to execute conditional logic, performing indirect calls via a jump table or updating a structure with values extracted from the stream.
- `process_data_stream`: Traverses a data stream to perform conditional jumps, calls other routines, and writes values from the stream into a structure.
- `process_data_table_entries`: Iterates through a table of 16-bit values, performing indirect calls when a negative value is encountered.
- `parse_data_table_conditional`: Iterates through a table of 16-bit values and performs updates or indirect calls only if a specific global variable is less than a value from the table.
- `initialize_complex_state`: Sets various global variables to constant values, clears memory, and performs a series of data structure initializations and table parsing.
- `count_matching_elements`: Iterates through a table of data and increments a counter if a specific byte at an offset within the table entry matches a specific value.
- `update_object_position`: Calculates and updates the spatial coordinates and movement vectors of an object structure.
- `update_object_position_and_state`: Updates an object's coordinate position by adding velocity components and handles boundary checks against screen limits.
- `update_object_position_and_bounds`: Updates an object's coordinate offsets and bounding box dimensions based on velocity and direction.
- `update_object_position_and_bounds_2`: Updates an object's coordinate offsets and checks for boundary collisions.
- `update_object_position_and_timers`: Updates an object's coordinate position and internal timers by applying velocity and decrementing frame counters.
- `update_object_position_and_state_2`: Calculates new coordinate values by applying velocity and direction modifiers to an object's position.
- `update_object_position_and_bounds_3`: Updates an object's coordinate offsets and velocity by incrementing or decrementing values based on direction flags.
- `update_object_state_and_position`: Updates an object's animation frame, decrements internal timers, and modifies its coordinate values based on velocity and direction flags.
- `update_object_animation_and_velocity_2`: Updates an object's internal animation frame counter and handles sprite index lookups from a table.
- `update_object_physics_and_bounds`: Updates an object's velocity, position, and animation frame counters while performing boundary collision checks.
- `update_object_position_and_bounds_4`: Updates an object's coordinate and velocity values while performing boundary checks against a defined area.
- `update_object_physics_and_bounds_2`: Updates an object's position and velocity by applying movement increments and performing boundary checks.
- `update_object_movement_and_counters`: Decrements internal frame and timer counters while updating an object's position based on velocity values.
- `update_object_spatial_bounds`: Updates an object's position and velocity by decrementing counters and applying movement increments.
- `update_object_state_and_position_2`: Updates an object's animation frame, decrements internal timers, and modifies its coordinate and velocity values based on direction flags.
- `update_object_position_and_state_3`: Calculates an object's new coordinate position by adding velocity components to its current location and performs boundary checks against a global limit.
- `update_object_physics_and_state`: Decrements internal counters and updates an object's position by applying velocity values.
- `update_object_physics_and_bounds_3`: Updates an object's position and velocity while performing boundary checks and decrementing internal timers.
- `update_object_position_and_state_4`: Calculates an object's new coordinate by adding velocity to its current position and performs boundary checks against a global limit.
- `update_object_position_and_bounds_5`: Updates an object's coordinate position by adding velocity values and performs boundary checks against screen or container limits.
- `update_object_position_and_state_5`: Calculates a new position for an object by adding velocity components to its current coordinates and performs boundary checks.
- `update_object_position_and_bounds_6`: Updates an object's coordinate and velocity values while performing boundary checks against a global limit.
- `update_object_kinematics_and_bounds_2`: Updates an object's position, velocity, and animation frame counters while performing boundary checks against global limits.
- `iterate_object_list`: Iterates through a table of objects, checking a bit flag in each entry and storing pointers to qualifying objects into a stack-like structure.
- `decrement_and_update_object`: Decrements a local value in a loop and calls a routine to update an object's animation and velocity.
- `update_object_animation_and_velocity`: Updates an object's internal animation frame counter and handles sprite index lookups from a table.
- `fetch_indexed_value`: Calculates an index using two input values and retrieves a word from a table based on that calculated index, returning a masked version of the result.
- `search_nested_table`: Performs a nested loop to search through a table using an index and a masked value retrieved from another routine.
- `modify_structure_offsets`: Updates several word-sized values within a structure using arithmetic operations and conditional negation based on a specific offset.
- `initialize_data_structure_3`: Clears and populates a large structure by copying various values and offsets from a source.
- `initialize_data_structure_2`: Clears numerous fields in a large structure and copies several values from another source into specific offsets.
- `reset_structure_fields`: Clears several fields within a structure and sets a specific bit in a byte at a given offset.
- `perform_arithmetic_sequence`: Executes a series of mathematical operations and bitwise manipulations on a global variable.
- `update_timer_value`: Compares a value in a structure against an input and updates a field with a negative constant if the difference falls within a specific range.
- `load_and_initialize_map_data`: Copies data from a source table into a destination memory block using nested loops and then parses a structured data header.
- `clear_and_initialize_data_tables`: Clears two consecutive memory blocks by zeroing out longwords and words respectively.
- `process_structured_data_table`: Decrements a counter and, upon reaching zero, iterates through a table to reset fields, copy strings, and skip whitespace.
- `initialize_control_demonstration_buffer`: Clears a memory table and then copies a string from a buffer while skipping whitespace characters.
- `clear_linked_list_and_table`: Traverses a linked list to remove elements and then fills a table with zeros.
- `process_memory_blocks`: Clears a range of memory, updates a global pointer, and iterates through a linked list of structures to decrement their internal values.
- `search_minimum_value`: Iterates through a list of objects to find the smallest unsigned short value.
- `process_object_attributes`: Clears a bit flag in a list of objects, sets a bit flag in another list, and then searches for the smallest unsigned short value within a specific range and bitmask criteria.
- `find_minimum_list_value`: Traverses a linked list of structures to find the smallest unsigned short value that meets specific bitwise and range criteria.
- `process_conditional_logic`: Evaluates a bitmask from a register and executes a sequence of conditional branches and indirect calls involving global pointers.
- `verify_magic_constant`: Checks a specific memory offset for a constant value and performs several indirect calls based on the result.
- `update_global_pointer`: Updates a global variable with a value and then sets it to zero before returning a concatenated result.
- `calculate_data_offsets`: Calculates and stores relative offsets for elements within a data structure into a table, with the calculation method depending on a global flag.
- `generate_lookup_tables`: Populates a table with incrementing values and generates a second table of bit-manipulated bytes, then uses these to fill a global buffer.
- `manage_state_counters`: Updates a global counter, executes a function pointer if it is non-zero, and performs conditional resets and bitwise inversions on global variables.
- `tiny_ori_2`: Small routine of 1 instruction that merges 5 into d0.
- `write_constant_sequence`: Writes a specific sequence of constant values to a global variable.
- `check_and_assign_pointer`: Checks if a global variable is zero and, if not, assigns the current stack pointer to that global variable before returning the value pointed to by the new address.
- `update_global_with_stack_address`: If a global variable is non-zero, it updates that variable with the address of a local stack buffer.
- `hash_and_load_resource_table`: Computes a checksum of a string to generate a hash and searches for a matching entry in a data block.
- `validate_and_cache_data_block`: Checks a value against a threshold and recursively processes data blocks while updating a global variable with calculated indices.
- `verify_and_process_data_block`: Validates a data structure by checking for specific marker values and performing a checksum calculation across a range of memory.
- `validate_and_load_disk_data`: Verifies the integrity of a data block using a checksum and marker validation before updating a global table with the processed data.
- `update_status_flag`: Checks specific bits within a global variable and conditionally calls other routines before storing the constant value eight into a memory location.
- `write_constant_to_offset`: Stores the value eight into a memory location at a fixed offset from a pointer.
- `store_constant_value`: Stores the constant value eight into a memory location at a specific offset from a pointer.
- `modify_hardware_bitmask`: Checks a value against a threshold and modifies a global byte using bitwise operations and a loop.
- `update_hardware_status`: Modifies a global variable and several other memory locations while interacting with CIAA and CIAB port registers.
- `update_conditional_pointer`: Checks a bit in a byte and, if set, calls a function via a pointer to update a global variable with a value from the stack.
- `process_conditional_pointer`: Checks a bit in a byte and, if set, calls a function via a pointer to update a global variable.
- `reset_global_values`: Sets a global variable to zero and another memory location to all ones while touching a CIAB register.
- `store_indexed_value`: Stores a global variable into a memory location calculated using that same variable as an index.
- `increment_global_counter`: Increments a global variable and performs a check to see if it has wrapped or returned to zero.
- `calculate_string_length`: Iterates through a null-terminated string to determine its length.
- `tiny_ori_3`: Small routine of 1 instruction that merges 110 into d0.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `distribute_table_values_2`: Iterates through a table and copies values into multiple memory locations at fixed intervals.
- `fill_memory_blocks`: Fills multiple memory blocks with values retrieved from a table based on a provided index.
- `distribute_table_values`: Copies values from a table into several memory locations at fixed offsets.
- `skip_whitespace_and_copy`: Iterates through a buffer, skipping space characters, and processes data until a specific byte threshold is met.
- `copy_structured_data`: Copies specific bytes from a source table into a destination buffer using a structured loop.
- `transfer_large_blocks`: Transfers data from one memory area to another by copying specific word segments in a loop.
- `extract_interleaved_data`: Iterates through a source memory area to copy specific interleaved bytes and words into a destination buffer.
- `reorganize_memory_segments`: Copies data from a source table to a destination, rearranging the byte order and offsets during the process.
- `copy_interleaved_data`: Copies data from a source to a destination by selecting specific words at regular intervals.
- `copy_memory_block`: Copies sixteen bytes from a source pointer to a destination pointer.
- `load_resource_segments`: Uses a table of offsets to locate and copy multiple blocks of interleaved data from a source buffer to a destination.
- `traverse_linked_list_2`: Iterates through a list of pointers, performing conditional updates to the list structure and calculating differences between values.
- `clear_pointer_chain`: Iterates through a linked list of pointers, setting each referenced memory location to zero until a null pointer is encountered.
- `calculate_linear_feedback`: Performs a bitwise shift and conditional XOR operation on a global variable based on a loop count.
- `execute_infinite_loop`: This routine is empty as it enters an infinite loop without performing any meaningful operations.
- `validate_resource_header`: Checks a memory location for specific magic marker values.
- `decompress_bitstream_data`: This routine performs bitstream decompression by traversing a series of offset tables and bit-length descriptors.
- `tiny_ori_4`: Small routine of 1 instruction that merges 110 into d1.
- `copy_and_swap_words`: Copies a block of data from one memory location to another and then performs a byte-swap on a series of 16-bit words.
- `decode_variable_bitstream_data`: Decodes a compressed bitstream using variable-length bit sequences and lookup tables to populate memory buffers.
- `decompress_bitstream_data_6`: Decodes a compressed bitstream using variable-length sequences and a lookup table to populate memory buffers.
- `decode_variable_bitstream_3`: Decodes a compressed bitstream by extracting variable-length sequences of bits from memory using a lookup table and bitwise shifts.
- `calculate_bit_shift`: Performs bitwise shifts and subtractions on register values and memory contents to derive a new value.
- `extract_bitstream_data_2`: Extracts a sequence of bits from a series of 16-bit words and updates several global variables with the resulting bit values and remaining counts.
- `extract_bitstream_from_word_array`: Extracts a sequence of bits from a series of 16-bit words by shifting and masking.
- `extract_bitstream_bits`: Iterates through a sequence of 16-bit words to extract individual bits based on a variable-length bitstream logic.
- `relocate_hunk_data`: Iterates through a list of memory segments to perform relocation and data copying.
- `decode_bitstream_with_lookup_tables`: Reads a variable-length bitstream from a source buffer and uses values from multiple lookup tables to reconstruct data.
- `decompress_variable_bitstream`: Expands a compressed bitstream into a destination buffer using a lookup table and variable-length bit decoding.
- `decode_variable_length_bitstream`: Reads a sequence of bits from a source buffer to reconstruct multi-byte values using a lookup table for variable-length integer decoding.
- `decompress_bitstream_data_2`: Reads a bitstream from a source buffer to reconstruct multi-byte values.
- `decode_variable_length_bitstream_5`: Reads a sequence of bits from a source buffer to reconstruct multi-byte values using a lookup table.
- `decode_variable_length_bitstream_4`: Reads a sequence of variable-length bits from a source buffer to traverse lookup tables.
- `decode_bitstream_to_buffer`: Reads a sequence of variable-length bits from a source buffer and uses the resulting values as indices into lookup tables to retrieve data.
- `decode_variable_length_bitstream_2`: Decodes a bitstream by reading individual bits to traverse a series of lookup tables.
- `extract_bitstream_data`: Reads a variable number of bits from a source buffer and uses the resulting values to index into tables for retrieving data.
- `decode_variable_length_bitstream_3`: Reads a sequence of variable-length bits from a source buffer and uses the resulting values as indices into lookup tables to retrieve data.
- `decompress_bitstream_data_5`: Decodes a variable-length bitstream by reading bits from a buffer and using them as indices into lookup tables.
- `decode_variable_bitstream`: Decodes a bitstream using variable bit-length values to traverse lookup tables.
- `decompress_bitstream_data_3`: Reads a bitstream from a source buffer using a variable bit-length decoding scheme.
- `decode_variable_bitstream_2`: Decodes a compressed bitstream using variable-length bit sequences and a lookup table.
- `decompress_bitstream_data_4`: Processes a compressed bitstream by reading bits from a source buffer and using them to index into lookup tables.
- `reset_global_variable`: Sets a global variable to zero and returns.
- `parse_and_unpack_asset_data`: Searches a data buffer for specific magic markers to identify asset headers, then unpacks bit-packed data and performs a memory copy of variable-length compressed elements into a destination buffer.
- `populate_destination_buffer`: Populates a destination buffer by copying bytes from a source table using a structured loop.
- `copy_structured_data_buffer`: Populates a destination buffer by copying bytes from a source table using a structured loop.
- `execute_conditional_loop`: Checks a global variable and executes a loop that calls another routine multiple times while decrementing a local counter.
- `clear_memory_block_3`: Fills a specific memory range with zeros using a loop.
- `clear_memory_block_4`: Iterates through a memory range to set each longword to zero.
- `zero_memory_table`: Iterates through a memory area and sets each longword to zero.
- `configure_and_copy_data`: Checks a value in a structure, writes a constant to a global variable, and performs several hardware configurations, table distributions, and string processing tasks.
- `decrement_and_check_global`: Decrements a global variable and performs conditional checks based on whether the value reaches zero.
- `process_string_data`: Skips whitespace characters in a buffer and copies data until a null terminator is encountered.
- `perform_recursive_calls`: Checks a global variable and performs multiple recursive calls to another routine based on that value.
- `execute_loop_count`: Executes a loop a fixed number of times based on a local counter.
- `convert_and_call_function`: Checks a global variable multiple times and then converts its value into an ASCII character to be used as a function pointer for a call.
- `parse_ascii_numeric_string`: Converts numeric characters from a buffer into a value and iterates through the string while skipping spaces.
- `convert_nibbles_to_ascii`: Converts a 20-bit value into a sequence of ASCII numeric characters by masking and shifting bits.
- `update_bcd_value`: Performs multi-byte binary coded decimal arithmetic on a value stored in memory starting at an offset from a global variable.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d0.
- `increment_and_recurse_state`: Increments a byte at a location relative to a pointer and then enters a complex recursive state machine that modifies global variables and iterates through a table based on conditional checks.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `process_state_machine_recursion`: Manages a complex state machine by performing recursive calls based on the value of a global variable.
- `manage_state_machine_transitions`: This routine implements a complex state machine that uses recursive calls to transition through various stages.
- `increment_and_wrap_global`: Increments a global variable and resets it to one if it reaches specific threshold values.
- `clear_and_loop`: Sets a global variable to zero and enters a loop that continues until a register contains zero.
- `initialize_music_resource_tables`: Loads data from a music resource file into memory and populates several tables with interleaved values and constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_byte_from_pointer`: Loads a byte from the memory location pointed to by a register into a global variable.
- `load_two_bytes`: Loads two consecutive bytes from a memory location into a global variable.
- `load_byte_from_pointer_2`: Copies a byte from a memory address pointed to by a register into a global variable.
- `load_byte_from_pointer_3`: Reads a byte from a pointer and stores it in a global variable.
- `load_byte_from_pointer_4`: Stores a byte from a memory address into a global variable.
- `update_global_state_2`: Executes a routine that modifies a global variable based on comparisons and loop logic.
- `update_global_state`: Modifies a global variable based on comparisons with a provided value and executes logic within loops.
- `update_buffer_with_logic`: Performs bitwise operations to merge bytes from a source into a destination buffer and calls a function pointer within a loop.
- `clear_two_offsets`: Sets two specific byte offsets within a structure pointed to by a register to zero.
- `set_specific_offsets`: Sets one byte offset in a structure to a maximum value and another offset to zero.
- `set_mixed_offsets`: Sets one byte offset in a structure to zero and another offset to a maximum value.
- `initialize_structure_and_copy_data`: Waits for a specific bit flag in a structure to clear before initializing several fields within that structure and performing a large block memory copy from a stack-based buffer into the structure.
- `copy_memory_block_2`: Waits for a specific bit flag in a structure to clear, then initializes several fields within that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine performs no operations and returns immediately to its caller.
- `return_from_subroutine_4`: The routine is empty and performs no operations before returning to the caller.
- `return_no_operation`: The routine consists of a single instruction that immediately returns to its caller, with no other work performed.
