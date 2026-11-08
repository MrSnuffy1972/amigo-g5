# Game 896 (microprose golf (1991)(microprose)[cr flt][a]): code recovery summary

## Functions (part 3 of 3)

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a].adf:Install_to_DH0
(continued)

- `initialize_installation_environment_2`: Opens required libraries, locates a specific task, and forbids execution to manage message passing.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
