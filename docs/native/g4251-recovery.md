# Game 4251 (liberator (1992)(scorpius software)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04251). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 128 of 128 named (125 author symbols), 128 of 128 described (41 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1923 of 1923 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4154 of 4154 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 308829 |
| code | 19202 |
| data | 187675 |
| padding | 1414 |
| unused | 384000 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Liberator (1992)(Scorpius Software).adf:bigcli | 1 | 41 | 1 |
| Liberator (1992)(Scorpius Software).adf:1 | 2 | 3117 | 102 |
| Liberator (1992)(Scorpius Software).adf:ending | 2 | 978 | 24 |
| Liberator (1992)(Scorpius Software).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Liberator (1992)(Scorpius Software).adf:bigcli

- `initialize_system`: Initializes system variables and hardware registers, sets up interrupt handlers, and performs memory management operations including library calls and task management.

### Liberator (1992)(Scorpius Software).adf:1

- `PROGRAM`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `INITIALISE`: Short code fragment that copies 0 to a fixed memory address, then copies 1 to a fixed memory address, then copies an address to a fixed memory address, then calls another routine, then copies 1920 to a fixed memory address, then calls another routine, then clears a fixed memory address, then returns.
- `OWNBLIT`: Wrapper that calls the graphics library function OwnBlitter and returns its result to the caller.
- `INITVARS`: Sets up game screen parameters, initializes various counters and flags, configures missile and bomb coordinates, and prepares game state variables for gameplay.
- `EXIT`: Short code fragment that calls another routine, then copies 32800 to a fixed memory address, then returns.
- `DISOWNBLIT`: Wrapper that calls the graphics library function DisownBlitter and returns its result to the caller.
- `MAIN`: Controls the main game loop including screen initialization, scrolling, input handling, and various game state updates.
- `CHEATER`: Short code fragment that tests bit 10 of a fixed memory address, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then compares a fixed memory address with 65, then branches if the last result was not zero, then copies 1 to a fixed memory address, then returns.
- `CHECKON`: Monitors specific sound flag values and triggers corresponding DMA channel enables based on conditional comparisons with predefined threshold values.
- `CHECKOFF`: Processes sound effect timers and interrupt flags for multiple audio channels, updating DMA control registers and interrupt request bits based on channel states.
- `PAUSE`: Checks for pause key input through CIA serial register, executes background sound functions when pause is detected, and manages joystick button states during pause.
- `TESTDEAD`: Checks collision flags to determine if a player has died, plays explosion sounds, updates visual effects, and manages game state variables when death conditions are met.
- `CLEAR`: Short code fragment that clears a fixed memory address (6 times in a row), then returns.
- `POINT`: Renders point-related graphics at multiple positions using a print function with varying X coordinates while maintaining consistent Y coordinates and graphic data pointers.
- `SETPTS`: Configures copper list instructions by writing word values to specific memory locations, setting up control registers and data words for hardware rendering operations.
- `SCOFF`: Reduces hit counters for multiple game objects and clears their corresponding hit flags when counters reach zero, managing object state transitions.
- `BOMOFF`: Short code fragment that copies a fixed memory address to a0, then clears the memory at a0, then clears a fixed memory address (3 times in a row), then returns.
- `BOMCHECK`: Validates bomb firing conditions based on hit status, position matching, and address checks before triggering sound effects and setting bomb coordinates.
- `MOVEBOM`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then adds 4 to a fixed memory address, then compares a fixed memory address with 310, then branches if the comparison was less, then clears a fixed memory address, then returns.
- `LANDED`: Manages landing sequence counters, handles player safety counts, processes joystick input for landing detection, and updates game flags based on landing conditions.
- `CHECKHIT`: Tests for collisions between missile and enemy objects using overlap detection functions, decrements hit counters, and triggers reset and off-missile routines when collisions occur.
- `RESETPT1`: Executes sound effects and clears bomb state, updates copper list with hit graphics data, displays hit points at specific coordinates, and sets hit counter to maximum value.
- `OFFMIS1`: Clears missile-related flags and coordinates when a missile is deactivated, executes sound effects, and updates hardware registers for missile state management.
- `OFFMIS2`: Clears missile position counters and resets a bit in a global flag variable, then calls two other routines.
- `RESETPT2`: Writes graphics data to a specific memory area, updates coordinates for a bad guy, and sets a hit flag.
- `RESETPT3`: Updates the hit status of a bad guy at offset 3 in a global table and calls a routine to print a point value.
- `RESETPT4`: Initializes hit point data for a specific enemy, updates display coordinates, and marks the enemy as hit.
- `OVERLAP`: Performs coordinate-based collision detection between two rectangular areas and returns a boolean result.
- `TESTLAND`: Checks if a player has landed, updates various game state variables, and triggers landing sounds when conditions are met.
- `RDWAVE`: Sets initial values for enemy counts, deltas, and Y positions, then selects and executes a wave pattern based on a random or sequential index.
- `SUBMOVES1`: Handles the reading and processing of wave data for sound generation, likely related to audio effects or music playback.
- `SUBMOVES2`: Updates the position of a bad guy based on delta values, decrements a counter, and sets new delta values from a data table when the counter reaches zero.
- `SUBMOVES3`: Updates the horizontal and vertical position of a game object using delta values stored in global variables, then modifies those deltas based on a control table entry read from memory.
- `SUBMOVES4`: Updates the x and y coordinates of a game object using delta values stored in global variables, then modifies the delta values based on data from a table depending on a control byte, and advances the table pointer.
- `MOVEBASE`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then compares a fixed memory address with 310, then branches if the comparison was greater or equal, then adds 1 to a fixed memory address (2 times in a row), then returns.
- `WAVE1`: Sets up initial positions, graphics addresses, and color values for multiple game objects in wave 1, then calls a random number generator.
- `WAVE4`: Configures the second wave of enemies with different positions, heights, and color settings compared to WAVE1.
- `OFF100`: Sets up the fourth wave of enemies with specific positions, heights, and color configurations.
- `WAVE2`: Initializes the fifth wave of enemies with distinct positions, heights, and color settings.
- `WAVE5`: Configures the eighth wave of enemies with specific positions, heights, and color configurations.
- `WAVE8`: Initializes the ninth wave of enemies with specific positions, heights, and color settings.
- `WAVE9`: Sets up the seventh wave of enemies with distinct positions, heights, and color configurations.
- `WAVE7`: Initializes the sixth wave of enemies with specific positions, heights, and color settings.
- `WAVE6`: Configures the third wave of enemies with different positions, heights, and color settings compared to other waves.
- `WAVE3`: Copies the SUPERX sprite to both MIS1ADDR and MIS2ADDR, and initializes its position in the COP_START table.
- `COPYSUPX`: Writes a sequence of words to memory locations starting at an address loaded from a global variable, with specific values and offsets.
- `COPYPLOB`: Sets mission pointers to specific data structures and initializes a sequence of words at a calculated memory address with predefined values.
- `MOVEMIS`: Initializes the scrolling mechanism for the game's background.
- `FIRECHECK`: Initializes various game variables and data structures needed for the game's operation, including setting up player and enemy states.
- `LASERSND`: Short code fragment that copies 0 to a fixed memory address, then copies 697 to a fixed memory address, then copies 856 to a fixed memory address, then copies 64 to a fixed memory address, then copies 1 to a fixed memory address, then copies 15 to a fixed memory address, then returns.
- `STARTSND`: Configures audio channel registers based on a flag value, sets up sound parameters for two audio channels, and enables DMA for audio output.
- `MANUPSND`: Short code fragment that copies 16412 to a fixed memory address, then copies 1417 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 8 to a fixed memory address, then copies 15 to a fixed memory address, then returns.
- `HIMFIRESND`: Short code fragment that copies 29756 to a fixed memory address, then copies 829 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 8 to a fixed memory address, then copies 10 to a fixed memory address, then returns.
- `STEPSSND`: Short code fragment that copies 22260 to a fixed memory address, then copies 1417 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 4 to a fixed memory address, then copies 20 to a fixed memory address, then returns.
- `BASESND`: Short code fragment that copies 26454 to a fixed memory address, then copies 1652 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 4 to a fixed memory address, then copies 20 to a fixed memory address, then returns.
- `HIMDEADSND`: Short code fragment that copies 31414 to a fixed memory address, then copies 2907 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 8 to a fixed memory address, then copies 25 to a fixed memory address, then returns.
- `LANDEDSND`: Short code fragment that copies 19246 to a fixed memory address, then copies 1508 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 8 to a fixed memory address, then copies 15 to a fixed memory address, then returns.
- `HIMHITSND`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies 4952 to a fixed memory address, then copies 2235 to a fixed memory address, then copies 214 to a fixed memory address, then copies 64 to a fixed memory address, then copies 4 to a fixed memory address, then copies 13 to a fixed memory address, then returns.
- `XSND`: Short code fragment that copies 1394 to a fixed memory address, then copies 1779 to a fixed memory address, then copies 428 to a fixed memory address, then copies 64 to a fixed memory address, then copies 1 to a fixed memory address, then copies 20 to a fixed memory address, then returns.
- `BGSND`: Short code fragment that copies 37228 to a fixed memory address, then copies 1896 to a fixed memory address, then copies 428 to a fixed memory address, then copies 18 to a fixed memory address, then copies 32770 to a fixed memory address, then returns.
- `ARMSND`: Short code fragment that copies 9422 to a fixed memory address, then copies 3496 to a fixed memory address, then copies 214 to a fixed memory address, then copies 64 to a fixed memory address, then copies 1 to a fixed memory address, then copies 20 to a fixed memory address, then returns.
- `PRINTYOU`: Calls another routine twice with different coordinates and memory addresses to update display information for player position.
- `PRINTBOM`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then copies 5 to d2, then copies a fixed memory address to a0, then calls another routine, then returns.
- `PRINTBADDY`: Calls another routine four times with enemy-specific coordinates and memory addresses to update display information for multiple enemies.
- `PRINTMIS`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then copies 14 to d2, then copies a fixed memory address to a0, then calls another routine, then copies a fixed memory address to d0, then copies a fixed memory address to d1, then copies a fixed memory address to a0, then calls another routine, then returns.
- `JOYCHECK`: Reads joystick input from a hardware register, updates position variables based on button states, and enforces boundary limits on the coordinates.
- `SCROLLINIT`: Initializes the scrolling process by setting up initial scroll parameters, calling initialization routines, and managing the scrolling animation loop with interrupt handling.
- `FIRETEST`: Short code fragment that adds 1 to a fixed memory address, then compares a fixed memory address with 127, then branches if the last result was zero, then tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns, then copies 1 to a fixed memory address (2 times in a row), then returns.
- `RANDOM`: Short code fragment that clears d1, then clears d0, then copies 17231 to a0, then copies a fixed memory address to d1, then adds 1 to a fixed memory address, then copies an indexed entry from a0 to d0, then copies d0 to d1, then adds 150 to d1, then returns.
- `SPRON`: Short code fragment that copies 32800 to a fixed memory address, then returns.
- `SPROFF`: Short code fragment that copies 32 to a fixed memory address, then returns.
- `PRINTPOINT`: Short code fragment that copies a0 to a1, then adds 1 to a0, then calls another routine (2 times in a row), then returns.
- `HORZSHOW`: Short code fragment that shifts d0 right by 1, then branches if the carry flag is set, then copies d0 to the memory at a0, then adds 2 to a0, then clears bit 0 of the memory at a0, then returns, then copies d0 to the memory at a0, then adds 2 to a0, then sets bit 0 of the memory at a0, then returns.
- `CHY`: Writes a byte value to memory, modifies specific bits in adjacent memory locations based on comparison results, and updates another memory location with a calculated value.
- `TITLESCREEN`: Manages the title screen display, handles input from keyboard and joystick, and controls transitions to other screens.
- `WIPE`: Clears memory regions in a screen buffer by writing zero bytes, calls a delay routine, and repeats this process multiple times to create a wiping effect.
- `CLS`: Short code fragment that copies 51200 to d0, then copies a fixed memory address to a0, then copies 0 to the memory at a0 (advancing a0), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `SCROLL`: Copies color table data to the appropriate memory locations for use in the game's graphics rendering.
- `REPOINT`: Writes a series of words to a copper list structure, updates memory addresses with calculated values, and increments counters in a loop to configure display parameters.
- `PRINTLINES`: Renders character lines using bitplane data and updates display pointers, handling two line segments with repeated blitting operations.
- `OPENGFX`: Short code fragment that copies a fixed memory address to a6, then copies 17184 to a1, then clears d0, then calls another routine, then copies d0 to a fixed memory address, then returns.
- `CLOSEGFX`: Wrapper that calls the graphics library function FreeSprite and returns its result to the caller.
- `INITSCROLL`: Allocates memory for bitmaps and copper lists, populates the copper list with display parameters, and configures hardware registers for scrolling display.
- `EXITSCROLL`: Releases allocated memory for bitmaps and copper lists, restores graphics library handles, and disables DMA channels.
- `INITBITMAP`: Allocates memory for bitmap and copper lists, populates the copper list with display parameters, and configures hardware registers for bitmap display.
- `EXITBITMAP`: Releases allocated memory for bitmap and copper lists, restores graphics library handles, and disables DMA channels.
- `COPYSCREEN`: Copies pixel data from a source screen buffer to a destination bitmap using multiple pointers and loop iterations.
- `COPYCOLS`: Short code fragment that copies an address to a0, then copies 32 to d0, then copies the memory at a1 to the memory at a0, then adds 2 to a0, then adds 2 to a1, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `DELAY`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then returns.
- `MT_INIT`: Initializes music playback by setting up song data pointers, sample start addresses, and configuring audio hardware registers.
- `MT_END`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `MT_MUSIC`: Controls the music playback loop by incrementing a counter, checking conditions, and calling other routines.
- `MT_NONEWALLCHANNELS`: Processes four audio channels for music playback by checking effects and playing voices, then sets up DMA control for audio output.
- `MT_PLAYVOICE`: Initializes voice parameters from sample data and sets up audio registers for playback.
- `MT_CHECKEFX`: Reads an effect command from a data structure, processes it based on its value, and updates channel parameters accordingly.
- `MT_PERNOP`: Performs the scrolling operation by updating the display registers and handling interrupt requests for smooth scrolling animation.
- `MT_SETTONEPORTA`: Sets up tone portamento effects for music playback, calculating and applying pitch changes over time to create smooth transitions between notes.
- `MT_TONEPORTNOCHANGE`: Updates the pointers to the current scroll data, ensuring that the correct portions of the background are displayed during scrolling.
- `MT_VIBRATO2`: Implements vibrato effect for music playback by modulating the pitch of a note using a predefined vibrato table and parameters.
- `MT_CHECKMOREEFX`: Processes extended effect commands from a pattern row, updating various playback parameters including song position, volume, and instrument settings based on command values.
- `MT_SETFINETUNE`: Sets fine-tuning parameters for music playback, likely adjusting pitch or volume with precise control over musical notes.
- `MT_UPDATEFUNK`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### Liberator (1992)(Scorpius Software).adf:ending

