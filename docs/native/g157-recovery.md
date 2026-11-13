# Game 157 (AlphaWaves v1.0 0626): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00157). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1099289 top-level bytes in 49 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 96 raw-word lines |
| Functions named and described | pass, 636 of 636 named (397 author symbols), 636 of 636 described (69 by rule, 558 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2963 of 2974 (99.63%) |
| Recovered without load address (counted in proof 1, flagged) | 13362 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12365 of 12618 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 328299 |
| code | 50670 |
| data | 720288 |
| padding | 32 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 120 | 12 |
| GAME | 1 | 8963 | 407 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1063 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3358 | 155 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| ALLEMAND.QBN (raw code file, load address unknown) | 1 | 0 | 0 |
| ANGLAIS_.QBN (raw code file, load address unknown) | 1 | 0 | 0 |
| ESPAGNOL.QBN (raw code file, load address unknown) | 1 | 0 | 0 |
| FRANCAIS.QBN (raw code file, load address unknown) | 1 | 0 | 0 |
| ITALIEN_.QBN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- AlphaWaves_v1.0_0626/AlphaWaves_v1.0_0626.hdf: hard-disk image (AmigaDOS OFS, 1248 blocks): its filesystem parses and all 42 files in it are byte-identical to the extracted tree AlphaWaves_v1.0_0626_hdf, which is recovered file by file; no block outside the filesystem holds data
- 53 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 98 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 5 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 4 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- AlphaWaves_v1.0_0626/AlphaWaves_v1.0_0626_hdf/AlphaWaves/data/ALLEMAND.QBN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlphaWaves_v1.0_0626/AlphaWaves_v1.0_0626_hdf/AlphaWaves/data/ANGLAIS_.QBN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlphaWaves_v1.0_0626/AlphaWaves_v1.0_0626_hdf/AlphaWaves/data/ESPAGNOL.QBN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlphaWaves_v1.0_0626/AlphaWaves_v1.0_0626_hdf/AlphaWaves/data/FRANCAIS.QBN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlphaWaves_v1.0_0626/AlphaWaves_v1.0_0626_hdf/AlphaWaves/data/ITALIEN_.QBN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 2 call target
- 5 never-loaded region(s), 13362 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `VBLINT` (GAME): This vertical blank interrupt routine updates audio registers, manages a sound-effect or music-note counter, and updates a palette or a subset of palette registers. Kind: manual/code-wheel/colour-card lookup. Pass path: the code continues to the palette and sound/music processing logic (u1__OTEPALETTE, u1__GOPALETTE, etc.) if the check passes or if the key/card is valid.
- `update_global_counters` (runtime segment an address): Checks a bit in a custom register and, if set, increments two global longwords, copies a byte from a hardware register to a global longword, and enters a loop that decrements a global word until it reaches zero before checking a bit in a global longword. Kind: checksum or integrity trap. Pass path: The routine likely performs a checksum/CRC validation of the runtime segment or specific data blocks; the 'pass' path is the execution flow that continues normally after the call, whereas the 'fail' path (not visible due to decompile error).
- `process_complex_data` (runtime segment an address): Performs a sequence of operations including reading joystick or mouse input, updating audio state, comparing and updating a table, and processing a global data table through various subroutines. Kind: disk hardware/integrity check. Pass path: The branch not taken during the 'decompile failed' flow (likely the path following a successful return from the routine, assuming the error is caused by a complex control flow graph used to obfuscate the check)..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_subsystem_initialization`: Sets a global constant and executes a sequence of hardware-related initialization routines while updating a global pointer or state.
- `call_indirect_subroutine`: Performs an indirect call to a routine located at an offset from a global pointer.
- `execute_indirect_function`: Loads a function pointer from a global table and executes it.
- `execute_indirect_function_2`: Loads a function pointer from a table and executes it.
- `execute_indirect_function_3`: Loads a function pointer from a table and executes it.
- `execute_indirect_calls`: Loads a pointer from a global variable into a1, calls a routine at an offset of 16634 from that pointer, then loads a second pointer from the same global variable, calculates a value using a bitwise NOT on 63, calls another routine at an offset of 12 from a pointer in a2, and returns.
- `write_word_to_memory`: Writes the word value 65153 to a specific memory location and returns.
- `set_global_constant`: sets a global variable to a constant value.
- `update_global_state_2`: Checks a global variable and a memory location, then potentially calls another routine and increments a value at a memory offset.
- `call_empty_routine`: Executes another routine that performs no observable work and then returns.
- `empty_routine_4`: The routine performs no observable work and ends without modifying registers or memory.

### GAME

- `START`: Copies a copperlist or custom copperlist, initializes chip-related pointers, and loads necessary files and graphics.
- `US0`: Initializes global pointers for music, text, and palette data, loads certain resources from disk, and sets up the initial menu state and screen buffers.
- `US1`: Loads music and credit text files from disk, configures audio DMA pointers, and initializes the main menu or single-player mode.
- `MUSICORIGINAL`: Checks if a music identifier has changed, then stops the current music, copies new music, and starts it again.
- `CHOIXMUSIC`: Checks a flag and a table, then stops and starts music with a new identifier if a certain condition is met.
- `COPYMUSIC`: Copies a fixed block of memory from a source pointer to a destination pointer using a loop that processes ten-longword chunks.
- `TESTPROTECTION`: Generates random values to construct a string, then compares a string or a value from a table against a calculated protection key, displaying text or error characters if they do not match.
- `KR_DOMAP`: Resets a portion of a global palette or color table to a black or neutral value and then calls routines to redraw or display a map.
- `LITOPTIONSDUMENU`: Updates several global pointers and copies a sequence of instructions from a source to a destination.
- `FINDTEXT`: Scans a string or buffer for a specific sequence of characters involving certain delimiters and a character match, skipping or advancing pointers based on finding specific characters.
- `AFFICHETEXTERSC`: Iterates through a string looking for a specific terminator, calling a centering routine for each character, and then adjusts a vertical offset or position or a global variable via a loop.
- `AFFICHELIGNECENTREE`: Calculates the length of a string until a newline character, then calls a drawing routine a specific number of times before and after the text to attempt a centered alignment.
- `LOADGAME`: Attempts to open a file named via a global string and read a fixed-size block of data into memory, or enters a loop that clears the screen, swaps buffers, and displays a menu if the file or disk access fails.
- `COMDISK`: Copies a sequence of bytes from one memory location to another and calls three different routines.
- `FINDISK`: Disables and then re-enables interrupts using a temporary state change.
- `SAVEERR`: Updates a global state or backup buffer by copying a sequence of data blocks from a source pointer, then synchronizes certain score-related or timer-related global variables.
- `AFFICHENORMAL`: Copies a block of data from a source pointer to a destination pointer using a loop, then clears a secondary offset within the destination buffer to zero.
- `CONSTANTTIME`: Adds a constant offset to a global variable and stores the result in a pointer-referenced memory location.
- `NUM2STR`: Converts a signed integer into a string representation by prepending a sign character and calculating decimal digits in reverse order.
- `N2S_LOOP`: Iteratively extracts decimal digits from an integer and stores them in a buffer before a counter reaches zero.
- `PRNUM`: Iterates through a buffer of string-converted numbers and updates a bitmask or character set in a global buffer by masking and or-ing bytes.
- `N2S_BUF`: Reads a word from a source address offset by a register and stores it to a destination address.
- `SAVEDEMO`: Uses a system call to access a string and another system call to perform a file or string operation.
- `NEWTABLO`: Resets a data table or buffer with a fixed value and copies a new set of parameters or coordinates from a source list.
- `MS_CHECK`: Updates a text buffer by copying or padding characters and then enters a loop that redraws the background and checks for keyboard input, specifically looking for a certain key or a keypress that triggers a decrementing counter and clears several memory-mapped or global data fields.
- `MS_WAITK`: Waits for a keypress while updating a background drawing and clearing certain memory or register-based flags.
- `MS_QUIT2`: This routine is an empty stub that performs no operations before returning.
- `MODIFIESCORE`: Clears a bit in a global variable, initializes a background, and executes a sequence of display updates and key checks similar to the reset routine.
- `MS_NOUPDATE`: Resets a demo counter, plays original music, and executes a loop that draws a background and checks for a keypress until a certain timer threshold is reached.
- `FONDUAUNOIR`: Sets a global variable to a specific value and enters a loop that waits until that variable reaches zero.
- `FONDUAUBLANC`: Restores registers from a global variable, stores that same global variable into another location, and then branches to a previous instruction if the value at that location is not zero.
- `INITDESSINEFOND`: Sets certain global parameters to zero or fixed values, configures a drawing mode, and resets a portion of the screen or drawing buffer using clearing and screen-swapping routines.
- `DESSINEFOND`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `DF_BACK`: Updates an object's position and color using a lookup table and random or calculated offsets, then swaps two global values.
- `DF_BACKINV`: Updates background or object position or coordinates using trigonometric or random values, and modifies a global color variable and a color table.
- `DF_NORM`: Clears the screen and copies a pair of values from a global variable or table to another location before waiting for a vertical blank.
- `modify_stack_and_branch`: Pops a byte from the stack into a memory location, then branches to a location if the carry flag is set, otherwise it subtracts 7 from a value at an offset from a2.
- `SELECTOPTIONS`: Initializes the menu or settings state, then enters a loop that calls a menu-drawing routine to handle user input for starting a single-player game, modifying speeds, or exiting.
- `RESET`: Stops music, fades a color, and resets several hardware registers and global variables to a default state.
- `MAKEMENU`: Parses a string-based menu definition looking for specific delimiters and tags to determine menu item count and layout.
- `AFFICHETEXTESANSATTENTE`: Calls a routine to display text on a background and repeatedly calls another routine while the return value is negative.
- `AFFICHETEXTESURFOND`: Checks a global flag to potentially call another routine, then clears certain memory locations and updates a pointer-referenced address.
- `ATSF_1`: Checks a memory value and a keypress, then executes a loop that calls a character drawing routine and a background drawing routine.
- `TEXT2SCR`: Performs a bitwise OR operation between a source buffer and a destination buffer, then copies a block of data from a source pointer to a destination pointer if a certain condition is met.
- `TEXT2SCRI`: Iterates through a buffer to apply a bitwise NOT and AND mask to a destination, then copies a block of data from a source to a destination.
- `AC_CLEAR`: Clears a block of memory by setting dwords to zero and then copies a block of data from a source buffer to a destination buffer if a certain global flag is non-zero.
- `AFFICHECARACTERESUIVANT`: Iterates through a string or a list of control codes, updating a pointer and rendering or clearing character or menu data.
- `AFFICHECARD0`: Checks a character or control code and, if it is a printable character, copies a sequence of attribute or tile data from a lookup table to a screen buffer or a temporary buffer.
- `KEYPRESSED`: Checks a status or input via a library call and returns a bitwise combination of a keyboard or joystick register and a flag.
- `CLRKEY`: Resets certain joystick and keyboard registers to zero and waits until a library call returns zero.
- `COPYSCREEN`: Performs a bulk memory copy or block transfer from a source pointer to a destination pointer using an unrolled loop.
- `AFFICHECARTE`: Iterates through a 2D grid or table to draw polygons or filled areas based on values stored in a map-related data structure.
- `AC_10`: This routine writes a sequence of different values to a memory location through a pointer and calls a routine to display a card or text.
- `UPLOOP`: The routine decrements a countdown word in the game's data block and returns early while it is still nonzero.
- `UPLOOP2`: Executes a loop that updates a counter, clears the screen, and calls a sequence of drawing and movement routines while checking a termination condition.
- `DESSTOUT`: Increments a global counter, executes a sequence of drawing and input routines, and swaps certain global values.
- `DESSINEOMBRE`: Calculates and writes projected 3D coordinates for a list of shadow-casting vertices or edges into a buffer.
- `set_bits_and_apply_masks`: Sets bit 6 in d2 and d6, then performs bitwise OR operations on d0 using the constants 210, 241, 253, 10, and 22.
- `DESSINEVAISS`: Resets certain global state variables and calculates a horizontal offset or width based on a difference between two values.
- `DV_DEMO`: Updates a structure field, clamps a value, and calls a routine from a table.
- `DV_VAISSEAUX`: Jump stub: passes control straight on to DV_TETRA.
- `jump_to_dv_mickey`: Jump stub: passes control straight on to DV_MICKEY.
- `jump_to_dv_speedy`: Jump stub: passes control straight on to DV_SPEEDY.
- `jump_to_dv_flieg`: Jump stub: passes control straight on to DV_FLIEG.
- `jump_to_dv_jomps`: Jump stub: passes control straight on to DV_JOMPS.
- `DV_ROCKET`: Calculates and stores a sequence of 3D or 2D coordinates or direction vectors into a buffer, using a lookup table to determine pathing or direction, and potentially calls a routine to handle certain object or perspective-related updates.
- `DV_MICKEY`: Calculates new or reversed directional offsets and updates a list of object coordinates by iterating through a direction table and calling a function to compute new positions.
- `DV_JOMPS`: Iterates through a table of function pointers to update a buffer of directional or perspective-transformed coordinates.
- `DV_SPEEDY`: Iterates through a table of direction-based function pointers to update a list of 3D coordinates and then calls a routine to calculate perspective or concave-related transformations.
- `DV_FLIEG`: Iterates through a table of function pointers to calculate and store new vertex coordinates or perspective-transformed values into a global buffer.
- `DV_TETRA`: Iterates through a table of function pointers to calculate and store new vertex or object coordinates in a buffer.
- `VAISSOTHER_I`: Checks if a vessel's ID matches a target, then updates its position, calculates relative distance, and updates a global position table.
- `VOI_VAISSEAUX`: Jump stub: passes control straight on to VOI_TETRA.
- `jump_to_voi_mickey`: Jump stub: passes control straight on to VOI_MICKEY.
- `jump_to_voi_speedy`: Jump stub: passes control straight on to VOI_SPEEDY.
- `jump_to_voi_flieg`: Jump stub: passes control straight on to VOI_FLIEG.
- `jump_to_voi_jomps`: Jump stub: passes control straight on to VOI_JOMPS.
- `VOI_ROCKET`: Sets a top-offset and a reaction-color, potentially updating the color based on a joystick input bit.
- `VOI_JOMPS`: Sets a top-offset and a reaction-color based on joystick input, then initializes several fields in a structure with fixed or calculated values.
- `VOI_MICKEY`: Updates a table of coordinates using values from a routine, applies a negation and an offset, and modifies certain hardware or global registers.
- `VOI_SPEEDY`: Sets a top-offset and a reaction-color, potentially updating the color based on a bit in a joystick-related table.
- `VOI_FLIEG`: Sets a top-offset, certain structure fields, and certain color variables, potentially updating the colors based on a joystick input bit.
- `VOI_TETRA`: Calculates a new boundary or offset for a tetrahedral object based on a value from a table and updates a secondary boundary or state.
- `VOI_COMMUN`: Calculates a difference between two values from a table and stores it in a global variable, then copies certain values to a pointer and updates a transformation result.
- `VOI_NSEEN`: Stores a constant value into a memory location relative to a pointer.
- `OMBREOTHER_I`: Writes a byte to a pointer and, if certain values in a table match, updates a global variable with a difference and copies several values to a pointer and a transformation result.
- `INITOL`: Checks a flag and a pointer-based checksum or ID to decide whether to copy existing level data or load new data from a table.
- `ADDONE`: Increments a numeric string stored in memory, handling a carry-over or a terminator character.
- `SUBONE`: Iterates backward through a string, decrementing character values and replacing certain characters with a digit, until a specific terminator or a certain range is reached.
- `MKSCORE`: Increments a score or timer value while decrementing a counter, and updates a secondary timer or score if certain conditions or joystick-related flags are met.
- `FASTPRT`: Iterates through a source buffer, using a character-to-index lookup table to retrieve a bitmask and applying that bitmask to multiple non-contiguous memory locations via a destination pointer.
- `DEPLACE`: Calculates a new position by adding an offset to an existing coordinate and clamps the resulting value within a fixed range.
- `TSTJOYST`: Iterates through a buffer of input data, extracting certain bits to update a global array or table, and then proceeds to execute a sequence of initialization and main loop logic if certain conditions are met.
- `ORGSPEEDS`: copies a block of words from one table to another using a loop.
- `MODIFSPEEDS`: Copies a block of data from one table to another using a loop.
- `INCUBE`: This routine executes a sequence of instructions but performs no operations or state changes before returning.
- `INCUBE2`: Clamps the values in registers d0, d1, and d2 within specific ranges by comparing them against various constants and overwriting them if they fall outside those bounds.
- `RANDOM`: multiplies a value at a memory location by a constant and adds one.
- `AZIMPOLAR`: Calculates a value based on a rotation result and a difference between a register and a table entry, then returns a flag indicating if the result is non-zero.
- `AP_NEG`: returns a value representing all bits set.
- `CAPPOLAR`: Performs a calculation using a return value from another routine and certain register values, returning a boolean-like result based on whether the result is positive.
- `TOUCHING`: Calls a routine that checks for a collision or contact condition.
- `TC_0`: Iteratively reads 16-bit words from a table pointed to by a1, adds a value from a register, compares it against a constant, and performs a bitwise AND on register d7 with a specific mask based on the comparison result.
- `GEREOMBRE`: Checks a collision or contact condition via another routine and, if a certain range or bitmask condition is met, updates a value in a table or structure and copies a value from another memory location.
- `GO_RET`: Small routine of 2 instructions that loads 255 into d6, then returns.
- `OBJPRGS`: Empty routine: returns to the caller immediately without doing anything.
- `POSETRAJ_I`: Iterates through a list of trajectory or object data, updating a global pointer or index and calling object-specific drawing or processing routines.
- `TRAJECT_I`: Updates a 3D-like trajectory or vector by calculating or updating polar-to-cartesian-style offsets and applying a rotation-based or incremental change to a global table.
- `SPEEDER_I`: If a certain distance is below a threshold, it updates a value in a table, modifies several global offsets using a pointer, and plays a sound.
- `BIGONE_I`: If a certain distance is below a larger threshold, it updates a value in a table, modifies several global offsets using a pointer, and plays a sound.
- `STARWAR_I`: If a certain distance is below a certain threshold, it updates a value in a table, modifies several global offsets using a pointer, and plays a sound.
- `EXPLOR_I`: Updates a trajectory and, if a certain distance threshold is not met, modifies a global value and updates a set of coordinates by adding the difference between a target and a current position.
- `ALIEN_I`: Calculates the distance to an object and either updates a direction vector or triggers a repositioning and rendering routine.
- `ALIEN_CONTACT`: Updates several pointer-based values and coordinates, then calls another routine to update a pointer.
- `PROTECT_I`: Checks a bitmask in a structure and, if a distance-based condition is met, calculates and stores several offset values, triggers a sound, and increments several values in a table or structure.
- `PROTECT_NO`: stores a specific word value into a memory location.
- `PROTECT_NOBIS`: stores a different word value into a memory location.
- `PROTKEY_I`: Checks a bitmask in a table, and if a certain bit is not set, updates a bitmask and increments several values in a table or a global structure.
- `LONGX2_I`: stores a constant value, a zero, and a constant value into a sequence of memory locations relative to a pointer, then calls another routine.
- `LONGX3_I`: Updates a memory-resident object or structure by writing a constant value and a new length, then conditionally updates a coordinate or size-related value and a secondary offset or position-related value based on a comparison.
- `LONGX4_I`: stores a constant value, a zero, and a constant value into a memory block or structure, then calls a routine to generate a shadow or shade.
- `LONGX5_I`: Writes a constant value and a zeroed value to a global variable and calls another routine.
- `LONGY2_I`: Sets three consecutive words in a memory block or structure to specific constant values and then calls a routine to generate a shadow or shade.
- `LONGY3_I`: Updates a global structure by setting a coordinate and a velocity-related value, then modifies a secondary velocity or position variable based on a comparison with a threshold.
- `LONGY4_I`: Sets three consecutive words in a memory block or structure using a pointer held in a register, then calls another routine.
- `LONGY5_I`: Sets a sequence of three words in a memory block or structure, then calls another routine.
- `FALLING_I`: Updates a value in a table if certain conditions are met, and potentially modifies a value in a global structure and calls a routine to handle shadows.
- `FALLING_SETBACK`: Modifies a value in a structure based on a comparison, potentially resets a flag, calls another routine, and updates a value in a structure if a bitmask condition is met while triggering a sound.
- `FALLING_INIT`: Sets a value in a structure, copies a value, and conditionally updates a coordinate and a variable while calling a sound routine.
- `CLIGN_I`: Decrements a counter in a structure and resets a byte if the counter falls below a certain threshold.
- `CLIGN_EFF`: stores a byte and a word zero into a memory location.
- `CATCHER_I`: Calculates a new trajectory or path for a target or object by interpolating or smoothing a position vector.
- `AUTO1_I`: Updates a trajectory routine, resets a shadow-related byte, and calculates a new value for a global variable based on the sum of two other global values, clamped to a minimum threshold.
- `AUTO2_I`: Clears a value in a table, calls another routine, and updates a global offset based on a clamped calculation before playing a sound.
- `AUTO3_I`: Calculates a new distance value by subtracting a value from a pointer-referenced memory location and clamps it against a minimum threshold.
- `AUTO4_I`: Calculates a new value for a global variable by summing two other global values and clamping it against a third, then updates a separate global variable by subtracting a bit-shifted version of that result.
- `AUTO5_I`: Calculates a new value for a global variable by subtracting two other global values and clamping it to a minimum threshold.
- `AUTO6_I`: Updates a trajectory routine, resets a shadow-related byte, and calculates a new shadow or offset value based on the sum of two memory-resident shorts.
- `AUTO7_I`: Clears a value in atable, calls another routine, and updates a global offset based on a clamped calculation before playing a sound.
- `AUTO8_I`: Calculates a new value for a global variable based on other global variables and updates a third global variable.
- `AUTO9_I`: Performs a calculation involving a global variable, a temporary value, and a limit, then stores the result back to a global variable.
- `AUTO10_I`: Updates a trajectory and certain memory values, then clamps a calculated value against a minimum threshold before updating a coordinate and a counter.
- `LABP_I`: Checks a distance value and conditionally writes a byte to a table, then executes a conditional block that updates a coordinate.
- `PTELEV_I`: Calls a routine and then subtracts a constant from a value in a structure if a condition is met.
- `BULLEV_I`: Toggles a bit in a byte at a certain offset from a pointer, calls another routine, and restores the bit to its original state.
- `BULLE_R`: writes a word containing all bits set to a memory location pointed to by a register minus two bytes.
- `BULLEP_I`: Checks a timer or distance-based condition to update a global counter and a secondary variable, or triggers a sequence that modifies a list of coordinates, calls a sound routine, and executes drawing or perspective-based rendering routines.
- `BULLEP_D`: Checks a distance value and, if below a threshold, updates several global variables and plays a sound; otherwise, it performs a rendering routine involving circles and transformed polygons.
- `BULLEP_T`: Draws a circle and then iterates through a list of coordinates to apply a transformation and draw a perspective-transformed polygon or a series of lines.
- `EXPLODE_I`: Updates a table of values by adding register contents to certain indices, right-shifting those indices, and then adding or subtracting constants to other table indices.
- `OIZO_I`: Updates a set of global coordinates and a bounding box, modifies a rotation or projection parameter, and triggers sound or coordinate-based updates if certain distance or random conditions are met.
- `BONUS_I`: Checks a bitmask in a global structure, and if a bit is not set, potentially increments a value in a global structure and calls a sound routine.
- `BONUS_S`: Stores a 32-bit register value into a global variable and a second location calculated via a pointer in a global variable.
- `BONUS_M`: Checks a distance value and, if below a threshold, increments a global variable, triggers a sound, and modifies several values in a structure or table.
- `DIAMOND_I`: Checks a bitmask in a global structure, and if a bit is not set, updates a bitmask in a table and a corresponding value in another table, then increments a value in a table.
- `DIAMOND_M`: Sets a bit in a variable and a corresponding memory location, then increments a global variable and triggers a sound if a distance condition is met.
- `SORTIE2_I`: Updates object position or velocity using a weighted average or decay-based smoothing, while resetting certain boundary or state-related memory or registers.
- `SORTIE3_I`: Updates a global object's position, velocity, and certain physical properties using a weighted-average smoothing or damping technique.
- `SORTIE4_I`: Updates a global object's position and velocity using a smoothing or dampening calculation, while also resetting certain certain status or boundary-related flags and clearing or decrementing certain counters.
- `SORTIE1_I`: Updates certain object or particle properties, including a smoothing or lerping-like calculation for position or velocity, and resets or increments certain state-related or coordinate-related memory offsets.
- `PORTE_O`: Updates several values in a global structure, including a calculation involving a rotation routine, and applies a decay-like reduction to certain values.
- `TRANSN_I`: Increments a value in a structure and potentially increments another value, then updates a pointer-based value using a masked input.
- `TRANSE_I`: Increments a value at a pointer and potentially increments a previous value in the structure, then updates a pointer-based value using a masked input.
- `TRANSS_I`: Decrements a value in a structure and potentially decrements another value, then updates a pointer-based value using a masked input.
- `TRANSW_I`: Decrements a value in a table and potentially decrements a secondary value if a threshold is reached, then adds a masked bit or value to another table entry.
- `ROTATEG_I`: If a certain condition is met, increments a value at a pointer and a second value at an offset, then clears certain memory or sets a byte to a fixed value.
- `ROTATED_I`: If a certain condition is met, decrements a value at a pointer and a second value at an offset, then clears certain memory or sets a byte to a fixed value.
- `ROTAG_I`: Increments a value at a pointer and a second offset, then sets a byte in a table to a fixed value and clears two other bytes.
- `ROTAD_I`: Decrements a value at a pointer and a second offset, then sets a byte in a table to a fixed value and clears two other bytes.
- `PLAQUEN_I`: Calls another routine and, if a certain flag is set, increments a table entry and a secondary offset, then resets several bytes in a table.
- `PLAQUEE_I`: Checks a global flag and, if non-zero, increments a pointer-based value and a secondary offset.
- `PLAQUES_I`: Updates a pointer-based structure by incrementing a value and decrementing another if a certain condition is met, then resets a byte and several words in the structure to fixed values.
- `PLAQUEW_I`: Calls another routine and conditionally increments a pointer-based value and decrements a secondary offset.
- `SMLPLAQ_I`: calls another routine and then adds a masked bitset from a data register to a value stored at a pointer-based memory location.
- `TELEPORT_I`: Iteratively updates certain coordinates using random offsets and then checks a collision or contact condition while decrementing a counter.
- `TELEPORT_INACT`: Decrements a 16-bit value stored at an offset from a pointer.
- `PLAQUE_I`: Small routine of 1 instruction that loads 13 into d6.
- `PLAQUE_X`: Checks a bitmask in a register and, if certain conditions are met, updates a global variable with a clamped value and decrements another global variable.
- `PLAQUE2_X`: Calls a shadow routine and, if certain bitmask conditions are met, updates a value in a global structure and calls a sound routine.
- `PLAQUE_IR`: Small routine of 3 instructions that loads 0 into d7, then adds 2 to a7, then returns.
- `MURNS_I`: Calls a routine to get a value, then conditionally updates a pointer-based value and potentially updates several other pointer-based values before calling a sound routine.
- `MUREW_I`: Updates a global shadow value and calculates a new offset or direction based on a difference between a global variable and a value from another routine.
- `CUBE1000_I`: Updates a global shadow or light value and, if a certain bitmask condition is met, calculates new offset or projection values and triggers a sound.
- `CUBE500_I`: Updates a global shadow value and calculates new offset or position values based on a shadow-related routine, then triggers a sound if a certain bitmask condition is met.
- `CUBE400_I`: Calculates and stores a new shadow or offset value into a structure member based on a value from a routine, and updates a vector or coordinate offset if a certain bitmask condition is met.
- `CUBE300_I`: Calculates and stores a new shadow or offset value into a global structure based on a value from another routine.
- `CUBE200_I`: Calculates a shadow-related value and updates a global or structure-based offset or vector.
- `CUBE100_I`: Calls a shadow routine and, if a certain bitmask condition is met, calculates and stores new values in a table and calls a sound routine.
- `REBOND_I`: Updates a particle's position and velocity, applying a bounce or reset if a boundary or distance threshold is exceeded.
- `MCHENILLE_I`: Iterates through a list of object structures to update certain object properties, trigger drawing routines, and check for collisions or proximity.
- `TCHENILLE_I`: Updates the 3D position and rotation of an object using polar-to-cartesian or rotation-based coordinate transformations, checks for collisions or contact, and manages an object list or queue.
- `TCHENILLE_CONTACT`: The routine is empty and performs no operations or data manipulations.
- `CHASSR_I`: Updates a global object's position or velocity-related offsets based on the difference between a target and a source, and triggers a sound if the distance is below a certain threshold.
- `CHASSR_CONTACT`: Checks if a distance value is below a threshold and, if so, updates a global pointer with a constant and modifies several values using a difference or sum before calling a sound routine.
- `SUIVEUR_I`: Executes a follow routine and a rotation routine, then updates a table or array with a new value and an incremented coordinate.
- `FOLLOW_I`: Updates two different memory locations by adding a bit-shifted value derived from a routine call, then sets a byte in a table to a fixed value.
- `CANON_I`: Executes a follow routine and checks a random value; if a condition is met, it initializes a new entry in a table with a starting position and direction, and increments a counter or index.
- `TESTEMISSILE`: Iterates through a list of pointers in a global table to check if a specific bitmask matches a certain value.
- `TM_TOUCH`: Small routine of 2 instructions that loads 255 into d7, then returns.
- `FIREP_I`: Calls another routine and, if a certain flag is set, updates a certain value if it is below a threshold, then writes new data to a table or list and increments a counter.
- `MISSILE_I`: Updates the position and rotation-based offset of a missile object, checks for screen boundary or collision, and manages a list of object properties.
- `OBJCLR`: Iterates through a memory range to copy data, decrements a global counter, and resets several global or table-based values using data from a pointer.
- `DISTANCE`: Calculates the Manhattan distance between two 3D points and returns a maximum value if an overflow or certain condition is detected.
- `DESSINECUBE`: Sets a series of memory locations to specific constant values, resets certain pointers or counters, and calls two other routines to update or apply a transformation.
- `MODIFCLIP`: Clears a large block of memory by setting certain offsets to zero using a loop.
- `DESSINEOCUB`: Initializes a set of vertex coordinates with a fixed offset and then iterates through a table of direction vectors to calculate and store transformed 2D screen coordinates for a wireframe cube.
- `SWAPVARS`: If certain conditions in a global structure are met, copies a block of 8-word-sized data from one memory location to another.
- `DESSINEMONDE`: Iterates through a list of objects to apply 3D transformations and update their screen-space or clip-space coordinates.
- `DM_TIMOUT`: Sets a byte to zero and executes a loop that checks certain conditions, updates global or table-based values, and calls a drawing routine.
- `XP1_DIR`: Returns the current value held in a register.
- `YP1_DIR`: The routine returns the current contents of a data register without performing any operations or accessing memory.
- `ZP1_DIR`: Returns the address or value of a specific memory location.
- `XP2_DIR`: The routine returns an uninitialized or undefined word from a register without performing any operations.
- `YP2_DIR`: The routine returns the current value of a data register without performing any operations or accessing memory.
- `ZP2_DIR`: returns an uninitialized or undefined word from a register or memory location.
- `XP3_DIR`: The routine is empty and returns an uninitialized or undefined word.
- `YP3_DIR`: the routine returns an uninitialized or undefined word from a register without performing any operations.
- `ZP3_DIR`: The routine returns the current value held in a data register without performing any operations or accessing memory.
- `XP4_DIR`: The routine returns an uninitialized word from a data register.
- `YP4_DIR`: returns an uninitialized word from a data register.
- `ZP4_DIR`: returns an uninitialized or indeterminate word from a register.
- `XP5_DIR`: The routine is empty and returns an uninitialized or undefined value.
- `YP5_DIR`: this routine returns an uninitialized or undefined word-sized value from a register.
- `ZP5_DIR`: returns an uninitialized or undefined word from a register.
- `XP10_DIR`: The routine returns the current value stored in a data register without performing any operations.
- `YP10_DIR`: The routine returns the current contents of a data register without performing any operations.
- `ZP10_DIR`: the routine returns an uninitialized or undefined word from a register.
- `XM1_DIR`: Subtracts zero from registers d0, d1, and d2 and then returns.
- `YM1_DIR`: This routine is empty and returns an uninitialized or undefined word.
- `ZM1_DIR`: the routine returns an undefined or uninitialized word without performing any operations or accessing memory or registers.
- `XM2_DIR`: The routine is empty and returns an uninitialized or undefined word from a register.
- `YM2_DIR`: The routine is empty and returns an undefined or uninitialized value.
- `ZM2_DIR`: Subtracts zero from the contents of registers d0, d1, and d2 before returning.
- `XM3_DIR`: This routine contains no executable instructions and returns an uninitialized value.
- `YM3_DIR`: Subtracts zero from the 16-bit values in registers d0, d1, and d2 before returning.
- `ZM3_DIR`: Subtracts zero from the word values in registers d0, d1, and d2 before returning.
- `XM4_DIR`: Clears the contents of data registers zero, one, and two by subtracting zero from them and then returns.
- `YM4_DIR`: the routine returns an uninitialized or undefined word from a data register without performing any operations.
- `ZM4_DIR`: This routine performs no logic and returns an uninitialized value.
- `XM5_DIR`: The routine returns an uninitialized or undefined word-sized value from a register.
- `YM5_DIR`: This routine is empty and returns an uninitialized or undefined word value.
- `ZM5_DIR`: Returns the current contents of a data register as a word.
- `XM10_DIR`: This routine returns the current value held in a data register without performing any operations or accessing memory.
- `YM10_DIR`: The routine returns an uninitialized or undefined word without performing any operations.
- `ZM10_DIR`: The routine returns the current value held in a data register without performing any operations.
- `ORIGIN_DIR`: Returns a 64-bit value consisting of two zeroed 32-bit integers.
- `POINT1_DIR`: Performs a complex calculation using values from a table and returns a concatenated result that includes a rotation-like transformation.
- `POINT2_DIR`: Performs a complex calculation using different offsets from a table and returns a concatenated result that includes a rotation-like transformation.
- `POINT3_DIR`: Calculates a new 3D vector or direction by applying a rotation matrix or quaternion-based transformation to an existing vector stored in memory.
- `POINT4_DIR`: Calculates a new 2D or 3D vector or coordinate by applying a 2x2 or 3x3 rotation matrix or transformation matrix stored in a global structure.
- `POINT5_DIR`: Performs a complex calculation using values from a table and returns a 64-bit result containing a transformed vector.
- `POINT6_DIR`: Calculates a new 2D or 3D rotation matrix or vector transformation using a series or product of cross-products or matrix-vector multiplications.
- `POINT7_DIR`: Calculates a new 2D or 3D vector or rotation matrix element by performing a fixed-point multiplication and subtraction using values from a global structure.
- `POINT8_DIR`: Calculates a new 2D vector or matrix-transformed coordinate using a 2x2 or 3x3 matrix-vector multiplication or rotation-like transformation.
- `POINT9_DIR`: Calculates a new 2D vector or direction by applying a 2D rotation matrix or complex number multiplication to an existing vector stored in memory.
- `POINT10_DIR`: Calculates a new 2D or 3D vector or matrix component using a complex multiplication and subtraction routine involving several memory-resident or register-resident or pointer-offset values.
- `XP1_LDIR`: The routine is empty and returns an undefined or uninitialized word value.
- `YP1_LDIR`: The routine returns the current value of a data register without performing any operations or modifying any memory or registers.
- `ZP1_LDIR`: clears the contents of a data register by loading it with zero.
- `XP2_LDIR`: the routine is empty and returns an uninitialized or undefined word.
- `YP2_LDIR`: Returns the currentcontents of a data register as a word.
- `ZP2_LDIR`: The routine returns an uninitialized or undefined value from a register.
- `XP3_LDIR`: Adds zero to registers d0, d1, and d2 and then returns.
- `YP3_LDIR`: Sets the contents of registers d0, d1, and d2 to zero and returns.
- `ZP3_LDIR`: The routine contains no executable instructions and returns an undefined value.
- `XP4_LDIR`: This routine returns a value from a register without any prior modification.
- `YP4_LDIR`: Adds zero to registers d0, d1, and d2, then returns.
- `ZP4_LDIR`: This routine returns an undefined value from a register.
- `XP5_LDIR`: The routine does not perform any logic and returns an undefined value.
- `YP5_LDIR`: The routine is empty and returns an undefined or uninitialized word.
- `ZP5_LDIR`: The routine returns an undefined value without any side effects.
- `XP10_LDIR`: The routine returns an undefined word value.
- `YP10_LDIR`: the routine acts as a stub that returns an undefined or uninitialized word without performing any operations.
- `ZP10_LDIR`: the routine returns an uninitialized or undefined word without performing any operations.
- `XM1_LDIR`: clears the contents of a data register.
- `YM1_LDIR`: the routine is empty and returns an undefined or uninitialized value.
- `ZM1_LDIR`: Subtracts zero from registers d0, d1, and d2, then returns.
- `XM2_LDIR`: Clears the contents of a data register and returns the result.
- `YM2_LDIR`: the routine is empty and returns an undefined or uninitialized word.
- `ZM2_LDIR`: This routine returns the current value stored in a data register without performing any operations.
- `XM3_LDIR`: clears the contents of a data register and returns the result.
- `YM3_LDIR`: Sets the contents of registers d0, d1, and d2 to zero before returning.
- `ZM3_LDIR`: The routine returns an uninitialized word from a data register without performing any operations.
- `XM4_LDIR`: The routine contains no executable instructions and returns an uninitialized value.
- `YM4_LDIR`: Clears the contents of a data register before returning a value.
- `ZM4_LDIR`: The routine does not perform any logic and returns an uninitialized value.
- `XM5_LDIR`: clears the d0 register and returns a null value.
- `YM5_LDIR`: Subtracts zero from the values in registers d0, d1, and d2 before returning.
- `ZM5_LDIR`: The routine returns the current contents of a data register without performing any operations or modifying any memory or registers.
- `XM10_LDIR`: The routine performs no actions and returns an uninitialized value.
- `YM10_LDIR`: This routine is empty and returns an undefined or uninitialized word value.
- `ZM10_LDIR`: Returns an uninitialized or undefined word from a register.
- `ORIGIN_LDIR`: returns a 64-bit value consisting of two zeroed 32-bit integers.
- `POINT1_LDIR`: Calculates a new coordinate or vector using a rotation routine and values from a table, then returns a concatenated result.
- `POINT2_LDIR`: Performs a complex 2D or 3D vector transformation or rotation using a matrix-like multiplication or rotation-matrix-based coordinate recalculation.
- `POINT3_LDIR`: Applies a 2D rotation or scaling transformation to a vector using a rotation matrix or a set of coefficients stored in a global structure.
- `POINT4_LDIR`: calculates a new vector or coordinate using a rotation-based matrix-like transformation involving a rotated value and several global parameters.
- `POINT5_LDIR`: Performs a complex 2D or 3D coordinate transformation or rotation using a series of multiplications and bit-shifts, updating a set of destination or temporary registers with the calculated or scaled-down vector components.
- `POINT6_LDIR`: Calculates a new vector or coordinate using a rotation-based transformation involving a 2D rotation matrix or complex number multiplication.
- `POINT7_LDIR`: Calculates a new set of 2D or 3D coordinates by applying a rotation or scaling matrix to an existing vector.
- `POINT8_LDIR`: Calculates a new 2D vector or coordinate by applying a rotation or scaling transformation using values from a global structure.
- `POINT9_LDIR`: Performs a complex 2D vector transformation or rotation-scaling operation using a series of 32-bit-style fixed-point multiplications and subtractions.
- `POINT10_LDIR`: Performs a complex calculation involving a rotation routine and several table-based values, returning a concatenated result.
- `AFFOBJ`: Iterates through a table of object definitions to calculate and store projected or transformed coordinates into a destination buffer.
- `AO_CONCAV`: Calculates perspective-transformed vertex coordinates for a polygon and performs a scanline-style fill or edge-drawing routine.
- `AO_GOOD`: Iterates through a list of vertex counts, copies vertex data from a table, and applies a transformation or draws lines or a filled polygon.
- `DOCIRCLE`: Iterates a fixed number of times, calling subroutines to calculate sine and cosine, and stores the resulting values into a global memory location.
- `FACFILL`: Calculates a value based on differences between table entries and, depending on a flag, either fills a polygon or draws a series of lines using a decreasing counter.
- `FF_NONORIENTED`: Iterates through a list of vertex coordinates to determine the minimum and maximum horizontal and vertical extents.
- `TRANSXYZ`: Conditionally calls a rotation routine based on a value in a global structure, and then calls another rotation routine.
- `PERSPECT`: Calculates a perspective projection by scaling or shifting input values based on a bit-shift amount and a divisor, then adding an offset.
- `TRIROTATE`: Performs a complex coordinate transformation using a table of values to calculate new coordinates via multiplication, subtraction, and bit-shifting.
- `TRIROTATEL`: Calls a rotation routine or its return value multiple times.
- `ROTATE`: Checks if a register is non-zero and, if so, calls a sequence of four trigonometric-based routines.
- `XCOSY`: the routine is empty and performs no operations or data modifications.
- `XSINY`: The routine is empty and performs no operations or data transfers.
- `TRIGINIT`: Initializes multiple sets of trigonometric lookup tables by calculating sine or cosine-related offsets and storing precomputed or scaled values into global arrays.
- `LTRIGINIT`: Initializes several lookup tables by calculating sine-based or rotation-based offsets and storing scaled or multiplied values into a series of global memory locations.
- `CLSNORM`: Clears a large, contiguous block of memory or a data buffer by setting a range of array indices to zero.
- `CLS`: Sets a large range of memory or a buffer to a constant maximum value.
- `LINE`: Calculates intermediate coordinates along a line segment using a linear interpolation or bresenham-like approach, checking if the calculated points fall within a specified bounding box.
- `LI_AFFLIN2YME`: Iteratively sets a specific bit in a large buffer at multiple distant offsets while decrementing a counter and updating a pointer.
- `LI_AFFLIN2YPE`: Iteratively sets a specific bit in a bitmask at a large offset within a buffer, while decrementing a counter and advancing a pointer.
- `LI_AFFLIN3YME`: Sets a specific bit within a byte at a memory location and at fixed offsets in a large buffer.
- `LI_AFFLIN3YPE`: Iteratively sets a bit in a large buffer at specific offsets and modifies a counter or pointer.
- `LI_AFFLINVERT`: sets a specific bit in a memory buffer at regular intervals using a pointer and a loop.
- `LI_TABPROG`: toggles a specific bit within a set of bit-planes or bit-masks located at regular intervals in a global memory buffer.
- `SWAPSCRN`: Swaps the contents of two memory locations and updates a third, then calls a routine to update the screen and waits for a vertical sync.
- `INIT20`: This routine waits for a certain input or flag, then opens a file, reads a block of data, and copies specific segments of that data into a global buffer.
- `FILLPOLY`: Iterates through a list of polygon vertices to calculate a bounding box and clip the polygon coordinates against a defined screen or window boundary.
- `FILLPOLY0`: the routine performs no operations and returns immediately.
- `INITVAR`: calls a routine multiple times and then clears a sequence of bytes in a global array.
- `IV0`: Iterates through a memory range, setting each byte to zero, until a counter reaches a certain value.
- `BIOS1_4`: Checks a bit in a global variable and returns either zero or a full bitmask.
- `BIOS2_4`: Searches a table for a matching byte, stopping if a terminator or a match is found, and returns a following byte.
- `GEMDOS1_2`: iterates through a table of integers to find a matching byte or a terminator.
- `GEMDOS9_6`: the routine is empty and performs no operations.
- `GEMDOS10_6`: The routine is empty and performs no operations.
- `XBIOS5_12`: the routine is empty and performs no operations or data manipulation.
- `LECTURE`: Updates a global variable based on the state of a hardware register and modifies a bitmask based on certain conditions.
- `LECTURE1`: Updates a global variable and a bitmask based on the state of a hardware register and certain bitwise checks.
- `SOURIS`: Calculates and clamps the relative mouse movement or position within a defined or or clamped coordinate range.
- `SOURIS1`: Calculates a new offset or delta based on a comparison of a global variable and other registers, then stores the result into a memory location via a pointer.
- `SOURIS100`: Compares a value from a hardware port against a global variable and updates a secondary global variable or a pointer-based destination with the difference or a clamped value.
- `SOURIS11`: calculates a new value based on a difference and stores it in a global variable and a pointer-referenced location.
- `SOURIS110`: Calculates the difference between a register value and a global variable, stores the result in a pointer, and resets a global variable.
- `OPENDOS`: Opens the DOS library and stores the resulting base address in a global variable.
- `OPENGRAPH`: Opens the graphics library and stores the resulting base address in a global variable.
- `CLOSEDOS`: Calls a routine to close a library using a pointer or address.
- `OPENLIB`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `CLOSELIB`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `XVSYNC`: The routine executes an infinite loop that does not modify any registers or memory.
- `VSYNC`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `TEMPOX`: loops until a global variable reaches a certain value relative to an input.
- `TESTEXIT`: Pushes a0 onto the stack, saves d0 through d7 and a0 through a6 to a table, and branches to a new address in a0.
- `TEST`: stores several registers into global memory and executes a sequence of functions via a pointer until a certain character is detected.
- `TESTFLASH`: Writes a constant value to a global variable, which corresponds to a hardware color register.
- `TESTRIEN`: The routine consists of a single return instruction with no other work, so it does nothing when called.
- `RESTORE`: Resets certain hardware registers, stops a timer, and copies a block of data from a table into the copperlist or a specific memory range.
- `PERMIT`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `FORBID`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `INITTIMER`: Writes a constant value to a global variable and sets a hardware timer register or a global variable to another value.
- `INITTIMER0`: Opens a resource and then calls a routine using an offset from the returned base address.
- `STOPTIMER`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `INTCODE`: checks a global variable and calls a randomizing routine before incrementing a global timer.
- `INTMUS`: Iterates through a loop checking bits in a value, incrementing a global variable and potentially setting a global variable to a power of two or a specific bitmask.
- `IT2`: updates a global variable with a bitmask and increments a pointer and a counter until a bit-check fails.
- `MU_STARTMUSIC`: initializes several music-related global variables and pointers to zero or default values and calculates an insertion delay.
- `MU_CALCINS`: Iterates over a table of entries, skipping zero entries, and for each nonzero entry updates fields based on a running value and a constant, possibly resetting a field to a fixed address.
- `MU_STOPMUSIC`: Sets a global variable and a music playback pointer to zero, then writes a constant value to another global variable.
- `MU_PLAYSONG`: Increments a volume or step counter and, if a certain threshold or count is reached, resets pointers, updates a song pointer, and calls several routines to set or reset music effects and instructions.
- `MU_SETINSTR`: Parses an input value to calculate a pointer into a data table and updates various global variables or pointers related to audio notes, slides, and volume based on a type flag.
- `MU_SETEFFECT`: Checks a hardware register and updates several global variables, including note, volume, or slide values, or modifies a pointer or a flag based on specific bit patterns.
- `MU_EVERYVERT`: Increments frequency or slide-rate values for audio channels and resets channel-specific registers or pointers if certain flags are set.
- `VBLINT`: This vertical blank interrupt routine updates audio registers, manages a sound-effect or music-note counter, and updates a palette or a subset of palette registers.
- `GOPALETTE`: Iterates through a range of palette entries, incrementing or decrementing color components to transition a palette toward a target state.
- `OTEPALETTE`: Iterates through a range of palette entries and decrements the lower nibble of certain color values if they are non-zero.
- `LOADSOUND`: Opens a file and reads a certain amount of data into a buffer, then updates a pointer and a length or offset value.
- `XNOSOUND`: Sets a sound-related global variable and a DMA control register or a global variable to zero.
- `GOSON`: Updates a sound pointer and certain registers or global variables if a provided value is non-zero.
- `METPALETTE`: Writes a sequence of incrementing values into a global table and a secondary table, or copies values from a source pointer.
- `METPALETTEBIS`: Writes values from a source pointer into a destination table, incrementing the destination or a value register.
- `METECRAN`: Calls a routine three times and then stores a sequence of constant or returned values into a memory location.
- `SW10`: Stores a 32-bit value and a 16-bit value into a destination table using a pointer.
- `LOADFILE`: Opens a file using a library call, reads a certain amount of data into a buffer if a pointer is non-zero, and then closes the file.
- `CLOSE`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `OPEN`: Copies a string from a source address to a destination buffer and then copies a second string until a null terminator is reached, finally calling a library function to open a file handle.
- `CREATE`: Copies a string from a source address to a destination buffer and then copies a second string until a null terminator is reached, finally calling a library function to assign a handle.
- `READ`: Calls a function pointer from a library base to update a global variable representing a length.
- `WRITE`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `MINIINIT`: Clears a block of memory and resets several global or table-based variables to zero or a default string value.
- `ONEPLAYER`: Writes several constant values into a block of memory relative to a pointer.
- `DESSINEMOUSE`: Calculates an offset using values from a table, loads a pointer from a structure in a4, and performs bitwise operations to update memory using a pattern from a table, then enters a loop controlled by d7.
- `DM_FULLWRD`: Iteratively masks and or-is words from a source pointer into a destination pointer, skipping certain offsets.
- `SETHAM`: The routine consists solely of a return instruction, performing no register, memory, or hardware operations.
- `CLEARHAM`: The routine consists of a single return instruction with no body, so it performs no work and has no effects.
- `PLAYSOUND`: Checks a bit in a global variable and calls another routine if the bit is clear.
- `INPUTCLIP`: The routine is empty and performs no operations or data manipulations before returning.
- `OUTPUTCLIP`: Swaps the upper and lower 16-bit halves of a register and stores the result at a memory location pointed to by a register.
- `CLIP1FRONTIERE`: Iteratively calls routines to check or output clip-related data and updates a global or pointer-based value with a value from a memory location.
- `FILLPOLYGONE`: Calculates the bounding box of a set of vertices and clips the polygon against the screen boundaries.
- `OCTANT`: Sets several hardware registers and a global variable to specific values, and configures a DMA-related register or table entry using a loop.
- `PATCH2`: Calculates a new range or boundary between two coordinate values and updates a memory-resident table or buffer.
- `PH1`: Iterates through a table, performing a bitwise AND between a value and table entries, while also updating a third table at a fixed offset.
- `PH0`: Iterates through a buffer and performs a bitwise AND operation on certain memory locations using a value or a mask.
- `PH2`: Iterates through a memory buffer, applying a bitwise AND mask to a series of longwords and word-sized values.
- `PH3`: Iterates through a table to perform bitwise AND operations on certain indices using a calculated bitwise-not mask.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fall_through_to_string_reset`: The routine consists of a single instruction and serves only as an entry point reached by address-taken references from outside the static flow, with no independently recoverable logic.
- `sync_string_buffer`: Checks a condition to determine if a string buffer should be updated with new string data.
- `reset_d0_and_update_string`: Sets a data register to a fixed maximum value and updates a string buffer with new text data if a certain condition is met.
- `update_string_buffer`: Updates a memory buffer with new string data if a certain condition is met, likely used for switching localized text or menu labels.
- `copy_memory_on_mismatch`: Compares two global variables and, if they differ, copies a block of memory and updates a global variable.
- `update_state_on_pointer_dereference`: Checks a bit in a global variable and, if certain pointer-based conditions or value mismatches are met, copies memory and updates a global variable.
- `copy_memory_blocks`: Copies a fixed number of ten-byte blocks from a source pointer to a destination pointer.
- `execute_conditional_logic_2`: A large routine that performs conditional checks on a global variable and a memory location, calling several other routines that update global states, buffers, and pointers or perform memory copies.
- `update_global_state`: Updates several global variables and memory locations, including a loop that copies a sequence of bytes and a loop that checks a table to set a bitmask.
- `parse_string_delimiters`: Scans a string for specific character sequences, including certain delimiters, and skips or processes data until a newline or a matching character is found.
- `scan_string_for_delimiter`: Scans a string for a specific two-character delimiter and then decrements a pointer or value in a loop.
- `scan_string_and_execute`: Scans a string for a newline character and then executes another routine a certain number of times based on the distance from the start of the string to a calculated offset.
- `initialize_global_pointers`: Assigns a series of values from one set of global variables to another set of global variables before calling other routines.
- `execute_recursive_calls`: The routine performs a sequence of three recursive calls to itself before returning.
- `copy_and_clear_buffer`: Copies a block of data from a source buffer to a destination buffer using an unrolled loop, then clears a secondary range of the destination buffer to zero.
- `update_string_buffer_and_position`: Updates a string buffer with a new string or a space-padded version of a string, and adjusts a pointer or coordinate based on a search through a table.
- `reformat_string_buffer`: This routine modifies a string buffer by copying or overwriting a range of bytes with spaces and then copying a sequence of bytes from a source pointer.
- `reset_buffer_and_decrement_counter`: Sets certain memory locations to a fixed value and decrements a global counter or pointer if a certain condition is met.
- `store_values_to_memory`: Stores the constant an address into a memory location and stores a value from an offset of 104 in a4 into another memory location.
- `execute_conditional_logic`: Performs a series of conditional checks on a global variable and a memory location, potentially calling other routines or entering a loop, before setting a memory location to a fixed value.
- `set_global_pointer_and_loop`: Assigns a constant value to a global variable and loops until that variable matches a certain value.
- `execute_infinite_loop`: Performs an infinite loop that never exits.
- `initialize_global_parameters`: Sets several global or memory-mapped offsets or counters to zero or a fixed value.
- `update_pointer_and_counter_2`: Increments a global counter and updates a pointer or certain memory-mapped values based on a masked bit-field check.
- `initialize_global_pointer`: Sets a global pointer to a specific address and then executes a routine that updates a global pointer, increments a global counter, and conditionally copies a value from a global variable.
- `update_pointer_and_counter`: Updates a global pointer, increments a global counter, and conditionally copies a value from a global variable to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_constant_values`: Adds 0 to d0, 94 to d1, and 65514 to d2, then returns.
- `add_constant_values_2`: Adds 65464 to d0, 65520 to d1, and 65464 to d2, then returns.
- `add_constant_values_3`: Adds 65392 to d0, 24 to d1, and 136 to d2, then returns.
- `add_constant_values_4`: Adds 0 to d0, 188 to d1, and 65524 to d2, then returns.
- `add_constant_values_5`: Adds 65392 to d0, 65504 to d1, and 65392 to d2, then returns.
- `add_constant_values_6`: Adds 65320 to d0, 36 to d1, and 204 to d2, then returns.
- `add_constants_to_registers`: Adds 0 to d0, 282 to d1, and 65470 to d2 and returns.
- `add_constants_to_registers_2`: Adds 65320 to d0, 65520 to d1, and 65320 to d2 and returns.
- `add_constant_values_7`: Adds 65248 to d0, 48 to d1, and 272 to d2, then returns.
- `add_d1_d2_constants`: Adds 376 to register d1 and 65448 to register d2, then returns.
- `add_d0_d1_d2_constants`: Adds 65248 to register d0, 65536 to register d1, and 65248 to register d2, then returns.
- `add_d0_d1_d2_constants_2`: Adds 65144 to register d0, 60 to register d1, and 340 to register d2, then returns.
- `add_d1_d2_constants_2`: Adds 470 to register d1 and 65426 to register d2, then returns.
- `add_d0_d1_d2_constants_3`: Adds 65144 to register d0, 65456 to register d1, and 65144 to register d2, then returns.
- `add_d0_d1_d2_constants_4`: Adds 52528 to register d0, 120 to register d1, and 660 to register d2, then returns.
- `add_constants_to_registers_3`: Adds 0 to d0, 940 to d1, and 65316 to d2 and returns.
- `add_d0_d1_d2_constants_5`: Adds 52528 to register d0, 65376 to register d1, and 52528 to register d2, then returns.
- `subtract_d0_d1_d2_constants`: Subtracts 65464 from register d0, 12 from register d1, and 68 from register d2, then returns.
- `subtract_d0_d1_d2_constants_2`: Subtracts 0 from register d0, 94 from register d1, and 65514 from register d2, then returns.
- `subtract_specific_values`: Subtracts 49120 from d0, 49168 from d1, and 49144 from d2, then returns.
- `subtract_d0_d1_d2_constants_3`: Subtracts 65424 from register d0, 24 from register d1, and 136 from register d2, then returns.
- `subtract_from_d1_d2`: Subtracts 188 from register d1 and 292 from register d2, then returns.
- `subtract_from_d0_d1_d2`: Subtracts 248 from register d0, 224 from register d1, and 248 from register d2, then returns.
- `subtract_from_d0_d1_d2_alt`: Subtracts 40 from register d0, 36 from register d1, and 204 from register d2, then returns.
- `subtract_constant_values`: Subtracts 0 from d0, 282 from d1, and 49086 from d2, then returns.
- `subtract_different_values`: Subtracts 49448 from d0, 49648 from d1, and 49448 from d2, then returns.
- `subtract_from_d0_d1_d2_v2`: Subtracts 48 from register d0, 48 from register d1, and 272 from register d2, then returns.
- `subtract_from_d1_d2_alt`: Subtracts 376 from register d1 and 264 from register d2, then returns.
- `subtract_from_d0_d1_d2_v3`: Subtracts 48 from register d0, 192 from register d1, and 48 from register d2, then returns.
- `subtract_from_d0_d1_d2_v4`: Subtracts 152 from register d0, 60 from register d1, and 340 from register d2, then returns.
- `subtract_third_set`: Subtracts 0 from d0, 470 from d1, and 49266 from d2, then returns.
- `subtract_fourth_set`: Subtracts 48760 from d0, 49104 from d1, and 48760 from d2, then returns.
- `subtract_from_d0_d1_d2_v5`: Subtracts 120 from register d0, 120 from register d1, and 680 from register d2, then returns.
- `subtract_from_d1_d2_v2`: Subtracts 0 from register d0, 956 from register d1, and 16676 from register d2, then returns.
- `subtract_from_d0_d1_d2_v6`: Subtracts 120 from register d0, 256 from register d1, and 120 from register d2, then returns.
- `zero_registers`: Sets d0, d1, and d2 to zero and then returns.
- `load_data_and_branch`: Copies three words from an offset of a4 into d0, d1, and d2, then branches to another location.
- `load_data_and_branch_2`: Copies three words from an offset of a4 into d0, d1, and d2, then branches to another location.
- `load_registers_and_branch`: Loads three 16-bit words from an offset of 476 in a structure pointed to by a4 into registers d0 through d2, then performs a short branch to another routine.
- `load_registers_and_branch_2`: Loads three 16-bit words from an offset of 482 relative to a pointer in a4 into data registers d0 through d2, then performs a short branch to another location.
- `load_registers_and_branch_3`: Loads three 16-bit words from a memory location offset by a pointer in a4 into registers d0 through d2, then performs a short branch to another routine.
- `load_registers_and_branch_4`: Loads three 16-bit words from an offset of 494 relative to a pointer in a4 into data registers d0 through d2, then performs a short branch to another routine.
- `load_registers_and_branch_5`: Loads three 16-bit words from an offset of 499 relative to a pointer in a4 into registers d0 through d2, then performs a short branch to another location.
- `load_and_branch_data`: Loads three words from offset 506 of a structure in a4 into d0, d1, and d2, then branches to another routine.
- `load_and_branch_data_offset_512`: Loads three words from offset 512 of a structure in a4 into d0, d1, and d2, then branches to another routine.
- `load_and_branch_data_offset_518`: Loads three words from offset 518 of a structure in a4 into d0, d1, and d2, then branches to another routine.
- `perform_complex_math`: Performs a series of multiplications, subtractions, swaps, and additions using values from a table at offset 166, 168, 170, 172, 174, and 176 relative to a6, updating d1 and d2.
- `execute_triple_subroutine_calls`: Pushes registers to the stack, then performs three separate calls to a subroutine using different values from a table in a4 and the stack, before returning.
- `calculate_conditional_math`: Moves d5 to d0 and d6 to d1, then if d7 is not zero, it calls two different routines using d7 and d1, performs arithmetic with the results, and returns; otherwise it returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (addi.w) and then falls through into the code that follows, with no return of its own.
- `multiply_table_value`: Masks d1 with 511, doubles it, uses the result to fetch a word from a table relative to the program counter into d2, multiplies d0 by d2, doubles d2, and swaps its high and low words before returning.
- `process_table_and_constants`: Fetches several words from a table using offsets derived from a4, calls a routine, and then performs a long sequence of additions and subtractions using various constant values and register contents.
- `execute_sequence_of_math_operations`: Loads four values from a table using an index from a global variable, calls two subroutines, and then performs a long sequence of loading values from various memory locations and adding them to registers d3, d4, and d5.
- `clear_hunk_relocation_table`: Clears a large block of memory by writing zeros to a sequence of offsets within a structure pointed to by a value at an offset of 84 from a register, specifically targeting a range of indices starting from 7999 down to 7343.
- `interpolate_bitfield_coordinates`: Performs a linear interpolation between two coordinate points using a table of values and updates a set of boundary values in a structure.
- `interpolate_and_update_bitfield`: Performs linear interpolation between two points using a table of values and updates a bitfield buffer by clearing and setting bits at three memory offsets in a loop.
- `update_bitfield_buffer_from_table`: Calculates a memory address based on a table index and bit offsets, then enters a loop to clear specific bits at three memory offsets and set a bit at a fourth offset while iterating through a data stream.
- `clear_and_set_bits_loop`: Clears three bits at specific offsets from a pointer in a6 using d1, sets a bit at offset 24000 from a6, and loops using d3 before branching to another routine.
- `modify_bitfield_stream`: Copies four values from a source pointer to a destination, then enters a loop that clears a bit at a specific offset in three different memory locations and sets a bit at a different offset in a fourth location, while updating the pointer and bit index.
- `modify_bitfield_structure_bits`: Clears a bit at an offset from a pointer in a6, clears two additional bits at offsets 8000 and 16000 from that pointer, sets a bit at offset 24000 from the pointer, and then branches to another routine.
- `process_bitfield_conditional`: Copies four values from a source pointer to a destination and enters a loop that clears a bit at a specific offset in three memory locations and sets a bit in a fourth, while adjusting the pointer and bit index based on a conditional check.
- `clear_and_set_bits_loop_d2`: Clears three bits at specific offsets from a pointer in a6 using d1, sets a bit at offset 24000 from a6, and loops using d2 before branching to another routine.
- `manipulate_bit_stream`: Loops through a memory block, clearing and setting specific bits based on the value in d1, while updating a pointer and a counter.
- `clear_and_set_bits`: Clears three specific bits at offsets from a pointer in a6 using d1, sets a bit at an offset from a6, and then loops using d2 before restoring registers and returning.
- `copy_data_and_adjust_register`: Sets a 16-bit value in d6, checks the sign of d4, and if negative, negates d6, exchanges d3 with a3, loads a pointer from a nearby address, copies four values from a source pointer to a destination pointer, and subtracts a3 from d3.
- `clear_and_set_bits_with_offset`: Clears three bits at specific offsets from a pointer in a6 using d1, sets a bit at offset 24000 from a6, adds d6 to a6, and loops using d3 before branching to another routine.
- `swap_and_call_subroutines`: Loads values from a structure pointed to by a4, swaps them, stores them in a global variable and back into the structure, and then calls two different routines before returning.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (addq.b) and then falls through into the code that follows, with no return of its own.
- `tiny_ori_4`: Small routine of 1 instruction that merges 112 into d6.
- `tiny_ori_5`: Small routine of 1 instruction that merges 152 into d6.
- `tiny_ori_6`: Small routine of 1 instruction that merges 184 into d6.
- `perform_logical_or`: Performs a bitwise logical OR of zero onto register d7.
- `copy_string_if_not_ini`: Compares a longword at a specific memory address to a constant value; if they do not match, it copies a word from the source address to a destination address and then branches to another routine, otherwise it branches to a different routine.
- `manage_system_state_and_hardware`: Updates global counters and variables based on hardware register bits, clears DMA and specific global variables, initializes CIA registers from a table, and calls various subroutines including those for clearing memory blocks, swapping values, and executing conditional logic.
- `clear_global_bytes`: Calls another routine seven times and then loops five times to clear bytes in a global memory block.
- `clear_memory_block`: Decrements a word in d0 and uses it as a loop counter to clear bytes in a memory block pointed to by a0 until the counter reaches zero.
- `check_byte_and_branch`: Clears d0, loads a byte from a memory location into d0, and if the bit 0 is set, branches to another routine; otherwise it clears d0 and returns.
- `set_d0_to_ff`: Sets d0 to 255 and then returns.
- `process_joystick_input`: Reads the joystick port and CIAA fire button register to construct a 32-bit value based on bitwise patterns and specific bit checks, then stores the result in a global variable.
- `process_mouse_input`: Reads the mouse port and CIAA fire button register to construct a 32-bit value based on bitwise patterns and specific bit checks, then stores the result in a global variable.
- `update_mouse_movement_deltas`: Calculates absolute differences for two 16-bit input values, applies clamping logic based on a global state, and updates two memory locations with the resulting adjusted deltas.
- `compare_subtraction_results_2`: Subtracts two 16-bit values and compares the result with a third 16-bit value, then branches based on the comparison outcome.
- `calculate_input_delta_2`: Calculates a difference based on a value from the mouse port and a global variable, stores two resulting 16-bit values in a memory block pointed to by a0, and updates a global variable.
- `compare_subtraction_results`: Subtracts d1 from d0, then subtracts the result from d6, and branches based on the comparison of d6 and d4.
- `calculate_input_delta`: Subtracts two global 16-bit values from values in d4 and d7, stores the results in a memory block pointed to by a0, and sets two global variables to the value 32896 split into two bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `compare_and_branch`: Adds a long value from a memory location to d0, compares d0 to that same memory location, and branches if d0 is less than the value.
- `save_and_branch_pointer`: Pushes a0 to the stack, saves d0 through d7 and a0 through a6 to a structure, loads a new address into a0, and branches to another routine.
- `wait_for_byte`: Calls a routine at the address in a0 and loops until the byte at a global variable is 115, then restores a0 from the stack and returns.
- `set_color_register`: Writes the word value 3840 to the color register at a fixed address and returns.
- `reset_dma_and_restore_registers`: Clears a bit in a global variable, calls routines to clear relocation tables and swap subroutines, and clears DMA and registers.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `call_permit`: Wrapper that calls the exec library function Permit and returns its result to the caller.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_forbid`: Wrapper that calls the exec library function Forbid and returns its result to the caller.
- `initialize_cia_registers`: Writes 129 to a CIAA control register and copies two bytes from a table starting at a memory address into two different CIAA registers.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `open_resource_and_library`: Uses a value from a global variable to call a library function at an offset of -788, stores the result in a global variable, then uses that result to call another library function at an offset of -6 and returns.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `increment_global_counter`: Saves registers to the stack, checks a global variable, calls a routine at a fixed address, increments a global variable, and restores registers before returning.
- `update_global_counter`: Iterates through a loop four times, checking bits in a value derived from a global variable and incrementing that global variable if a bit is set.
- `update_interrupt_mask`: Calculates a bitmask by shifting a value in d2, writes it to the interrupt request register, increments d1 and d2, and then restores registers and returns from an interrupt.
- `iterate_channel_records`: Performs a loop that checks if a long word at the pointer in a0 is zero, otherwise it increments a0 by 16 and continues the loop until a counter in d7 reaches zero.
- `advance_channel_period_chain`: Updates a chain of four-long channel records in sequence, replacing each value and adjusting its accumulator by the difference, and uses a loop to iterate through the records.
- `clear_audio_volumes`: Clears two words at a fixed address, clears four words at a fixed address, a fixed address, a fixed address, and a fixed address, and writes 15 to the register at a fixed address before returning.
- `update_audio_state`: Increments a global counter, checks it against a limit, and if the limit is reached, increments another counter and potentially loops to update a pointer and a global variable based on values in a table.
- `load_audio_parameters`: Uses a value in d0 to calculate an offset into a table, then loads four-byte and two-byte values from that table into various global variables based on a selection value in d2.
- `update_audio_settings`: Parses a 16-bit value to determine a command type and a signed byte value, then updates various global variables or modifies specific bits in a global variable depending on the command.
- `update_audio_counters`: Increments four sets of global counters, checks if any counters are non-zero to load new data from a table and reset the pointer, and finally updates a DMA control register based on the status of those pointers.
- `update_global_counters`: Checks a bit in a custom register and, if set, increments two global longwords, copies a byte from a hardware register to a global longword, and enters a loop that decrements a global word until it reaches zero before checking a bit in a global longword.
- `process_complex_data`: Performs a sequence of operations including reading joystick or mouse input, updating audio state, comparing and updating a table, and processing a global data table through various subroutines.
- `compare_and_update_table`: Compares two sets of data in loops, modifying a global variable if nibbles do not match, and then copies a series of values from a table into several global variables.
- `process_global_data_table`: Iterates through a table starting at a global variable, decrementing values in a loop if specific bits are set, and then performs a series of memory writes to several global variables before returning a value.
- `parse_parameter_string`: Iterates through a loop that processes data from a structure, calls several routines to update a global variable and a 16-bit value, and modifies a character buffer.
- `parse_and_update_parameters`: Performs a loop that modifies a structure pointed to by a parameter, updates a global variable with a concatenated value, and calls another routine multiple times while performing arithmetic on values retrieved from a pointer.
- `store_pointer_to_global`: Stores the 32-bit value from register a0 into a global variable.
- `clear_dma_and_registers`: Clears two 32-bit global variables and sets the DMA control register to fifteen.
- `update_structure_elements`: If a value in d0 is non-zero, it calculates offsets to update a global variable and copies data from a memory location to a new position in a table.
- `setup_pointers_and_branch`: Moves the contents of d0 to a0, loads a long value from a memory location into a1, and branches to another routine.
- `fill_and_copy_table`: Fills a table starting at an offset of sixty-eight from a global pointer with incrementing 16-bit values and copies 16-bit words from a source pointer to a destination pointer in a loop that runs fifteen times.
- `call_subroutine_with_values`: Loads a long value from a memory location into a0, sets d2 to 4992, and calls another routine four times with different values in d1.
- `store_swapped_words`: Stores two 16-bit words from register d1 and the high word of register d0 into a destination pointed to by a0, increments d1 by two, and then stores the updated d1 and the low word of d0 into the next two positions at a0.
- `execute_conditional_logic_3`: Stores a value in a global variable, calls another routine, and then either calls another routine and performs additions or calls another routine and increments a global variable before returning.
- `copy_byte_from_string`: Loads two pointers from memory, reads a byte from the first pointer, and if the byte is not zero, writes it to the second pointer and branches to another location.
- `copy_string_and_call_function`: Copies a null-terminated string from a global pointer to a destination pointed to by a1, then calls a function from a table and stores the result in a global variable.
- `branch_and_set_value`: Branches to another routine and sets a 16-bit global variable to two hundred.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `branch_to_address`: Performs a single branch to another address.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `call_external_routine`: Calls a routine using three global variables as arguments and stores the return value in a global variable.
- `jump_to_address`: Loads three values from a table into registers d1, d2, and d3, then jumps to the address stored in a global variable.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_dual_parameter_structures_2`: Clears multiple word and long word fields in a structure pointed to by a4, performs conditional logic involving bit tests and comparisons, calls a routine, and populates several fields with constant values.
- `write_structure_data`: Writes nine word values to offsets of a structure pointed to by a4, then writes nine different word values to offsets of a second structure pointed to by a value at offset 72 of the first structure.
- `initialize_dual_parameter_structures`: Populates two separate memory structures with a series of constant word values and register contents.
- `apply_bitmask_loop_2`: Calculates an offset using two words from a table, accesses a structure at offset 84 of a4, and enters a loop that runs 16 times, performing bitwise operations and shifts on data from a table before applying them to the structure.
- `apply_bitmask_loop`: Loops through a list of words from a source pointer, performing bitwise AND and OR operations on a destination structure at offset 0, and applying bitwise AND to three specific offsets within that structure before incrementing the destination pointer by 20.
- `call_conditional_subroutine`: Tests bit 6 of a byte at offset 67 of a4; if the bit is set, it branches to another routine, otherwise it pushes d0 to the stack, calls another routine, and pops the value back into d0 before returning.
- `compare_long_word`: Loads a long word from a pointer in a0 into d1, compares it with a word in a2, and sets d4 based on the signed comparison result before returning.
- `store_register_to_pointer`: Stores the contents of a data register into the memory location pointed to by an address register and returns.
- `perform_complex_blitter_math`: Calls a routine twice, performs conditional checks, and if certain conditions are met, executes a loop that performs multiplication and division on swapped register values before calling another routine and returning.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `process_blitter_data`: Performs a loop to update values in a table, calls another routine four times with different data, and configures blitter hardware registers before executing a loop that performs bit manipulation and calls another routine.
- `configure_blt_hardware`: Sets various Blitter registers including color, address, and size based on values from a table and d1, and performs a loop that iterates up to four times to update hardware settings and DMA control.
- `update_buffer_table`: Stores a word into a memory location, performs a loop that calculates an offset into a table, compares words from two pointers in a5 and a6, and calls another routine while updating a table with swapped values.
- `compare_and_swap_buffers`: Performs a loop that pushes a word to a stack, then calculates an offset into a table, compares words from two pointers in a5 and a6, and conditionally swaps the pointers in a5 and a6 before calling another routine.
- `dispatch_blitter_logic`: Calculates an offset for a3, then checks a value from a global variable to decide whether to call one of three different routines, or perform a series of XOR and table lookup operations.
- `update_blitter_long_values`: Performs a loop that updates long words at specific offsets in a structure pointed to by a6 using values from d5 and d4, then branches to another routine.
- `process_word_xor_and_table_lookup`: Calculates a value by XORing a word from a table with a word from a global variable, performs a series of conditional XOR operations on a second routine's data, and performs a table lookup using values from a5 and a6.
- `update_blitter_word_values`: Performs a loop that updates words at specific offsets in a structure pointed to by a6 using values from d5 and d4, then branches to another routine.
- `process_word_xor_and_complex_lookup`: Calculates a value by XORing a word from a table with a word from a global variable, performs a series of conditional XOR operations on a second routine's data, and performs a complex lookup using values from a5 and a6 and a table in a0 and a1.
- `update_blitter_mixed_values`: Performs a loop that updates long and word values at specific offsets in a structure pointed to by a3 using values from d4, d5, d6, and d7, then branches to another routine.
- `xor_word_values`: Loads three pointers into a0, a1, and a2 from a table, then performs a bitwise XOR between two words retrieved from global variables and stores the result in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: The routine is empty and performs no operations before returning.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `return_from_subroutine`: The routine is empty and performs no operations before returning to the caller.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `return_no_op_stub`: The routine consists of a single instruction that returns immediately, with no register or hardware access and no library calls.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `return_no_op`: The routine consists of a single instruction that returns immediately, performing no work.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, so it only transfers control back to its caller.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `return_to_caller`: the routine is empty and simply returns control to the calling routine.
- `return_immediately_stub`: The routine consists of a single instruction that returns to its caller without performing any work, leaving registers and hardware state unchanged.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_18`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_19`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_20`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_21`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_22`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_23`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_24`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_25`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_26`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_via_pointer`: Performs a bitwise AND on a longword at the address in a3 using d4, and performs a bitwise OR on a longword at offset 7968 of the address in a3 using d5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_memory_with_registers`: Ors a longword at the address in a3 with d5 and increments a3, ors a word at the new address in a3 with d7 and increments a3, ors a longword at offset 7962 of the address in a3 with d5, and ands a word at offset 7966 of the address in a3 with d6.
