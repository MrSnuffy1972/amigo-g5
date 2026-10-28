# Paula output stage (vendored, not reinvented)

Karl, 13 Oct 2026: use the proven open-source models of Paula's sound rather than writing our own.

From pt2-clone by Olav Sorensen (8bitbubsy), BSD 3-Clause (LICENSE-pt2-clone), fetched 13 Oct 2026 from
github.com/8bitbubsy/pt2-clone/src, unmodified:
- pt2_blep.c / .h: band-limited step synthesis (BLEP, by aciddose). Paula holds each sample until the next one at
  its own rate (zero-order hold); BLEP reproduces those steps at 44.1 kHz without the aliasing of point sampling.
- pt2_rcfilters.c / .h: the motherboard's 1-pole RC low-pass and high-pass filters and the 2-pole Sallen-Key "LED"
  filter, with component values from the A500 rev 6A and A1200 rev 1D4 schematics (in chipset.c).
pt2_header.h is our shim (ASSERT, PI). The register-level Paula model (DMA, periods, interrupts) stays in chipset.c:
the translated game drives Paula's registers directly, so that part has to behave like the chip.
Model: environment AMIGO_PAULA=a500 (default), a1200, or raw (no filters, no BLEP).
