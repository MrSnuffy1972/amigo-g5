# Game 995 (war zone (1991)(core design)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00995). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 13824 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 126 of 126 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 44525 |
| code | 404 |
| data | 621775 |
| unused | 234496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| War Zone (1991)(Core Design)[cr FLT].adf:bootblock | 1 | 35 | 2 |
| logged disk load from War Zone (1991)(Core Design)[cr FLT].adf boot-block load | 1 | 87 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| War Zone (1991)(Core Design)[cr FLT].adf bytes 95744-100864 (code never loaded by a reference run) | 1 | 0 | 0 |
| War Zone (1991)(Core Design)[cr FLT].adf bytes 105984-108032 (code never loaded by a reference run) | 1 | 0 | 0 |
| War Zone (1991)(Core Design)[cr FLT].adf bytes 108544-115200 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: custom-format (non-DOS) disk, 1284 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: the boot block reads 45056 bytes from disk offset 675840 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: bytes 95744-100864 look like code no run loaded: recovered position-independently (load address unknown)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: bytes 105984-108032 look like code no run loaded: recovered position-independently (load address unknown)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: bytes 108544-115200 look like code no run loaded: recovered position-independently (load address unknown)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: 4 logged loads cover 58880 bytes of the image
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: blocks 122-122 (bytes 62464-62976) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.57)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: blocks 126-163 (bytes 64512-83968) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.98)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: blocks 168-168 (bytes 86016-86528) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: blocks 172-182 (bytes 88064-93696) outside the filesystem hold data, not code (opcode density 0.037, entropy 6.65)
- war zone (1991)(core design)[cr flt]/War Zone (1991)(Core Design)[cr FLT].adf: blocks 226-226 (bytes 115712-116224) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- 3 never-loaded region(s), 13824 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 580096 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### War Zone (1991)(Core Design)[cr FLT].adf:bootblock

- `load_and_jump_to_entry`: Boot-block entry that issues exec disk-read requests, calls system library routines and jumps to the pointer obtained from the first load, returning minus one if that pointer is null.
- `issue_disk_read_and_return`: Fills an exec I/O request with a destination address, length and offset, sends it with DoIO, calls one unnamed routine, and returns a fixed value.

### logged disk load from War Zone (1991)(Core Design)[cr FLT].adf boot-block load

- `unpack_bitstream_into_buffer_2`: Unpacks a packed stream using the bit-reader unnamed routines, writing literal bytes or copying earlier output backward until the destination reaches the start of the buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_color_and_halt`: Sets the first color register to a constant value and enters an infinite loop, otherwise it jumps to a specific address.
