/* amigo_img.h: the kit's one image writer (Cowork, 18 Oct 2026, Karl approved "PNG + raw RGBA only").
 * Two formats replace the PPM/BMP mix:
 *   .rgba  raw RGBA for fast machine comparison. Layout: 12-byte header = the 4 bytes "RGBA", width (uint32 little-endian),
 *          height (uint32 little-endian); then width*height pixels, 4 bytes each in R, G, B, A order (A always 255), rows top-down.
 *   .png   for humans and storage (stb_image_write, already in the kit for reccap.c; 8-bit RGB, no alpha).
 *   .ppm   legacy P6, kept ONLY as the rollback format.
 * The switch: environment variable AMIGO_IMG = rgba | png | ppm picks the format of per-frame dumps (harness f%05d, port p%05d,
 * and png for pictures that are kept, e.g. settled). Unset = AMIGO_IMG_DEFAULT below (rgba). Rollback = AMIGO_IMG=ppm (or change the default and rebuild). Readers accept all three. */
#ifndef AMIGO_IMG_H
#define AMIGO_IMG_H
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#ifndef INCLUDE_STB_IMAGE_WRITE_H          /* main.c with SDL already has it via reccap.c */
#define STB_IMAGE_WRITE_IMPLEMENTATION
#define STB_IMAGE_WRITE_STATIC
#include "stb_image_write.h"
#endif
#ifndef AMIGO_IMG_DEFAULT
#define AMIGO_IMG_DEFAULT "rgba"
#endif
/* the dump format's file extension, without the dot: "rgba", "png" or "ppm" */
static const char *amigo_img_ext(void) {
    const char *e = getenv("AMIGO_IMG"); if (!e || !*e) e = AMIGO_IMG_DEFAULT;
    if (!strcmp(e, "png")) return "png";
    if (!strcmp(e, "ppm")) return "ppm";
    return "rgba";
}
/* extension for a picture that is KEPT or shown to people (settled, shots): png, or ppm when rolled back */
static const char *amigo_img_keep_ext(void) { return strcmp(amigo_img_ext(), "ppm") ? "png" : "ppm"; }
/* write px (0x00RRGGBB, w*h, top-down) to path; the format follows the path's extension (.png, .ppm, else .rgba) */
static int amigo_img_save(const char *path, const uint32_t *px, int w, int h) {
    size_t n = strlen(path); const char *x = n > 4 ? path + n - 4 : path;
    if (!strcmp(x, ".png")) {
        uint8_t *rgb = malloc((size_t)w * h * 3); if (!rgb) return 0;
        for (int i = 0; i < w * h; i++) { uint32_t c = px[i]; rgb[i * 3] = (uint8_t)(c >> 16); rgb[i * 3 + 1] = (uint8_t)(c >> 8); rgb[i * 3 + 2] = (uint8_t)c; }
        int ok = stbi_write_png(path, w, h, 3, rgb, w * 3); free(rgb); return ok;
    }
    FILE *f = fopen(path, "wb"); if (!f) return 0;
    if (!strcmp(x, ".ppm")) {
        fprintf(f, "P6\n%d %d\n255\n", w, h);
        for (int i = 0; i < w * h; i++) { uint32_t c = px[i]; fputc((c >> 16) & 255, f); fputc((c >> 8) & 255, f); fputc(c & 255, f); }
    } else {
        uint8_t hd[12] = {'R', 'G', 'B', 'A', (uint8_t)w, (uint8_t)(w >> 8), (uint8_t)(w >> 16), (uint8_t)(w >> 24),
                          (uint8_t)h, (uint8_t)(h >> 8), (uint8_t)(h >> 16), (uint8_t)(h >> 24)};
        fwrite(hd, 1, 12, f);
        uint8_t *b = malloc((size_t)w * h * 4);
        if (b) { for (int i = 0; i < w * h; i++) { uint32_t c = px[i]; b[i * 4] = (uint8_t)(c >> 16); b[i * 4 + 1] = (uint8_t)(c >> 8); b[i * 4 + 2] = (uint8_t)c; b[i * 4 + 3] = 255; }
                 fwrite(b, 1, (size_t)w * h * 4, f); free(b); }
    }
    fclose(f); return 1;
}
#endif