- `PROGRAM`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `INITIALISE`: Call stub: calls INITBITMAP, then returns to its own caller.
- `EXIT`: Call stub: calls EXITBITMAP, then returns to its own caller.
- `MAIN`: Short code fragment that calls another routine (2 times in a row), then copies a fixed memory address to d0, then masks d0 with an address, then compares d0 with 73728, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `call_mt_end`: Call stub: calls MT_END, then returns to its own caller.
- `COPYSCREEN`: Copies pixel data from five source screens to a destination screen, advancing through memory in steps of 10240 bytes, while decrementing a counter until zero.
- `MT_INIT`: Initializes music playback by setting up song data pointers, sample start addresses, and configuring audio hardware registers.
- `MT_END`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `MT_MUSIC`: Controls the ending music playback loop by incrementing a counter, checking conditions, and calling other routines.
- `MT_NONEWALLCHANNELS`: Processes all four audio channels for music playback by checking effects and playing voices, then sets up DMA control for audio output.
- `MT_PLAYVOICE`: Initializes voice parameters from music data, sets up sample pointers and period values, and configures audio registers for playback.
- `MT_CHECKEFX`: Reads an effect command from a data structure, processes it based on its value, and updates tone parameters accordingly.
- `MT_PERNOP`: Processes a note operation based on a counter value and stores the result period in a global variable.
- `MT_SETTONEPORTA`: Calculates and sets tone portamento parameters by finding the closest period value in a table.
- `MT_TONEPORTNOCHANGE`: Updates tone portamento values and handles glissando effects by checking against period table values.
- `MT_VIBRATO2`: Applies vibrato effect to the current note by calculating a modulation value from a table and adding it to the period.
- `MT_CHECKMOREEFX`: Processes extended effect commands from a pattern row, updating various playback parameters including song position, volume, and instrument settings based on command values.
- `MT_SETFINETUNE`: Reads a byte from offset 3 of the structure in a6, masks its lower nibble, and stores the result at offset 18 of the same structure.
- `MT_UPDATEFUNK`: Reads a nibble from a structure byte, uses it as an index into a global table, adds the result to another byte in the same structure, and performs conditional updates based on bit testing.
- `DELAY`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then returns.
- `CLS`: Short code fragment that copies 51200 to d0, then copies a fixed memory address to a0, then copies 0 to the memory at a0 (advancing a0), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `COPYCOLS`: Short code fragment that copies an address to a0, then copies 32 to d0, then copies the memory at a1 to the memory at a0, then adds 2 to a0, then adds 2 to a1, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `INITBITMAP`: Opens graphics library, allocates memory for bitmap and copper lists, populates the copper list with display parameters, and configures hardware registers for bitmap display.
- `EXITBITMAP`: Disables DMA channels, opens the graphics library, and frees memory blocks for bitmap and copper list data before closing the library.

### Liberator (1992)(Scorpius Software).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
