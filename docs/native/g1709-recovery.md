# Game 1709 (titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01709). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902240 top-level bytes in 92 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (2 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 443 of 443 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 652 of 652 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 263234 |
| code | 1956 |
| data | 583205 |
| padding | 23125 |
| unused | 30720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:main | 8 | 231 | 6 |
| Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:Train | 2 | 214 | 7 |
| Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 195 | 1 |

## Cases handled

- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK08.BIN: high-entropy data file (7.75 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL03.TBL: high-entropy data file (7.91 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL10.TBL: high-entropy data file (7.95 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:objet2.spr: high-entropy data file (7.73 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK06.BIN: high-entropy data file (7.63 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL01.TBL: high-entropy data file (7.87 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:jeu32.pat: high-entropy data file (7.91 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:fox.spr: high-entropy data file (7.78 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK11.BIN: high-entropy data file (7.67 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL09.TBL: high-entropy data file (7.92 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL07.TBL: high-entropy data file (7.92 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL05.TBL: high-entropy data file (7.91 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL13.TBL: high-entropy data file (7.89 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK09.BIN: high-entropy data file (7.86 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK16.BIN: high-entropy data file (7.72 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL11.TBL: high-entropy data file (7.95 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK07.BIN: high-entropy data file (7.62 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:jeu42.pat: high-entropy data file (7.89 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK14.BIN: high-entropy data file (7.65 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:monster2.spr: high-entropy data file (7.89 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:BRIK05.BIN: high-entropy data file (7.87 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL08.TBL: high-entropy data file (7.97 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:titus.pc1: high-entropy data file (7.65 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL06.TBL: high-entropy data file (7.94 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:present2.pc1: high-entropy data file (7.73 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL14.TBL: high-entropy data file (7.87 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL04.TBL: high-entropy data file (7.92 bits/byte), classified as packed data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:LEVEL12.TBL: high-entropy data file (7.96 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:main: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- titus the fox - to marrakech and back (1992)(titus)[cr cp][t + 3 cp]/Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:Train: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_dma_and_coprocessor` (runtime segment an address (play-rs-x25k)): Disables interrupts, opens a library, and configures DMA channels and coprocessor jump vectors before re-enabling interrupts and returning. Kind: DMA/Coprocessor-based hardware integrity check. Pass path: The branch following the routine that continues normal execution (likely the one not taken during a failed hardware/checksum mismatch)..

## Functions


### Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:main

- `free_memory_and_decrement_counters`: Releases a block of memory and executes a loop that decrements a 16-bit counter twice until it reaches negative one.
- `decrement_until_minus_one`: Decrements a 16-bit counter register in a loop until it reaches minus one, then returns.
- `decode_bits_in_four_passes`: Performs four passes, each calling a helper through a stored pointer and then running a loop that reads bit-coded values.
- `decode_backward_bitstream_runs`: Decodes a bit-coded stream read backward from memory, reading length fields and copying runs of bytes to the destination.
- `decode_backward_bitstream_blocks`: Larger variant of the backward bit-stream decoder that reads bit-coded counts and offsets and copies runs from earlier output.
- `modify_and_push_register_values`: Shifts the least significant bit of a register right, applies a bitwise OR with a constant, and pushes the modified register, a second register, and a 16-bit register onto the stack before restoring a set of registers and branching to another routine.

### Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:Train

- `run_entry_with_task_memory`: Program entry point of the train module, shown without pseudo-code.
- `stub_immediate_return`: The decompiled body is only an immediate return with no work, although the listing records twenty instructions, so only that empty return is described.
- `decrement_word_until_wrap`: Decrements a 16-bit register in a loop until it wraps past zero, then returns.
- `decode_table_driven_copy_runs`: Reads values from a lookup table and copies bytes from an earlier position of the output into the current position, looping until the output limit is reached.
- `copy_byte_on_counter_boundary`: When the low seven bits of a counter register are zero, copies one byte from the address register into a shared global variable, otherwise it returns.
- `repeat_call_until_counter_negative`: Calls one unnamed routine repeatedly, decrementing a byte counter on each pass, until the counter goes negative.
- `expand_run_length_bytes`: Copies bytes from input to output, and when a byte equals a stored marker value it reads a count and writes that byte repeatedly.

### Titus the Fox - To Marrakech and Back (1992)(Titus)[cr CP][t + 3 CP].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_and_coprocessor`: Disables interrupts, opens a library, and configures DMA channels and coprocessor jump vectors before re-enabling interrupts and returning.
