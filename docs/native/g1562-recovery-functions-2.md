# Game 1562 (monkey island 2 - le chuck's revenge (1992)(lucasarts)[cr conet](disk 01 of 11)[a]): code recovery summary

## Functions (part 2 of 2)

### Monkey Island 2 - Le Chuck's Revenge (1992)(LucasArts)[cr Conet](Disk 01 of 11)[a].adf:CRMonkey2
(continued)

- `copy_code_to_low_memory`: Copies a 48-byte block of code from the routine's own address into low memory at address 48, then returns zero.
- `conditionally_copy_stack_word`: Compares a 16-bit value in d0 to 9999; if they are not equal, it returns, otherwise it copies a 16-bit word from the stack into d0 and returns.

### Monkey Island 2 - Le Chuck's Revenge (1992)(LucasArts)[cr Conet](Disk 01 of 11)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
