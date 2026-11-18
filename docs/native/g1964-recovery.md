# Game 1964 (leander (1991)(psygnosis)[cr skr][t +3 goonies](disk 1 of 3)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01964). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (851968 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (4 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 147 of 147 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 191 of 191 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 950 |
| code | 810 |
| data | 848153 |
| padding | 7 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf:bootblock | 1 | 49 | 2 |
| logged disk load from Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf boot-block load | 1 | 142 | 10 |

## Cases handled

- leander (1991)(psygnosis)[cr skr][t +3 goonies](disk 1 of 3)[b]/Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf: custom-format (non-DOS) disk, 1658 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 169348 of its 169348 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- leander (1991)(psygnosis)[cr skr][t +3 goonies](disk 1 of 3)[b]/Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf: the boot block reads 2048 bytes from disk offset 1536 to an address (found statically); recovered as a unit
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 displacement
- leander (1991)(psygnosis)[cr skr][t +3 goonies](disk 1 of 3)[b]/Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf: 1 logged loads cover 2048 bytes of the image
- leander (1991)(psygnosis)[cr skr][t +3 goonies](disk 1 of 3)[b]/Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- 1 packed track region(s), 846336 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf:bootblock

- `allocate_memory_and_init_request`: Boot block that calls the exec library to allocate memory and makes one library call at offset minus 456, then stores fixed values into a request structure, including a pointer to a fixed address.
- `read_block_and_jump`: Fills in an I/O request for a 1536-byte read into a fixed buffer address, issues it through the device I/O call, then jumps into the code loaded at that buffer.

### logged disk load from Leander (1991)(Psygnosis)[cr SKR][t +3 Goonies](Disk 1 of 3)[b].adf boot-block load

- `initialize_boot_screen_and_poll_keyboard`: Clears a block of memory from 70000 to 7c000, draws text glyphs to a bitplane, and enters a loop that polls for keyboard input to toggle three global bytes or exit when a specific key is pressed, before calling a routine to copy code to low memory.
- `copy_code_to_low_memory`: Copies 132 bytes from a code block in the boot block to a fixed low-memory address with a byte loop, then returns.
- `frag_move_jmp`: Short code fragment that copies 184 to a fixed memory address, then jumps to another point in the code.
- `frag_move_jmp_2`: Short code fragment that copies 200 to a fixed memory address, then jumps to another point in the code.
- `frag_move_jmp_3`: Short code fragment that copies 210 to a fixed memory address, then jumps to another point in the code.
- `frag_move_jmp_4`: Short code fragment that copies 230 to a fixed memory address (2 times in a row), then jumps to another point in the code.
- `configure_video_attributes`: Loads a base address into register a0, reads from several memory locations with offsets, and writes specific values to memory locations based on conditional checks.
- `decode_keyboard_serial_byte`: Takes the raw byte from the keyboard serial register, inverts it and rotates it right by one bit, and stores the result in a global.
- `call_three_routines_sequentially`: Calls three unnamed routines one after another and returns.
- `draw_text_glyphs_to_bitplane`: Draws each character of a zero-terminated string into a bitplane 320 pixels wide, copying an eight-row glyph from a font table for each character and moving one column right per character.
