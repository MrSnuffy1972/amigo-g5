# Game 440 (Super Skidmarks): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00440). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5406720 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (2 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 267 of 267 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2384377 |
| code | 632 |
| data | 2770788 |
| padding | 5675 |
| unused | 245248 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Installer | 2 | 267 | 8 |

## Cases handled

- super skidmarks (1995)(acid software)(aga)(disk 1 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 1 of 6)[HD].adf:smcd32a.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- super skidmarks (1995)(acid software)(aga)(disk 2 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 2 of 6)[HD].adf:smcd32b.lha: high-entropy data file (7.90 bits/byte), classified as packed data
- super skidmarks (1995)(acid software)(aga)(disk 6 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 6 of 6)[HD].adf:smcd32f.lha: high-entropy data file (7.97 bits/byte), classified as packed data
- super skidmarks (1995)(acid software)(aga)(disk 2 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 2 of 6)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- super skidmarks (1995)(acid software)(aga)(disk 3 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 3 of 6)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- super skidmarks (1995)(acid software)(aga)(disk 4 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 4 of 6)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- super skidmarks (1995)(acid software)(aga)(disk 5 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 5 of 6)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- super skidmarks (1995)(acid software)(aga)(disk 6 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 6 of 6)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- super skidmarks (1995)(acid software)(aga)(disk 1 of 6)[hd]/Super Skidmarks (1995)(Acid Software)(AGA)(Disk 1 of 6)[HD].adf:c/Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Installer

- `process_hunk_relocation_data`: Walks through a sequence of AmigaDOS hunk-type markers, relocating and copying the data blocks they reference into freshly allocated memory, then stores the final marker and returns.
- `decrement_counter_return`: Decrements a 16-bit counter in d1 until it reaches zero, then returns without performing any other action.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_byte_store_buffer`: Sets up several constant registers, loads a byte from the pointer in a0, and if it is non-zero it calls another routine and shifts the value before storing bytes into a buffer via a pointer in a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_64bit_fetch_longword`: Rotates a 64-bit value across d5 and d0 by a shift count in d4, fetching the next longword from the pointer in a0 when the value becomes zero, then returns.
- `store_byte_and_call_dos`: Stores a byte from register d1 into a global variable through a pointer in a2, then sets bits 0, 3, 5, and 6 of register d1 and calls the DOS library.
