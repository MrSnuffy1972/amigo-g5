# Game 3183 (combine (1989)(zuidenveld)(nl)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03183). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901632 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (0 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 68 of 68 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2560 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 579 of 579 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15099 |
| code | 3186 |
| data | 787717 |
| padding | 910 |
| unused | 94720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Combine (1989)(Zuidenveld)(Nl).adf:bootblock | 1 | 17 | 1 |
| logged disk load from Combine (1989)(Zuidenveld)(Nl).adf boot-block load | 1 | 12 | 0 |
| logged disk load from Combine (1989)(Zuidenveld)(Nl).adf boot-block load | 1 | 125 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 96 | 3 |
| Combine (1989)(Zuidenveld)(Nl).adf bytes 22528-23552 (code never loaded by a reference run) | 1 | 140 | 1 |
| Combine (1989)(Zuidenveld)(Nl).adf bytes 382976-384512 (code never loaded by a reference run) | 1 | 189 | 1 |

## Cases handled

- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: custom-format (non-DOS) disk, 1571 non-empty blocks: matched against track-loaded segments
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: the boot block reads 2048 bytes from disk offset 20480 to an address (found statically); recovered as a unit
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: the boot block reads 16384 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 68 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: bytes 22528-23552 look like code no run loaded: recovered position-independently (load address unknown)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: bytes 382976-384512 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: 4 logged loads cover 20992 bytes of the image
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 59-86 (bytes 30208-44544) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.10)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 97-138 (bytes 49664-71168) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.12)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 140-143 (bytes 71680-73728) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 145-151 (bytes 74240-77824) outside the filesystem hold data, not code (opcode density 0.001, entropy 0.88)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 161-192 (bytes 82432-98816) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.31)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 195-202 (bytes 99840-103936) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.68)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 205-207 (bytes 104960-106496) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 209-215 (bytes 107008-110592) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.15)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 220-225 (bytes 112640-115712) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.99)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 228-256 (bytes 116736-131584) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.71)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 258-266 (bytes 132096-136704) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.62)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 268-271 (bytes 137216-139264) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.94)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 273-279 (bytes 139776-143360) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.18)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 284-320 (bytes 145408-164352) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.71)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 322-328 (bytes 164864-168448) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 332-335 (bytes 169984-172032) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.10)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 337-343 (bytes 172544-176128) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.16)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 355-386 (bytes 181760-198144) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 390-390 (bytes 199680-200192) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.92)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 392-392 (bytes 200704-201216) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.97)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 396-399 (bytes 202752-204800) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.74)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 401-407 (bytes 205312-208896) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.95)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 413-695 (bytes 211456-356352) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.34)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 708-711 (bytes 362496-364544) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.87)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 716-746 (bytes 366592-382464) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.66)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 756-837 (bytes 387072-429056) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.80)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 840-939 (bytes 430080-481280) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.61)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 944-1107 (bytes 483328-567296) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.36)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1117-1132 (bytes 571904-580096) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.77)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1144-1149 (bytes 585728-588800) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.16)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1151-1170 (bytes 589312-599552) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.94)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1174-1179 (bytes 601088-604160) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.81)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1182-1187 (bytes 605184-608256) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.83)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1189-1200 (bytes 608768-614912) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.83)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1204-1209 (bytes 616448-619520) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.53)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1211-1230 (bytes 620032-630272) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.38)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1234-1238 (bytes 631808-634368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.09)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1242-1247 (bytes 635904-638976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.97)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1250-1259 (bytes 640000-645120) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.13)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1268-1268 (bytes 649216-649728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1271-1271 (bytes 650752-651264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.96)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1276-1321 (bytes 653312-676864) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.01)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1324-1378 (bytes 677888-706048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.99)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1380-1385 (bytes 706560-709632) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.20)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1387-1406 (bytes 710144-720384) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.06)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1410-1534 (bytes 721920-785920) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.98)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1543-1633 (bytes 790016-836608) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.18)
- combine (1989)(zuidenveld)(nl)/Combine (1989)(Zuidenveld)(Nl).adf: blocks 1636-1759 (bytes 837632-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.65)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Combine (1989)(Zuidenveld)(Nl).adf:bootblock

- `initialize_boot`: Sets up I/O request structures with specific command and address values, performs a hardware I/O operation, and then configures multiple data fields in a global structure before jumping to a routine located at a calculated address.

### logged disk load from Combine (1989)(Zuidenveld)(Nl).adf boot-block load

- `prepare_and_execute_io`: Configures an I/O request structure with command 2 and specific data pointers, executes it using DoIO, then configures the same structure with command 9 and executes it again.
- `update_dma_registers`: Updates DMA control registers and handles interrupt requests while managing a global variable through a loop of memory operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_game_loop`: Manages the main game loop by checking a condition, updating global variables, performing I/O operations, and handling input state changes.
- `update_score_counter`: Counts occurrences of a specific value in a memory range, updates a global variable with the count, and performs arithmetic operations on score-related variables before returning.
- `format_number_to_string`: Converts a number into a three-digit decimal string representation, storing each digit as an ASCII character in a buffer.

### Combine (1989)(Zuidenveld)(Nl).adf bytes 22528-23552 (code never loaded by a reference run)

- `initialize_dma_and_io`: Initializes DMA memory patterns and sets up I/O subsystems by opening and closing libraries, configuring hardware registers, and managing interrupt flags.

### Combine (1989)(Zuidenveld)(Nl).adf bytes 382976-384512 (code never loaded by a reference run)

- `setup_display_and_input`: Configures display and input handling by opening libraries, setting up hardware registers, and managing a loop that processes input states.
