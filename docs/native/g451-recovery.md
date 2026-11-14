# Game 451 (Fightin' Spirit): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00451). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4527408 top-level bytes in 121 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 299 of 299 named (0 author symbols), 299 of 299 described (20 by rule, 56 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3244 of 3244 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8469 of 8472 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 302736 |
| code | 27748 |
| data | 3964845 |
| padding | 28815 |
| unused | 203264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:LightForce.exe | 2 | 235 | 8 |
| run | 2 | 590 | 15 |
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:FS | 4 | 1698 | 56 |
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Installer | 2 | 267 | 8 |
| Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 399 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5262 | 198 |

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
- runtime segment an address: 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2094 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:LightForce.exe: ProPack (RNC) (xfd) stream found by ancient scan (19244 -> 95796 bytes) but only 0 of 18 sampled windows are in reference RAM: not used
- fightin' spirit (1996)(neo)(aga)(m3)(disk 1 of 5)/Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:Installer: PowerPacker 4.0 (xfd) stream found by ancient scan (63436 -> 115144 bytes) but only 0 of 57 sampled windows are in reference RAM: not used
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
- `init_record_table_entry`: Makes a series of calls through a function pointer in a5, storing returned register values into fields of the structure in a1 at offsets 0, 4, 8, 28, 32, 36, 40, 44, 48, 52, 56, 60 and 64 (some as copies of table entries indexed by values from the structure).
- `build_record_and_init_table`: Copies a byte run from the structure in a1 into a table entry at an index read from that structure, writing a fixed type value and copied longwords into the table, then sets a flag at offset 20 and calls two other routines.
- `fill_table_entry_and_loop`: Copies a 32-bit field from the structure in a1 into a table, zeroes a run of longword fields at an index taken from that structure, and then runs a loop over a counter in the structure, storing each byte from a source buffer into a table; it also writes an error-style constant and calls the stores_entry_state_and_loops routine.
- `stores_entry_state_and_loops`: Writes a byte and several longwords into a table indexed by a value read from the structure in a1, copying fields from a second indexed entry and setting a flag at offset 40 of the structure.
- `adds_offset_and_branches`: Loads the constant 1 into the low longword of d1, adds the longword at offset 84 of the structure in a1 to it, and then branches to a location 552 bytes further on.
- `tests_long_field_and_branches`: Tests the long at offset 40 of the structure in a1 and branches to a later point when it is nonzero, otherwise it branches to a different later point.
- `clear_flag_and_branch_on_limit`: Ands the long at offset 48 of the structure in a1 with 128, and when the result is zero branches to a later point; otherwise it ORs the high bits of the field at offset 48, compares it against 133, and either branches to another point or branches to a further shared tail.
- `reset_object_and_release_slot`: Copies the field at offset 64 of the structure in a1 into offset 108, then calls a routine and stores its result at offset 28 of the structure.
- `clear_and_reset_object_state`: Stores the value at offset 28 plus offset 36 of the structure in a1, scaled by four, into a table indexed through a0 and then calls the routine that unlinks and decrements a counter.
- `unlink_and_decrement_counter`: Writes the value in d1 and the value at the start of the structure in a1 into fields of the structure in a1, then conditionally makes calls through a5, clears a longword in a table at an offset derived from fields of the structure, and decrements a longword counter in the table if it is not -1.
- `initialize_dispatch_slot_state`: Copies two longwords selected by the index in d1 from the structure in a0 into the structure in a1, then calls a routine through a pointer and fills several longword fields of the a1 structure with values taken from the a0 structure and the call results, setting bit 0 of a flag field when a condition holds.
- `dispatch_through_jump_table`: Stores the longword at offset 32 of the structure in a1 into d1 and copies it into the field at offset 24 of the structure, then sets bit 0 of the flags word at offset 32 and tests a table entry indexed by the word at offset 0.
- `load_long_then_jump`: Loads the longword at offset 32 of the structure in a1 into d1 and then jumps through a6 without returning to the caller.

### Fightin' Spirit (1996)(Neo)(AGA)(M3)(Disk 1 of 5).adf:FS

- `initialize_game_structure_2`: Clears two long variables in a global structure, then calls a routine that returns a pointer to a game structure and checks a flag at offset 172 of that structure.
- `store_a1_and_return`: It stores the 32-bit value from a1 into the global variable at offset 24d4, then returns via rte.
- `initialize_game_structure`: Initializes a game structure by storing routine addresses and data values at many of its offsets, copies a block of words from a table into the structure, and clears several fields.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
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
- `jump_to_copy_and_clamp_values`: Jump stub: passes control straight on to copy_and_clamp_values.
- `jump_to_copy_and_clamp_values_2`: Jump stub: passes control straight on to copy_and_clamp_values.
- `jump_to_copy_and_clamp_values_3`: Jump stub: passes control straight on to copy_and_clamp_values.
- `jump_to_routine_4`: Jumps unconditionally to another routine and does not return.
- `return_constant_one`: It doubles a value from a0 twice, advances the pointer by four times that value, then loads one into d0 and returns.
- `copy_and_clamp_values`: It contains several branches, including loops, and copies data between memory locations, but it makes no subroutine calls.
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
- `clear_pointer_list_entries`: Reads longwords from a list at a3, advancing the pointer each time, and for each non-zero value stores the next longword from the list at offset 184 of that value, repeating until a zero entry is read.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `store_global_pointer_vector`: Stores the address of a global variable into the longword at absolute low-memory address 128 by way of a0.
- `set_global_pointer_from_table`: Computes an address from a PC-relative base and a word-sized offset read from a table indexed by d0, and stores that address into a global variable.
- `process_compressed_data`: Stores a modified byte from a0 into a structure, then repeatedly calls a routine from a table until it returns non-zero, calls two other routines, and finally calls a routine that copies and clamps values before disabling a flag and returning.
- `call_library_set_flag`: Fetches the base of a library from a global variable and calls a routine at offset -150, then stores the value 2 in the byte at offset 2 of that structure.
- `unload_segment_and_clear_flag`: Saves a6, loads a library base from a global pointer, calls the DOS UnLoadSeg function with the segment in d1, clears the byte at offset 2 of the structure pointed to by a global variable, and restores a6.
- `read_input_process_response`: Stores a character from the input at offset 8 minus 49 into the byte at offset 852 of a global structure, then calls a routine at offset -30.
- `copy_block_and_unpack_bits`: Copies a block of long words from one memory area to another, then reads a 16-bit value from a source pointer and writes its bits, in little-endian order, as consecutive bytes to a destination pointer, repeating the process until the source pointer reaches the destination pointer.
- `probe_numbered_files_bitmask`: Stores a byte from the structure pointer at offset 3 into offset 5, and if that byte is zero, loops 8 times opening a file named from a global string, closing it on success and setting the corresponding bit in a result mask that is stored into offset 5.
- `copy_fields_to_hardware_registers`: Writes the word value 32767 into two hardware register fields of a block addressed through a global structure, copies longword fields from the global structure into two global variables, and writes masked words derived from fields at offsets 50 and 52 into hardware registers.
- `reset_view_and_hardware_registers`: Copies two longwords from a sub-structure into the global structure, calls the graphics library to load a null view and wait for the top of the frame twice, writes masked words into hardware registers, stores two global addresses into the hardware structure, and clears a byte at offset 2 of a global structure.
- `initialize_library_request`: Initializes a device library interface by filling a request structure with fields and, if a matching entry exists in a global table, copies that entry's data into the request and clears a block of eight longwords, then copies a variable-length block from a source table into a destination table.
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

