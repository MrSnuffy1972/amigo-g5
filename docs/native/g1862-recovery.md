# Game 1862 (final fight (1991)(u. s. gold)[cr csl][t +6 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01862). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1808432 top-level bytes in 138 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 66 of 66 named (0 author symbols), 66 of 66 described (5 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1541 of 1541 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 66820 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1964 of 1964 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 69710 |
| code | 7040 |
| data | 1257665 |
| padding | 17825 |
| unused | 456192 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:final | 2 | 229 | 6 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:final.exe | 2 | 233 | 8 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:Crystal | 2 | 85 | 3 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:bootblock | 1 | 54 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 304 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 947 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 100 | 6 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level6.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level3.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level1.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level5.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level2.amap#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bred2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (14988 -> 28678 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:guy.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (23204 -> 46450 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:weapons.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (4380 -> 6646 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bred.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (15676 -> 28422 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:heads.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (7696 -> 8626 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:trainstrap.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (2564 -> 2672 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bigboss.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (18680 -> 34928 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:axl.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17196 -> 33600 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:gymnast.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (18364 -> 40350 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:axl2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17036 -> 33600 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:chars.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (7732 -> 9390 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:cody.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (37776 -> 68972 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:haggarpics.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:j.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (12192 -> 22100 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:rasta.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (24080 -> 39182 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:giant2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17812 -> 34890 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:chandelier.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (4688 -> 6908 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bred3.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (16008 -> 28550 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:guys.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (13244 -> 17766 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:maptrain.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (13760 -> 17062 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:knifeguy.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17744 -> 33564 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:dontusedrugs.iff: high-entropy data file (7.68 bits/byte), classified as packed data
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:j2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (12440 -> 22236 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:giant.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (17324 -> 37092 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:fflogo.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (9972 -> 13506 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:cop.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (26668 -> 41304 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:townmap.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (10168 -> 13694 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bill.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (21856 -> 38870 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:haggar.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (23696 -> 49310 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:knifeguy2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (16616 -> 33564 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:swordsman.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (19276 -> 34380 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:guys_ring.iff: high-entropy data file (7.90 bits/byte), classified as packed data
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:mod.mit: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (132768 -> 180488 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bill2.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (20468 -> 38870 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:burntguy.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (2744 -> 4876 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level6.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (15000 -> 22260 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level2.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (28592 -> 61440 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:weapons.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (4380 -> 6646 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level8.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6660 -> 16000 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level1.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (31740 -> 61184 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level4.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6472 -> 7684 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:cody.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (37776 -> 68972 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level3.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6768 -> 8708 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:tv.iff: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (24152 -> 31964 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level7.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (16804 -> 32128 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:glass.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (6644 -> 12550 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level1.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (5328 -> 6660 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level6.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (30996 -> 57472 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level5.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (33660 -> 54912 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level5.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (12040 -> 18436 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:flames.sms: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (3880 -> 4870 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:haggar.mps: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (23696 -> 49310 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level4.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (27376 -> 46848 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level2.amap: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (8188 -> 10756 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level3.chites: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (36556 -> 58368 bytes)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:l8mask.bin: Vic2: Huffman compressor with RLE-packed, unpacked with ancient (2240 -> 6400 bytes)
- runtime segment an address: 276 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 700 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 63 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level6.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level3.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level1.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level5.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:level2.amap#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 2 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf: blocks 1738-1759 (bytes 889856-901120) hold ASCII text
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:final: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- final fight (1991)(u. s. gold)[cr csl][t +6 legend](disk 1 of 2)/Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:final.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 5 never-loaded region(s), 66820 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 11264 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_and_process_graphics_data` (runtime segment an address): Copies a block of data to a new memory location and performs bitwise shifts and logical operations on a series of values. Kind: custom data decompression/decryption and bit-manipulation loader. Pass path: The routine performs a complex, non-standard bitwise transformation (shifting and masking) on data loaded into memory (likely graphics or sprites) using a key derived from a data stream. This is characteristic of a loader that decrypts asse.
- `modify_memory_and_flags` (runtime segment an address (play-rs-x25k)): Sets bits in a global variable and writes specific values to memory locations using an offset. Kind: checksum/integrity trap. Pass path: the routine continues execution normally after the memory writes; failure likely triggers a crash or jump to a sabotage loop via the modified function pointer.

## Functions


### Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:final

- `run_memory_allocation_entry`: Program entry of 128 instructions that calls the exec library to allocate and free memory and makes three further routine calls.
- `spin_delay_then_library_call`: Spins a 16-bit counter down through a loop as a timing delay, and makes one library call at offset minus 210.
- `read_bits_with_refill`: Reads bits from a bit buffer, refilling it from a longword before the current pointer when it empties, and calls a flag check helper and then two further routines.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `read_bits_from_buffer`: Reads a run of bits from a bit buffer, refilling the buffer from the longword before the current pointer when it empties, and accumulating the extracted bits into an output value.

### Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:final.exe

- `process_hunk_file_entry`: Program entry of 128 instructions that calls the exec library to allocate and free memory and compares values with AmigaDOS hunk type codes, which suggests executable loading or relocation work.
- `decrement_counter_loop`: Decrements a 16-bit counter in a loop until it reaches zero and then returns without performing any other action.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `read_bits_with_refill_2`: Reads bits from a bit buffer, refilling it from a longword before the current pointer when it empties, and calls a flag check helper and then three further routines.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bits_from_stream`: Consumes a specified number of bits from a packed bit stream, advancing through the source words and accumulating the extracted bits into an accumulator.
- `read_bits_with_display_toggle`: Reads a run of bits from a bit buffer, refilling the buffer from the longword before the current pointer when it empties, and accumulating the extracted bits into an output value while toggling a bit in the display control register.
- `stub_bad_instruction_data`: Begins with a trap call and a few memory writes, then reaches bad instruction data, so no complete body is recoverable.

### Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:Crystal

- `decode_bit_packed_stream`: Program entry of 69 instructions that writes the colour register and calls a depacker bit-reader refill routine and a multi-bit bit-reader routine.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Final Fight (1991)(U. S. Gold)[cr CSL][t +6 Legend](Disk 2 of 2).adf:bootblock

- `find_resident_and_start_copper`: Boot block that finds a resident module through the exec library, makes several library calls at fixed offsets, clears a block of longwords in a loop, then loads the copper list pointer and starts the copper.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_process_graphics_data`: Copies a block of data to a new memory location and performs bitwise shifts and logical operations on a series of values.
- `clear_flag_after_countdown`: If bit eight of a global variable is set, it performs a countdown loop and clears bit six of that variable.
- `run_four_handlers_when_flagged`: When bit five of a state byte is set, calls four helper routines and resets the state byte to that bit value.
- `advance_state_counter_step`: Updates a global counter by decrementing it if it is non-zero, or by incrementing it by a specific value if it is zero.
- `render_text_characters_to_buffer`: Runs only when the low nibble of a counter equals fourteen.
- `step_state_and_set_offsets`: Steps a global state byte down by sixteen twice.
- `copy_counted_word_blocks`: Decrements a counter, and when it reaches zero resets it to a fixed count and restores the saved pointer.
- `clear_memory_block`: Fills a block of memory with zeros and writes several constant values to a series of global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_values`: Sets multiple global variables to a specific constant value and calls several other routines before returning a concatenated result.
- `set_global_constant`: Calls several other routines and then stores a constant value into a global variable.
- `write_constant_values`: Writes several constant values to a global variable and then stores a value at an offset from a pointer.
- `clear_and_initialize_data`: Clears a large block of memory by writing zeros and then populates several memory locations with specific word values.
- `modify_memory_and_flags`: Sets bits in a global variable and writes specific values to memory locations using an offset.
- `search_and_update_tables`: Searches through a data block for a specific byte value, then uses the result to populate a table with calculated offsets and clears several other memory locations.
- `reset_audio_volumes`: Sets four memory locations to zero and sets a global variable to fifteen.
- `update_conditional_byte_2`: If a byte at a pointer is non-zero, it sets a destination byte to a constant and clears the source byte; otherwise, it increments the destination.
- `update_conditional_byte`: If a byte at a pointer is non-zero, it sets a destination byte to a constant and clears the source byte; otherwise, it increments the destination byte by two if it is below a certain threshold.
- `clamp_value_within_range`: Extracts a byte from a memory location, calculates a signed difference, and clamps the resulting value within a specific range before storing it in a global variable.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `extract_bits_to_global`: Extracts a byte and a specific bit from a passed longword and stores them into a global variable.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `update_global_bit`: Extracts a specific bit from a register and stores the result into a global variable.
- `execute_conditional_logic`: Checks a global variable against a limit and conditionally calls another routine and copies data blocks.
- `call_if_less_than_limit`: Calls another routine if the value in a register is less than a specific constant.
- `copy_data_blocks`: Performs a nested loop to copy multiple blocks of data from a source table to a destination buffer using calculated offsets.
- `update_table_values`: Iterates through a table of pointers to modify bytes based on specific range checks and conditional logic.
- `check_value_in_range`: Compares a value against a range defined by several parameters and returns zero if the value is within the range, or a maximum value otherwise.
- `wait_for_input`: Polls a global variable in a loop until a specific bit is set, effectively waiting for a hardware status change.
- `write_multiple_constants`: Executes a loop that writes a series of constant values to various offsets from a pointer.
- `wait_for_bit_set`: Polls a byte at an offset from a pointer until a specific bit is set.
- `convert_to_ascii`: Converts a numeric value to its ASCII character equivalent and stores it in a memory location.
- `process_byte_decrement_loop`: Decrements a global byte value through a loop and modifies various hardware registers and memory locations based on the result.
- `filter_and_copy_shorts`: Iterates through a list of short integers and copies only the positive values to a new destination table.
- `initialize_object_table`: Calculates offsets based on a global pointer and populates a structured table with specific integer values and constants.
- `fill_pattern_table`: Populates a large memory area with a repeating pattern of specific short integer values.
- `modify_short_array`: Iterates through an array of short integers and applies a mathematical transformation to each element.
- `update_audio_parameter_and_table`: Modifies audio-related variables or hardware registers based on specific bitmask values and increments a counter.
- `update_sprite_attribute_data_2`: Updates sprite position and attribute values by calculating offsets into a table based on a nibble extracted from a source word.
- `update_sprite_attribute_data`: Calculates an offset into a table based on a nibble extracted from a source word to update sprite position and attribute values.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_memory_relocation_2`: Performs memory relocation, pointer patching, and data copying.
- `perform_memory_relocation`: This routine is identical to R2 and performs memory relocation, pointer patching, and data copying.
- `relocate_and_patch_memory`: Updates a list of pointers by adding an offset, writes a constant to a memory block, and copies a small amount of data.
- `relocate_and_patch_pointers`: Writes a constant value to a memory block, updates a list of pointers by adding an offset, and copies a small amount of data from the code segment to a destination.
- `update_pointer_via_offset_2`: The routine performs a calculation using a program counter relative load and updates a value.
- `update_pointer_via_offset`: The routine performs a calculation using a program counter relative load and updates a value.
