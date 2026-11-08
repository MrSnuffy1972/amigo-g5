# Game 202 (RallyChampionships v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00202). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (7577651 top-level bytes in 211 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 139 of 139 named (0 author symbols), 139 of 139 described (26 by rule, 111 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1260 of 1265 (99.60%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2944 of 2949 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 351863 |
| code | 10400 |
| data | 7215314 |
| padding | 74 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 287 | 16 |
| GO.EXE | 2 | 267 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 466 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1919 | 79 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 8 |

## Cases handled

- RallyChampionships_v1.1/RallyChampionships_v1.1.hdf: hard-disk image (AmigaDOS OFS, 8448 blocks): its filesystem parses and all 206 files in it are byte-identical to the extracted tree RallyChampionships_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/AlteScenary1.raw: high-entropy data file (7.73 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/AlteScenary2.raw: high-entropy data file (7.69 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/CalibraScel.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Cara.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carb.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carc.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Card.Dat: high-entropy data file (7.75 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Care.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carf.Dat: high-entropy data file (7.75 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carriba.Dat: high-entropy data file (7.70 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carribc.Dat: high-entropy data file (7.62 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carribd.Dat: high-entropy data file (7.65 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Carribf.Dat: high-entropy data file (7.60 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/CelicaScel.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Copiloti.iff: high-entropy data file (7.92 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/DeltaScel.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/EscortScel.iff: high-entropy data file (7.86 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/OPTION.EXE: high-entropy data file (7.64 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/PRES.EXE: high-entropy data file (7.89 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/PorscheScel.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryDw0.raw: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryDw1.raw: high-entropy data file (7.84 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryDw2.raw: high-entropy data file (7.70 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryDw3.raw: high-entropy data file (7.74 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryDw4.raw: high-entropy data file (7.79 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryUp0.raw: high-entropy data file (7.80 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryUp1.raw: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryUp2.raw: high-entropy data file (7.79 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryUp3.raw: high-entropy data file (7.80 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/ScenaryUp4.raw: high-entropy data file (7.80 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop1.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop2.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop3.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop4.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop5.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop6.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop7.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Shop8.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/flagselect.exe: high-entropy data file (7.69 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/peugeotScel.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/scenarydw6.raw: high-entropy data file (7.79 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/scenaryup6.raw: high-entropy data file (7.73 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/tracka.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackb.tab: high-entropy data file (7.68 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackc.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackd.tab: high-entropy data file (7.64 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/tracke.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackf.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackg.tab: high-entropy data file (7.63 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackh.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/tracki.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackj.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackk.tab: high-entropy data file (7.64 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackl.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackm.tab: high-entropy data file (7.64 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackn.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/tracko.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackp.tab: high-entropy data file (7.65 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackq.tab: high-entropy data file (7.65 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackr.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/tracks.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackt.tab: high-entropy data file (7.68 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/trackz.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Crash.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Grats1.Sampl: high-entropy data file (7.88 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/InsideLogo.iff: high-entropy data file (7.82 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Mano321go.raw: high-entropy data file (7.78 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Mod.Racer: high-entropy data file (7.87 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/OutFuel.iff: high-entropy data file (7.92 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/SelectRally.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/SinglePodio.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Timeout.iff: high-entropy data file (7.88 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Victory.iff: high-entropy data file (7.90 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/WorldImage.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/WorldPodio.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/mod0.pack: high-entropy data file (7.87 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/mod1.pack: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/mod2.pack: high-entropy data file (7.85 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/mod3.pack: high-entropy data file (7.88 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/mod4.pack: high-entropy data file (7.86 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/mod6.pack: high-entropy data file (7.88 bits/byte), classified as packed data
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/practice.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- runtime segment an address: 271 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 796 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/AlteScenary4.raw: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/AlteScenary6.raw: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Caution.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Jump.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/data/Data/Narrow.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- RallyChampionships_v1.1/RallyChampionships_v1.1_hdf/RallyChampionships/game.slave: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_memory_structure_and_jump`: Initializes a series of linked structures by writing specific values and function pointers into a memory block, updates a global pointer, and performs an indirect jump through a retrieved address.
- `return_negative_one`: Loads negative one into register d0 and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_byte_and_call_function`: Stores the byte value 82 at a memory location, then calls a routine through a pointer retrieved from a table.
- `pop_and_skip_pointer`: Pops a longword from the stack and adds 32 to the value now at the top of the stack before returning.
- `update_color_via_loop`: Sets a byte at a specific location to 1, then enters a loop that calculates a value based on a byte from a hardware register and writes it to the color register, finally clearing the color register.
- `check_hardware_status`: Checks specific bits in the custom chip and CIAA registers, calls another routine, and writes a word to a global variable before returning via an exception return.
- `modify_bit_and_loop`: Sets bit 6 of a byte at an offset of 3584 from a pointer, then enters a loop that compares a byte from a hardware register with itself, otherwise it returns after clearing bit 5 of the same byte.
- `initialize_linked_structures`: Updates a pointer in a table by adding 7 and masking bits, then uses that value to initialize two linked structures with specific offsets and values.
- `clear_memory_block`: Checks a bit in d1 and, if set, calls another routine and then loops to clear a block of memory by writing zeros to it.
- `manage_memory_block`: Compares a value in d0 against a field in a structure, then traverses a linked list of blocks to either find an exact match or split a block to satisfy a requested size.
- `insert_into_memory_map`: Calculates an aligned address and inserts it into a memory management structure by traversing a list and updating pointers and size fields.
- `push_and_branch`: Pushes a longword onto the stack and branches to another routine.
- `transfer_execution_flow`: Loads a target address from a relative offset into the program counter and branches to another location.
- `return_with_stack_adjustment`: Loads a value from a relative offset into the stack pointer, increments the stack pointer by four, and returns.

### GO.EXE

- `parse_amiga_hunk_data`: Iterates through a data structure to identify and process AmigaDOS hunk types, performing memory allocation and relocation of data segments.
- `process_data_and_call_library`: Decrements a word in d1 in a loop, then if d1 is not 3f2, increments a pointer in a3, performs a subtraction between d3 and a pointer, calls a library routine, and potentially stores a shifted value from a pointer into a global variable before calling another library routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `process_bitstream_data`: Loads a value from a pointer in a0, performs several calculations and calls another routine, then enters a loop that uses d2 as a counter to move bytes.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_shifts_and_loop`: Performs a loop that uses d0 as a counter to shift values between d4 and d5, and calls other routines to manipulate data at offsets of a5 and a3.
- `execute_trap_sequence`: Moves a word from a table into d4, performs a bitwise OR on d0 with 152, calls a trap, and performs a bitwise OR on d1 with d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_and_initialize_resources`: Calls two routines to allocate memory, stores the resulting pointers in global variables, calls two more routines, and finally calls a routine via a pointer in a global variable.
- `manage_raster_updates`: Calls several routines, performs a loop that increments a global variable, calls a routine via a pointer, and then calls another routine via a pointer.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `setup_split_longword`: Loads a longword from a global variable into a0, sets a pointer in a1 to a global variable, sets d1 to 28, and calls a routine to split and store the longword.
- `store_split_longword`: Enters a loop that splits a longword from a0 into two words and stores them at offsets 2 and 6 of a structure, adds d1 to d0, and increments the structure pointer by 8 each iteration.
- `copy_words_to_structure`: Enters a loop that copies a word from a pointer in a0 to an offset of 2 in a structure, increments both pointers, and repeats for the count in d7.
- `adjust_values_in_loop`: Iterates through a loop to compare high, middle, and low nibbles of values from two source pointers and updates a 16-bit value in a table at offset 2 of a structure in a2 accordingly.
- `process_data_block`: Traverses a linked list of structures to find one containing a specific constant, then copies a variable-length sequence of bytes to a destination buffer based on a sign-extended length byte.
- `convert_nibbles_to_word`: Locates a structure in a list by searching for a constant, then performs a loop to pack three nibbles from consecutive bytes into a 16-bit word and stores them in a table.
- `manage_dynamic_memory_allocation`: Stores two pointers into global variables, calls a routine to allocate memory, and if successful, uses the returned address to perform further allocations and memory management operations.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `expand_variable_length_bitstream`: Reads a bitstream from a source pointer calculated from a structure in a3 and expands it into a destination buffer.
- `rotate_and_store_bits`: Calls another routine, performs a logical shift right on d0, and enters a loop using d1 that performs a left rotation on d3 before storing the resulting byte at the address pointed to by a2.
- `decode_and_expand_bitstream`: Reads a 32-bit word from a source pointer in a0, performs bitwise rotations to extract bits, and writes the resulting byte to a destination pointer in a2.
- `initialize_bitstream_counter`: Sets a counter in register d1 to 2, calls process_bitstream_data_2, increments register d2 by 7, and branches to another routine.
- `initialize_bitstream_expansion`: Sets a counter in register d1 to 8, calls process_bitstream_data_2, moves the resulting value from register d2 to d3, and branches to another routine.
- `test_and_branch_bitstream`: Shifts the 32-bit value in d0 right by one bit and branches based on the carry flag; it calls another routine if the carry flag is not set, otherwise it loads 8 into d1 and branches to another location.
- `process_bitstream_data_2`: Tests the least significant bit of a 32-bit value in d0 and calls other routines based on the result; it then performs a series of bitwise shifts and rotations on data in a memory buffer, eventually copying bytes from a source pointed to by a2 into a destination via a loop.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_and_loop`: Loads a longword from a pointer in a0, sets the status register, performs bit rotations on d0 and d2, and loops using d1 as a counter before returning.
- `rotate_and_decrement_counter`: Decrements d1, clears d2, shifts d0 left, and enters a loop using dbra that performs a left rotation on d2 before returning.
- `fill_zero_data`: Contains four identical instructions that perform a logical OR on register d0 with zero.
- `copy_memory_block`: Copies a block of memory from a location pointed to by a global variable to a destination, then jumps to a new address.
- `decrement_and_branch`: Subtracts 1 from d1 and branches to another routine if d1 is greater than or equal to 0; otherwise it returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_and_reset_pointer`: Calls a routine via a pointer in a6, stores the result in a global variable, and then sets a value at an offset of a pointer in d0 to all ones.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `configure_hardware_and_execute_state_loop`: Configures the Amiga custom chips by setting the DMA controller, interrupt enable register, and copper control registers, then enters a loop that manages resource state transitions, updates global status flags, and executes various callback routines based on values in a table.
- `execute_practice_routine`: Loads a constant into d1, calls another routine, calls a routine at the address in a0, writes 1 to a global variable, and branches to another location.
- `validate_resource_and_free_memory`: Loads a constant into a register, calls a routine to validate a resource table, calls another routine via a pointer in a0, sets a global word to 1, and then returns via a branch.
- `execute_out_of_time_routine`: Loads a constant into d1, calls another routine, loads a value from a PC-relative address into d0, loads a PC-relative address into a5, calls a routine at the address in a0, writes 1 to a global variable, stores d0 into a global variable, and branches to another location.
- `execute_callback_and_update_state`: Calls a routine with a constant passed in a register, then executes a function pointer stored in a register and updates a global variable with the return value and another global variable with a constant.
- `execute_callback_and_update_status`: Calls a routine with a constant passed in a register, then calls another routine via a pointer in a register and stores the result in a global variable.
- `execute_winner_single_routine`: Loads a constant into d1, calls another routine, loads a value from a PC-relative address into d0, loads a PC-relative address into d1, loads the address of a global variable into a6, loads a PC-relative address into a5, calls a routine at the address in a0, writes 1 to a global variable, and branches to another location.
- `execute_callback_and_reset_flag`: Calls a routine with a specific address in register d1, loads two values from the program code into d0 and d1, then executes a function pointer stored in a0.
- `prepare_and_execute_resource_validation`: Loads a constant into a register, calls a routine, executes a function via a pointer in a0, sets a global word to 1, and branches to another routine.
- `execute_inside_routine`: Loads a constant into d1, calls another routine, loads the address of a global variable into a6, calls a routine at the address in a0, writes 1 to a global variable, and branches to another location.
- `initialize_resource_structure`: Calls another routine, writes the value 48817 to four consecutive words at an offset of 156 from a pointer in a0, writes 256 to a location at offset 1356 from a0, calls the routine pointed to by a0, writes 1 to a global variable, and branches to another routine.
- `execute_resource_call`: Calls another routine, calls the routine pointed to by a0, writes 1 to a global variable, and branches to another routine.
- `update_resource_pointers`: Calls another routine, clears registers a1 through a6, loads four longwords from a table into d0 through d3, loads a pointer into a5, and loads a longword into d4, calls the routine pointed to by a0, writes 1 to a global variable, stores d0 through d4 into four global variables, adds d5 to a global variable, and branches to another routine.
- `store_resource_result`: Loads a pointer into a6, calls another routine, calls the routine pointed to by a0, writes 1 to a global variable, stores d0 into a global variable, and branches to another routine.
- `sync_resource_data`: Loads a pointer into a6, calls another routine, loads five longwords from a table into d0 through d5, clears registers a1 through a5, sets a6 to point to a global variable, calls the routine pointed to by a0, writes 1 to a global variable, stores d0 through d4 into five global variables, and branches to another routine.
- `calculate_offset_and_call`: Loads a pointer into a6, calls another routine, calculates an offset using a value from a table and a longword from a global variable, uses that offset to find a pointer in a second table, performs arithmetic on bytes from that pointer to calculate a value, loads a pointer into a5, calls the routine pointed to by a0, writes 1 to a global variable, and branches to another routine.
- `update_resource_pointers_2`: Calls another routine, loads five longwords from a table into registers, calls a function via a pointer in a0, updates five global longwords with the loaded values and the result of the call, and loops back to the start if a longword in a global variable is non-zero.
- `process_resource_table`: Clears several registers, calls another routine, calculates an index into a table using a value from a global variable multiplied by ten, and if a global longword is non-zero, calls a function via a pointer in a0 to update several global longwords and a global pointer.
- `initialize_and_call_function`: Loads several longwords into d1, d0, and a5 from constant offsets, calls a routine at the address in a0, writes a word to a longword at a specific address, performs a bitwise OR on a longword at another address, calls another routine, and branches to another routine.
- `clear_and_add_to_globals`: Clears a longword in a global variable, loads two values from a table, calls a routine, and adds d1 to two other global variables.
- `calculate_and_store_time`: Calculates an offset into a table using multiple values, compares a value in d0 against a calculated threshold, and either updates four bytes in a table with decomposed digits or returns a calculated value.
- `calculate_total_value`: Calculates a value by accessing a table using indices derived from a global variable and adding a constant to the result.
- `configure_blt_and_execute_callback`: Sets up Blitter parameters and copper registers, then executes a function pointer from a calculated table offset and iterates through a list of function pointers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_split_word_loops`: Calls a routine twice to split and store longwords into two different tables using d1 as a count and specific offsets.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `store_split_word`: Iterates through a loop to store a 32-bit value from a0 into a table at offset 2 and 6 of a structure in a1, then adds d1 to a0.
- `multiply_and_store_result`: Loads a structure from the stack, multiplies two 16-bit values from offsets 4 and 8, stores the result in a 16-bit word at offset 256, and negates the result.
- `transform_and_divide_value`: If a 16-bit value in d1 is greater than zero, it doubles a 32-bit value from a global variable, optionally XORs it with a constant if the result is zero or has the carry flag set, stores the result back to the global variable, and returns the quotient of the lower 16 bits of the result divided by d1, with the remainder in the upper 16 bits.
- `process_data_buffer`: Depending on a flag, it either accesses a global variable or a calculated offset in a table, then iterates through a list of pointers to move data, eventually updating a global variable and potentially calling a routine via a pointer and performing a descending XOR loop on a buffer.
- `execute_library_calls`: Calls a routine through a pointer in a table, and if the result is non-zero, calls another routine through a different pointer and then calls a third routine via a pointer in a table.
- `xor_global_buffer`: Performs a loop that XORs bytes from a global variable with a descending sequence of values starting from 279, until the counter reaches negative 1.
- `do_nothing`: Performs a logical OR on register d0 with zero.
- `check_interrupt_and_call_routine`: Tests bit 5 of the DMA control register and, if set, checks if a 16-bit value at a global address is non-negative; if both conditions are met, it calls another routine, writes 32 to the interrupt mask register, and returns from an interrupt.
- `push_address_and_offset`: Pushes a 32-bit value from a memory location onto the stack and then adds 32 to the stack pointer before returning.
- `process_data_and_clear_hardware`: Searches through a data block to find a specific byte value, then uses the result to calculate an offset into a table to clear 30 entries, and finally clears several hardware registers and a global variable.
- `increment_and_conditionally_reset_timer`: Increments a 32-bit value at a global address by one, then compares a byte loaded from a relative offset to another byte; if the comparison results in a carry, it clears the 32-bit value, checks a byte at a global address, and either calls another routine or branches to a different location.
- `reset_audio_channels_and_update_timer`: Resets the four audio channel location registers and then updates a global timer value by reading a byte from a table, shifting it, and adding a constant offset.
- `update_entity_state_structure`: Updates a structure in a6 by loading data from a table in a3 based on a type byte, modifies several 16-bit and 8-bit fields including offsets 4, 8, 10, 14, 18, 19, 36, and 40, and performs conditional updates to hardware registers and a global variable.
- `dispatch_structure_operation_by_type`: Reads a 16-bit value from offset 2 of a structure in a6 and branches based on its lower 12 bits; if non-zero, it reads a 4-bit type identifier from the same word and branches to specific sub-routines.
- `update_value_from_structure`: Contains multiple sub-routines that perform conditional logic on a byte from a structure to either copy a 16-bit value to a destination, or modify a 16-bit value in a structure using specific bitwise masks and constants.
- `validate_and_update_structure`: Loads a pointer from a6, compares a value from a table to a 12-bit mask of a word in a6, and if the table value is not greater than the mask, it performs a lookup to find a new value, writes that value to offset 24 of the structure in a6, clears a byte at offset 22, and if the new value is less than the value at offset 16, sets a byte at offset 22 to 1.
- `update_structure_values`: Performs arithmetic on a 16-bit value at an offset of a6, performs a table lookup using a value from a6, and writes the result to a destination in a5.
- `update_scaled_coordinate`: Calculates a scaled value by reading an index from a structure in a6, performing a table lookup or bitwise multiplication, and adding the result to a 16-bit value at offset 6 of a structure in a5.
- `dispatch_by_type_byte`: Reads a byte from a structure at an offset of a6, masks it, and branches to various other routines based on the resulting value.
- `allocate_and_initialize_structure_2`: Updates two global pointers, calls another routine, and then calls a library function to allocate memory of size 65538 bytes, storing the resulting pointer in a global variable.
- `allocate_and_initialize_structure`: Calls another routine to increment a global value by ten, then calls a library function to allocate memory and stores the resulting pointer in a global variable.
- `calculate_buffer_offset`: Copies a pointer to a3, calculates an offset using a value from a3, updates a0 to point to a specific location in a buffer, and branches to another location.
- `do_nothing_2`: Performs four consecutive logical OR operations on register d0 with zero.
- `clear_data_register_zero`: Sets the least significant byte of data register zero to zero.
- `zero_data_padding`: This area consists of zeroed bytes that serve as a target for absolute jumps or tests.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_data_register_zero_2`: Performs a logical OR operation on data register zero with zero.
- `zero_data_byte`: Performs a logical OR operation on a data register with zero, which has no effect on the register's value.
- `process_resource_table_and_validate`: Checks a global variable for a specific constant, then enters a loop that uses a value from a table to add values from a list to another table.
- `adjust_stack_and_branch`: Increments the stack pointer by four and branches to another location.
- `set_register_d3`: Sets register d3 to 4 and branches to another routine.
- `store_pointer_to_offset`: Stores a 32-bit value from a memory location into a global variable at an offset of 124 from the address in a0.
- `wait_for_disk_status`: Loops until bit 7 of the CIAA Port A Data register is set, then returns.
- `wait_for_hardware_bit`: Loops while bit 0 of the custom chip register is clear, then loops while bit 0 is set, and finally returns.
- `free_memory_list`: Iterates through a list of pointers starting at a location specified by a global variable, calling a routine through a pointer at an offset of negative 210 from the library base in a6 for each entry until a null pointer is encountered.
- `process_data_table`: Checks a global variable for a specific constant, then enters a loop that uses a value from a table to add values from a list to another table, calling another routine multiple times during the process.
- `update_global_state`: Clears a global variable, stores a value from d1 into another global variable, calls a routine through a pointer in a table, and if the result is non-zero, stores that result into a third global variable.
- `skip_if_byte_matches`: Compares a byte at an offset of 5 from a pointer in a0 to 58; if they are equal, it increments the pointer by 6 before returning.
- `accumulate_and_call_routine`: Calls a routine, adds a value from a global variable to another, and if a comparison between a global variable and a value in d3 passes, calls a routine through a pointer in a table and subtracts the value in d3 from the global variable.
- `clear_d0_register`: Performs a bitwise OR on register d0 with zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 126 into d1.
- `tiny_ori_3`: Small routine of 1 instruction that merges 106 into d0.
- `clear_memory_blocks`: Clears nine 32-bit words in a list starting at a specific location using a loop, then clears four consecutive 32-bit words at a different location.
- `iterate_decompress_list`: Loads several pointers and a count from a table, then enters a loop that calls another routine to process data for each entry in the list.
- `shift_and_copy_data`: Calculates a loop count from d1, then repeatedly copies a longword from the address in a0 to the address at offset -8 from a0 and increments a0 until the loop count reaches zero.
- `decompress_bitstream_data`: Reads a bitstream from a source buffer and expands it into a destination buffer by interpreting variable-length bit sequences.
- `set_constant_registers`: Sets d0 and d1 to various constant values and returns.
- `set_multiple_registers`: Sets d0 and d1 to various constant values through a series of bitwise OR operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `return_with_branches`: Returns immediately, followed by two branch instructions to addresses in a0 and b0.
