# Game 1863 (final fight (1991)(u. s. gold)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01863). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1816480 top-level bytes in 144 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 37 raw-word lines |
| Functions named and described | FAIL, 98 of 183 named (0 author symbols), 98 of 183 described (1 by rule, 90 by the local model) |
| Executed addresses (reference run) in recovered, described code | FAIL, 918 of 1170 (78.46%) |
| Recovered without load address (counted in proof 1, flagged) | 132356 bytes, 12 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4386 of 4394 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 69710 |
| code | 12542 |
| data | 1212549 |
| padding | 17871 |
| unused | 503808 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:final | 2 | 85 | 3 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:benny | 2 | 85 | 3 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:pal | 1 | 2 | 1 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:bootblock | 1 | 54 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 260 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 478 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2875 | 118 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:final#unpacked | 1 | 441 | 12 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level6.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level3.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level1.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level5.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level2.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bred2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (14988 -> 28678 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:guy.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (23204 -> 46450 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:weapons.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (4380 -> 6646 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bred.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (15676 -> 28422 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:heads.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (7696 -> 8626 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:trainstrap.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (2564 -> 2672 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bigboss.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (18680 -> 34928 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:axl.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17196 -> 33600 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:gymnast.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (18364 -> 40350 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:axl2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17036 -> 33600 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:chars.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (7732 -> 9390 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:cody.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (37776 -> 68972 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:haggarpics.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16200 -> 21882 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:j.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (12192 -> 22100 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:rasta.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (24080 -> 39182 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:giant2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17812 -> 34890 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:chandelier.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (4688 -> 6908 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bred3.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16008 -> 28550 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:guys.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (13244 -> 17766 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:maptrain.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (13760 -> 17062 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:knifeguy.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17744 -> 33564 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:dontusedrugs.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (6708 -> 8452 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:j2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (12440 -> 22236 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:giant.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (17324 -> 37092 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:fflogo.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (9972 -> 13506 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:cop.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (26668 -> 41304 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:townmap.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (10168 -> 13694 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bill.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (21856 -> 38870 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:haggar.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (23696 -> 49310 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:knifeguy2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16616 -> 33564 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:swordsman.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (19276 -> 34380 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:guys_ring.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (14484 -> 18256 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:mod.mit: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (132768 -> 180488 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bill2.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (20468 -> 38870 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:burntguy.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (2744 -> 4876 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level6.amap: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (15000 -> 22260 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:weapons.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (4380 -> 6646 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level2.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (28592 -> 61440 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level8.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (6660 -> 16000 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level4.amap: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (6472 -> 7684 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level1.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (31740 -> 61184 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:cody.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (37776 -> 68972 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:tv.iff: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (24152 -> 31964 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level3.amap: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (6768 -> 8708 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level7.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (16804 -> 32128 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:glass.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (6644 -> 12550 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level6.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (30996 -> 57472 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level1.amap: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (5328 -> 6660 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level5.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (33660 -> 54912 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level5.amap: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (12040 -> 18436 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:flames.sms: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (3880 -> 4870 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:haggar.mps: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (23696 -> 49310 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level4.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (27376 -> 46848 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level2.amap: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (8188 -> 10756 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level3.chites: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (36556 -> 58368 bytes)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:l8mask.bin: (Vic2) Data Cruncher (xfd)-packed, unpacked with ancient (2240 -> 6400 bytes)
- runtime segment an address: 199 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 61 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 363 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 385 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:final: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 40916 -> 65536 bytes), 27 of 29 windows confirmed in reference RAM, loaded at an address: recovered as u11
- final fight (1991)(u. s. gold)[cr qtx](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:benny: ByteKiller 1.3 (xfd) stream found by ancient scan (2828 -> 39664 bytes) but only 2 of 2 sampled windows are in reference RAM: not used
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level6.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level3.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level1.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level5.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr qtx](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:level2.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target, 1 displacement
- 6 never-loaded region(s), 132356 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 11264 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `hash_and_load_resource_table` (runtime segment an address (play-long-joy)): Computes a checksum of a string to generate a hash and searches for a matching entry in a data block. Kind: resource table hash check and disk-based loader. Pass path: DAT_00001af8 = 0; return 0;.
- `validate_and_load_disk_data` (runtime segment an address (play-long-joy)): Verifies the integrity of a data block using a checksum and marker validation before updating a global table with the processed data. Kind: disk track/checksum validation. Pass path: d0 == 0.
- `check_hardware_input` (runtime segment an address (play-long-joy)): Manipulates a bitmask for a hardware port and enters a loop that checks a value retrieved from a table. Kind: disk hardware/track check. Pass path: return 1.
- `reset_global_values` (runtime segment an address (play-long-joy)): Sets a global variable to zero and another memory location to all ones while touching a CIAB register. Kind: state reset/sanitization. Pass path: the routine clears/resets global state variables to a known baseline before proceeding with the next stage of loading or execution.
- `initialize_hardware_state_2` (runtime segment an address (play-long-joy)): Updates various hardware registers and global variables, modifies a bitmask for a hardware port, and populates multiple memory blocks using attribute tables. Kind: disk hardware/sync initialization/check. Pass path: execution continues to the next instruction after the hardware setup/sync calls.

## Functions


### Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:final

- `decode_bit_packed_stream`: Program entry of 69 instructions that writes the colour register and calls a depacker bit-reader refill routine and a multi-bit bit-reader routine.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:benny

- `decode_packed_stream`: The program entry routine that calls a bit-reader refill and a multi-bit read to decode a packed stream, and writes to the COLOR00 colour register.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:pal

- `store_constant_and_return`: Two-instruction program entry that stores a fixed value into a global variable and returns.

### Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Final Fight (1991)(U. S. Gold)[cr QTX](Disk 2 of 2).adf:bootblock

- `find_resident_and_start_copper`: Boot block that finds a resident module through the exec library, makes several library calls at fixed offsets, clears a block of longwords in a loop, then loads the copper list pointer and starts the copper.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sub_0000`: undescribed.
- `sub_00B8`: undescribed.
- `sub_0112`: undescribed.
- `sub_0140`: undescribed.
- `sub_014C`: undescribed.
- `sub_0172`: undescribed.
- `sub_01DC`: undescribed.
- `sub_01FC`: undescribed.
- `sub_0226`: undescribed.
- `sub_0242`: undescribed.
- `sub_025C`: undescribed.
- `sub_0266`: undescribed.
- `sub_026C`: undescribed.
- `sub_028C`: undescribed.
- `sub_02A8`: undescribed.
- `sub_02B4`: undescribed.
- `sub_02EA`: undescribed.
- `sub_0372`: undescribed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `relocate_code_and_patch_pointers`: Copies a block of code to a new memory location and performs a checksum verification.
- `relocate_and_patch_pointers_2`: Updates a list of pointers by adding an offset and copies a small amount of data from the code.
- `relocate_and_patch_pointers`: Writes a constant value to a memory block, updates a list of pointers by adding an offset, and copies a small amount of data from the code segment to a destination.
- `sub_00CE`: undescribed.
- `copy_and_clear_memory`: Copies a block of memory from a source to a destination using either forward or backward iteration depending on the address comparison.
- `clear_memory_block`: Repeatedly writes zero values to a specified memory range using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sub_0008`: undescribed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sub_0000`: undescribed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_byte`: Checks a specific bit in a byte and updates a global variable accordingly.
- `do_nothing`: This routine is empty and performs no operations.
- `initialize_data_structures`: Writes several constant values to offsets within a memory block pointed to by a register.
- `sub_00E4`: undescribed.
- `initialize_hardware_and_tables`: Writes values to various hardware registers and populates a memory table with a specific address through a loop.
- `fill_structured_table`: Iterates through a loop to write a sequence of values into a table, where each entry consists of a word, a long value, and another word.
- `initialize_zero_table`: Fills a table with alternating values, where one word is an incrementing constant and the next word is zero.
- `clamp_table_values`: Iterates through a table and updates its contents by combining nibbles from a source table, ensuring the resulting values are clamped within specific ranges.
- `initialize_runtime_data_tables`: Populates several memory tables with interleaved incrementing values, constants, and data from a source table.
- `populate_data_table`: Iterates through a loop to interleave values from a source table into a destination table using a specific incrementing pattern.
- `configure_hardware_registers`: Writes specific values to several hardware registers, including CIA and interrupt control registers, and a memory location.
- `update_global_pointer_via_table_2`: Calculates a bit position and an offset to modify a bit in a table and updates a global variable using a table index.
- `modify_bit_in_table`: Uses a value to calculate a bit position and an offset to set or clear a specific bit within a table.
- `update_global_pointer_via_table`: Uses a value in a register to perform bitwise operations and index into a table to update a global variable.
- `sub_05B8`: undescribed.
- `sub_05FE`: undescribed.
- `call_subroutine_and_store`: Calls another routine and then stores a constant value into a global variable.
- `initialize_system_tables_and_hardware`: Clears a large block of memory, initializes a table with function pointers, and performs a sequence of hardware-related setup calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_initialize_data_tables`: Clears two consecutive memory blocks by zeroing out longwords and words respectively.
- `process_structured_data_table`: Decrements a counter and, upon reaching zero, iterates through a table to reset fields, copy strings, and skip whitespace.
- `initialize_control_demonstration_buffer`: Clears a memory table and then copies a string from a buffer while skipping whitespace characters.
- `clear_memory_blocks`: Iterates through a list of pointers to zero out memory and then clears a separate block of memory using a loop.
- `process_memory_block_offsets`: Checks a pointer from a table, clears a specific memory range, and then iterates through a linked list to subtract an offset from various values.
- `traverse_linked_list_and_compare`: Iterates through a linked list by following pointers, calculating a value based on an offset from a memory address, and comparing it against zero.
- `process_linked_list_flags`: Iterates through a linked list to clear a bit in a byte, sets a bit in another byte based on table offsets, and searches for the minimum value in a list of structures.
- `process_linked_list_node`: Checks a pointer retrieved from a table, and if the pointed-to value is zero, executes a sequence of operations involving global variable increments and further function calls.
- `calculate_relative_offsets`: Calculates and stores relative offsets or halved offsets into a table based on the contents of a linked list structure.
- `generate_lookup_tables`: Populates a table with incrementing values and generates a second table of bit-manipulated bytes, then uses these to fill a global buffer.
- `manage_state_counters`: Updates a global counter, executes a function pointer if it is non-zero, and performs conditional resets and bitwise inversions on global variables.
- `sub_0778`: undescribed.
- `check_and_assign_pointer`: Checks if a global variable is zero and, if not, assigns the current stack pointer to that global variable before returning the value pointed to by the new address.
- `update_global_with_stack_address`: If a global variable is non-zero, it updates that variable with the address of a local stack buffer.
- `hash_and_load_resource_table`: Computes a checksum of a string to generate a hash and searches for a matching entry in a data block.
- `validate_and_cache_data_block`: Checks a value against a threshold and recursively processes data blocks while updating a global variable with calculated indices.
- `validate_and_load_disk_data`: Verifies the integrity of a data block using a checksum and marker validation before updating a global table with the processed data.
- `conditionally_call_routine`: Checks a global variable against specific bitmasks and sign bits to determine whether to call another routine.
- `update_global_with_stack_address_2`: Updates a global variable based on a comparison and bitwise operations, potentially entering a loop that waits for a non-zero value.
- `update_hardware_status`: Modifies a global variable and several other memory locations while interacting with CIAA and CIAB port registers.
- `check_hardware_input`: Manipulates a bitmask for a hardware port and enters a loop that checks a value retrieved from a table.
- `reset_global_values`: Sets a global variable to zero and another memory location to all ones while touching a CIAB register.
- `store_indexed_value`: Stores a global variable into a memory location calculated using that same variable as an index.
- `increment_global_counter`: Increments a global variable and performs a check to see if it has wrapped or returned to zero.
- `calculate_string_length`: Iterates through a null-terminated string to determine its length.
- `sub_128A`: undescribed.
- `sub_12A2`: undescribed.
- `sub_1304`: undescribed.
- `sub_130C`: undescribed.
- `sub_1330`: undescribed.
- `sub_134C`: undescribed.
- `return_stack_value`: This routine is empty and simply returns the value currently residing at the top of the stack.
- `initialize_hardware_state_2`: Updates various hardware registers and global variables, modifies a bitmask for a hardware port, and populates multiple memory blocks using attribute tables.
- `store_indexed_values`: Stores specific values into memory locations calculated using a base address and offsets.
- `sub_1402`: undescribed.
- `sub_140A`: undescribed.
- `fill_memory_blocks`: Fills multiple memory blocks with values retrieved from a table based on a global variable.
- `sub_1420`: undescribed.
- `skip_whitespace_and_copy`: Iterates through a buffer, skipping space characters, and processes data until a specific byte threshold is met.
- `copy_structured_data`: Copies specific bytes from a source table into a destination buffer using a structured loop.
- `transfer_large_blocks`: Transfers data from one memory area to another by copying specific word segments in a loop.
- `extract_interleaved_resource_data`: Iterates through a source table to extract specific bytes at fixed offsets and copies them into a destination buffer.
- `reorganize_memory_segments`: Copies data from a source table to a destination, rearranging the byte order and offsets during the process.
- `copy_interleaved_data`: Copies data from a source to a destination by selecting specific words at regular intervals.
- `copy_memory_block`: Copies sixteen bytes from a source pointer to a destination pointer.
- `sub_159E`: undescribed.
- `sub_15C8`: undescribed.
- `sub_160E`: undescribed.
- `clear_pointer_chain`: Iterates through a linked list of pointers, setting each referenced memory location to zero until a null pointer is encountered.
- `perform_crc_calculation`: Calculates a cyclic redundancy check by iterating through the bits of an input value and applying a polynomial XOR operation to a global variable.
- `multiply_and_shift`: Multiplies two values and returns the result shifted right by sixteen bits.
- `sub_17EE`: undescribed.
- `sub_1836`: undescribed.
- `sub_1838`: undescribed.
- `validate_resource_header`: Checks a memory location for specific magic marker values.
- `unpack_compressed_data`: This routine performs a multi-stage decompression of a data block, likely using a variation of a Huffman or bit-stream encoding.
- `copy_and_swap_data`: Copies a block of data from a source to a destination and then performs a series of 16-bit word swaps on a subsequent memory range.
- `decompress_variable_bitstream_2`: Expands a compressed bitstream into a destination buffer using variable-length codes and table lookups.
- `decompress_bitstream_data_2`: Processes a compressed bitstream using variable-length codes and table lookups to expand data into a destination buffer.
- `decode_variable_length_bitstream_8`: Processes a compressed bitstream by reading bits to perform table lookups and updating a bit-offset state.
- `calculate_bit_offset`: Calculates a bitwise difference and stores the resulting value, a shifted bitfield, and a new pointer into a table of global variables.
- `extract_bitstream_from_word_table_2`: Extracts a sequence of bits from a series of 16-bit words by shifting and masking values.
- `extract_bitstream_from_word_table`: Extracts a sequence of bits from a series of 16-bit words by shifting and masking values.
- `decode_bitstream_state`: Iterates through a bitstream by checking bits and performing table lookups to update a set of global variables representing the current decoding state.
- `sub_1CA6`: undescribed.
- `decode_variable_length_bitstream_7`: Decodes a compressed bitstream by performing table lookups based on extracted bit sequences.
- `decode_variable_length_bitstream_6`: Extracts data from a compressed bitstream by reading variable-length codes using a lookup table.
- `decode_variable_bitstream`: Decodes a variable-length bitstream by extracting bits through shifts and masks to perform table lookups.
- `decode_bitstream_table_lookup`: Decodes a variable-length bitstream by performing bitwise shifts and masking.
- `decode_variable_length_bitstream_5`: Extracts data from a compressed bitstream by reading variable-length codes using a lookup table.
- `decode_variable_length_bitstream_4`: Extracts data from a compressed bitstream using a lookup table and variable-length encoding.
- `decode_bitstream_with_lookup_table`: Extracts data from a variable-length bitstream and uses values found in a lookup table to perform further bitstream decoding or data expansion.
- `decode_variable_length_bitstream`: Reads a sequence of bits from a buffer to traverse a tree-based or table-based compressed data structure.
- `extract_bitstream_data`: Reads a variable number of bits from a bitstream buffer using a pointer and a bit-count tracker.
- `decode_variable_length_bitstream_3`: Extracts values from a bitstream using variable-length encoding and writes the resulting bytes to a destination buffer.
- `decompress_variable_bitstream`: Extracts values from a bitstream using variable-length encoding and a lookup table.
- `decode_variable_bitstream_2`: Decodes a variable-length bitstream by extracting bits from a source buffer and using them as indices into lookup tables.
- `extract_bitstream_data_2`: Reads a variable-length bitstream from a source buffer using bit-shifting and masking operations.
- `decode_variable_length_bitstream_2`: Reads a bitstream from a source buffer using bit-shifting and masking to extract values.
- `decompress_bitstream_data`: Performs bitstream decompression using a variable-length coding scheme.
- `reset_global_variable`: Sets a global variable to zero and returns.
- `sub_1E38`: undescribed.
- `sub_1EA8`: undescribed.
- `sub_1ED0`: undescribed.
- `sub_1EF0`: undescribed.
- `sub_1F10`: undescribed.
- `sub_1F3A`: undescribed.
- `sub_1F50`: undescribed.
- `sub_1F54`: undescribed.
- `sub_1F7A`: undescribed.
- `sub_1F90`: undescribed.
- `sub_1FC8`: undescribed.
- `parse_and_unpack_asset_data`: Searches a data buffer for specific magic markers to identify asset headers, then unpacks bit-packed data and performs a memory copy of variable-length compressed elements into a destination buffer.
- `sub_2150`: undescribed.
- `copy_structured_data_buffer`: Populates a destination buffer by copying bytes from a source table using a structured loop.
- `zero_memory_table`: Iterates through a memory area and sets each longword to zero.
- `sub_226E`: undescribed.
- `sub_22D6`: undescribed.
- `sub_2308`: undescribed.
- `sub_2346`: undescribed.
- `sub_2384`: undescribed.
- `sub_239A`: undescribed.
- `sub_23D2`: undescribed.
- `sub_23F2`: undescribed.
- `sub_240C`: undescribed.
- `sub_2440`: undescribed.
- `sub_244C`: undescribed.
- `sub_2468`: undescribed.
- `sub_24AA`: undescribed.
- `sub_24FA`: undescribed.
- `sub_253A`: undescribed.
- `sub_2562`: undescribed.
- `sub_25A4`: undescribed.
- `sub_25C6`: undescribed.
- `sub_2616`: undescribed.
- `sub_2630`: undescribed.
- `increment_and_wrap_global`: Increments a global variable and resets it to one if it reaches specific threshold values.
- `clear_and_loop`: Sets a global variable to zero and enters a loop that continues until a register contains zero.

### Final Fight (1991)(U. S. Gold)[cr QTX](Disk 1 of 2).adf:final#unpacked

- `sub_0000`: undescribed.
- `sub_0022`: undescribed.
- `sub_003E`: undescribed.
- `sub_00D8`: undescribed.
- `sub_0154`: undescribed.
- `sub_0260`: undescribed.
- `sub_1EB2`: undescribed.
- `sub_1EBA`: undescribed.
- `sub_1EE4`: undescribed.
- `sub_20E0`: undescribed.
- `sub_219E`: undescribed.
- `sub_21C6`: undescribed.
