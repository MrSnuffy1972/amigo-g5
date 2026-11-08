# Game 451 (Fightin' Spirit): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00451). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4506672 top-level bytes in 120 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 72 of 72 named (0 author symbols), 72 of 72 described (8 by rule, 62 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 768 of 770 (99.74%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2527 of 2529 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 301208 |
| code | 7690 |
| data | 3965695 |
| padding | 28815 |
| unused | 203264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:LightForce.exe | 2 | 235 | 8 |
| run | 2 | 117 | 2 |
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:FS | 4 | 1612 | 47 |
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Installer | 2 | 267 | 8 |
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 279 | 5 |

## Cases handled

- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Logo.bin: high-entropy data file (7.85 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Intro.dat: high-entropy data file (7.67 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Intro.bin: high-entropy data file (7.75 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharE.int: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:Loader.bin: high-entropy data file (7.85 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharC.int: high-entropy data file (7.79 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:Main.dat: high-entropy data file (7.87 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharA.int: high-entropy data file (7.79 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharL.int: high-entropy data file (7.77 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:Engine0.bin: high-entropy data file (7.81 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharJ.int: high-entropy data file (7.80 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharH.int: high-entropy data file (7.83 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharF.int: high-entropy data file (7.74 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharD.int: high-entropy data file (7.73 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharB.int: high-entropy data file (7.82 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:Engine1.bin: high-entropy data file (7.81 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharK.int: high-entropy data file (7.83 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:Main.bin: high-entropy data file (7.85 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharI.int: high-entropy data file (7.79 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 2 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 2 of 5).adf:CharG.int: high-entropy data file (7.82 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageD.gfx: high-entropy data file (7.87 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharC.gfx: high-entropy data file (7.90 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharA.gfx: high-entropy data file (7.89 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageB.gfx: high-entropy data file (7.87 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageC.ani: high-entropy data file (7.77 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageA.ani: high-entropy data file (7.79 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharB.snd: high-entropy data file (7.61 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageC.snd: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharD.gfx: high-entropy data file (7.88 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharN.gfx: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageA.snd: high-entropy data file (7.73 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharB.gfx: high-entropy data file (7.88 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageC.gfx: high-entropy data file (7.87 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageD.ani: high-entropy data file (7.69 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageA.gfx: high-entropy data file (7.89 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharN.aic: high-entropy data file (7.76 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageB.ani: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageG.snd: high-entropy data file (7.74 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageG.gfx: high-entropy data file (7.87 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharG.gfx: high-entropy data file (7.88 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageD.snd: high-entropy data file (7.75 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharC.snd: high-entropy data file (7.63 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageG.ani: high-entropy data file (7.64 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:CharA.snd: high-entropy data file (7.64 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 3 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 3 of 5).adf:StageB.snd: high-entropy data file (7.74 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageL.snd: high-entropy data file (7.69 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageE.ani: high-entropy data file (7.69 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageL.gfx: high-entropy data file (7.83 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharK.gfx: high-entropy data file (7.86 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageH.snd: high-entropy data file (7.74 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharH.snd: high-entropy data file (7.61 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageL.ani: high-entropy data file (7.70 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageH.gfx: high-entropy data file (7.85 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharH.gfx: high-entropy data file (7.89 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageE.snd: high-entropy data file (7.74 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageF.gfx: high-entropy data file (7.84 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharF.gfx: high-entropy data file (7.88 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:STAGEH.ANI: high-entropy data file (7.70 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharL.snd: high-entropy data file (7.64 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageE.gfx: high-entropy data file (7.87 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageF.ani: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageK.snd: high-entropy data file (7.71 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharK.snd: high-entropy data file (7.65 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharL.gfx: high-entropy data file (7.88 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:StageF.snd: high-entropy data file (7.77 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 4 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 4 of 5).adf:CharE.gfx: high-entropy data file (7.89 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:CharM.gfx: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:CharJ.snd: high-entropy data file (7.62 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:Endpart.dat: high-entropy data file (7.76 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:Engine2.bin: high-entropy data file (7.81 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:CharJ.gfx: high-entropy data file (7.89 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:StageM.snd: high-entropy data file (7.75 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:StageM.gfx: high-entropy data file (7.81 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:StageI.snd: high-entropy data file (7.76 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:CharM.int: high-entropy data file (7.78 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:StageI.gfx: high-entropy data file (7.88 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:CharI.gfx: high-entropy data file (7.84 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:Endpart.bin: high-entropy data file (7.72 bits/byte), classified as packed data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 5 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 5 of 5).adf:StageI.ani: high-entropy data file (7.72 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 131 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:LightForce.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:LightForce.exe

- `reverse_and_copy_struct_blocks`: Walks a linked list of structures from a global variable, and for each one whose first four bytes equal a fixed address it reverses a block of 32 bytes into a stack buffer and copies the remaining bytes, then calls the three routines at offsets 128, 256 and 384 of the allocated block and a final cleanup routine.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: It contains only a single return instruction and does nothing else.
- `fill_figure_table`: Fills a stack buffer with values, then loops through a key list writing bit masks and derived values into a table, and finally restores the stack and returns.
- `free_and_merge_node_lists`: It clears the header word of a linked list at a4, then for each node walks a self-referential chain and adds the resulting offset to the corresponding slot in the structure at a5.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_branch`: Copies the value in register d1 into register d0 and then branches into the middle of another routine.

### Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:FS

- `initialize_game_structure_2`: Clears two long variables in a global structure, then calls a routine that returns a pointer to a game structure and checks a flag at offset 172 of that structure.
- `store_a1_and_return`: It stores the 32-bit value from a1 into the global variable at offset 24d4, then returns via rte.
- `initialize_game_structure`: Initializes a game structure by storing routine addresses and data values at many of its offsets, copies a block of words from a table into the structure, and clears several fields.
- `process_and_dispatch_game_state`: Copies a block of 16-bit words from a source table to a destination table using a computed count, then calls a scaled-value routine and a routine pointed to by a vector table.
- `return_if_null_call`: Returns immediately when the structure pointer is null, otherwise loads a library base and calls a routine with a table address and a count.
- `read_and_send_key`: Executes trap 1 to read the keyboard status from the system library, then calls the queue handler with the byte at offset 150 of the structure in a6.
- `set_d7_ten`: Loads the value ten into d7 and branches into the middle of the following routine.
- `round_result_up_eight_2`: Copies two 16-bit words from indexed table locations into memory, calls another routine, and returns the input value rounded up to a mult.
- `round_result_up_eight`: It copies two 16-bit words from indexed table locations into memory, calls another routine, and returns the input value rounded up to a multiple of eight when non-zero.
- `build_indexed_table`: Sets up stack space, builds a table of offsets and pointers, then loops calling a routine indexed by a value from a0 until a condition is met, and finally restores the stack and returns.
- `return_constant_one_2`: It returns the constant value one to the caller.
- `jump_to_routine`: Jumps to another routine and does not return.
- `jump_to_routine_2`: Jumps to the routine located at address 2188 and never returns to this point.
- `jump_to_routine_3`: Jumps to another routine and never returns from here.
- `return_immediately`: It is an empty routine that returns immediately.
- `jump_to_routine_4`: Jumps unconditionally to another routine and does not return.
- `return_constant_one`: It doubles a value from a0 twice, advances the pointer by four times that value, then loads one into d0 and returns.
- `copy_and_clamp_values`: It contains several branches, including loops, and copies data between memory locations, but it makes no subroutine calls.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `interleave_and_store_bytes`: It runs a nested loop that bit-reorders and interleaves two input words, storing the eight resulting bytes into a table at fixed offsets for each iteration.
- `copy_and_clear_blocks`: Copies four bytes from a source pointer in a0 to a destination pointer in a1 eleven times, then calls the scaled-value routine with the word at offset 22 of the source.
- `load_byte_scaled`: Loads a pointer from a global variable and reads a byte from offset 151 of its structure into d0, then calls the scaled-value routine with that byte.
- `process_clamped_values`: Reads a byte from the source pointer in a0, rotates it, and calls the scaled-value routine eight times, then decrements the loop counter in d0 and repeats until the counter is exhausted.
- `clamp_scaled_value`: It computes a scaled value from d2 using branches based on the bits of d1, clamps the result between the bounds in d5 and d6, and stores the clamped byte to a1.
- `adjust_table_fields`: It walks a table of 16-bit words at a0, decrementing a field while it stays within range and masking it, and stops when it reaches the terminator.
- `clear_structured_slots`: It clears eight 32-bit slots in a structure at a6 offset 144 using a loop, then returns.
- `trap_and_call_handler`: Saves registers, issues a trap, writes a value to a hardware register, and calls another routine before restoring the registers and returning.
- `read_byte_then_trap`: Reads a byte from offset 150 of the structure in a6 and calls the scaled-value routine, then loads pointers from the structure and issues a trap to copy a block.
- `invoke_trap_via_dmacon`: Saves registers, issues a CPU trap with the value 54, clears the DMA channels via the control register, calls a helper routine, and returns.
- `xor_bytes_then_trap`: Calls a scaled-value routine, then reads two bytes from the structure, XORs them, and issues a trap unless a status bit is clear.
- `store_handler_pointer`: Loads the address of a routine into a0, clears d7, and stores that address into a global variable at offset 84, then falls through into the following routine.
- `call_indexed_handler`: Loads the address of the AGA-Handler V40.12 routine, indexes into a table using a 16-bit value from a global variable, calls the selected routine, and returns.
- `initialize_graphics_and_open_devices`: Initializes the graphics and intuition libraries, then repeatedly opens a device and copies its structure fields into a global table until a device cannot be opened.
- `process_compressed_data`: Stores a modified byte from a0 into a structure, then repeatedly calls a routine from a table until it returns non-zero, calls two other routines, and finally calls a routine that copies and clamps values before disabling a flag and returning.
- `call_library_set_flag`: Fetches the base of a library from a global variable and calls a routine at offset -150, then stores the value 2 in the byte at offset 2 of that structure.
- `read_input_process_response`: Stores a character from the input at offset 8 minus 49 into the byte at offset 852 of a global structure, then calls a routine at offset -30.
- `copy_block_and_unpack_bits`: Copies a block of long words from one memory area to another, then reads a 16-bit value from a source pointer and writes its bits, in little-endian order, as consecutive bytes to a destination pointer, repeating the process until the source pointer reaches the destination pointer.
- `initialize_library_request`: Initializes a device library interface by filling a request structure with fields and, if a matching entry exists in a global table, copies that entry's data into the request and clears a block of eight longwords, then copies a variable-length block from a source table into a destination table.
- `copy_word_pair_loop`: Sets up two pointers, then loops eight times copying a word from one pointer to another while advancing both, and finally branches into the middle of the preceding routine.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `store_pointer_mask_and_clear_dma`: Loads a pointer from a global variable and stores the value a fixed address at that address, then clears bit 7 of the blitter-to-copper transfer register and zeroes a global word, otherwise it returns.
- `decrement_byte_check`: Reads a byte from a global variable, decrements it, and returns immediately when the result is zero, otherwise loading a value and returning.
- `init_and_loop_joystick`: Clears eight consecutive words in a structure, stores a pointer and a computed value into a global variable, then loops sixteen times reading a joystick button and writing adjusted values to two pointers.
- `process_and_adjust_values`: Clears the word at the pointer in a1 and pushes a value onto the stack, then loops sixteen times checking a status byte at offset 6 of the structure in a6.
- `loop_and_check_stack_byte`: It runs a loop that counts down using d6 and jumps back to its start, and it returns when the byte pointed to by a7 is negative.
- `increment_byte_counters`: Clears a long at offset 160 of the structure in a4, then computes two pointer offsets from bytes at offsets 150 and 151, and increments bytes at those offsets depending on the sign of the byte at offset 157.
- `increment_struct_byte_counters`: Increments the byte at offset 1 of the structure in a0, the byte at offset 3 of the structure in a1, the byte at offset 2 of the first structure and the byte at offset 5 of the second structure, then returns the value 1.

### Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Installer

- `process_hunk_stream`: Parses an AmigaDOS hunk stream, copying each data section into a buffer and applying relocations, then calls exec.AllocMem to allocate memory for the hunk and exec.FreeMem to release temporary memory.
- `decrement_and_check`: Decrements a counter in a loop and returns when it reaches zero, otherwise advancing a pointer in a linked list.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_store_bytes`: It loads a long from the stack into d1, subtracts one from it, and returns early when that value is zero.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_and_accumulate_bits`: Loops a number of times given by d0, rotating d5 by the low bits of d4 into d1, pulling new data from a0 when d5 becomes zero, and accumulating the rotated bits into d0, then returns.
- `send_library_request_via_trap`: Reads a word from offset 192 of the structure in a2 into a data register, sets bits 3, 6 and 7 of that register, executes a library call trap, and combines the returned value into a second register with a bitwise or, then falls through.

### Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_audio_and_decode`: Clears audio and copper registers, waits for a flag, then repeatedly calls setup and decode routines, zeroes a block of memory, and configures audio channels before returning.
- `copy_value_pair_and_write_offsets`: Copies a 32-bit value from a global variable into paired 16-bit halves across two tables five times, advancing each source and destination by fixed amounts each pass, then writes a set of constant words into fixed offsets of the structure in a6 and stores a table address into a pointer field, otherwise it returns.
- `decode_compressed_stream_and_store`: Decodes a compressed byte stream into a destination buffer, copying literal runs directly and copying repeated runs a number of times indicated by a following byte, looping over multiple source blocks, then makes a library call and copies three words from a source pointer into fixed offsets of the structure in a6, otherwise it returns.
- `decode_compressed_stream`: Decodes a compressed byte stream from a0 into a buffer at a1, using a symbol table at 688aa and a linked-list of slots at 688ba and 688da in a2 and a3 to resolve entries.
- `store_pointer_and_constants`: Loads a pointer from a global variable into a register and stores it at four offsets of the structure in a6, then writes the constant 64 and the constant 226 into two further offsets, leaving the result in the structure.
