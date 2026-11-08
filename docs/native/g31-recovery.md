# Game 31 (Battleships): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00031). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903824 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (1 by rule, 20 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 449 of 449 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 60416 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 598 of 598 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 298 |
| code | 2586 |
| data | 445771 |
| padding | 1 |
| unused | 455168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Battleships.adf:bootblock | 1 | 67 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 334 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 197 | 4 |
| Battleships.adf bytes 11264-12288 (code never loaded by a reference run) | 1 | 0 | 0 |
| Battleships.adf bytes 12800-44544 (code never loaded by a reference run) | 1 | 0 | 0 |
| Battleships.adf bytes 53248-80896 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Battleships/Battleships.adf: custom-format (non-DOS) disk, 869 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 0009) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 295 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 118 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Battleships/Battleships.adf: bytes 11264-12288 look like code no run loaded: recovered position-independently (load address unknown)
- Battleships/Battleships.adf: bytes 12800-44544 look like code no run loaded: recovered position-independently (load address unknown)
- Battleships/Battleships.adf: bytes 53248-80896 look like code no run loaded: recovered position-independently (load address unknown)
- Battleships/Battleships.adf: 3 logged loads cover 60416 bytes of the image
- Battleships/Battleships.adf: blocks 2-12 (bytes 1024-6656) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.74)
- Battleships/Battleships.adf: blocks 90-91 (bytes 46080-47104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.13)
- Battleships/Battleships.adf: blocks 94-94 (bytes 48128-48640) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.20)
- Battleships/Battleships.adf: blocks 174-174 (bytes 89088-89600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.77)
- Battleships/Battleships.adf: blocks 182-182 (bytes 93184-93696) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.44)
- Battleships/Battleships.adf: blocks 184-184 (bytes 94208-94720) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- Battleships/Battleships.adf: blocks 188-189 (bytes 96256-97280) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- Battleships/Battleships.adf: blocks 198-205 (bytes 101376-105472) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.02)
- Battleships/Battleships.adf: blocks 212-276 (bytes 108544-141824) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.13)
- Battleships/Battleships.adf: blocks 284-312 (bytes 145408-160256) outside the filesystem hold data, not code (opcode density 0.053, entropy 5.27)
- Battleships/Battleships.adf: blocks 342-465 (bytes 175104-238592) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.58)
- Battleships/Battleships.adf: blocks 470-532 (bytes 240640-272896) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.68)
- Battleships/Battleships.adf: blocks 534-536 (bytes 273408-274944) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.27)
- Battleships/Battleships.adf: blocks 542-550 (bytes 277504-282112) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.72)
- Battleships/Battleships.adf: blocks 552-596 (bytes 282624-305664) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.81)
- Battleships/Battleships.adf: blocks 604-660 (bytes 309248-338432) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- Battleships/Battleships.adf: blocks 664-665 (bytes 339968-340992) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.10)
- Battleships/Battleships.adf: blocks 667-668 (bytes 341504-342528) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.12)
- Battleships/Battleships.adf: blocks 670-671 (bytes 343040-344064) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.06)
- Battleships/Battleships.adf: blocks 673-674 (bytes 344576-345600) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.89)
- Battleships/Battleships.adf: blocks 679-680 (bytes 347648-348672) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.39)
- Battleships/Battleships.adf: blocks 682-683 (bytes 349184-350208) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.42)
- Battleships/Battleships.adf: blocks 685-686 (bytes 350720-351744) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.37)
- Battleships/Battleships.adf: blocks 688-689 (bytes 352256-353280) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.09)
- Battleships/Battleships.adf: blocks 695-696 (bytes 355840-356864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- Battleships/Battleships.adf: blocks 698-699 (bytes 357376-358400) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- Battleships/Battleships.adf: blocks 701-702 (bytes 358912-359936) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.27)
- Battleships/Battleships.adf: blocks 704-705 (bytes 360448-361472) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.92)
- Battleships/Battleships.adf: blocks 710-720 (bytes 363520-369152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.18)
- Battleships/Battleships.adf: blocks 725-725 (bytes 371200-371712) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.97)
- Battleships/Battleships.adf: blocks 728-729 (bytes 372736-373760) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.10)
- Battleships/Battleships.adf: blocks 731-732 (bytes 374272-375296) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.12)
- Battleships/Battleships.adf: blocks 734-735 (bytes 375808-376832) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.06)
- Battleships/Battleships.adf: blocks 737-738 (bytes 377344-378368) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.89)
- Battleships/Battleships.adf: blocks 743-744 (bytes 380416-381440) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.39)
- Battleships/Battleships.adf: blocks 746-747 (bytes 381952-382976) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.42)
- Battleships/Battleships.adf: blocks 749-753 (bytes 383488-386048) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.95)
- Battleships/Battleships.adf: blocks 759-760 (bytes 388608-389632) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- Battleships/Battleships.adf: blocks 762-763 (bytes 390144-391168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- Battleships/Battleships.adf: blocks 765-766 (bytes 391680-392704) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.27)
- Battleships/Battleships.adf: blocks 768-769 (bytes 393216-394240) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.92)
- Battleships/Battleships.adf: blocks 774-784 (bytes 396288-401920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.18)
- Battleships/Battleships.adf: blocks 790-831 (bytes 404480-425984) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.43)
- Battleships/Battleships.adf: blocks 842-842 (bytes 431104-431616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- Battleships/Battleships.adf: blocks 854-863 (bytes 437248-442368) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.53)
- Battleships/Battleships.adf: blocks 870-900 (bytes 445440-461312) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.85)
- Battleships/Battleships.adf: blocks 917-1089 (bytes 469504-558080) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.48)
- 3 never-loaded region(s), 60416 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Battleships.adf:bootblock

