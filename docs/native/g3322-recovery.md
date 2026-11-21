# Game 3322 (datastorm (1989)(visionary design)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03322). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 46 of 46 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 105 of 105 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 496 |
| data | 895010 |
| padding | 494 |
| unused | 5120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Datastorm (1989)(Visionary Design).adf:bootblock | 1 | 105 | 5 |

## Cases handled

- datastorm (1989)(visionary design)/Datastorm (1989)(Visionary Design).adf: custom-format (non-DOS) disk, 1748 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4ED0) A7=an address (A7)=an address, slid to exec.library -an address): 556 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- datastorm (1989)(visionary design)/Datastorm (1989)(Visionary Design).adf: blocks 1598-1713 (bytes 818176-877568) hold 5 packed streams (BK: ByteKiller, C: Compact) found by scanning, 85% of the bytes
- datastorm (1989)(visionary design)/Datastorm (1989)(Visionary Design).adf: blocks 1716-1730 (bytes 878592-886272) outside the filesystem hold data, not code (opcode density 0.012, entropy 7.19)
- datastorm (1989)(visionary design)/Datastorm (1989)(Visionary Design).adf: blocks 1738-1759 (bytes 889856-901120) hold ASCII text
- 1 packed track region(s), 816640 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Datastorm (1989)(Visionary Design).adf:bootblock

- `frag_lea_movea_move_dbra_jmp`: Short code fragment that loads the address of a location in the code into a0, then copies an address to a2, then copies 1023 to d0, then copies the memory at a0 (advancing a0) to the memory at a2 (advancing a2), then loops back while counting down d0, then jumps to another point in the code.
- `boot_initialization_sequence`: Sets up I/O request parameters, enables interrupts, initializes stack pointers, executes a trap instruction, performs checksum calculations, and calls two other routines to set up boot data before jumping to a final address.
- `initialize_boot_data`: Writes a specific signature value to a memory location and performs conditional checks on global variable contents before updating the global variable with a saved value.
- `write_signature_loop`: Writes a signature value to memory locations in a loop, checking for specific conditions including address boundaries and global variable values before exiting.
- `setup_io_request`: Configures an I/O request structure with specific parameters and performs two asynchronous I/O operations using the Amiga OS exec library, followed by saving and restoring register states.
