# Game 2530 (virocop (1995)(graftgold)[cr hlm]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02530). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2704336 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 20 of 20 named (0 author symbols), 20 of 20 described (5 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 350 of 350 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 82432 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 400 of 400 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8215 |
| code | 1452 |
| data | 2250240 |
| padding | 13 |
| unused | 444416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf:bootblock | 1 | 58 | 3 |
| logged disk load from Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf boot-block load | 1 | 169 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 160 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf bytes 469504-551936 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1709 non-empty blocks: matched against track-loaded segments
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: custom-format (non-DOS) disk, 1358 non-empty blocks: matched against track-loaded segments
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: custom-format (non-DOS) disk, 1339 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 544 of the 907 PCs the canonical run logged (66685 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 4 64 KB pages each with >= 29491 distinct PCs executed, 65816 of its 66685 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 555 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: the boot block reads 8704 bytes from disk offset 892416 to an address (found statically); recovered as a unit
- runtime segment an address: 154 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: bytes 469504-551936 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 displacement
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: 2 logged loads cover 91136 bytes of the image
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 259-499 (bytes 132608-256000) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.04)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 501-507 (bytes 256512-260096) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.60)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 509-631 (bytes 260608-323584) outside the filesystem hold data, not code (opcode density 0.034, entropy 6.15)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 633-633 (bytes 324096-324608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.86)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 635-635 (bytes 325120-325632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 637-774 (bytes 326144-396800) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.90)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 777-796 (bytes 397824-408064) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.12)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.80)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 801-908 (bytes 410112-465408) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.93)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 910-910 (bytes 465920-466432) outside the filesystem hold data, not code (opcode density 0.012, entropy 1.78)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 915-915 (bytes 468480-468992) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1079-1345 (bytes 552448-689152) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.57)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1347-1387 (bytes 689664-710656) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.16)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1389-1491 (bytes 711168-763904) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.59)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1493-1493 (bytes 764416-764928) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.61)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1495-1623 (bytes 765440-831488) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.12)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1625-1625 (bytes 832000-832512) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.96)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1627-1641 (bytes 833024-840704) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.95)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1643-1643 (bytes 841216-841728) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.25)
- virocop (1995)(graftgold)[cr hlm](disk 1 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf: blocks 1645-1742 (bytes 842240-892416) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.18)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.35)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 6-6 (bytes 3072-3584) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.36)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 8-8 (bytes 4096-4608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.89)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 78-166 (bytes 39936-85504) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.72)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 168-168 (bytes 86016-86528) outside the filesystem hold data, not code (opcode density 0.012, entropy 1.25)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 170-276 (bytes 87040-141824) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.17)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 278-320 (bytes 142336-164352) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.02)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 322-390 (bytes 164864-200192) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.09)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 392-488 (bytes 200704-250368) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.91)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 490-562 (bytes 250880-288256) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.09)
- virocop (1995)(graftgold)[cr hlm](disk 2 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 2 of 3).adf: blocks 1290-1373 (bytes 660480-703488) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.07)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.29)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 6-6 (bytes 3072-3584) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 8-8 (bytes 4096-4608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.88)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 96-200 (bytes 49152-102912) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.73)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 334-370 (bytes 171008-189952) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.12)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 372-434 (bytes 190464-222720) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.93)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 436-586 (bytes 223232-300544) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.06)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 988-1024 (bytes 505856-524800) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.11)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 1264-1284 (bytes 647168-657920) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.14)
- virocop (1995)(graftgold)[cr hlm](disk 3 of 3)/Virocop (1995)(Graftgold)[cr HLM](Disk 3 of 3).adf: blocks 1286-1353 (bytes 658432-693248) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.14)
- 1 never-loaded region(s), 82432 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 8 packed track region(s), 959488 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf:bootblock

- `initialize_bootloader`: Initializes memory allocation for boot operations, sets up hardware registers including DMA and interrupt controllers, and configures color palette entries.
- `frag_movea_lea_move_dbra_bra_jmp`: Short code fragment that copies a5 to a7, then copies a5 to a0, then loads the address of a location in the code into a1, then copies 255 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then jumps to another point in the code, then loads the address of a fixed memory address into a7, then jumps to another point in the code.
- `setup_io_request`: Initializes an I/O request structure with specific command and address values, then executes a device I/O operation using the exec library.

### logged disk load from Virocop (1995)(Graftgold)[cr HLM](Disk 1 of 3).adf boot-block load

- `initialize_tables`: Initializes multiple tables by clearing them and filling them with data from a source buffer, then performs bit manipulation and data copying operations based on values read from those tables.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `perform_arithmetic_sequence`: Executes a loop that processes sequential word values from memory, performing arithmetic operations including multiplication and addition to update intermediate results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `swap_dma_buffer_state`: Saves the DMA control register, calls an initialization routine, and stores a saved pointer.
- `initialize_dma_channels`: Writes a value into two 16-bit fields of a structure and clears the DMA control registers between offsets 288 and 416, then returns.
- `process_keyboard_input`: Reads a bit pattern from the keyboard serial register and clears it, then compares the rotated value to the character '2'. If it matches, it writes the character '<' to a byte in RAM and clears a global variable; otherwise it writes '2' to that byte and stores the value 32 in the same global variable, then returns.
- `initialize_and_wait`: Initializes global variables, then waits in a loop until a condition becomes true and a joystick fire button is pressed, after which it sets another flag and waits for a counter to reach three before returning.
- `frag_btst_beq_rts`: Short code fragment that tests bit 5 of a fixed memory address, then branches if the last result was zero, then returns.
- `frag_tst_bne_move_subi_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies 1 to a fixed memory address (2 times in a row), then copies 700 to a fixed memory address, then subtracts 1 from a fixed memory address, then returns.
- `frag_movea_bsr_rts`: Short code fragment that copies an address to a0, then copies the memory at an offset from a0 to a4, then calls another routine, then returns.
- `frag_cmpi_beq_subi_bne_move_rts_addi`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 4 to a fixed memory address, then copies 0 to a fixed memory address, then returns, then adds 4 to a fixed memory address, then returns.
- `check_and_update_counter`: Tests a global variable for zero and branches if true, then tests another global variable for non-zero and branches if false.
- `frag_tst_beq_subi_cmpi_rts_move`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then subtracts 273 from a fixed memory address, then compares a fixed memory address with 819, then branches if the last result was zero, then returns, then copies 3 to a fixed memory address, then copies 0 to a fixed memory address, then returns.
- `copy_table_entries`: Copies ten bytes from a source table into a destination table at offsets 0, 80, 160, 240, 320, 400, 480, 560, 640 and 700 for each of 70 source entries, advancing the source pointer past the terminator.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_code_segments`: Copies code segments from one memory location to another, performing a loop to transfer multiple long words between addresses.
