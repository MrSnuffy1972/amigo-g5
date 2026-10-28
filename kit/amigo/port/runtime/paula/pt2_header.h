/* Shim for the files vendored from 8bitbubsy's pt2-clone (BSD 3-Clause, see LICENSE-pt2-clone): they include
 * pt2_header.h only for ASSERT() and PI. Ami-Go, 13 Oct 2026. */
#pragma once
#define ASSERT(x) ((void)0)
#ifndef PI
#define PI 3.14159265358979323846
#endif
