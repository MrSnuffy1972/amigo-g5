# Game 181 (Hunter): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00181). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (944076 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 38 raw-word lines |
| Functions named and described | pass, 203 of 203 named (0 author symbols), 203 of 203 described (14 by rule, 189 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2814 of 2815 (99.96%) |
| Recovered without load address (counted in proof 1, flagged) | 14336 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3987 of 4070 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 62811 |
| code | 18220 |
| data | 521471 |
| padding | 70 |
| unused | 341504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hunter.adf:bootblock | 1 | 45 | 2 |
| logged disk load from Hunter.adf boot-block load | 1 | 160 | 4 |
| logged disk load from Hunter.adf boot-block load | 1 | 0 | 0 |
| logged disk load from Hunter.adf boot-block load | 1 | 163 | 3 |
| logged disk load from Hunter.adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1436 | 87 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 551 | 43 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 155 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 124 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 946 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 148 | 1 |
| logged disk load from Hunter.adf | 1 | 44 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 256 | 13 |
| Hunter.adf bytes 34304-38912 (code never loaded by a reference run) | 1 | 39 | 1 |
| Hunter.adf bytes 45056-50176 (code never loaded by a reference run) | 1 | 0 | 0 |
| Hunter.adf bytes 56320-57856 (code never loaded by a reference run) | 1 | 3 | 1 |
| Hunter.adf bytes 58880-61440 (code never loaded by a reference run) | 1 | 0 | 0 |
| Hunter.adf bytes 168960-169472 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Hunter/Hunter.adf: custom-format (non-DOS) disk, 1078 non-empty blocks: matched against track-loaded segments
- Hunter/Hunter.adf: the boot block reads 5632 bytes from disk offset 84480 to an address (found statically); recovered as a unit
- Hunter/Hunter.adf: the boot block reads 5120 bytes from disk offset 22528 to an address (found statically); recovered as a unit
- Hunter/Hunter.adf: the boot block reads 16384 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Hunter/Hunter.adf: the boot block reads 8192 bytes from disk offset 224768 to an address (found statically); recovered as a unit
- runtime segment an address: 731 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 237 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f1000): 141 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f1000): 114 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f1000): 854 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 120 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Hunter/Hunter.adf: 23 executed PCs whose RAM was overwritten before any snapshot match its bytes 34448-34624: recovered as a unit run at an address
- 251 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 24 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Hunter/Hunter.adf: bytes 34304-38912 look like code no run loaded: recovered position-independently (load address unknown)
- Hunter/Hunter.adf: bytes 45056-50176 look like code no run loaded: recovered position-independently (load address unknown)
- Hunter/Hunter.adf: bytes 56320-57856 look like code no run loaded: recovered position-independently (load address unknown)
- Hunter/Hunter.adf: bytes 58880-61440 look like code no run loaded: recovered position-independently (load address unknown)
- Hunter/Hunter.adf: bytes 168960-169472 look like code no run loaded: recovered position-independently (load address unknown)
- Hunter/Hunter.adf: 10 logged loads cover 49664 bytes of the image
- Hunter/Hunter.adf: 55 track-loaded blocks found in recovered runtime segments and classified through them
- Hunter/Hunter.adf: blocks 34-42 (bytes 17408-22016) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.67)
- Hunter/Hunter.adf: blocks 55-64 (bytes 28160-33280) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.87)
- Hunter/Hunter.adf: blocks 77-86 (bytes 39424-44544) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.32)
- Hunter/Hunter.adf: blocks 99-108 (bytes 50688-55808) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.95)
- Hunter/Hunter.adf: blocks 121-130 (bytes 61952-67072) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.56)
- Hunter/Hunter.adf: blocks 132-134 (bytes 67584-69120) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.24)
- Hunter/Hunter.adf: blocks 137-138 (bytes 70144-71168) outside the filesystem hold data, not code (opcode density 0.031, entropy 4.25)
- Hunter/Hunter.adf: blocks 143-152 (bytes 73216-78336) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- Hunter/Hunter.adf: blocks 177-177 (bytes 90624-91136) outside the filesystem hold data, not code (opcode density 0.051, entropy 4.56)
- Hunter/Hunter.adf: blocks 221-221 (bytes 113152-113664) outside the filesystem hold data, not code (opcode density 0.016, entropy 4.17)
- Hunter/Hunter.adf: blocks 227-227 (bytes 116224-116736) outside the filesystem hold data, not code (opcode density 0.047, entropy 3.79)
- Hunter/Hunter.adf: blocks 249-251 (bytes 127488-129024) outside the filesystem hold data, not code (opcode density 0.043, entropy 4.30)
- Hunter/Hunter.adf: blocks 264-273 (bytes 135168-140288) outside the filesystem hold data, not code (opcode density 0.033, entropy 3.61)
- Hunter/Hunter.adf: blocks 286-295 (bytes 146432-151552) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.81)
- Hunter/Hunter.adf: blocks 297-306 (bytes 152064-157184) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.39)
- Hunter/Hunter.adf: blocks 319-328 (bytes 163328-168448) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.54)
- Hunter/Hunter.adf: blocks 332-333 (bytes 169984-171008) outside the filesystem hold data, not code (opcode density 0.059, entropy 2.91)
- Hunter/Hunter.adf: blocks 341-350 (bytes 174592-179712) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.83)
- Hunter/Hunter.adf: blocks 363-372 (bytes 185856-190976) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.95)
- Hunter/Hunter.adf: blocks 385-394 (bytes 197120-202240) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.56)
- Hunter/Hunter.adf: blocks 407-416 (bytes 208384-213504) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- Hunter/Hunter.adf: blocks 429-438 (bytes 219648-224768) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.16)
- Hunter/Hunter.adf: blocks 473-481 (bytes 242176-246784) outside the filesystem hold data, not code (opcode density 0.019, entropy 5.64)
- Hunter/Hunter.adf: blocks 504-504 (bytes 258048-258560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- Hunter/Hunter.adf: blocks 517-526 (bytes 264704-269824) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.38)
- Hunter/Hunter.adf: blocks 539-548 (bytes 275968-281088) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.07)
- Hunter/Hunter.adf: blocks 627-636 (bytes 321024-326144) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- Hunter/Hunter.adf: blocks 649-658 (bytes 332288-337408) outside the filesystem hold data, not code (opcode density 0.000, entropy 7.10)
- Hunter/Hunter.adf: blocks 660-669 (bytes 337920-343040) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.14)
- Hunter/Hunter.adf: blocks 671-680 (bytes 343552-348672) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.44)
- Hunter/Hunter.adf: blocks 682-691 (bytes 349184-354304) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.87)
- Hunter/Hunter.adf: blocks 693-702 (bytes 354816-359936) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.12)
- Hunter/Hunter.adf: blocks 704-713 (bytes 360448-365568) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.03)
- Hunter/Hunter.adf: blocks 715-724 (bytes 366080-371200) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.79)
- Hunter/Hunter.adf: blocks 726-735 (bytes 371712-376832) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.95)
- Hunter/Hunter.adf: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.99)
- Hunter/Hunter.adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.25)
- Hunter/Hunter.adf: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.56)
- Hunter/Hunter.adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.25)
- Hunter/Hunter.adf: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- Hunter/Hunter.adf: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.74)
- Hunter/Hunter.adf: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.73)
- Hunter/Hunter.adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.78)
- Hunter/Hunter.adf: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.61)
- Hunter/Hunter.adf: blocks 858-867 (bytes 439296-444416) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.90)
- Hunter/Hunter.adf: blocks 869-878 (bytes 444928-450048) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.58)
- Hunter/Hunter.adf: blocks 880-889 (bytes 450560-455680) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.96)
- Hunter/Hunter.adf: blocks 891-900 (bytes 456192-461312) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.60)
- Hunter/Hunter.adf: blocks 902-903 (bytes 461824-462848) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.28)
- Hunter/Hunter.adf: blocks 913-922 (bytes 467456-472576) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.93)
- Hunter/Hunter.adf: blocks 935-944 (bytes 478720-483840) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.06)
- Hunter/Hunter.adf: blocks 946-955 (bytes 484352-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- Hunter/Hunter.adf: blocks 957-966 (bytes 489984-495104) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.69)
- Hunter/Hunter.adf: blocks 968-977 (bytes 495616-500736) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.90)
- Hunter/Hunter.adf: blocks 979-988 (bytes 501248-506368) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.00)
- Hunter/Hunter.adf: blocks 990-993 (bytes 506880-508928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- Hunter/Hunter.adf: blocks 998-999 (bytes 510976-512000) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.74)
- Hunter/Hunter.adf: blocks 1001-1002 (bytes 512512-513536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.48)
- Hunter/Hunter.adf: blocks 1012-1021 (bytes 518144-523264) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.21)
- Hunter/Hunter.adf: blocks 1034-1043 (bytes 529408-534528) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.64)
- Hunter/Hunter.adf: blocks 1056-1065 (bytes 540672-545792) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.88)
- Hunter/Hunter.adf: blocks 1078-1087 (bytes 551936-557056) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.85)
- Hunter/Hunter.adf: blocks 1100-1109 (bytes 563200-568320) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.74)
- Hunter/Hunter.adf: blocks 1111-1120 (bytes 568832-573952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.72)
- Hunter/Hunter.adf: blocks 1122-1131 (bytes 574464-579584) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.78)
- Hunter/Hunter.adf: blocks 1133-1142 (bytes 580096-585216) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.55)
- Hunter/Hunter.adf: blocks 1144-1147 (bytes 585728-587776) outside the filesystem hold data, not code (opcode density 0.009, entropy 5.61)
- Hunter/Hunter.adf: blocks 1155-1164 (bytes 591360-596480) outside the filesystem hold data, not code (opcode density 0.005, entropy 1.82)
- Hunter/Hunter.adf: blocks 1166-1175 (bytes 596992-602112) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.39)
- Hunter/Hunter.adf: blocks 1177-1186 (bytes 602624-607744) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.25)
- Hunter/Hunter.adf: blocks 1188-1197 (bytes 608256-613376) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.18)
- Hunter/Hunter.adf: blocks 1199-1208 (bytes 613888-619008) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.50)
- Hunter/Hunter.adf: blocks 1210-1219 (bytes 619520-624640) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.14)
- Hunter/Hunter.adf: blocks 1221-1230 (bytes 625152-630272) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.00)
- Hunter/Hunter.adf: blocks 1232-1241 (bytes 630784-635904) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.05)
- Hunter/Hunter.adf: blocks 1243-1252 (bytes 636416-641536) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- Hunter/Hunter.adf: blocks 1254-1263 (bytes 642048-647168) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.00)
- Hunter/Hunter.adf: blocks 1265-1274 (bytes 647680-652800) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.94)
- Hunter/Hunter.adf: blocks 1276-1285 (bytes 653312-658432) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.16)
- Hunter/Hunter.adf: blocks 1287-1296 (bytes 658944-664064) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.39)
- Hunter/Hunter.adf: blocks 1309-1311 (bytes 670208-671744) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.15)
- Hunter/Hunter.adf: blocks 1316-1318 (bytes 673792-675328) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.90)
- Hunter/Hunter.adf: blocks 1331-1340 (bytes 681472-686592) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- Hunter/Hunter.adf: blocks 1353-1362 (bytes 692736-697856) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.76)
- Hunter/Hunter.adf: blocks 1375-1384 (bytes 704000-709120) outside the filesystem hold data, not code (opcode density 0.005, entropy 1.10)
- Hunter/Hunter.adf: blocks 1397-1406 (bytes 715264-720384) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.81)
- Hunter/Hunter.adf: blocks 1419-1428 (bytes 726528-731648) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- Hunter/Hunter.adf: blocks 1441-1450 (bytes 737792-742912) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.54)
- Hunter/Hunter.adf: blocks 1463-1472 (bytes 749056-754176) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.35)
- Hunter/Hunter.adf: blocks 1485-1489 (bytes 760320-762880) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.40)
- Hunter/Hunter.adf: blocks 1536-1538 (bytes 786432-787968) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.23)
- Hunter/Hunter.adf: blocks 1551-1560 (bytes 794112-799232) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.71)
- Hunter/Hunter.adf: blocks 1573-1582 (bytes 805376-810496) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.15)
- Hunter/Hunter.adf: blocks 1595-1604 (bytes 816640-821760) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.49)
- Hunter/Hunter.adf: blocks 1617-1626 (bytes 827904-833024) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.14)
- Hunter/Hunter.adf: blocks 1639-1648 (bytes 839168-844288) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.76)
- Hunter/Hunter.adf: blocks 1661-1670 (bytes 850432-855552) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.98)
- Hunter/Hunter.adf: blocks 1683-1692 (bytes 861696-866816) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.87)
- Hunter/Hunter.adf: blocks 1705-1711 (bytes 872960-876544) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.45)
- Hunter/Hunter.adf: blocks 1756-1758 (bytes 899072-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.11)
- 5 never-loaded region(s), 14336 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 9 packed track region(s), 40448 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_cia_b_port_mask` (runtime segment an address): Chooses one of two bit masks depending on a record flag, stores the mask in the record, and writes its inverse, limited to seven bits, to a CIA-B port register. Kind: disk hardware/status check. Pass path: the instruction following the call (e.g., bsr.b an address or bsr.w an address).
- `toggle_counter_and_flag` (runtime segment an address): Steps a global counter down by two and wraps it to eighteen when it goes negative. Kind: checksum or state-based integrity check. Pass path: the branch following the call that does not lead to a failure/loop (in this context, the execution flow that continues to the next functional block rather than looping or crashing).
- `configure_hardware_and_copy_data` (runtime segment an address (f1000)): Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range. Kind: disk hardware/track check. Pass path: the branch following the bsr.w an address call (the bne.b an address instruction is the failure path).
- `initialize_hardware_and_interrupts` (first-run image an address): Configures custom hardware registers, sets up interrupt vectors in a table, writes values to CIA registers, and executes two subroutines before populating a table with four entries using a loop. Kind: disk hardware/interrupt initialization and trap setup. Pass path: the code continues to the next instruction after the call (the instructions following the call in the provided evidence suggest a check of the status register/hardware state via an address(a5) and an address(a5)).

## Functions


### Hunter.adf:bootblock

- `load_disk_sectors_to_ram`: Performs two loops of I/O requests to load data from disk into memory; the first loop loads four sectors starting at a specific offset, and the second loop loads fourteen sectors starting at a different offset, finally jumping to the newly loaded code.
- `issue_two_io_requests`: Populates two separate request blocks with command value two, length, offset, and transfer fields, then executes each via an I/O request routine and finally calls a fixed routine.

### logged disk load from Hunter.adf boot-block load

- `call_external_subroutines`: Saves registers to the stack, calls two subroutines, restores registers, and returns.
- `copy_and_process_hunk_list`: Copies a block of data from a source to a destination and then iterates through a list of headers, performing bit shifts and arithmetic on a value to determine how many bytes to copy from a table into a destination buffer via another routine.
- `store_and_shift_long`: Stores a longword from a pointer in a0 into d0, stores a byte from d0 into a memory location at offset 5 of a5, sets the carry flag, and performs a logical shift right on d0 before returning.
- `process_data_stream`: Performs a loop that shifts a value in d0 and calls another routine if d0 is zero, then enters a second loop that processes bytes from a source pointer into a destination buffer, potentially jumping to a pointer at the end of the sequence.

### logged disk load from Hunter.adf boot-block load

- `decompress_bitstream_data`: Copies a block of memory to a destination and then processes a bitstream to expand data by reading a longword and performing bitwise shifts and conditional checks to determine how many bytes to read from a table and store into a destination buffer.
- `store_byte_and_rotate_long`: Loads a longword from a pointer in a0, stores its lowest byte into a word at the location pointed to by a5, sets the carry flag, and rotates the longword in d0 left by one bit before returning.
- `process_data_stream_2`: Performs a loop that shifts a value in d0 and calls another routine if d0 is zero, then enters a second loop that processes bytes from a source.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_table_entry_fields`: Multiplies an index by 22 to find an entry in a fixed table.
- `update_record_fields`: Moves four words from a structure into registers, calls two different routines, and then moves five words from registers and a value from a structure into a destination table.
- `call_routine_with_flag_word`: Stores the maximum positive 16-bit value in a flag word, calls a routine through a pointer held in the record, then writes a different fixed pattern into the same word.
- `store_position_and_offset_fields`: Stores two input words into record fields, then writes two derived fields made by adding 128 and 44 to those inputs.
- `copy_table_row_into_record`: Copies ten consecutive words from a table row, selected by one input and scaled by 22, into the record.
- `load_indexed_table_block`: Stores an input index in the record and looks up a block in a fixed table scaled by 46.
- `decrement_and_branch_if_negative`: Decrements a 16-bit value in d3 and returns if the result is negative, otherwise it continues to a nop instruction and returns.
- `set_buffer_address_and_value`: Calculates an address by adding an offset to a pointer from a structure, adds a constant to d1, stores d1 into a structure, and branches to another routine.
- `fill_strided_longwords_by_step`: Fills a run of entries in a record-owned buffer with longword values, starting from an input value and adding a step read from the record.
- `copy_table_block_into_record`: Copies sixteen longwords from a block of a global table, selected by an input index scaled by 64, into the record starting at a fixed offset.
- `copy_indexed_block_to_global`: Stores the global table's base address in a record field, then copies sixteen longwords from a block selected by an input index scaled by 64 back into the start of that table.
- `modify_status_and_pointer`: Performs a bitwise OR on d0 with a constant, performs a bitwise OR on d0 with itself, increments a pointer in a0 by eight, and branches to another routine.
- `reset_status_and_set_word_bits`: Clears a status byte at offset 149 and a longword at offset 3 of a structure in a5, calls clear_status_and_buffer, and performs a bitwise OR on a word at offset 96 of a structure in a0 using an index in d5.
- `clear_status_and_buffer`: Sets d0 to a value composed of two constants, clears bit 5 of a byte at offset 149 of the structure in a5, and clears a longword at offset 388 of the structure in a5.
- `return_absolute_difference_to_target`: Uses a record word when a status bit is clear and the value fifteen when it is set, subtracts a second record word from it, and returns the absolute value of the result.
- `clear_and_call_subroutines`: Clears a longword at offset 532 of a5, pushes a pointer from a0 to the stack, calls a subroutine, pops the pointer, calls another subroutine, clears d0, and branches to a location within itself.
- `push_and_call_subroutine`: Clears d0, branches to a location that pushes a word from offset 398 of a5 onto the stack, pushes d1 to the stack, calls a subroutine, pops a word from the stack into offset 398 of a5, and branches to another location or calls another subroutine.
- `wait_for_status_bit_clear`: Loops until bit 5 of a byte at offset 149 of the structure in a5 is clear.
- `set_status_flag_from_input`: When the input value is nonzero it calls a routine and sets bit five of a status byte.
- `store_four_parameter_words`: Stores the first input into two record fields, the second input into a third field, and a fourth value into a fourth field, then returns.
- `negate_and_reload_parameter`: Negates one stored word, copies a second stored word into the first field of its pair, and sets bit 5 of the status byte.
- `decrement_and_check_counter`: Decrements a byte at offset 404 of the structure in a5 and returns if it is negative; otherwise, it updates offset 404 with a byte from offset 405, decrements a word at offset 406, and conditionally clears bit 4 of offset 149 or calls another routine if bit 5 of offset 149 is set.
- `process_signed_count_loop`: If d0 is non-zero, it calculates a loop count based on the absolute value of d0, calls another routine, and then enters a loop that calls another routine a number of times determined by adding a word from offset 400 of the structure in a5 to a value derived from d0.
- `advance_position_and_call_routine`: Adds a value in d5 to a word at offset 368 of a5, calls a subroutine fifteen times in a loop, and then calls the same subroutine a variable number of times based on a word at offset 370 of a5 if that value minus sixteen is nonnegative.
- `add_signed_offset_to_bytes`: Adds a table byte, read from 192 bytes past the input pointer, into three consecutive bytes.
- `return_high_nibble_third_byte`: Reads three consecutive bytes and returns the high nibble of the last one.
- `compute_nibble_differences_into_buffer`: Writes a run of bytes into a global buffer, each one the high nibble of the difference between a routine code byte and the matching byte in a second buffer.
- `copy_strided_code_bytes`: Copies three bytes per pass from a fixed source region into a global buffer.
- `clear_strided_buffer_after_wait`: Waits until bit five of a status byte is clear, then zeroes three bytes per pass in a global buffer.
- `pack_nibbles_into_buffers`: Extracts the low nibble from each of two 16-bit words and stores them into the high nibble of two bytes in two separate destination buffers, repeating this process twice for each word via two loops.
- `setup_control_values`: Writes a value to offset 558 of a structure, performs bitwise operations on d5 and d0, writes d0 to offset 592 of the structure, moves a value from offset 412 of the structure into d0, calls four subroutines, and branches to an earlier instruction.
- `build_nibble_table_from_list`: Zeroes a sixteen-entry byte table, calling a routine after each write.
- `process_flagged_status_bytes`: Decrements a 16-bit value at offset 544 of a5 and resets it to fifteen if it becomes negative, then checks various bits in a byte at offset 162 of a5 to conditionally decrement other values at offsets 354, 352, 350, and 364, or calls another routine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 184 into d0, then merges 0 into d0.
- `invert_flagged_table_bytes`: Walks an eight-byte-spaced table from its last entry to its first, inverting the byte if a specific bit in a structure is set.
- `write_nibble_across_rows`: Sets a status flag, then writes a source byte to one buffer location.
- `step_counter_toward_limit`: When a status bit is set and two other conditions hold, it moves a byte field one step toward zero.
- `write_running_sums_until_end`: Walks pairs of longwords, writing a running total into the first of each pair and adding the second value into the total.
- `add_longword_from_table`: Loops through a table, adding a longword from the table to a register and incrementing the table pointer by eight each iteration until a negative value is encountered.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_blocks_and_set_marker`: Clears a run of 32-byte blocks, with the block count derived from the difference between a computed address and a fixed base.
- `scan_list_for_entry`: Calls another routine, then enters a loop that checks a value from a table and a bit in a global variable, calling another routine within the loop.
- `modify_register_values`: Saves several words from the stack, adds two values from offsets of a structure in a6 to d0 and d1, decrements d0 and d1, and branches to another routine.
- `initialize_interrupt_vectors_and_hardware`: Copies seven longwords from a memory location to a buffer, copies several words from custom hardware registers into the buffer, clears a range of memory using a loop, installs seven subroutine addresses into a table, and sets several hardware registers and a status bit.
- `initialize_list_head_entries`: Writes two words and a sentinel longword into a global buffer, then stores the buffer's address in the record and in a second pointer.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `perform_no_operation`: This routine performs a no-operation and passes execution to the subsequent routine.
- `nop_padding_instruction`: This routine does nothing and serves as a single-instruction padding element within the initialization sequence of initialize_interrupt_vectors_and_hardware.
- `mask_and_store_word`: Reads a word from an offset of thirty in a structure pointed to by a6, applies a bitwise AND mask, stores the result at offset one hundred fifty-six in the same structure, sets a word at offset three hundred eighty-four to all ones, and returns from an interrupt.
- `call_routine_in_loop`: Calls another routine and loops while the return value in d0 is not zero.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_nop_rts`: Small routine of 2 instructions that does nothing for one instruction, then returns.
- `check_status_bits`: Returns the result of masking a byte from a global variable with the constant twelve.
- `check_status_and_return`: Calls another routine and returns if the zero flag is not set.
- `wait_for_status_bit_clear_2`: Loops until bit 1 of a byte at a global offset is cleared.
- `find_containing_list_entry`: Scans a list of eight-word entries for the first one whose four bounds enclose the record's two coordinate values.
- `copy_masked_flags_with_limit`: Copies a masked value from one record word into a second word using a fixed bit mask.
- `process_cia_input`: Checks bit 3 of a byte from a CIA register, performs a loop to clear a bit in a CIA register, and uses a table to update a value in a structure before calling another routine.
- `call_and_branch`: Calls another routine and then branches to another location.
- `update_input_character_and_status`: Sets bit 6 of a status byte at offset 152 of a structure, stores an input byte into offset 448, and performs a table lookup to update offset 449.
- `append_character_to_ring_buffer`: Writes an input character into a 32-entry buffer at a running index, then advances the index and wraps it to the start after the last entry.
- `copy_text_with_control_codes`: Copies a null-terminated text string into a buffer, passing control characters and high-bit codes to routines and storing printable characters directly.
- `increment_byte_repeatedly`: Loads a longword from a pointer in a3 into d1, then performs three successive additions of a byte from d0 to itself, branching if the result is negative after each addition.
- `copy_text_until_asterisk`: Calls another routine, then copies bytes from a source to a destination until an asterisk is encountered, finishing with a space character.
- `copy_bytes_until_asterisk`: Copies bytes from a source to a destination using a counter in d0, and if the byte is an asterisk, it stops and writes an asterisk to the destination; otherwise, it continues until the counter reaches zero.
- `skip_asterisk_terminated_strings`: Finds the start of an asterisk-terminated string in a table using the low five bits of an input, then skips forward past as many strings as a second input gives.
- `clear_pending_value_and_advance`: Calls a routine, clears a longword field if it is nonzero, then calls a second routine.
- `parse_string_expression`: Parses a string to extract numeric values or special characters, performing bitwise manipulations on data in a table and updating a memory structure through a loop.
- `clear_byte_grid_rows`: Calls one routine, then zeroes a rectangular block of bytes in a buffer.
- `calculate_offset_via_multiplication`: Performs a series of arithmetic operations involving bit shifts, subtractions, and a multiplication of two values from a structure to calculate a result.
- `check_vowel_characters`: Calls routines and returns a value, with a referenced character list of vowels in upper and lower case.
- `sum_table_values_until_match`: Walks bytes from a pointer until one equals the input byte.
- `load_word_from_offset`: Loads a word from an offset of a0 and d3 into d0, then loads a word from a constant offset of a2 into d0 and branches.
- `prepare_ascii_conversion`: Sets d3 to three and branches to another routine.
- `set_d3_and_branch`: Sets d3 to zero and branches to another routine.
- `convert_integer_to_ascii_string`: Converts a signed longword in d2 into a sequence of five ASCII decimal digits and stores them in a table starting at an offset from a pointer calculated from d3, otherwise it stores two ASCII digits at that location and returns.
- `parse_decimal_digit_string`: Converts a string of decimal digit characters into a number by multiplying each digit by a weight from a table and summing the results.
- `handle_masked_input_flags`: Saves registers, masks a word from offset 30 of a6, stores the result in offset 156 of a6, and if bit 5 is set, updates offset 204 of a5 and calls a subroutine.
- `dispatch_jump_table`: Tests bit 7 of a value at an offset from a5 and branches to a location found via a table in a0, or tests bit 7 of another value and branches to a different location via a second table.
- `advance_and_compare_counter`: Calls a subroutine, checks bit 7 of a byte at offset 153 of a5, and if set, increments a 16-bit counter at offset 420 of a5 and a 16-bit counter at offset 422 of a5 after comparing the first against a value from a table.
- `modify_register_values_2`: Calculates a value based on bits from a CIA register and a structure, updates several fields in a structure with clamped arithmetic results, and modifies a byte in a5 using a bitmask derived from a structure word.
- `decrement_counter_loop_2`: Iterates through a list of bytes pointed to by a value in a5, decrementing each byte and calling routines if the byte is negative, then updates two bytes in a5 based on the final byte processed.
- `decrement_counter_loop`: Decrements a 16-bit counter in d0 until it reaches negative one.
- `clear_status_bit`: Clears bit 1 of a byte at offset 148 of a structure in a5.
- `initialize_status_and_fields`: Waits until bit one of a status byte is clear, then sets it.
- `set_status_and_initialize_record`: Sets bit one of a byte at offset 148 of a structure in a5, then stores a longword, copies a longword from a structure, clears several words in a structure in a6, and branches to another routine.
- `zero_words_and_trailing_byte`: Zeroes consecutive 16-bit words for half of an input length, then zeroes one trailing byte when the input length is odd.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_longword_at_pointer`: Stores a longword into the memory location pointed to by a4.
- `call_four_routines_in_loop`: Calls four subroutines in sequence and branches back to the start of the sequence if the negative flag is set after the fourth call.
- `set_marker_and_call_routines`: Sets bit 0 of a byte at offset 154 of a5, writes a longword to a location pointed to by a0, calls a subroutine, writes a second longword to a location pointed to by a0, calls another subroutine, and clears bit 5 of a byte at offset 155 of a5.
- `subtract_and_loop_on_memory`: Subtracts a longword from another longword loaded from memory, then enters a loop that continues while the result is not zero and a counter in d0 is not zero.
- `load_table_entry_into_record`: Stores an address and an index in the record, then reads a table row selected by the index and scaled by 32.
- `write_marker_and_poll_loop`: Writes a marker longword to a fixed location, then enters a loop that calls a subroutine, clears a bit in a byte at offset 6 of a1, and if a bit is set, calls another subroutine and updates a byte at offset 7 of a1.
- `access_data_table_via_index`: Uses a value in a data register as an index to read a 32-bit integer from a table and performs conditional branching based on the sign and value of the retrieved integer and a bit in a memory location.
- `access_table_and_call_routines`: Saves registers, performs a table lookup using a scaled index in d0, and if the result is positive and bit 5 of a byte at offset 155 of a5 is set, calls two subroutines.
- `return_zero_or_negative_one`: Returns zero or negative one depending on the path taken.
- `decrement_and_branch`: If the negative flag is set, it returns; otherwise, it performs two subtractions on a byte from a global variable and branches if either result is zero.
- `process_flagged_record_state`: Calls a subroutine, and if the zero flag is clear, stores a value at offset 5666 of a5 and performs a table lookup using a scaled index from offset 5634 of a5 to copy a longword.
- `update_hardware_state_and_process_timers`: Stores three values into a structure at offsets 3634, 3636, and 3642 from a5, modifies a bit in a status byte, and calls poll_hardware_status and initialize_timing_record.
- `write_cia_port_sequence`: Writes six byte values to one CIA location, clearing one bit at a time from bit six down to bit three and then restoring all bits.
- `poll_hardware_status`: Calls three different subroutines and then returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `wait_port_bit_with_timeout`: Polls a bit in a CIA-A port value and returns as soon as that bit is clear.
- `set_cia_b_port_mask`: Chooses one of two bit masks depending on a record flag, stores the mask in the record, and writes its inverse, limited to seven bits, to a CIA-B port register.
- `toggle_counter_and_flag`: Steps a global counter down by two and wraps it to eighteen when it goes negative.
- `execute_conditional_calls`: Calls a subroutine, and if the result is not zero, calls another subroutine and returns; otherwise, it calls a subroutine, sets d6 to negative one, calls a third subroutine, and returns.
- `check_disk_status_and_clear_flag`: Tests bit 4 of the CIAA Port A register and, if not set, clears a 16-bit word in a global variable before returning; otherwise, it branches to another routine.
- `update_offset_based_on_status`: Tests a word at offset 5798 of a structure, branches if positive, calls a subroutine, calculates a difference between a word at offset 5796 and 5798, and adds that difference to the word at offset 5798 if the difference is not zero.
- `execute_three_subroutines`: Calls three different subroutines in sequence and then decrements a word in register d6 within a loop that repeats until d6 is negative one.
- `busy_wait_fixed_count`: Counts down from a fixed value of 4616 as a busy-wait, then returns.
- `write_two_bytes_to_pointer`: Writes a byte from register d2 to the address in register a3, then immediately overwrites that same address with a byte from register d1 before returning.
- `process_signed_counter_and_flags`: Checks the sign of a word in d6, negates it if negative, and performs several bitwise operations involving a byte from a pointer and a byte from a record before decrementing d6 and returning.
- `initialize_timing_record`: Stores an input word and a set of fixed 16-bit constants into a record, stores two callback pointers, and copies a longword from a global variable.
- `configure_record_from_parameters`: Writes a series of fixed values into a record, including a flag word and a longword copied from another record.
- `wait_flag_bit_with_timeout`: Calls a routine, then polls bit one of a word at offset 30 of the structure in a6, giving up after sixteen thousand passes, and finally stores sixteen thousand in a word at offset 36 of the structure in a6.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `halt_in_endless_loop`: Enters an endless loop and never returns.
- `set_structure_values`: Writes several words to a structure pointed to by a6, loads a longword from a location pointed to by a5 into a6 offset 32, and branches to another routine.
- `process_data_table`: Iterates through a table to find specific markers, performs several bitwise and arithmetic operations involving values from a structure in a5, and copies data to a destination pointed to by a1.
- `return_minus_one`: Returns minus one as its result, without other work.
- `combine_masked_values`: Performs bitwise AND operations on three registers and returns the result of ORing the modified second and third registers.
- `copy_longwords_conditionally`: Calls another routine and, if it returns a non-zero value, copies up to 128 longwords from a source pointed to by a register to a destination calculated from a4 and d4, provided the destination address does not exceed a value masked from d6.
- `return_minus_one_short`: Loads negative one into register d0 and returns.
- `busy_delay_from_input`: Counts down from an input value as a short busy-wait, then returns.
- `fill_alternating_pattern_block`: Fills a block of longwords with an alternating bit pattern, adds one word of the same pattern at the end, and then writes another repeated longword pattern in a loop that makes calls on each pass.
- `rotate_and_call`: Rotates a longword in d0 left by one bit, calls another routine, and then rotates the longword in d0 right by one bit.
- `update_bitfield_values`: Performs bitwise operations on a longword in d0 using d3, stores the result at the address in a0, and then modifies a byte at offset one of that address based on the bits of the stored longword.
- `store_seven_bit_value`: Stores the input byte with its top bit cleared at the address held in an address register, then returns.
- `poll_until_zero_twice`: Calls a routine repeatedly until the value in d0 becomes zero, then performs the same process for a second set of values loaded from a pointer in a1, and returns.
- `build_crc16_lookup_table`: Builds a 256-entry lookup table of 16-bit checksum values by shifting each entry through eight bits with a 16-bit CRC polynomial.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `expand_bitstream_to_buffer`: Copies a block of data from a source to a destination, then processes a bitstream where bit-shifts and calls to another routine determine an expansion factor and an offset used to write bytes into a target buffer.
- `shift_and_store_byte`: Loads a longword from a pointer in a0, stores a byte from it into d7 and a word into a5, then performs a logical shift left on d0.
- `process_data_stream_3`: Performs a loop that shifts a value in d0 and calls another routine if the value is zero, then executes a block that copies bytes from a source to a destination and may jump to an address in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores the address of this routine into a global variable and then executes trap zero.
- `wait_for_hardware_flag`: Polls bit 6 of a hardware register, then executes a sequence of subroutines based on the results of various conditional branches and bit tests, finally returning via an interrupt return.
- `configure_hardware_and_copy_data`: Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range.
- `restore_saved_hardware_words`: Disables DMA and interrupt bits with a clear-all value, then writes two saved control words back to those registers.
- `check_and_increment_byte`: Checks a byte at an offset; if it is negative one, it increments a global variable and loops until the byte is zero.
- `decrement_global_word`: Checks a byte at offset six of the structure in a6; if it is not negative one, it decrements a word at a memory location calculated from the program counter and then checks the byte again.
- `clear_memory_range`: Fills a range of memory starting at a specific address with zeros using a loop.
- `parse_and_copy_data`: Reads a byte from a source, checks if it is a termination marker or a specific control value, and either returns, skips a sequence, or copies eight bytes from a table to a destination buffer with a calculated offset.
- `modify_registers`: Performs bitwise OR operations on registers d0, d4, and d4, and compares a byte at a pointer in a6 with a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_hardware`: Loads the address of a custom chip into a register, clears a longword at a specific location, and calls seven different subroutines before returning.
- `copy_entry_fields_to_buffer`: Walks a null-terminated table of record pointers, copying one byte from twelve records spaced 36 bytes apart into a global buffer for each entry.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a3 one byte at a time, using a dbra loop counter, and returns.
- `forbid_and_configure_hardware`: Makes a system library call, then stores interrupt and DMA words from the record with the top bit forced on.
- `fill_accumulating_longwords`: Writes a running longword value into a table, storing its high and low words at a stride of eight bytes.
- `update_library_function_pointers`: Calls a library routine to obtain a pointer, uses it to fetch a value, calls another library routine, and then stores two constant values and two routine addresses into a structure at a specific offset.
- `tiny_or`: Small routine of 1 instruction that merges (a0) into d1.
- `tiny_and`: Small routine of 1 instruction that masks d0 with d0.
- `scale_table_pairs_by_four`: Multiplies both words of each of forty-four pairs in a fixed table by four.
- `build_shifted_row_masks`: Fills sixteen rows of eight longwords with values taken from a global block.
- `build_reciprocal_table`: Builds a table of 16-bit reciprocals by dividing a fixed large constant by each even divisor from 256 up to just under 8192 and storing the quotient.
- `run_frame_loop_with_calls`: Runs an endless loop that waits until a masked timer value matches a fixed value, then makes nine calls and one indirect call through a table.
- `increment_text_counter`: Loads a pointer from a table, performs multiplications on a value from a global variable, calls another routine, and increments global variables if a counter reaches thirteen.
- `render_text_glyph_columns`: Looks up each character in a font table, indexed by its offset from the first printable character, and copies its eight column bytes into a bitmap buffer.
- `tiny_ori`: Small routine of 1 instruction that merges 90 into d6.
- `update_text_and_score`: Calculates a character index based on values in a structure, updates a global string with a numeric value, and copies font data to a destination buffer before calling render_text_glyph_columns.
- `copy_row_blocks_between_buffers`: Copies blocks of rows between two buffers, moving longwords in groups for each row.
- `clear_mirrored_table_rows`: Zeroes ten longwords in each of two table rows, one selected by an index and the other by its mirror from the far end.
- `advance_double_buffer_offsets`: When a countdown reaches zero it resets it to six, advances a masked offset forward and a second offset backward by two, and copies table words selected by those offsets into output locations.
- `scan_leading_text_marker`: When a countdown reaches zero it resets it to six, then reads a string pointer and skips leading characters that match a marker letter, setting a fixed value for each one.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `tiny_ori_3`: Small routine of 1 instruction that merges 166 into d6.
- `combine_indexed_table_values`: Writes a 16-bit control value to a custom-chip register, then reads several table entries through index offsets and combines them with arithmetic before calling a routine.
- `modify_byte_and_registers`: Moves a byte from a memory location at an offset of a6 and d0 into d6, then performs several bitwise OR operations on d0.
- `copy_table_pairs_with_index`: Walks forty-four table pairs, reading each pair and writing computed values into an output table.
- `wait_then_update_table_outputs`: Waits until bit 6 of a byte at offset 2 of a structure is clear, then enters a loop that performs table lookups and arithmetic to update several fields in the structure, including a pointer, two 16-bit values, and a 32-bit value, before calling another routine and updating a global linked list.
- `set_bit_and_modify_register`: Performs a bitwise OR on d0 and sets a bit in a byte at a memory location pointed to by a6 and decremented.
- `prepare_interpolation_parameters`: Compares two words in d0 and d2, swaps registers if necessary, and calls another routine to process the values.
- `interpolate_span_endpoints`: Computes interpolated values for two endpoints using linear interpolation based on several sign and boundary conditions, then updates two values in a table via pointers and decrements a global counter.
- `initialize_record_after_wait`: Writes a control word with the set bit and one enable bit, waits for a status bit to clear, then sets several record fields to constants, including all-ones and small limit values.
- `initialize_timer_record_fields`: Waits until a status bit is clear, then sets several record fields to constants and to a value computed from a global.
- `set_timer_record_from_global`: Waits until a status bit is clear, then sets a record longword to a fixed constant, clears a word, copies a global longword into the record, and sets a timer word to 10004.
- `swap_globals_and_store_halves`: Swaps two global longwords, then stores the low and high words of the earlier value into two fixed memory words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `remap_keyboard_serial_code`: Adjusts a keyboard character code through a long chain of comparisons that add or subtract ten to selected values.

