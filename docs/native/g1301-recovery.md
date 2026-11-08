# Game 1301 (dungeon master (1988)(ftl)[hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01301). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (0 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6 of 6 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 64 of 68 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 510904 |
| code | 198 |
| data | 185697 |
| padding | 4641 |
| unused | 199680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| dm | 144 | 19 | 2 |
| exec | 5 | 11 | 1 |
| killptr | 3 | 11 | 1 |
| bye | 4 | 17 | 1 |
| lpatch | 4 | 10 | 1 |

## Cases handled

- dungeon master (1988)(ftl)[hd]/Dungeon Master (1988)(FTL)[HD].adf:DM/Dungeon.dat: high-entropy data file (7.85 bits/byte), classified as packed data
- dungeon master (1988)(ftl)[hd]/Dungeon Master (1988)(FTL)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- dungeon master (1988)(ftl)[hd]/Dungeon Master (1988)(FTL)[HD].adf:DM/dm: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- dungeon master (1988)(ftl)[hd]/Dungeon Master (1988)(FTL)[HD].adf:DM/lpatch: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### dm

- `open_explode_library_conditional`: Sets register d0 to zero, loads the library base pointer, and calls the open routine to load explode.library, then calls a second library entry point, arithmetic-shifts the result right by the value in d4, clears d7, and returns if the result is not negative.
- `select_handler_by_flags`: Tests the carry flag and a signed byte comparison to jump to one of several handler addresses, then calls the selected routine via a branch.

### exec

- `open_explode_library`: Sets register d0 to two, loads the library base pointer, and calls the open routine to load explode.library, then calls a second library entry point.

### killptr

- `open_explode_library_shift`: Sets register d0 to two, loads the library base pointer, and calls the open routine to load explode.library, then calls a second library entry point and shifts the result left by eight.

### bye

- `initializes_explode_library`: Loops calling a routine from a table until it returns a nonzero value, then calls another routine from a second table.

### lpatch

- `open_explode_library_arshift`: Sets register d0 to one, loads the library base pointer, and calls the open routine to load explode.library, then calls a second library entry point and arithmetic-shifts the result right by four.
