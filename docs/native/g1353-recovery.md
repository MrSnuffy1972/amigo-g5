# Game 1353 (xybots (1989)(domark)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01353). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 22 of 22 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 40 of 40 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 128 |
| data | 356875 |
| padding | 2933 |
| unused | 541184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xybots (1989)(Domark).adf:Xybots | 2 | 22 | 3 |
| Xybots (1989)(Domark).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- xybots (1989)(domark)/Xybots (1989)(Domark).adf:XYB.02ED: high-entropy data file (7.75 bits/byte), classified as packed data
- xybots (1989)(domark)/Xybots (1989)(Domark).adf:XYB.0269: high-entropy data file (7.63 bits/byte), classified as packed data
- xybots (1989)(domark)/Xybots (1989)(Domark).adf:XYB.02AB: high-entropy data file (7.66 bits/byte), classified as packed data
- xybots (1989)(domark)/Xybots (1989)(Domark).adf:XYB.0227: high-entropy data file (7.67 bits/byte), classified as packed data
- xybots (1989)(domark)/Xybots (1989)(Domark).adf:XYB.051D: high-entropy data file (7.83 bits/byte), classified as packed data
- xybots (1989)(domark)/Xybots (1989)(Domark).adf:XYB.032F: high-entropy data file (7.85 bits/byte), classified as packed data
- xybots (1989)(domark)/Xybots (1989)(Domark).adf:Xybots: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Xybots (1989)(Domark).adf:Xybots

- `store_return_address`: Computes the address of the following instruction into a0 and stores that address into a global variable at offset 32, then falls through into the next routine without returning.
- `init_system_and_dma`: Disables interrupts by writing to the status register, sets up the stack pointer, enables DMA channels by writing to the DMA enable register, and configures a parallel port control register by setting and clearing its bits, then returns.
- `copy_bytes_then_jump`: Copies bytes from the source pointed to by a1 into the destination pointed to by a0, decrementing a byte counter in d0 until it reaches zero, then loads the value an address into d0, subtracts 4 from a0, and jumps to the routine whose address is in a2.

### Xybots (1989)(Domark).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
