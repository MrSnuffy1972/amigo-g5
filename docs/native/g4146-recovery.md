# Game 4146 (kick' up (19xx)(m. silbernagl)[t +1 trilogy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04146). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (3 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 186 of 186 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 492 |
| data | 81092 |
| padding | 1128 |
| unused | 818176 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kick' Up (19xx)(M. Silbernagl)[t +1 Trilogy].adf:load | 2 | 176 | 11 |
| Kick' Up (19xx)(M. Silbernagl)[t +1 Trilogy].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- kick' up (19xx)(m. silbernagl)[t +1 trilogy]/Kick' Up (19xx)(M. Silbernagl)[t +1 Trilogy].adf:load: emu (own depacker stub) stream found by ancient scan (72812 -> 11550 bytes) but only 0 of 1 sampled windows are in reference RAM: not used
- kick' up (19xx)(m. silbernagl)[t +1 trilogy]/Kick' Up (19xx)(M. Silbernagl)[t +1 Trilogy].adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kick' Up (19xx)(M. Silbernagl)[t +1 Trilogy].adf:load

- `decode_data_stream`: Reads compressed data from a bit stream and writes decompressed bytes to a destination buffer, handling both literal bytes and back references with variable lengths.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `setup_dma_channels`: Configures DMA control registers to enable specific channels and sets up memory address pointers for data transfer operations.
- `transfer_and_shift`: Moves data between memory locations, performs bit shifting operations, and handles conditional branches based on shift results while managing loop counters.
- `frag_moveq_bra`: Short code fragment that loads 7 into d1, then loads 8 into d4, then jumps to another point in the code.
- `frag_move_negx_nop_roxr_rts`: Short code fragment that copies the memory just below a0 (moving a0 down) to d0, then negates the memory at a5 with the extend flag, then pauses for one no-operation step, then copies 16 to the condition codes, then rotates d0 right through the extend flag by 1, then returns.
- `frag_moveq_move_add_addq_bsr_dbra_cmpa`: Short code fragment that loads 9 into d1, then copies d2 to d3, then adds d2 to d1, then adds 2 to d3, then calls another routine, then copies an indexed entry from a2 to the memory just below a2 (moving a2 down), then loops back while counting down d3, then compares a1 with a2, then branches if the comparison was less, then jumps to another point in the code.
- `frag_subq_moveq_lsr_bne_bsr_roxl_dbra_r`: Short code fragment that subtracts 1 from d1, then loads 0 into d2, then shifts d0 right by 1, then branches if the last result was not zero, then calls another routine, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `load_data_block`: Copies data from one memory location to another while incrementing both pointers, then performs conditional jumps based on comparison results with immediate values.
- `process_buffer_data`: Reads bytes from a source buffer and writes them to a destination, with loop control and conditional branching based on byte comparisons and counters.

### Kick' Up (19xx)(M. Silbernagl)[t +1 Trilogy].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
