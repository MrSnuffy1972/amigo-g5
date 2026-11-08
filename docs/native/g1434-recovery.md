# Game 1434 (fire hawk (1993)(codemasters)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01434). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 59 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (5 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 95 of 95 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 368620 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1834 of 1834 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 215026 |
| code | 5124 |
| data | 377178 |
| padding | 10416 |
| unused | 293376 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Else | 2 | 117 | 2 |
| A1200RUN | 1 | 126 | 2 |
| Type | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| CheckKick | 1 | 97 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| Wait | 2 | 117 | 2 |
| Fire Hawk (1993)(Codemasters).adf:VC | 1 | 180 | 3 |
| Ram-Handler | 2 | 10 | 1 |
| Fire Hawk (1993)(Codemasters).adf:DCLOADER | 1 | 314 | 15 |
| Fire Hawk (1993)(Codemasters).adf:DC | 2 | 267 | 8 |
| Fire Hawk (1993)(Codemasters).adf:bootblock | 1 | 18 | 1 |
| Fire Hawk (1993)(Codemasters).adf:GAME.DAT#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP2: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (5831 -> 15360 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP3: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (4893 -> 15360 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP4: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (775 -> 6144 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP5: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (6474 -> 16384 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP6: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (5482 -> 15360 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:SHIPSIDE: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (7180 -> 24480 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:ISLAND: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (8160 -> 12808 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:HIGH: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (4161 -> 24576 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:LOGO: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (3565 -> 6912 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:SUB: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (12989 -> 24718 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:TYPE: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (7483 -> 32000 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:BIGCOCK: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (3117 -> 7680 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:TITLE: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (45064 -> 61568 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:GAME.DAT: IMP: File Imploder-packed, unpacked with ancient (136416 -> 368620 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP0: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (1505 -> 4608 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:HAND: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (1429 -> 4424 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:MAP1: Ice: Pack-Ice v2.0 - v2.20-packed, unpacked with ancient (4729 -> 11520 bytes)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:GAME.DAT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fire hawk (1993)(codemasters)/Fire Hawk (1993)(Codemasters).adf:VC: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 368620 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_long_word`: Copies a long word from register d1 to register d0 and then branches to another routine.

### Else

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_long_word`: Moves a long word from register d1 into register d0 and performs a short branch to another location.

### A1200RUN

- `execute_system_initialization`: Parses a string until a newline, modifies a global buffer using an XOR mask, calls several library functions to manage memory and files, and performs a system trap.
- `search_and_modify_list`: Traverses a list to find a specific entry matching two criteria, stores its address and a word from its offset into global variables, clears a word at that offset, and then continues searching the list to modify a word if another specific entry is found.

### Type

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `transfer_and_branch`: Moves the contents of register d1 into d0 and then branches to another location.

### Assign

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `copy_long_word_2`: Copies a 32-bit value from register d1 to register d0 and then branches to another routine.

### Execute

- `find_max_and_prepare_buffer_5`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_long_word_2`: Moves a 32-bit value from register d1 into register d0 and performs a short branch.

### CheckKick

