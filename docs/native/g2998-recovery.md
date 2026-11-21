# Game 2998 (bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02998). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6307840 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (2 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 287 of 287 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2635 |
| code | 686 |
| data | 5129852 |
| padding | 8843 |
| unused | 1165824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:Cls | 2 | 20 | 1 |
| Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:LhA | 2 | 267 | 8 |

## Cases handled

- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 1 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:C.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 1 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:1.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 1 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:GAME.lha: high-entropy data file (7.98 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 2 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 2 of 7).adf:2.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 3 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 3 of 7).adf:3.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 3 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 3 of 7).adf:6.lha: high-entropy data file (8.00 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 4 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 4 of 7).adf:4.lha: high-entropy data file (7.95 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 5 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 5 of 7).adf:5.lha: high-entropy data file (7.91 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 5 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 5 of 7).adf:COMMON2.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 6 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 6 of 7).adf:COMMON1.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 7 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 7 of 7).adf:8.lha: high-entropy data file (7.99 bits/byte), classified as packed data
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 1 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:LhA: PowerPacker 4.0 (xfd) stream found by ancient scan (36576 -> 53592 bytes) but only 0 of 26 sampled windows are in reference RAM: not used
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 2 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 2 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 5 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 5 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 6 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 6 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bump 'n' burn cd32 (1994)(grandslam)(aga)[h trader's dream][b](disk 1 of 7)/Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:LhA: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:Cls

- `open_dos_library`: Opens the DOS library through a function pointer stored in a global and calls a library entry on the result, returning the outcome of a second call or a fixed code of twenty when a flag is set.

### Bump 'N' Burn CD32 (1994)(Grandslam)(AGA)[h Trader's Dream][b](Disk 1 of 7).adf:LhA

- `process_hunk_headers`: Reads and processes AmigaDOS hunk headers from a memory block, allocating memory for each hunk and copying data into the allocated blocks.
- `decrement_and_branch`: Decrements a word value in register D1 until it reaches zero, using a branch condition to control the loop execution.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_and_shift`: Loads a value from memory, performs several shift operations on it, and uses the result to calculate an address offset for further processing.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_rotate`: Performs multiple bit shifts and rotates on data in registers D4 and D5, using a loop counter in D0 to control the number of iterations.
- `move_and_trap`: Moves a word value from a memory location to register D4, performs an OR operation on register D0 with a constant byte value, and then executes a trap instruction.