- `initialize_and_poll_library_structures`: Initializes a structure at offset 28 with a command word of 2, a size of an address, a pointer of 10752, and a value of an address, then calls the library entry at -462 and polls the entry at -468 until it returns nonzero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `install_traps_and_handlers`: Writes a trap vector and handler address into global variables, fills a table with sixteen-byte entries in a loop, then calls several routines and enters supervisor mode after a library call.
- `toggle_flag_by_value`: XORs bit 7 of the byte at global variable offset a fixed address when d2 holds 12 or 15, and XORs bit 1 of that same byte when d2 holds 13 or 14, otherwise returning without touching it.
- `increment_and_loop_counter`: Increments a 16-bit counter at a global variable and resets it to zero when it reaches six, then repeatedly calls another routine while the returned value is not one.
- `decrement_if_nonzero`: Subtracts 273 from the 16-bit value pointed to by a0 when that value is not zero, otherwise leaving it unchanged.
- `increment_and_copy_words`: Increments a 16-bit global counter and resets it to zero when it reaches six, then copies seven 16-bit words from the table at 38342 into the destination starting at 38346, advancing the source by two and the destination by four per word.
- `increment_if_unequal`: Adds 273 to the 16-bit value pointed to by a0 when it differs from the 16-bit value pointed to by a1, otherwise leaving it unchanged.
- `call_and_indirect_jump`: Calls the routine at a fixed address, stores 33664 in a global variable, then jumps indirectly through that value plus offset -30.
- `multiply_and_library_call`: Multiplies d0 by 6, sets up a library call with d1 through d6 and d0, and returns.
- `increment_and_copy_buffer`: Increments a 16-bit counter at offset a fixed address and returns unless it reaches 2, then clears it and copies 47 bytes from a source table into a destination buffer.
- `configure_and_call_sequence`: Sets a global variable to eighty-four and, four times in a row, stores a pair of word values into fixed offsets of another routine's structure before calling a routine that processes a byte sequence.
- `index_table_and_call_library`: Clears d1 through d6, multiplies d0 by 8 to index a table, adds the fetched value plus 3 to a global counter, then makes a library call.
- `process_byte_sequence`: Reads successive bytes from a source pointed to by a2 and calls another routine for each until the byte equals minus one hundred and six or the loop counter reaches sixty-four, then returns.
- `handle_library_and_counters`: Sets a global variable to eighty-four, makes two library calls with different parameter words, then increments two separate byte counters and, when each reaches its threshold, resets it and reads a byte that either resets a pointer or invokes another routine.
- `increment_counters_and_call_libraries`: Increments a counter at offset a fixed address and returns unless it reaches 2, then clears it and calls one library, increments a counter at offset a fixed address, and returns unless it reaches 360.
- `open_library_and_register_handlers`: Opens a library, stores its base pointer in a global variable, then registers six handlers and makes two more library calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_interrupts_and_clear_globals`: Enables interrupts via the interrupt enable register, stores 65535 in a global variable, zeroes four audio volume registers and three global variables, then returns.
- `iterate_structures_on_counter`: Increments a global counter and resets it to zero when it reaches six, then calls another routine for each of four structures whose byte at offset 3 is nonzero, otherwise calls that routine while a flag byte is nonzero.
- `dispatch_byte_nibbles`: Examines nibbles of a byte at offset 3 of the structure in a6 and dispatches to branches that adjust a 16-bit value at offset 16, look up a value in a table, or call another routine, returning otherwise.
- `store_indexed_table_values`: Reads a 32-bit entry from a table indexed by the high nibble of a byte at offset 2 of the structure in a6, then stores the fetched pointer and two adjacent words into the structure and adds a value from the entry to a running counter before writing results through pointers in a5.