- `parse_numeric_string`: Opens a library and converts a string of up to five ASCII digits into a numeric value, comparing the result against a 16-bit integer stored in a structure.
- `xor_data_block`: Performs a loop that XORs 585 consecutive bytes at a global variable location with the constant value 167 and then returns.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_6`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `move_long_word`: Transfers a 32-bit value from register d1 to register d0 and then executes a branch.

### Wait

- `find_max_and_prepare_buffer_7`: Walks a linked list to find the maximum value in a field, writes a header with sequential words into a buffer, and sets up a stack frame for a subsequent call.
- `assign_long_word`: Stores the 32-bit contents of register d1 into register d0 before branching to another location.

### Fire Hawk (1993)(Codemasters).adf:VC

- `initialize_system_registers`: Clears bit 0 of a data register, loads a longword into the stack pointer, moves a PC-relative address into a memory location, and calls a trap.
- `initialize_decompression_header`: Sets a 16-bit value in a custom hardware register, writes a bitwise inverted constant to a global variable, calls another routine, and then writes three 32-bit values to a table before jumping to an address in that table.
- `decompress_imp_data_block`: Checks for a specific four-byte signature at the start of a data block, then performs a custom decompression routine that uses a series of bit-shifting and byte-expansion operations to expand compressed data into a destination buffer.

### Ram-Handler

- `execute_indirect_jump`: Sets a register with a value derived from a bitwise operation, calls a routine at an offset from a library base, and then performs an indirect jump using a value from that library base.

### Fire Hawk (1993)(Codemasters).adf:DCLOADER

- `setup_hardware_and_copy_routine`: Writes constant values to three hardware registers, clears a global variable, and then copies 256 longwords from the current routine to a destination specified in a global variable before executing a trap.
- `initialize_system_and_jump_to_table`: Clears a memory range up to a specific address, calls another routine, performs a trap, and if the result is not zero, enters an infinite loop; otherwise, it copies 256 bytes from one location to another and jumps to the destination address.
- `initialize_hardware_and_copy_block`: Writes constants to a global and to a CIA-B port register, saves two address values into globals, and calls several routines.
- `set_clear_byte_flags`: Computes a byte address from an input address register plus a constant offset, sets four bits in that byte and clears two others, leaving the remaining bits unchanged.
- `set_clear_byte_bits`: Computes a byte address from an input address register plus a constant offset, sets two bits in that byte, clears one bit and sets it again, leaving the other bits unchanged.
- `parse_compressed_header_and_verify`: Calculates a checksum by iterating through a buffer until a null byte is found, then uses the result to index into a table to retrieve a value.
- `verify_and_decompress_block`: Compares a sequence of bytes from a source buffer against a target list, performing a case-insensitive check, and if a match is found, decompresses data by copying longwords and bytes based on a length field.
- `divide_index_and_compare`: Divides a value in d0 by eleven, compares the quotient to a word at offset 26 of a structure in a4, and if they differ, calls another routine with a value containing the swapped high and low words of the quotient before calling two more routines.
- `toggle_control_bits_in_loop`: Sets bit 2 of a byte at offset 256 of a structure, then enters a loop that compares a value in d6 with a value from a structure, toggling bits 0 and 1 of the byte at offset 256 and calling another routine until the values match.
- `write_control_bytes`: Writes four specific byte values to offsets of 2048, 3328, 1536, and 1792 from a base pointer in a6, then branches to a fixed address and returns.
- `clear_flags_and_wait`: The routine writes constant values to several byte locations referenced through a base pointer, then spins until a specific flag bit in one of those locations becomes set, and finally clears a bit in another location before returning.
- `toggle_bit_while_status_set`: Sets bit 1 of a byte at offset 256 of a structure, then repeatedly toggles bit 0 of that byte and calls another routine while bit 4 of a hardware register remains set, finally calling another routine and clearing a word at offset 26 of a structure.
- `configures_disk_sync_transfer`: Writes a set of 16-bit control words to hardware registers offset from a base register, including a sync word and a DMA-related value, and stores a 32-bit pointer from a second base-relative location into a register-offset slot.
- `decompress_data_block`: Searches a buffer for a specific 16-bit pattern, performs a bitwise transformation on data blocks using XOR and bit-shifting operations, and writes the results to a destination table while checking a checksum.
- `return_constant_byte`: Returns a byte value formed by performing a bitwise OR between a constant and the contents of register d0.

### Fire Hawk (1993)(Codemasters).adf:DC

- `parse_amiga_hunk_data`: Iterates through a data structure containing AmigaDOS hunk type codes, performing memory allocation and relocation operations based on specific hunk markers.
- `walk_linked_list_and_calculate`: Decrements a register in a loop, then walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header to a destination pointed to by a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `extract_bitstream_data`: Initializes several registers, loads a longword from a pointer in a0, and enters a loop that calls other routines to extract bits and write bytes to a destination buffer pointed to by a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bitstream`: Performs a loop that extracts bits from a source longword using a shift count from a register, updates a running value with the extracted bits, and writes bytes to a destination buffer pointed to by a3.
- `perform_trap_operation`: Moves a 16-bit word from an offset of a register into d4, performs a bitwise OR on d1 with a constant, calls a trap, and then performs a bitwise OR on d1 with the result of the trap.

### Fire Hawk (1993)(Codemasters).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Fire Hawk (1993)(Codemasters).adf:GAME.DAT#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
