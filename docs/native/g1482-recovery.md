# Game 1482 (defender (1994)(ratsoft)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01482). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 53 of 53 named (51 author symbols), 53 of 53 described (2 by rule, 51 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1498 of 1498 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5034 of 5034 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 270809 |
| code | 22164 |
| data | 56180 |
| padding | 1567 |
| unused | 550400 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Defender (1994)(Ratsoft)[a2].adf:Defender | 6 | 5016 | 52 |
| Defender (1994)(Ratsoft)[a2].adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Defender (1994)(Ratsoft)[a2].adf:Defender

- `initialize_game_loop`: The routine clears a global flag, calls another routine via a table, checks a field in the returned pointer and conditionally calls two more table routines to set the flag.
- `returnMsg`: Small routine of 1 instruction that merges 0 into d0.
- `start`: The routine opens the DOS library, conditionally decrypts a high-score table in memory by XORing words with two constants and writes the result back, then closes DOS. It then initializes global state and hardware registers, runs the main game loop calling audio, map, shots, bobs, scan and score routines, and on exit writes the table back with the same XOR and closes DOS.
- `titles_etc`: The routine clears per-frame lists and resets a set of global variables that track scrolling, wave timing, ship position and level counters.
- `p7_j4`: The routine iterates over a table of bytes, extracts nibbles and accumulates them into two global variables, calling a delay routine when the accumulator is non-zero, and updates audio channel pointers.
- `textype`: The routine reads a 16-bit index table from a global text table until a sentinel value and for each entry copies eight bytes from a font table to three bitplane buffers, using a running offset that is advanced by a fixed stride and reset between entries.
- `audio`: This routine manages a software sound queue for the Amiga audio hardware.
- `audio_j7`: The routine processes a sound request and updates hardware registers.
- `next_wave`: The routine updates a global timer and, when it expires, reads a byte from a level table using a position index.
- `drawbobs`: The routine walks a linked list of bob descriptors stored in a global table.
- `drawscan`: Waits for the blitter to become idle, then writes control words and pointers into a blitter control block to configure a scanline copy.
- `drawicons`: Iterates over a global enemy list and, for each active entry, computes a screen offset from map coordinates, selects a byte from a scanbit table and from four icon tables, shifts it and ORs it into six bitplane buffers and sets corresponding collision bytes.
- `drawmap`: The routine computes a scroll mask from a global map x value and stores it in a global scroll y variable.
- `your_shots`: Iterates over a fixed-size laser slot table and creates a new laser when a fire flag is set, storing ship position and facing.
- `you`: The routine updates a global ship position, velocity and thrust variables based on control flags, clamps values, computes a map offset and ship image index, writes thrust to a hardware audio register, stores the ship y coordinate, and writes three words through a pointer derived from an output pointer, using a table lookup for the image.
- `control1`: The routine reads a 16-bit value from an address pointed to by a0 plus an offset and uses its low bits to set a control flag, with additional bits influencing a direction flag.
- `control2`: The routine tests a number of bytes in a keypress table and updates a control byte and several global variables accordingly.
- `rez`: The routine updates a per-entry counter and flags in a table pointed to by a1 using d1 as an index, and conditionally invokes another routine to draw bits.
- `drawbits`: The routine iterates over a small set of rows and, for each row, computes a screen offset and writes masked bits from a source table into three bit-plane buffers, OR-ing the bits into two consecutive words per row.
- `stars`: Iterates over a list of star entries and updates their horizontal offset using a global map offset, then rewrites off-screen stars with new positions and writes a pattern byte into three bit-plane buffers using a computed index and a mask derived from a global counter.
- `addlife`: The routine waits for a hardware flag to clear, then if a lives counter is below a limit it programs a blitter operation by writing to several custom chip registers and sets up source and destination pointers from a global variable and a table indexed by a play-off value and the lives counter.
- `addsmart`: The routine waits for a hardware busy flag, checks a smart count limit, programs blitter registers for a copy operation and sets up source and destination pointers derived from a global variable and a smart index, then returns.
- `takelife`: The routine checks a lives counter and, if it is between one and five, waits for the blitter to be idle twice, programs the blitter control and mask registers and the blitter source and destination pointers, then waits for the blitter again and copies a byte from a table indexed by a playoff value and the lives counter to a destination pointer, and finally sets a blitter size register.
- `takesmart`: The routine checks a smart counter between one and five.
- `score_draw`: The routine checks a global variable against itself and calls another routine if they differ.
- `dwarem`: The routine builds two indices from a digit value in D0 and a mask in D1, then copies bytes from a glyph table referenced by A1 to two destination buffers referenced by A2 and A3 in a tiled pattern.
- `new_vblank`: The routine reads the Amiga interrupt enable and request registers, masks the result, and conditionally calls another routine if the copper interrupt bit is set, and calls another routine if the vertical blank interrupt bit is set.
- `vblint`: The routine writes a constant to a global variable and returns.
- `copint`: The routine runs when a flag is set, clears the flag and a global variable, updates animation counters and scroll state, calls several subroutines for drawing and game logic, handles a countdown timer and hyper mode timeout, and spawns a new enemy entry in a table when a level timer expires.
- `keyboardint`: The routine clears a keyboard interrupt flag, resets a global state variable, and updates a key state table using a value from a global variable.
- `key_j1`: The routine clears a global variable and a byte through a pointer, then spins until a specific bit in a global variable is set, clears that global variable and sets the same bit again, and returns.
- `enemy`: The routine walks a table of enemy entries and updates fields for each active entry, handling termination when a sentinel is found.
- `scores`: The routine adds three byte values using BCD arithmetic with carry propagation, stores the result in a global variable, copies the low nibble-masked value to another global, and updates a spare flag and an extra test variable when the high nibble changes.
- `set_bomb`: The routine checks a set of conditions involving a global variable, a counter and a map coordinate.
- `bombs`: The routine iterates over a fixed-size bomb list, decrements a per-entry timer and removes entries whose timer has expired or is invalid.
- `set_shot`: The routine conditionally writes to a shot list entry and associated fields using several global variables and inputs.
- `shots`: The routine iterates over a fixed-size shot list and updates each active entry’s position counters, removes entries whose counters expire or leave the play area, and decrements a global counter on removal.
- `change_screens`: The routine toggles a screen flag and updates several global plane base values and plane pointers, saving the current plane pointers into backup plane variables and then assigning the same global variable to three plane pointers, then calls another routine to clear the screen.
- `clear_screen`: The routine waits until a global variable clears a bit, then programs the blitter registers to copy a full-screen area from a source plane to the destination, using a 32-bit depth and a specific blitter control word, and finally starts the blitter operation.
- `initialise`: The routine saves registers, kills the system, changes the vertical blank handler, sets interrupts, sets the screen and creates a new copper list.
- `setscreen`: The routine writes constant values to a set of global variables that appear to hold palette and tile palette entries.
- `newcopper`: The routine stores the address of a copper list in a global variable and writes a value to another global variable.
- `change_vblank`: The routine stores a new address into a global variable and writes to INTENA to enable interrupts.
- `setints`: The routine stores a pointer to another routine in a global variable and writes values to the CIAA control and timer registers, then masks a byte in a global variable.
- `close_down`: The routine calls another routine to kill the system and then calls another routine to revive the system.
- `save_registers`: The routine opens the graphics library and stores the current view pointer, a copper list pointer, and several hardware register values into global variables.
- `kill_system`: The routine stores a constant value into a global variable and into another memory location, then returns.
- `revive_system`: The routine stores several saved values into global variables, writes a saved DMA control value and a saved interrupt enable value to hardware registers, then waits until a bit in a global variable is set.
- `closegraphics`: The routine calls a function pointer stored in a global variable and then returns.
- `opengraphics`: The routine repeatedly calls a function pointer stored in a global variable until the returned value is non-zero, storing the result in a global variable.
- `opendos`: The routine repeatedly calls a function pointer stored in a global variable until the returned value is non-zero.
- `closedos`: The routine calls a function pointer stored in a global variable and then returns.

### Defender (1994)(Ratsoft)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
