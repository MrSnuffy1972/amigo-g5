# Game 713 (tetris pro (1993)(logic systems)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00713). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (1 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 154 of 154 (100.00%) |
| Independent decoder cross-check (Ghidra against capstone) | 223 of 223 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 628 |
| code | 568 |
| data | 315821 |
| padding | 1447 |
| unused | 582656 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Tetris Pro (1993)(Logic Systems).adf:tetris | 11 | 205 | 7 |
| Tetris Pro (1993)(Logic Systems).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- tetris pro (1993)(logic systems)/Tetris Pro (1993)(Logic Systems).adf:data tetris: high-entropy data file (7.99 bits/byte), classified as packed data
- tetris pro (1993)(logic systems)/Tetris Pro (1993)(Logic Systems).adf:data tetris2: high-entropy data file (7.99 bits/byte), classified as packed data
- tetris pro (1993)(logic systems)/Tetris Pro (1993)(Logic Systems).adf:data tetris3: high-entropy data file (7.98 bits/byte), classified as packed data
- tetris pro (1993)(logic systems)/Tetris Pro (1993)(Logic Systems).adf:tetris: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Tetris Pro (1993)(Logic Systems).adf:tetris

- `initialize_game_state`: The routine calls another routine multiple times, invokes a function pointer taken from a global variable, clears a word at an address derived from a register, and returns the concatenation of two input registers.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `initialize_board_state`: The routine clears a fixed region of memory to zero, then repeatedly invokes other routines to update game state and shift rows of bytes in a buffer.
- `self`: The routine processes a 16-bit value by repeatedly shifting it right and mixing in bits from a table of 16-bit words, building an index from the extracted bits.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `self_2`: The routine copies a short from a source pointer into a destination table and updates a running sum.

### Tetris Pro (1993)(Logic Systems).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
