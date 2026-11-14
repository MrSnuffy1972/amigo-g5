# Game 2102 (tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02102). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802336 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 198 of 198 named (59 author symbols), 198 of 198 described (23 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 17 of 17 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 4096 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2007 of 2007 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 143750 |
| code | 6094 |
| data | 1523449 |
| padding | 15379 |
| unused | 113664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| EndCLI | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 11 | 2 |
| 8 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| 14 | 1 | 2 | 1 |
| mathtrans.library | 17 | 848 | 106 |
| mathieeedoubbas.library | 3 | 643 | 71 |
| version.library | 2 | 41 | 7 |
| Disk-Validator | 1 | 10 | 2 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf bytes 428032-430080 (code never loaded by a reference run) | 1 | 0 | 0 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf bytes 437248-439296 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file c/type (header 2176 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file c/cd (header 1908 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/silc.instr (header 502 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/Str.instr (header 128 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/Brss.instr (header 128 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/P2.instr (header 128 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/Vled.instr (header 502 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/bass.instr (header 502 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/hhat.instr (header 128 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/def.instr (header 502 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/P1.instr (header 128 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file instruments/MnLd.instr (header 502 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file samples/crwd (header 8094 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file samples/whis (header 520 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file scores/Coin (header 1618 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file fb (header 205184 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file KBlocks.1 (header 406205 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file s/readme (header 496 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file s/startup-sequence (header 34 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file fonts/TVFNums/7 (header 488 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file fonts/BNewClipboard/7 (header 1080 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file fonts/Score/9 (header 836 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file fonts/TVF/10 (header 1420 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file fonts/TVFII/8 (header 1244 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: file libs/diskfont.library (header 3968 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: file KBlocks.0 (header 352150 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: file KBlocks.1 (header 385002 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: file stat/TEND (header 2744 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: file stat/seas (header 14998 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: bytes 428032-430080 look like code no run loaded: recovered position-independently (load address unknown)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: bytes 437248-439296 look like code no run loaded: recovered position-independently (load address unknown)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: 2 logged loads cover 4096 bytes of the image
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 8-8 (bytes 4096-4608) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.53)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 12-12 (bytes 6144-6656) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.15)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 36-36 (bytes 18432-18944) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.79)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 42-42 (bytes 21504-22016) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.25)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 44-44 (bytes 22528-23040) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.43)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 46-46 (bytes 23552-24064) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.14)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 48-48 (bytes 24576-25088) outside the filesystem hold data, not code (opcode density 0.020, entropy 6.74)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 52-52 (bytes 26624-27136) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.44)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 62-62 (bytes 31744-32256) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.45)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 66-67 (bytes 33792-34816) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.78)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 69-69 (bytes 35328-35840) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.33)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 75-75 (bytes 38400-38912) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.88)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 109-109 (bytes 55808-56320) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.92)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 111-111 (bytes 56832-57344) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.97)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 119-120 (bytes 60928-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 6.36)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 132-133 (bytes 67584-68608) outside the filesystem hold data, not code (opcode density 0.033, entropy 5.74)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 143-571 (bytes 73216-292864) outside the filesystem hold data, not code (opcode density 0.038, entropy 6.46)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 659-659 (bytes 337408-337920) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.81)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 757-757 (bytes 387584-388096) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.80)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 759-759 (bytes 388608-389120) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.72)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 774-774 (bytes 396288-396800) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 814-815 (bytes 416768-417792) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.47)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 818-820 (bytes 418816-420352) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.18)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 825-826 (bytes 422400-423424) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 842-844 (bytes 431104-432640) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.57)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 870-872 (bytes 445440-446976) outside the filesystem hold data, not code (opcode density 0.030, entropy 5.89)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 876-879 (bytes 448512-450560) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.32)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 895-896 (bytes 458240-459264) hold ASCII text
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 898-900 (bytes 459776-461312) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.50)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 903-1696 (bytes 462336-868864) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.51)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1702-1702 (bytes 871424-871936) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.36)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1704-1704 (bytes 872448-872960) hold ASCII text
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1710-1710 (bytes 875520-876032) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.12)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1719-1719 (bytes 880128-880640) outside the filesystem hold data, not code (opcode density 0.023, entropy 5.68)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1727-1727 (bytes 884224-884736) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.33)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1732-1732 (bytes 886784-887296) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.48)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1739-1739 (bytes 890368-890880) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.80)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1750-1750 (bytes 896000-896512) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.17)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 15-710 (bytes 7680-364032) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.59)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 814-820 (bytes 416768-420352) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 825-846 (bytes 422400-433664) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.06)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 852-857 (bytes 436224-439296) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.71)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 895-1654 (bytes 458240-847360) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1662-1662 (bytes 850944-851456) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.46)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1664-1664 (bytes 851968-852480) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.80)
- tv sports football (1988)(cinemaware)[cr qtx][v saddam-saddam ii](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 2 never-loaded region(s), 4096 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 3584 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### EndCLI

- `record_unrecovered_entry_block`: The program entry of a 115-instruction block, for which the decompiler produced no pseudo-code, so its behaviour is not recovered.
- `forward_single_routine_call`: A two-instruction routine whose address is taken in code; it calls one unnamed routine and is called by one unnamed routine.

### Port-Handler

- `record_unrecovered_entry_block_2`: The program entry of a 306-instruction block, for which the decompiler produced no pseudo-code, so only the entry point is recovered and its behaviour is not recovered.

### Disk-Validator

- `forward_program_entry_call`: The program entry of a nine-instruction routine that calls one unnamed routine.
- `stub_address_taken_routine`: A two-instruction routine whose address is taken in code and which is called by one unnamed routine.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.

### 8

- `stub_program_entry_routine`: The program entry of a two-instruction routine with no calls.

### 8

- `initialize_font_data`: An entry point routine containing two instructions.

### 14

- `return_from_entry`: This routine consists of two instructions and performs no operations before returning.

### mathtrans.library

- `stub_library_entry_single_instruction`: A one-instruction library entry point in the mathtrans library, reached through a handler pointer stored in data.
- `stub_library_pointer_routine`: A two-instruction routine in the mathtrans library that is reached through a pointer stored in data.
- `release_memory_via_exec_free`: An 18-instruction routine in the mathtrans library that calls the exec memory-free function.
- `stub_library_small_routine`: A three-instruction routine in the mathtrans library reached through a pointer stored in data and called by one unnamed routine.
- `call_library_for_mathffp`: A 13-instruction routine in the mathtrans library that calls a library function by its offset from a library base, with the name mathffp.library referenced in its text.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_vector_then_forward`: A 14-instruction routine in the mathtrans library that calls a library function at offset minus 414 of a library base and then one unnamed routine.
- `forward_single_routine_call_2`: A four-instruction routine in the mathtrans library that calls one unnamed routine.
- `chain_two_library_routines`: Nineteen-instruction library routine reached through a pointer in the data hunk and from two other unnamed routines; it calls two further unnamed routines.
- `run_leaf_routine_single_caller`: Eighteen-instruction library routine with no outgoing calls, reached through a data pointer and from one other routine.
- `forward_to_library_stub_call`: Six-instruction library wrapper that calls the library stub at offset minus 528 and one other unnamed routine; one routine calls it.
- `process_data_via_pointer`: This library routine executes 24 instructions to manipulate data through a pointer.
- `pass_control_one_routine`: Two-instruction library routine whose address is taken directly and which passes control on to one other routine; one routine calls it.
- `dispatch_library_function_via_pointer`: This library routine executes 31 instructions to facilitate the execution of other library routines through pointers stored in data.
- `sequence_two_library_calls`: Thirteen-instruction library routine that calls two other library routines; one routine refers to it through a data pointer.
- `dispatch_two_library_routines`: Eighteen-instruction library routine that calls two other library routines and is called from two places.
- `invoke_three_library_routines`: Twelve-instruction library routine that calls three other library routines and is referenced from one place.
- `forward_to_single_routine`: Four-instruction library routine that calls one other library routine and is referenced through a data pointer.
- `relay_single_routine_call`: Five-instruction library routine that calls one other library routine and has one caller.
- `call_library_vector_function`: Thirteen-instruction library routine that calls the library stub at offset minus 528 and is called from five places.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `coordinate_three_library_routines`: Nineteen-instruction library routine with its address taken directly; it calls three other library routines and has one caller.
- `relay_to_one_routine`: Seven-instruction library routine that calls one other library routine and is referenced through a data pointer.
- `execute_chained_library_routines`: This routine consists of 26 instructions and performs a sequence of calls to two different library routines.
- `jump_to_single_routine`: Two-instruction library routine with its address taken directly; it calls one other library routine and has one caller.
- `pass_control_single_routine`: Two-instruction library routine with its address taken directly that makes a single call to one other library routine.
- `forward_call_to_routine`: Two-instruction library routine with its address taken directly that calls one other library routine; it has one caller.
- `relay_single_routine_call_2`: Five-instruction library routine that calls one other library routine and is referenced through a data pointer.
- `dispatch_two_library_routines_2`: Sixteen-instruction library routine with its address taken directly; it calls two other library routines and has five callers.
- `execute_multiple_library_calls`: This library routine consists of 34 instructions and executes several other library routines in succession.
- `relay_single_routine_call_3`: Thirteen-instruction library routine that calls one other library routine and has one caller.
- `execute_six_subroutines`: This library routine executes six other library routines in succession.
- `orchestrate_six_library_routines`: Seventeen-instruction library routine that calls six other library routines and is referenced from two places.
- `forward_call_to_routine_2`: Two-instruction library routine that calls one other library routine and is referenced through a data pointer.
- `run_block_calling_one_routine`: Eight-instruction library routine that calls one other library routine and is referenced from two places.
- `dispatch_two_library_routines_3`: Five-instruction library routine that calls two other library routines and has one caller.
- `dispatch_two_library_routines_9`: This routine executes two other library routines in sequence.
- `dispatch_two_library_routines_4`: Four-instruction library routine that calls two other library routines and is referenced through a data pointer.
- `coordinate_three_library_routines_2`: Sixteen-instruction library routine with its address taken directly; it calls three other library routines and has one caller.
- `coordinate_four_library_routines`: Eighteen-instruction library routine with its address taken directly; it calls four other library routines and has one caller.
- `orchestrate_six_library_routines_2`: Eighteen-instruction library routine with its address taken directly; it calls six other library routines and has three callers.
- `forward_call_to_routine_3`: Three-instruction library routine that calls one other library routine and is referenced through a data pointer.
- `transfer_pointer_value`: This two-instruction library routine moves a value from a register into a memory location specified by a pointer.
- `relay_single_routine_call_4`: Five-instruction library routine that calls one other library routine and has two callers.
- `run_short_leaf_routine`: Five-instruction library routine with no outgoing calls, referenced through a data pointer from one place.
- `run_short_leaf_routine_2`: Four-instruction library routine with no outgoing calls, referenced through a data pointer from one place.
- `dispatch_indirect_library_calls`: This routine executes a sequence of 27 instructions to facilitate various indirect calls to other library routines.
- `process_indirect_data_pointer`: This routine consists of 36 instructions and is invoked via a pointer stored in data.
- `run_block_calling_one_routine_2`: Twelve-instruction library routine that calls one other library routine and has two callers.
- `store_register_to_pointer`: This five-instruction library routine stores a value from a register into a memory location specified by a pointer.
- `coordinate_three_library_routines_3`: Seven-instruction library routine with its address taken directly; it calls three other library routines and has four callers.
- `transfer_two_words`: This two-instruction library routine moves two words of data from one memory location to another.
- `execute_multiple_library_calls_2`: This library routine consists of 23 instructions and invokes four different library routines.
- `process_data_via_library_calls`: This library routine consists of 39 instructions and invokes three different library routines.
- `call_library_vector_and_helpers`: Six-instruction library routine with its address taken directly; it calls the library stub at offset minus 528 and two other library routines, and has one caller.
- `dispatch_two_library_routines_5`: Six-instruction library routine with its address taken directly; it calls two other library routines and has one caller.
- `coordinate_three_library_routines_4`: Five-instruction library routine that calls three other library routines and has two callers.
- `run_leaf_routine_addressed`: Eight-instruction library routine with no outgoing calls, address taken directly, and one caller.
- `dispatch_library_subroutines`: This ten-instruction library routine calls three other library routines.
- `coordinate_three_library_routines_5`: Ten-instruction library routine that calls three other library routines and has one caller.
- `relay_single_routine_call_5`: Six-instruction library routine that calls one other library routine and has one caller.
- `dispatch_two_library_routines_6`: Six-instruction library routine that calls two other library routines and is referenced through a data pointer.
- `execute_minimal_routine`: This library routine consists of five instructions and calls another unnamed routine.
- `call_unnamed_routine`: This library routine consists of two instructions and calls one unnamed routine.
- `run_leaf_routine_many_callers`: Seven-instruction library routine with no outgoing calls, referenced through a data pointer and called from fifteen other routines.
- `run_leaf_routine_pointer_ref`: Eight-instruction library routine with no outgoing calls, referenced through a data pointer from one other routine.
- `invoke_two_routines_from_gap`: Two-instruction block in an unreached gap that calls two other library routines; nothing in the listed facts reaches it.
- `load_pointer_and_branch`: Loads a memory address from offset 310 into register a0 and then branches unconditionally to offset 0.
- `load_address_and_jump`: Loads a memory address from offset 318 into register a0 and then jumps unconditionally to offset 20.
- `fetch_and_branch`: Loads a memory address from offset 326 into register a0 and then branches unconditionally to offset 0.
- `read_address_and_continue`: Loads a memory address from offset 334 into register a0 and then continues execution at the next instruction.
- `invoke_and_return`: Loads a memory address from offset 342 into register a0 and then invokes another routine before returning.
- `forward_call_and_branch`: Loads a memory address from offset 350 into register a0 and then branches unconditionally to offset 20.
- `call_forwarded_routine`: Loads a memory address from offset 358 into register a0 and then branches unconditionally to offset 20.
- `execute_and_continue`: Loads a memory address from offset 366 into register a0 and then continues execution at the next instruction.
- `call_and_branch_to_offset`: Loads a memory address from offset 374 into register a0 and then branches unconditionally to offset 20.
- `dispatch_two_library_routines_7`: Two-instruction library routine referenced only from a pointer in the data hunk, which calls two other library routines.
- `dispatch_two_library_routines_8`: Two-instruction library routine referenced from a data pointer, which calls two other library routines.
- `call_leaf_routine`: A library routine consisting of two instructions that calls another library routine.
- `dispatch_library_routines`: This two-instruction library routine acts as a dispatcher by calling two separate library routines.
- `dispatch_library_calls`: This two-instruction routine executes a leaf routine and then calls another library routine.
- `dispatch_library_routines_2`: This two-instruction library routine executes a leaf routine via a pointer and then passes control to another library routine.
- `relay_single_routine_call_6`: Eleven-instruction library routine that calls one other library routine and is referenced only from a pointer in the data hunk.
- `execute_leaf_routine`: This two-instruction library routine acts as a wrapper that invokes a leaf routine via a pointer retrieved from a data structure.
- `invoke_leaf_routine_via_pointer`: This two-instruction routine retrieves a function pointer from a data structure and executes it.
- `call_leaf_and_orchestrate`: This two-instruction library routine calls a leaf routine via a data pointer and an orchestration routine that calls six other library routines.
- `dispatch_indirect_library_calls_2`: This library routine executes a sequence of instructions to facilitate various indirect calls to other library routines.
- `call_leaf_and_coordinate`: This two-instruction library routine calls a leaf routine via a data pointer and a coordination routine that calls three other library routines.
- `call_leaf_and_vector`: This two-instruction library routine calls a leaf routine via a data pointer and a routine that calls a library stub and two other library routines.
- `call_leaf_and_dispatch`: This two-instruction library routine calls a leaf routine via a data pointer and a dispatch routine that calls two other library routines.
- `call_leaf_and_addressed`: This two-instruction library routine calls a leaf routine via a data pointer and another leaf routine via a direct address.
- `invoke_leaf_routine_2`: This library routine consists of two instructions and calls a leaf library routine.
- `forward_call_to_routine_4`: Two-instruction library routine referenced from a data hunk entry in the second hunk; it calls one other library routine.
- `call_leaf_by_pointer`: This two-instruction library routine calls a leaf routine that is referenced through a data pointer.
- `call_leaf_many_callers`: This two-instruction library routine calls a leaf routine that is reached from a data pointer and has eleven callers.
- `call_leaf_routine_2`: This routine executes a single leaf routine via a pointer retrieved from data.
- `call_leaf_pointer_ref`: This two-instruction library routine calls a leaf routine that is referenced from a data pointer and has four callers.
- `invoke_leaf_routine`: This routine acts as a wrapper that calls a leaf routine via a pointer retrieved from a data structure.
- `forward_call_to_routine_5`: Two-instruction library routine with its address taken directly; it calls one other library routine and is called from five places.
- `forward_call_to_routine_6`: Two-instruction library routine with its address taken directly; it calls one other library routine and is called from six places.
- `call_leaf_routine_3`: This library routine consists of two instructions and calls another library routine.
- `run_leaf_routine_many_callers_2`: Ten-instruction library routine with no outgoing calls, reached from a data pointer and from eleven other routines.
- `run_leaf_routine_pointer_ref_2`: Seven-instruction library routine with no outgoing calls, referenced from a data pointer and called from four other routines.
- `run_leaf_routine_many_callers_3`: Twenty-instruction library routine with no outgoing calls, reached from a data pointer and from four other routines.

### mathieeedoubbas.library

- `residentMI`: One-instruction entry named residentMI in the IEEE double-precision library; it is the program entry and is referenced from a data pointer.
- `libInit2`: Two-instruction library entry named libInit2 that is referenced from a data pointer; the listed facts show no calls.
- `MIExpunge`: Eighteen-instruction expunge routine named MIExpunge that calls the system memory-free function and tail-branches to Exp_InUse; MIClose calls it.
- `Exp_InUse`: Three-instruction routine named Exp_InUse, reached by a branch from MIExpunge; the listed facts show no calls of its own.
- `MIOpen`: Five-instruction routine named MIOpen that ends by tail-branching to openFail.
- `openFail`: Two-instruction routine named openFail, reached by a branch from MIOpen; the listed facts show no calls of its own.
- `MIClose`: Seven-instruction routine named MIClose that calls MIExpunge.
- `DPFix`: This library routine consists of 33 instructions and is invoked by multiple callers including a wrapper and two branching routines.
- `dfi4`: Four-instruction routine named dfi4 that tail-branches to DPFix and is reached by a branch from it.
- `dfi6`: Two-instruction routine named dfi6 that tail-branches to DPFix and is reached by a branch from it.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: This library routine consists of 29 instructions and is invoked by another routine within the same library.
- `DPAbs`: Eight-instruction routine named DPAbs, referenced by the MIIEEEDPAbs entry; no outgoing calls are listed.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: Two-instruction author routine named dtst_1 with no listed calls or callers.
- `dtst_2`: Two-instruction author routine named dtst_2 with no listed calls or callers.
- `cc_1`: Ten-instruction routine called from the MIIEEEDPCmp and MIIEEEDPTst entries; it makes no calls of its own in the listed facts.
- `cc_2`: Eight-instruction routine called from the MIIEEEDPAbs and MIIEEEDPNeg entries; it makes no calls of its own in the listed facts.
- `cc_3`: Eleven-instruction routine tail-branched to from the MIIEEEDPAdd, MIIEEEDPDiv, MIIEEEDPMul and MIIEEEDPSub entries.
- `MIIEEEDPFix`: Six-instruction entry called from MIIEEEDPFloor that calls DPFix.
- `MIIEEEDPFlt`: Seven-instruction entry called from MIIEEEDPFloor that calls DPFlt.
- `MIIEEEDPCmp`: Two-instruction entry called from MIIEEEDPFloor that tail-branches to the first comparison-flags routine.
- `MIIEEEDPTst`: Two-instruction entry called from test_d6_d7 that tail-branches to the first comparison-flags routine.
- `MIIEEEDPAbs`: Two-instruction entry that calls DPAbs and tail-branches to the second comparison-flags routine.
- `MIIEEEDPNeg`: Two-instruction entry called from MIIEEEDPCeil that tail-branches to the second comparison-flags routine.
- `MIIEEEDPAdd`: Two-instruction entry called from test_d6_d7 that tail-branches to the third comparison-flags routine.
- `MIIEEEDPSub`: Two-instruction entry called from lt_zero that tail-branches to the third comparison-flags routine.
- `MIIEEEDPMul`: Two-instruction entry that tail-branches to the third comparison-flags routine.
- `MIIEEEDPDiv`: This library routine performs a division operation using a tail branch to another routine.
- `MIIEEEDPCeil`: Three-instruction entry that calls MIIEEEDPFloor and MIIEEEDPNeg.
- `MIIEEEDPFloor`: Sixteen-instruction routine called from MIIEEEDPCeil; it calls the compare, fix and float entries and tail-branches to lt_zero and test_d6_d7.
- `test_d6_d7`: Nine-instruction routine called from MIIEEEDPFloor; it calls the add and test entries and tail-branches to lt_zero.
- `lt_zero`: Six-instruction routine reached from MIIEEEDPFloor and test_d6_d7; it calls the subtract entry.
- `fptrap`: Seven-instruction floating-point trap entry referenced from the data hunk; it tail-branches to fpoptab.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `forward_to_bad_trap`: Single instruction in an unreached gap that tail-branches to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap_2`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap_3`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap_4`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap_5`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap_6`: Jump stub: passes control straight on to badtrap.
- `jump_to_dfadd`: Jump stub: passes control straight on to dfadd.
- `jump_to_dfsub`: Jump stub: passes control straight on to dfsub.
- `jump_to_dfmul`: Jump stub: passes control straight on to dfmul.
- `jump_to_dfdiv`: Jump stub: passes control straight on to dfdiv.
- `jump_to_dfneg`: Jump stub: passes control straight on to dfneg.
- `dfsub`: Six-instruction routine named dfsub that tail-branches to dfadd.
- `dfadd`: Fifteen-instruction routine called from dfsub that tail-branches to dfa11 and dfa7.
- `dfa11`: This routine consists of 42 instructions and performs a sequence of three tail branches to other routines.
- `dfa14`: Seven-instruction routine called from dfa11 that tail-branches to dfa7.
- `dfa13`: Fourteen-instruction routine called from dfa11 that tail-branches to dfa5 and dfa9.
- `dfa5`: Twenty-instruction routine called from dfa13 that tail-branches to dfa7 and dfa8.
- `dfa8`: Six-instruction routine called from dfa5 that tail-branches to dfa7.
- `dfa7`: Ten-instruction routine called from seven double-precision routines; it tail-branches to dfauo and dftst.
- `dfa9`: Three-instruction routine called from dfa13 and dfauo that tail-branches to dfa7.
- `dfauo`: Nine-instruction routine called from dfa7 that tail-branches to dfa7 and dfa9.
- `dfmul`: 134-instruction routine, the largest in the double-precision set listed, called from dfm4 and dfmuo; it tail-branches to dfm4, dfmuo and dftst.
- `dfm4`: Three-instruction routine called from dfmul and dfmuo that tail-branches to dfmul.
- `dfmuo`: Eight-instruction routine called from dfmul that tail-branches to dfm4 and dfmul.
- `dfdiv`: This library routine executes a double-precision floating-point division operation.
- `dfd10`: Thirteen-instruction routine called from dfdiv that tail-branches to dfd4 and dfdiv.
- `dfd4`: This routine consists of 22 instructions and performs tail branches to two other routines.
- `dfsetz`: Three-instruction routine called from dfdiv and dfduo that tail-branches to dfd4.
- `dfdbyz`: Two-instruction routine called from dfdiv that tail-branches to dfduo.
- `dfduo`: Eight-instruction routine called from dfd4 and dfdbyz that tail-branches to dfd4 and dfsetz.
- `dfneg`: Six-instruction routine named dfneg that tail-branches to dftst.
- `dfcmp`: This library routine performs a comparison by executing a tail branch to another routine.
- `dftst`: Eight-instruction routine called from dfa7, dfcmp, dfd4, dfmul and dfneg; the listed facts show no calls of its own.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### version.library

- `enter_version_library_entry`: Two-instruction program entry of the version library; the listed facts show no calls.
- `run_version_library_block_a`: Four-instruction version-library routine with no listed calls, referenced from the data hunk.
- `execute_library_handler`: A library routine consisting of four instructions that is invoked via a pointer in data.
- `relay_version_library_call`: Seven-instruction version-library routine that calls one other routine and is referenced from the data hunk.
- `forward_version_library_call`: Eight-instruction version-library routine called from one other routine that makes one call of its own.
- `call_library_offsets_pair`: Fourteen-instruction version-library routine that calls two Amiga OS library functions at offsets minus 210 and minus 252.
- `run_version_library_block_c`: Two-instruction version-library routine with no listed calls.

### Disk-Validator

- `start_disk_validator_entry`: Nine-instruction program entry of the disk validator that calls one other routine.
- `run_validator_marker_stub`: Single-instruction routine in the disk validator, address taken directly and referenced from one routine.

### TV Sports Football (1988)(Cinemaware)[cr QTX][v Saddam-Saddam II](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_pointer_then_spin`: Eleven-instruction segment that calls the same pointer-held routine twice and then enters an endless empty loop; it also makes an Amiga OS library call at offset minus 222.
