# Game 677 (Rugby - The World Cup): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00677). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (0 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 194 of 194 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 93844 |
| code | 652 |
| data | 797483 |
| padding | 5 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rugby - The World Cup.adf:bootblock | 1 | 33 | 1 |
| logged disk load from Rugby - The World Cup.adf boot-block load | 1 | 94 | 2 |
| logged disk load from Rugby - The World Cup.adf boot-block load | 1 | 65 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- Rugby - The World Cup/Rugby - The World Cup.adf: custom-format (non-DOS) disk, 1734 non-empty blocks: matched against track-loaded segments
- Rugby - The World Cup/Rugby - The World Cup.adf: the boot block reads 2048 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Rugby - The World Cup/Rugby - The World Cup.adf: the boot block reads 109056 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Rugby - The World Cup/Rugby - The World Cup.adf: 2 logged loads cover 111104 bytes of the image
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 433-447 (bytes 221696-229376) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.11)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 449-952 (bytes 229888-487936) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 954-954 (bytes 488448-488960) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.15)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 956-956 (bytes 489472-489984) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.37)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 958-958 (bytes 490496-491008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.26)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 960-960 (bytes 491520-492032) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.26)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 962-962 (bytes 492544-493056) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.37)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 964-964 (bytes 493568-494080) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 966-966 (bytes 494592-495104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.37)
- Rugby - The World Cup/Rugby - The World Cup.adf: blocks 968-968 (bytes 495616-496128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.47)
- 3 packed track region(s), 509952 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rugby - The World Cup.adf:bootblock

- `init_and_run_jumptable`: Initializes a structure pointed to by a register with fixed constants and a starting address, then repeatedly invokes a routine from a jump table until a flag in that structure becomes zero, and finally calls two more routines.

### logged disk load from Rugby - The World Cup.adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Rugby - The World Cup.adf boot-block load

- `copy_long_words_and_jump`: Copies a block of long words from a source pointed to by a1 to a destination pointed to by a0 using a counter in d7, then performs an indirect jump through a2.
- `copy_bytes_and_jump`: Copies bytes from a source pointed to by a1 to a destination pointed to by a0 using a counter in d7, then performs an indirect jump through a2.
- `copy_word_and_wait_status`: Copies a word from one structure into another, waits until two status words in the source become non-zero, and then stores fixed constants into several fields of the structure and a global variable.
- `test_bits_and_branch`: Calls another routine, then tests bit 0 of the byte at offset -1 from a0 and bit 6 of the byte at a0 to perform conditional branches.
- `clear_bit_and_store`: Clears bit 7 of d0 and stores the resulting byte into the memory location pointed to by a0.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_color_register`: Writes the value 3840 to the color register at offset 0 of the hardware registers and then branches to itself.