- `tiny_subq_rts`: Small routine of 2 instructions that subtracts 8 from d6, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_custom_chips_and_wait`: Disables DMA and interrupts, waits for the blitter-busy flag to clear, clears the interrupt request register and the background color, fills a 32-entry color register block with a constant, sets up the copper list pointer and display control register, then waits for interrupt request bits and resets the audio channel pointers and lengths before disabling interrupts and DMA again.
- `fill_copper_pointer_table`: Loops 6 times writing the low and high words of a value that advances by 40 each step into two word-spaced tables, then stores fixed word constants at offsets 142 through 148 and 256 through 260 of the structure in a6, a longword pointer at offset 128, a zero word at offset 136, and a word at offset 150.
- `decode_run_length_buffer`: Reads a run-length encoded stream from a1, copying literal runs or repeating a byte with a count derived from the top bit of each control byte into a destination buffer, looping over 40-byte rows and 255 outer passes.
- `decode_nibble_stream_into_buffer`: Reads a stream of packed bytes through a source pointer, splitting each into high and low nibbles that index two small tables.
- `initialize_sprite_registers_and_wait`: Stores a long value from a global variable and two words of constants into a block of memory addressed by a6 at offsets 160 to 184, then busy-waits until bits 7 or 8 of the word at offset 30 are set, writes 384 to the word at offset 156 and updates the long at offset 160 and the word at offset 164 with a new value.
- `wait_for_bit5_then_set_word`: Writes 32 to the word at offset 156 of the structure in a6, then loops until bit 5 of the word at offset 30 is set, writes 32 to that same word again, and returns.
- `wait_for_bit4_then_set_word`: Writes 16 to the word at offset 156 of the structure in a6, then loops until bit 4 of the word at offset 30 is set, writes 16 to that same word again, and returns.
- `return_immediately_stub`: Empty; it contains only a return instruction and returns to its caller without reading or writing any data.
- `fill_words_with_wait_value`: Calls another routine that writes 16 to a word in the structure in a6 and waits for a status bit, then repeatedly calls a second routine to read a value and stores it as a word at a1, counting down from the word at a0 plus one.
- `fill_words_with_wait_value_32`: Calls another routine that writes 32 to a word in the structure in a6 and waits for a status bit, then repeatedly calls a second routine to read a value and stores it as a word at a1, counting down from the word at a0.
- `encode_inline_data_bytes`: Contains a sequence of arithmetic, bit-test and logical instructions operating on byte, word and long operands at registers and indexed addresses, with no branch back into itself and no subroutine call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatches_interrupts_and_timing_loops`: Large interrupt and timing handler that reads the CIA port A register for fire button and disk status, calls acknowledge_interrupt_and_dispatch twice, and calls several helpers that initialize custom chip registers, fill copper-style table entries, poll port bits, and step words toward targets.
- `acknowledge_interrupt_and_dispatch`: Tests bit 5 of the interrupt request register high byte and, when it is set, calls a routine through a pointer in a global variable, calls another routine, and increments a global counter.
- `decrement_flag_then_branch_to_copy`: Decrements the word at negative offset 1195 from a5 and, if the result is still non-negative, returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_counter_then_branch_to_copy_2`: Points a0 at a global buffer and sets d0 to 31, then subtracts 1 from a word counter in the global area and, when that counter is still non-negative, branches back to the copy step.
- `decrement_counter_then_branch_to_copy`: Decrements the word at negative offset 1981 from a5 and, if the result is negative, branches to another routine's entry point.
- `run_frame_loop_until_fire`: Initializes custom chip registers and several structures, then repeatedly calls routines to fill copper-style entries and waits on a vertical-sync style byte in the structure at a6.
- `clear_table_and_set_copper_entries`: Zeroes a table of words by walking a pointer in steps of two until a countdown held in a register passes below zero, then writes a set of values into fields of the structure in a6 at offsets 40 through 116, including a long at offset 128 and a word at offset 150.
- `write_setup_block_and_wait`: Stores 8 into the word at offset 150 of the structure in a6, then calls a routine that waits for a byte at offset 6 to match after adding 7.
- `wait_for_byte_arithmetic_match`: Adds 7 to the byte at offset 6 of the structure in a6 and loops until the result, truncated to a byte, equals the original byte, then returns.
- `alternate_and_fill_table_entries`: Selects one of two long constants by comparing a global variable at a negative offset from a5, stores the chosen value into another global variable, then writes four table entries of split high and low words, stepping the value by 30 each time.
- `draw_scanline_block_pairs`: Loads a word from a global variable and, when it is nonzero and positive, reads a second word from another global variable and, if that is also nonzero and positive, stores a 32-bit pointer and a 16-bit value into the structure in a6 and repeats a call to another routine while its returned count is nonzero.
- `initializes_and_sets_timing_fields`: Calls another routine to get a value, then writes constants 60 and 2400 into words of the structure in a6 and stores a value derived from the returned word at offset 102, then loops two more times copying longwords and a word into the structure around offset 80 and calls another routine each iteration.
- `saves_registers_and_calls_pointer`: Saves registers d0-d7 and a0-a6 on the stack, calls the routine whose address is stored in a PC-relative global longword, then calls another routine and restores all saved registers before returning.
- `read_next_word_from_pointer`: Reads a flag byte from a PC-relative location and, when it is zero, reads the word at the current pointer, advances the pointer, stores it back to a fixed address and loops while the word is greater than a PC-relative value, then returns the word.
- `clears_longword_block`: Repeatedly stores zero into 16 consecutive longwords starting at the address in a1 and advances a1 by 64 bytes per pass, counting d0 down from its initial value until the counter wraps past zero.
- `wait_while_bit6_set`: Loops while bit 6 of the byte at offset 2 of the structure in a6 is set, and otherwise returns.
- `wait_until_word_changes`: Adds a PC-relative 16-bit value to the register holding the data and loops until the sum no longer equals that same constant, then returns.
- `poll_cia_port_bits`: Reads the CIAA port A register in a loop, testing its high bit and a bit-shifted value, and stops when the test fails or a counter in a global variable wraps to a fixed value.
- `saves_registers_and_clears_flag`: Saves all data and address registers on the stack, stores a4 into a global long, clears a global byte, then calls another routine that initialises the audio channels and sound timers, restores the saved registers and returns.
- `initialises_sound_and_cia_timers`: Initialises the four audio channel volumes to zero and enables all four audio DMA channels, then builds a table of per-entry records from a data structure and fills the following entries with defaults.
- `saves_registers_and_calls_setup`: Saves registers d0-d7 and a0-a5 on the stack, calls the routine at a fixed nearby address, restores the same registers from the stack, loads the custom chip base address into a6, and returns.
- `initialize_custom_audio_and_dma`: Writes 8192 to the custom interrupt enable register, copies a longword from a constant table into a global variable, zeroes the volume registers of the four audio channels, and writes 15 to the DMA control register.
- `advance_sequence_table_state`: Reads the next word from a sequence table, updating a few global counters and index variables, and when the wait counter reaches zero it steps through the table entries and stores decoded values into globals.
- `wrap_clamp_position_word`: Computes the word at offset 20 minus the word at offset 0 of the structure in a1, masks it to 12 bits, clamps it to a minimum of 113, stores it back at offset 20, and branches to another routine that also handles the same value.
- `add_capped_position_word`: Adds the word at offset 0 to the word at offset 20 of the structure in a1, caps the sum at 856 by clearing the word at offset -2 when it exceeds that limit, stores the result at offset 20, and branches to another routine.
- `branches_to_position_handler`: Consists of a single unconditional branch to a routine further along the code.
- `branch_to_later_routine`: Unconditionally branches forward to a point in the code at offset 12590 without changing any registers or memory.
- `branch_to_later_position_routine`: Unconditionally branches forward to a point in the code at offset 12556 without changing any registers or memory.
- `jump_to_earlier_routine`: Contains a single unconditional word branch to a routine located earlier in the code, so it passes control there without reading or writing any data of its own.
- `jump_to_mask_and_branch`: Contains a single unconditional word branch to a later point in the code and performs no data movement itself.
- `branches_to_table_lookup`: Consists of a single unconditional branch to a routine further along the code.
- `flag_entry_on_modulo_match`: Computes the 16-bit difference between a global variable and the word at the structure in a0, and when that difference is an exact multiple of the word at a1, ORs the byte at offset 49 of the structure in a0 with the byte four words before a1, then copies a long word and a word from a1 into the location in a3 only when that byte does not match the bit mask in a global variable.
- `branch_to_shared_tail`: Its only instruction is an unconditional long branch to a shared tail routine, so it transfers control there without touching any data itself.
- `decrement_word_until_zero`: Decrements the word pointed to by a1 and, when the result reaches zero, clears the word at offset -2 and the word at offset 4 of that structure, then branches to another routine.
- `decrement_word_and_copy_fields`: Decrements the word at the start of the structure in a1, and when the result reaches zero clears the word at offset minus 2, copies a word from offset 20 into the long at a3 plus 6 unless a global variable mask is set, ORs a byte from offset minus 8 into the byte at offset 49 of the structure in a0, and copies a long from offset 6 and a word from offset 14 into the memory at a3.
- `decode_packed_stream_entry`: Reads a control byte from the stream pointed to by a structure field and either stores a small count or a masked index into a second structure field, then conditionally copies four longwords from a table and dispatches through a computed jump.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `negates_speed_then_branches`: Negates the word at offset 30 of the structure in a1 when it is negative and the word at offset 6 is nonzero, then branches to a routine further along the code.
- `split_word_into_nibble_fields`: Reads the word at offset 6 of the structure in a1; when it is nonzero, stores its low nibble as a word at offset 36 if that nibble is nonzero, and stores its high nibble shifted right by two as a byte at offset 34 if that nibble is nonzero.
- `branches_into_speed_routine`: Consists of a single unconditional branch to a point inside another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `split_word_into_nibble_fields_alt`: Performs a no-operation, then branches to another routine that reads the word at offset 6 of the structure in a1; when nonzero, it stores the low nibble as a word at offset 42 if nonzero and the high nibble shifted right by two as a byte at offset 40 if nonzero.
- `jump_to_pointer_adjust_routine`: Contains a single unconditional word branch to a later point in the code and performs no data movement itself.
- `reset_counter_and_update_speed_word`: Clears the word at offset 4 of the structure in a1, loads a word from offset 6 and, when it is nonzero, shifts it left by 7 into the word at offset 44; it then subtracts that word from the word at offset 20, and when the result is nonzero and the speed does not exceed the old value it adds twice that speed to the long at offset 12, otherwise it sets the word at offset 20 to 1.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `clear_fields_store_scaled_pointer`: Clears the word at offset 4 of the structure in a1, clears the word at offset -2 of the structure in a0, then stores a long at offset 2 of a0 equal to the word at offset 6 of a1 doubled plus a global variable, and branches to another routine.
- `branches_to_clear_pointer_routine`: Branches unconditionally to another routine that clears fields and stores a scaled pointer.
- `copy_word_clear_fields`: Clears the word at offset 4 of the structure in a1 and the word at offset -2 of the structure in a0, copies the word at offset 6 of a1 into offset 34 of a0, clears the byte at offset -4 of a0, and branches to another routine.
- `dispatch_via_word_table_jump`: Reads the word at offset 6 of the structure in a1, keeps its high nibble and shifts it right by two to index a table, then calls the routine at that table entry.
- `clear_counter_store_value`: Clears the word at offset 4 of the structure in a1, then if the word at offset 6 is below 32 it stores that value to the word at offset 252 and the word at offset 0 of the structure in a0; otherwise it returns without storing.
- `adjust_pointer_and_return`: Tests d0, and when it is nonzero compares it against 70 and branches to a later point on a match; otherwise it adds the word at offset 8 of a1 to d0, stores a pointer computed from a0 and d0 at offset 52 of a1, increments the return address on the stack by 4, and returns.
- `searches_position_table_for_entry`: Compares the word at offset 26 of the structure in a1 against a table of words relative to a0, walks up or down through the table in steps until it finds the bracketing entry, then stores that entry's pointer at offset 52 of the structure in a1, or clears the word at offset 4 when the entry is the same as the current one.
- `search_position_table_entry`: Compares the word in d1 against a set of word values in a table relative to a4 and selects a matching entry, then stores that entry's pointer at offset 52 of the structure in a1 when it differs from d2, otherwise clears the word at offset 4 of a1.
- `clear_word_field`: Clears the word at offset 4 of the structure in a1 and returns.
- `selects_table_word_by_nibble`: Reads a word from the structure in a1 and another word from offset 18, then looks up a table entry using the low nibble of the word from the source pointer, doubled as an index, and conditionally branches on a byte in the table.
- `select_table_word_by_nibble`: Takes the low nibble of d1, doubles it, and reads a word from the table pointed to by the long at offset 44 of the structure in a1 at that index; if the byte at offset -10 of a1 ANDed with a global variable is zero, it writes that word to offset 6 of the structure in a3.
- `conditionally_store_d0_word`: If the byte at offset -10 of the structure in a1 ANDed with a global variable is zero, it writes the word in d0 to offset 6 of the structure in a3; otherwise it returns without writing.
- `wrap_and_clamp_position_word`: Computes the word at offset 20 of the structure in a1 minus the word at offset 0, masks it to 12 bits, clamps it to a minimum of 113 and stores it back to offset 20; if the byte at offset -4 ANDed with a global variable is zero, it also writes the result to offset 6 of the structure in a3.
- `add_capped_position_and_reset`: Adds the word at offset 0 of the structure in a1 to the word at offset 20, capping the sum at 856 and clearing the word at offset -2 when the cap is reached; it stores the result back to offset 20 and, if the byte at offset -4 ANDed with a global variable is zero, writes it to offset 6 of the structure in a3.
- `negates_fields_and_clamps_speed`: Negates the word at offset 30 of the structure in a1 when it is negative and the word at offset 6 is nonzero, then if d0 is nonzero it reads a word from a table in a0 at an offset from offset 8 and stores it at offset 28, subtracting the word at offset 26 and negating the word at offset 30 when the result is negative.
- `negate_negative_word_field`: If the word at offset 30 of the structure in a1 is negative, it replaces it with its negation; otherwise it returns unchanged.
- `tests_and_advances_word_field`: Loads the word at offset 22 of the structure in a1 into d2 and returns immediately when it is zero; otherwise it loads the word at offset 24 into d1, branches on its sign, and adds the word at offset 20 to d1, comparing the result with d2 and branching accordingly.
- `advance_word_and_search_table`: Adds d1 to the word at offset 20 of the structure in a1, resetting the words at offsets 22 and -2 and setting the value to d2 if the result does not exceed d2; when the byte at offset 42 is nonzero it scans up to 36 words of a table for the first entry not greater than the value, then stores that entry to offset 6 of the structure in a3 if the byte at offset -8 ANDed with a global variable is zero.
- `sums_clamps_stores_byte_pair`: Calls two other routines, then adds the byte at offset 5 of the structure in a1 to the low byte at offset 0, clamps the result to 0 when negative or to 64 when above 63, and stores it at offset 4.
- `split_word_into_fields`: Reads the word at offset 6 of the structure in a1; if it is nonzero, stores its low nibble as a word at offset 36 when that nibble is nonzero, and stores its high nibble shifted right by 2 as a byte at offset 34 when that nibble is nonzero.
- `masks_scales_speed_value`: Masks d0 to its low seven bits using the byte at offset 29 of the structure in a1, then reads the byte at offset 27 into d2.
- `tests_flag_and_inverts_d0`: Tests the byte at offset 29 of the structure in a1 and, when it is negative, bitwise-inverts the low byte of d0; otherwise d0 is left unchanged before continuing to a later branch target.
- `scale_and_store_speed`: Looks up a byte from a table in a0 using the upper bits of d0, multiplies it by a 16-bit value at offset 30 of the structure in a1, and shifts the product right by 7 to get a signed amount that is negated when the byte at offset 29 is negative.
- `branch_to_masking_routine`: Branches back to a routine located earlier in the code, passing control without touching any data in this routine itself.
- `clamps_byte_sum_and_clears_word`: Adds the byte at offset 5 of the structure in a1 to the word at offset 0, and when the result is non-negative and not above 64 it keeps the value, otherwise it sets it to 64 in d1 or leaves it at 64 after the compare.
- `clear_fields_conditionally_zero`: Clears a word at offset -2 and a word at offset 4 of the structure in a1, then clears a word at offset 8 of the structure in a3 only when the byte at offset -8 of a1 ANDed with a global variable is zero, and otherwise returns.
- `split_nibbles_into_fields`: Reads the word at offset 6 of the structure in a1 and, if it is nonzero, stores its low nibble as a word at offset 42 and its high nibble shifted right by 2 as a byte at offset 40, then returns.
- `masks_byte_and_branches_conditionally`: Clears the upper bits of d0 with a 7-bit mask applied to the byte at offset 35 of the structure in a1, then reads the byte at offset 33 into d2.
- `conditionally_inverts_index`: Tests the byte at offset 35 of the structure in a1 and bitwise-inverts the low byte of d0 when it is negative, then branches into another routine.
- `scales_word_and_adds_byte`: Shifts d0 right by two, reads a byte from a table in a0 indexed by d0, multiplies it by the word at offset 36 of the structure in a1 and shifts the product right by seven.
- `scale_word_and_add_byte`: Stores the high byte of a global variable multiplied by 40 into a word at offset 8 of the structure in a3 when the byte at offset -8 of a1 ANDed with another global variable is zero, then adds the byte at offset 34 to the byte at offset 35 of the structure in a1 and returns.
- `updates_countdown_and_speed_word`: Tests d0 and, when it is nonzero, saves the return address and loads a 16-bit value from offset 6 of a1; when that value is nonzero it shifts it left by seven and stores it at offset 44.
- `decrements_field_and_adds_double`: Subtracts the word at offset 44 of the structure in a1 from the word at offset 20, and when the result is not positive it doubles the value in d1 and adds it to the long at offset 12, then returns.
- `set_word_field_to_one`: Writes the word value 1 to offset 20 of the structure in a1 and returns.
- `clear_fields_and_store_offset_pointer`: Clears a word at offset 4 of the structure in a1 and a word at offset -2 of the structure in a0, then stores into a long at offset 2 of a0 the value of a global table entry indexed by twice the word at offset 6 of a1, and returns.
- `copy_word_and_clear_fields`: Clears a word at offset 4 of the structure in a1 and a word at offset -2 of the structure in a0, copies the word at offset 6 of a1 into offset 34 of a0, clears the byte at offset -4 of a0, and returns.
- `dispatches_through_word_indexed_jump`: Takes the word at offset 6 of the structure in a1, keeps its high nibble, shifts it right by two to form a table offset, and jumps through a jump table placed after the instruction.
- `clears_word_and_sets_port_bit`: Clears the 16-bit word at offset 4 of the structure in a1 and then branches to another routine.
- `stores_table_word_and_masks`: When d0 is nonzero, stores a word read from the table in a0 at index d0 plus the word at offset 8 of a1 into the word at offset 26 of a1.
- `stores_table_word_and_clears_flag`: When d0 is nonzero, stores a word read from the table in a0 at index d0 plus the word at offset 8 of a1 into the word at offset 26 of a1.
- `clears_word_and_stores_nibble_byte`: Branches to a later point in the routine that clears the word at offset 4 of the structure in a1 and stores the low nibble of the word at offset 6 as a byte at offset 48 of the same structure.
- `clears_word_and_stores_status_flags`: Clears the word at offset 4 of the structure in a1 and stores the low two bits of the word at offset 6 into the byte at offset 33.
- `computes_scaled_word_from_nibble`: Branches to a later point in the routine that clears the word at offset 4 of the structure in a1 and stores at offset 8 the low nibble of the word at offset 6 multiplied by 72 plus 598.
- `decrements_byte_and_reloads_timer`: Clears the word at offset 4 of a1 and, when the low four bits of the word at offset 6 are nonzero, decrements the byte at offset 49 and sets a word at offset -2 of a0 to 63 when it is nonzero.
- `clears_word_and_stores_status_flags_2`: Clears the word at offset 4 of the structure in a1 and stores the low four bits of the word at offset 6 into the byte at offset 39.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `sets_word_and_masks_low_nibble`: Stores the value 36 into the word at offset 4 of the structure in a1, masks the word at offset 6 down to its low four bits, and clears offset 4 again when that masked value is zero or d0 is zero.
- `clear_word_and_test_bit`: Clears the 16-bit word at offset 4 of the structure in a1, then if the word at offset 6 of that structure has its lowest bit clear, clears bit 1 of the CIAA port A register (CIAA_PRA) and returns; otherwise it returns without touching the port.
- `set_fire_button_status_bit`: Sets the bit numbered by the low three bits of d2 in the byte at the CIAA port A register, leaving the other bits unchanged, and returns.
- `updates_position_word_and_clamps`: If d0 is nonzero, stores a word read from the table in a0 (indexed by d0 plus the word at offset 8 of a1) into the word at offset 26 of a1, then clears the word at offset 4 and masks the word at offset 6 to its low four bits, and calls the routine that computes, masks and clamps the position word at offset 20.
- `accumulates_position_and_resets`: If d0 is nonzero, stores a word read from the table in a0 (indexed by d0 plus the word at offset 8 of a1) into the word at offset 26 of a1, then clears the word at offset 4 and masks the word at offset 6 to its low four bits, and calls the routine that adds the word at offset 0 to the word at offset 20 with a cap of 856.
- `store_low_nibble_as_byte`: Clears the word at offset 4 of the structure in a1 and stores the low nibble of the word at offset 6 as a byte at offset 48 of the same structure, then returns.
- `clear_flags_from_status_word`: Clears the 16-bit word at offset 4 of the structure in a1, then stores the low two bits of the word at offset 6 into the byte at offset 33.
- `clear_byte_field`: Clears the byte at offset 32 of the structure in a1 and returns.
- `compute_word_from_nibble`: Clears the word at offset 4 of the structure in a1, then stores at offset 8 the low nibble of the word at offset 6 multiplied by 72 and increased by 598, and returns.
- `counts_down_byte_and_resets_timer`: Clears the word at offset 4 of a1, and if the low four bits of the word at offset 6 are nonzero it decrements the byte at offset 49 and, when that byte is still nonzero, writes the word constant 63 into the buffer in a0, stores 63 minus the word at offset 50 into the word at offset 34 of a0, and writes 255 to the byte at offset 38; otherwise it copies a word from a global variable into the word at offset 50 of a1.
- `copies_word_into_offset_50`: Copies the word from a global variable into the structure in a1 at offset 50, then returns.
- `clear_flags_from_status_word_2`: Clears the word at offset 4 of the structure in a1, then keeps the low 4 bits of the word at offset 6 and stores them in the byte at offset 39.
- `clears_byte_at_offset_38`: Clears the byte at offset 38 of the structure in a1 to zero, then returns.
- `sets_field_and_masks_flags`: Stores 36 into the word at offset 4 of the structure in a1 and masks the word at offset 6 down to its low four bits.
- `sets_flag_and_copies_entry_when_divisible`: If the difference between a global variable and the word at the structure in a0 is divisible by the word at a1, it ORs the byte at offset -8 of a1 into the byte at offset 49 of a0, and when the global variable masked with that byte is zero it copies the long word at offset 6 and the word at offset 14 of a1 into the location pointed to by a3; otherwise it returns.
- `clears_word_and_masks_next_word`: Clears the word at offset 4 of the structure in a1, then ANDs the word at offset 6 with 15 and calls another routine that clamps a byte sum, then returns.
- `clears_masks_and_negates_byte`: Clears the word at offset 4 of the structure in a1, ANDs the word at offset 6 with 15, negates the byte at offset 7, then calls another routine that clamps a byte sum and returns.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `decrements_counter_and_clears_fields`: Decrements the word pointed to by a1, and if the result is not zero, branches to a later point in the routine.
- `initializes_state_and_adjusts_offset`: Stores 48 into the word at offset 4 of a1, and if d0 is nonzero and the low four bits of the word at offset 6 are nonzero it adds the word at offset 8 to d0 and stores the word from the table in a0 at that index into the word at offset 26, then pops two words from the stack and adds 6 to the return address.
- `clears_word_at_offset_4`: Clears the word at offset 4 of the structure in a1 to zero, then returns.
- `decrements_word_and_conditionally_copies`: Decrements the word at the structure in a1, branches back to an earlier instruction when it is not zero, and otherwise clears the word at offset -2 and tests a byte at offset -8 against a flag from a table, then copies a word into a3 and branches to another routine.
- `handles_cia_interrupt_dma_enable`: Reads the CIA-B interrupt control register and, when its bit 1 is set, writes 25 to the CIA-B control register B, loads a DMA control value from a global variable into the DMA control register, and stores a long from a global variable into a global variable.
- `restarts_four_audio_channels`: Reads the CIA-B interrupt control register and, when bit 1 is set, reloads the location and length registers of each of the four audio channels from table values unless the matching bit in a global variable is set.
- `steps_words_toward_target_nibbles`: Loops over the words of the buffer in a1 for d0 plus one iterations, reading the source words from a0.
- `steps_words_toward_target_loop`: Saves registers, then repeats a loop that checks the byte at offset 6 of a6 against 255 and, when the wait condition is met, calls the routine that steps the buffer words toward their targets, restoring registers and decrementing d2 until it reaches the end.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `fills_longs_with_shifted_masked_words`: Saves registers, then for d0 plus one iterations reads a word from a0, shifts it right by the count in d1, ANDs it with the mask in d2 and stores the result as a word at a1, advancing a1 by 4 and a0 by 2 each time.
- `writes_shifted_masked_words_packed`: Saves registers, then for d0 plus one iterations reads a word from a0, shifts it right by the count in d1, ANDs it with the mask in d2 and stores the result as a word at a1, advancing a0 and a1 by 2 each time.
- `copies_table_and_steps_words`: Copies 32 words from one table to another in a5-relative memory, then runs a nested loop that polls the byte at offset 6 of a6 and calls another routine for each of 10 passes, then returns.
- `copies_table_and_steps_words_zero`: Copies 32 words between two a5-relative tables, then runs a polling loop over 10 passes that calls another routine with a starting value of zero and returns.
- `copies_table_and_steps_words_two`: Copies 32 words between two a5-relative tables, then runs a polling loop over 10 passes that calls another routine with a starting value of two and returns.
- `loads_two_pointers_and_branches`: Loads the address of a fixed table into a0 and a second fixed address into a1, sets d0 to the constant 6354, and branches into the shared code at another location within the routine group.
- `loads_alternate_pointers_and_branches`: Loads a fixed table address into a0 and a second fixed address into a1, sets d0 to the constant 9393, and branches into the shared code at another location within the routine group.
- `indexes_jump_table_by_byte`: Reads a byte from a fixed offset relative to a global base register, shifts it right by 5 and then left by 4, and uses the result to indirectly call a routine at an offset from its own entry point.
- `multiply_d2_branch_shared`: Multiplies the word in d2 by the constant 14746 and then branches to the shared code at another location within the same routine group.
- `add_d2_multiply_branch_shared`: Adds the word in d2 to d4, multiplies d2 by the constant 15007, and then branches to the shared code at another location within the same routine group.
- `adds_scaled_value_then_branches`: Adds register d2 into register d4 twice, then multiplies d2 by the constant 19732 and branches into the shared store-and-clamp code that calls clamp_scaled_value_store_byte.
- `adds_value_thrice_then_branches`: Adds register d2 into register d4 three times, then multiplies d2 by the constant 26119 and branches into the shared store-and-clamp code that calls clamp_scaled_value_store_byte.
- `scales_negates_then_branches`: Multiplies d2 by the constant 14746, negates the word in d4, and branches into the shared store-and-clamp code that calls clamp_scaled_value_store_byte.
- `adds_scales_negates_then_branches`: Adds register d2 into register d4 once, multiplies d2 by the constant 15007, negates d4, and branches into the shared store-and-clamp code that calls clamp_scaled_value_store_byte.
- `adds_twice_scales_negates_branch`: Adds register d2 into register d4 twice, multiplies d2 by the constant 19732, negates d4, and branches into the shared store-and-clamp code that calls clamp_scaled_value_store_byte.
- `clamp_scaled_value_store_byte`: Computes a word from d3 minus the sum of three times d2 and d4, shifts it right by 6, and clamps it between the limits in d5 and d6 before storing it as a byte through a1.
- `multiply_d2_then_branch_3`: Multiplies the word in d2 by the constant 14746 and then branches to the shared code at the middle of another routine.
- `add_d2_to_d4_then_branch_2`: Adds the word in d2 to the word in d4, multiplies d2 by the constant 15007, and then branches to the shared code at the middle of another routine.
- `add_d2_twice_then_branch`: Adds the word in d2 to d4 twice, multiplies d2 by the constant 19732, and then branches to the shared code at the middle of another routine.
- `add_d2_three_times_branch`: Adds the word in d2 to d4 three times, multiplies d2 by the constant 26119, and then branches to the shared code at the middle of another routine.
- `multiply_d2_negate_d4_2`: Multiplies the word in d2 by the constant 14746, negates the word in d4, and then branches to the shared code at the middle of another routine.
- `add_d2_multiply_negate`: Adds the word in d2 to d4, multiplies d2 by the constant 15007, negates d4, and then branches to the shared code at the middle of another routine.
- `add_d2_twice_negate_d4`: Adds the word in d2 to d4 twice, multiplies d2 by the constant 19732, negates d4, and then branches to the shared code at the middle of another routine.
- `compute_and_store_four_samples`: Computes a value from d3, d2 and d4, clamps it between the limits in d5 and d6, and writes it as a byte through a pointer in a1, then repeats the computation for the following bytes using multiplier constants selected from the low bits of the byte read through a0.
- `scale_word_branch_shared`: Multiplies the word in d2 by 14746 and branches to a shared tail at a fixed offset.
- `add_scale_branch_shared`: Adds d2 into d4, multiplies d2 by 15007, and branches to a shared tail at a fixed offset.
- `add_double_scale_branch`: Adds d2 into d4 twice, multiplies d2 by 19732, and branches to a shared tail at a fixed offset.
- `add_triple_scale_branch`: Adds d2 into d4 three times, multiplies d2 by 26119, and branches to a shared tail at a fixed offset.
- `scale_d2_then_negate_d4`: Multiplies the low word of d2 by the constant 14746 and negates the low word of d4, then branches to the shared tail at the clamp-and-store code.
- `add_d2_to_d4_then_scale`: Adds the low word of d2 into the low word of d4, multiplies d2 by the constant 14999, negates d4, then branches to the shared tail at the clamp-and-store code.
- `add_d2_twice_then_scale`: Adds the low word of d2 into d4 twice, multiplies d2 by the constant 19732, negates d4, then branches to the shared tail at the clamp-and-store code.
- `clamp_store_then_dispatch`: Computes d3 minus the sum of d2 times three and d4, shifts it right by 6, clamps it between the limits in d5 and d6, stores the byte through a1, then jumps through a table indexed by the top three bits of d1.
- `scale_word_then_branch_shared`: Multiplies the word in d2 by 14746 and branches into the middle of another routine at a fixed offset, so it has no body of its own beyond that branch.
- `accumulate_scale_then_branch`: Adds the word in d2 into d4, multiplies d2 by 14495, and branches into the middle of another routine at a fixed offset.
- `add_doubled_then_scale`: Adds the register d2 into d4 twice, then multiplies d2 by 19732 and branches to a shared tail at a fixed offset.
- `add_tripled_then_scale`: Adds the register d2 into d4 three times, then multiplies d2 by 26119 and branches to a shared tail at a fixed offset.
- `scale_then_negate_word`: Multiplies d2 by 14746, negates the word in d4, and branches to a shared tail at a fixed offset.
- `add_scale_negate_word`: Adds d2 into d4, multiplies d2 by 15007, negates the word in d4, and branches to a shared tail at a fixed offset.
- `add_double_scale_negate`: Adds d2 into d4 twice, multiplies d2 by 19732, negates the word in d4, and branches to a shared tail at a fixed offset.
- `clamp_difference_then_dispatch`: Computes d3 minus three times d2 plus d4, shifts it right by 6, clamps it between the limits in d5 and d6, and stores the byte through a1.
- `scale_difference_then_clamp`: Multiplies the word in d2 by 14746 and then branches into the shared tail that computes d3 minus three times d2 plus d4, shifts it right by 6, and clamps the result between the limits in d5 and d6 before storing it through a1.
- `add_then_scale_then_clamp`: Adds the word in d2 into d4, multiplies d2 by 14495, and branches into the shared tail that computes the clamped value of d3 minus three times d2 plus d4 and stores it through a1.
- `accumulate_twice_scale_then_clamp`: Adds the word in d2 into d4 twice, multiplies d2 by 19732, and branches into the shared tail that computes the clamped value of d3 minus three times d2 plus d4 and stores it through a1.
- `accumulate_thrice_scale_then_clamp`: Adds the word in d2 into d4 three times, multiplies d2 by 26119, and branches into the shared tail that computes the clamped value of d3 minus three times d2 plus d4 and stores it through a1.
- `scale_negate_then_clamp`: Multiplies d2 by 14746, negates the word in d4, and branches into the shared tail that computes the clamped value of d3 minus three times d2 plus d4 and stores it through a1.
- `accumulate_scale_negate_clamp`: Adds d2 into d4, multiplies d2 by 14495, negates d4, and branches into the shared tail that computes the clamped value of d3 minus three times d2 plus d4 and stores it through a1.
- `accumulate_twice_scale_negate`: Adds d2 into d4 twice, multiplies d2 by 19732, negates d4, and branches into the shared tail that computes the clamped value of d3 minus three times d2 plus d4 and stores it through a1.
- `clamp_scaled_difference_to_byte`: Computes a value from d3 minus three times d2 plus d4, shifts it right by 6, clamps it between the limits in d5 and d6, and stores the result as a byte through the pointer in a1.
- `multiply_d2_then_branch_2`: Multiplies the low word of d2 by the constant 14746 and keeps the 32-bit product in d2, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `add_d2_to_d4_then_branch`: Adds the low word of d2 into d4, multiplies d2 by the constant 15007 keeping the 32-bit product, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `accumulate_d2_twice_then_branch`: Adds the low word of d2 into d4 twice, multiplies d2 by the constant 19732 keeping the 32-bit product, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `accumulate_d2_thrice_then_branch`: Adds the low word of d2 into d4 three times, multiplies d2 by the constant 26119 keeping the 32-bit product, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `multiply_d2_negate_d4_branch`: Multiplies the low word of d2 by the constant 14746, negates the low word of d4 in place, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `add_d2_multiply_negate_branch`: Adds the low word of d2 into d4, multiplies d2 by the constant 15007 keeping the 32-bit product, negates the low word of d4 in place, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `accumulate_multiply_negate_branch`: Adds the low word of d2 into d4 twice, multiplies d2 by the constant 19732 keeping the 32-bit product, negates the low word of d4 in place, then branches to the shared tail that computes the clamped byte from d2, d3, d4, d5 and d6.
- `compute_clamped_byte_then_dispatch`: Computes a 16-bit value as d3 minus (d2 times 3 plus d4), shifts it right by 6, and clamps it between the bounds in d5 and d6 before storing the low byte through a1.
- `multiply_d2_then_branch`: Multiplies the low word of d2 by the constant 14746 and then branches to the shared clamping code that stores a scaled byte through a1.
- `add_d2_to_d4_then_multiply`: Adds the low word of d2 into d4, multiplies the low word of d2 by the constant 14999, and then branches to the shared clamping code that stores a scaled byte through a1.
- `accumulate_d2_twice_multiply`: Adds the low word of d2 into d4 twice, multiplies the low word of d2 by the constant 19732, and then branches to the shared clamping code that stores a scaled byte through a1.
- `accumulate_d2_thrice_multiply`: Adds the low word of d2 into d4 three times, multiplies the low word of d2 by the constant 26119, and then branches to the shared clamping code that stores a scaled byte through a1.
- `multiply_d2_negate_d4`: Multiplies the low word of d2 by the constant 14746, negates the low word of d4, and then branches to the shared clamping code that stores a scaled byte through a1.
- `add_d2_multiply_negate_d4`: Adds the low word of d2 into d4, multiplies the low word of d2 by the constant 14999, negates the low word of d4, and then branches to the shared clamping code that stores a scaled byte through a1.
- `doubles_negates_and_scales`: Adds d2 to d4 twice, then multiplies d2 by 19732 (unsigned 16-bit), negates d4, and branches to 17266 where the result is clamped and stored as a byte through the pointer in a1.
- `clamps_scaled_value_into_byte`: Computes a scaled difference from d3, d2, d4 and a right shift by six, then stores it as a byte through the pointer in a1 after clamping it between the limit in d5 and the limit in d6.
- `scales_d2_and_branches_to_clamp`: Multiplies the low word of d2 by 14746 and branches to another routine that computes a clamped byte from d3, d2, d4 and the limits in d5.
- `adds_d2_to_d4_and_scales_d2`: Adds the low word of d2 into d4, multiplies d2 by 14495, and branches to another routine that computes a clamped byte.
- `adds_d2_twice_and_scales_d2`: Adds the low word of d2 into d4 twice, multiplies d2 by 19732, and branches to another routine that computes a clamped byte.
- `adds_d2_thrice_and_scales_d2`: Adds the low word of d2 into d4 three times, multiplies d2 by 26119, and branches to another routine that computes a clamped byte.
- `multiply_then_negate_and_branch`: Multiplies d2 by the constant 14746 using an unsigned 16-bit multiply, negates the word in d4, then branches to a shared tail at a later point in the code without returning here.
- `add_multiply_negate_and_branch`: Adds d2 into the word in d4, multiplies d2 by the constant 15007 using an unsigned 16-bit multiply, negates the word in d4, then branches to a shared tail at a later point in the code.
- `double_add_multiply_and_branch`: Adds d2 into the word in d4 twice, multiplies d2 by the constant 19732 using an unsigned 16-bit multiply, negates the word in d4, then branches to a shared tail at a later point in the code.
- `computes_clamped_byte_and_dispatches`: Computes the difference between d3 and the value of d2 times 3 plus d4, shifts it right by 6, clamps the result between the values in d5 and d6, stores it as a byte at the location in a1, and when d0 is at least 3 it returns d0 minus 3, otherwise it jumps through a table indexed by the high bits of the byte at a0 and returns its result.
- `expand_nibbles_to_words`: Reads a run of bytes through a pointer in a3, adds a constant from register d4 to each, clamps each sum to the range 0 to 15, and packs successive pairs into 16-bit words stored sequentially through a pointer in a0 at offsets 0, 4, 8 and onward.
- `pack_nibbles_into_words`: Reads consecutive bytes from the buffer in a3, subtracts the value in d4, clamps negative results to zero, then combines pairs of values into 16-bit words stored sequentially through the pointer in a0, with the last words written at offsets 0 to 24 in steps of 2.
- `pack_nibble_table_to_words`: Reads a run of bytes through a pointer, subtracts a base value held in a data register from each byte and clamps negative results to zero, then combines adjacent pairs into 16-bit words that are stored sequentially through a second pointer.
- `expand_nibbles_into_words`: Reads a run of bytes through the pointer in a3, adds a common offset from d4 to each byte, clamps each sum to the range 0 to 15, and packs successive pairs of these values into 16-bit words that are stored sequentially through the pointer in a0.
- `fills_strided_table_entries`: Writes a fixed set of words into a table reached through a pointer, storing a value derived from a global variable in the first slot of each 22-word group and a constant in the others, for up to 34 groups.
- `updates_two_sprite_control_blocks`: Computes two byte values from table entries and global variables, and when each result is non-negative it stores the value in a global variable, advances the table pointer by 2, waits while a hardware status bit is set, and then writes position and control words plus long-word pointers into a hardware block.