### logged disk load from Hunter.adf

- `jump_with_value`: Loads a pointer from an offset of 58 in the structure in a5 into a0, moves the word 5708 into d0, and jumps to a long address in d0.
- `enable_dma_and_interrupts`: Loads the custom hardware base address into a6, enables all DMA and interrupt channels by writing 32767 to the DMACON and INTENA registers, and performs a software interrupt.
- `copy_memory_block_2`: Copies 64 words from a source address in a pointer to a destination address in a pointer, decrementing a counter in a global variable for each block of eight words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_a5_state_loops`: Sets a byte and a word at fixed offsets in a structure addressed by a5 and runs nested polling loops that wait for those bytes to change, calling other routines during each wait.
- `toggle_flag_and_update_table`: Toggles a flag word in the structure addressed by a5, and when the flag becomes zero it advances a counter by two and writes one value into four table entries spaced sixteen bytes apart at a fixed offset of 168 bytes.
- `toggle_flag_and_fill_buffer`: Toggles a flag word at a fixed offset in the a5 structure, and when it becomes zero and a counter stays non-negative after subtracting two, it writes one value into four entries of the table at offset 168 with a stride of sixteen bytes.
- `split_nibbles_into_table`: Calls a routine 32 times, then iterates over a table of 32 words, splitting each word into four-bit nibbles and storing them with their negated values into a destination structure.
- `find_max_nibble`: Scans the four nibbles of a 16-bit word and keeps the largest one in a register.
- `update_timer_table_entries`: Decrements a countdown at a fixed offset in the a5 structure and, when it reaches zero, resets it to four and walks 32 timer records, decrementing each active count and otherwise comparing a masked value against a stored word and updating that word.
- `decrement_and_reset_table`: Decrements a countdown at a fixed offset in the a5 structure and, when it reaches zero, resets it to four and walks 32 words, subtracting one from each nonzero 4-bit field in the high, middle and low positions.
- `initialize_hardware_and_interrupts`: Configures custom hardware registers, sets up interrupt vectors in a table, writes values to CIA registers, and executes two subroutines before populating a table with four entries using a loop.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `build_register_value_table`: Fills a table with four groups of paired words holding incrementing register-style values and the high and low halves of a fixed address, then writes 80 words of incrementing values interleaved with zeros.
- `restore_saved_custom_registers`: Writes the disable-everything mask to the dma and interrupt enable registers relative to a6, then restores previously saved values into those registers and copies a pointer from a global structure into a data pointer slot.
- `write_interrupt_request`: Writes the constant 255 to one variable and 32 to a global variable, which is the interrupt request register, then returns from interrupt.
- `configure_cia_interrupt_control`: Writes the constant 255 to a global variable and 8192 to another global variable to configure the CIA interrupt control state.

### Hunter.adf bytes 34304-38912 (code never loaded by a reference run)

- `initialize_zero_buffer`: Clears a series of memory locations by performing multiple zeroing operations on a register and then jumps to a location via a pointer.

### Hunter.adf bytes 56320-57856 (code never loaded by a reference run)

- `store_byte_and_return`: Stores a byte from the address in a4 into memory and returns.
