# Game 2833 (battle duel - striving for glory (1995)(j. tertseige)[hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02833). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (2 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 17 of 17 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 64 of 64 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2752 |
| code | 208 |
| data | 626706 |
| padding | 3678 |
| unused | 267776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Installer | 2 | 32 | 3 |
| Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:LX | 2 | 32 | 3 |

## Cases handled

- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:English.lha: high-entropy data file (7.98 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Gfx_AGA.lha: high-entropy data file (8.00 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Deutsch.lha: high-entropy data file (7.98 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:MWB_Icons.lha: high-entropy data file (7.92 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Landscapes_ECS.lha: high-entropy data file (8.00 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Icons.lha: high-entropy data file (7.85 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Gfx_ECS.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Program.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Landscapes_AGA.lha: high-entropy data file (8.00 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Mods.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Installer: PowerPacker 4.0 Library (xfd) stream found by ancient scan (62928 -> 115144 bytes) but only 0 of 57 sampled windows are in reference RAM: not used
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:LX: PowerPacker 4.0 Library (xfd) stream found by ancient scan (12184 -> 17040 bytes) but only 0 of 8 sampled windows are in reference RAM: not used
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- battle duel - striving for glory (1995)(j. tertseige)[hd]/Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:LX: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:Installer

- `initialize_system`: Initializes the system by opening multiple libraries, setting up pointers, and managing memory allocation through several library calls.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_library`: Loads a library using a specific identifier and performs a bitwise OR operation on a data register before making a system call.

### Battle Duel - Striving for Glory (1995)(J. Tertseige)[HD].adf:LX

- `setup_executive`: Sets up the executive environment by opening libraries, handling conditional execution paths, and performing memory management operations.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `load_and_or`: Loads a word from a global variable at offset 3077, performs a bitwise OR operation on another register with the value 144, and makes a system call.
