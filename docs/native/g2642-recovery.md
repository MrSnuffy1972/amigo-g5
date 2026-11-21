# Game 2642 (AlchemyDemo v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02642). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (816127 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (1 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 383 of 383 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 55907 |
| code | 1066 |
| data | 759154 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| mine | 1 | 127 | 2 |
| snow | 1 | 127 | 2 |
| village | 1 | 127 | 2 |

## Cases handled

- AlchemyDemo_v1.0/AlchemyDemo_v1.0.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree AlchemyDemo_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_dma_transfer_2` (mine): Sets up DMA transfer parameters by writing high values to several hardware control registers and initializes a data pointer from another routine. Kind: disk hardware check. Pass path: ADKCON register is set correctly and DMA transfer proceeds normally.
- `initialize_interrupts` (snow): Enables interrupts and DMA channels by setting specific bits in hardware control registers, then loads data from a routine entry point to memory. Kind: disk hardware check. Pass path: ADKCON register is set correctly to enable disk operations, typically checking for proper disk drive initialization and hardware presence.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### mine

- `initialize_dma_transfer_2`: Sets up DMA transfer parameters by writing high values to several hardware control registers and initializes a data pointer from another routine.
- `initialize_dma_transfer`: Sets up DMA transfer parameters by writing high values to several hardware control registers and initializes a data pointer from a routine entry address.

### snow

- `initialize_interrupts`: Enables interrupts and DMA channels by setting specific bits in hardware control registers, then loads data from a routine entry point to memory.
- `setup_display_registers`: Configures display-related hardware registers with specific values and prepares a memory area for data transfer using a pointer loaded from another location.

### village

- `setup_graphics_system`: Initializes graphics hardware by configuring color and display registers, sets up a data transfer buffer, and begins copying data between memory locations.
- `configure_video_hardware`: Writes predefined values to video control registers and sets up a memory buffer address for DMA operations through a pointer in register a2.
