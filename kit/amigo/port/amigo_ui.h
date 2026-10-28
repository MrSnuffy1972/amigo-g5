/* amigo_ui.h - the drawing layer for the Ami-Go launcher (pre-start menu), the on-screen Amiga keyboard and the capture
 * indicator (Cowork 17 Oct 2026; included by menu.c, SDL builds only).
 *
 * Everything is drawn with SDL_RenderGeometry (SDL 2.0.18 or later; the kit uses 2.30.9): anti-aliased rounded rectangles,
 * circles and lines (each edge has a feathered one-pixel fringe), gradients from vertex colours, and text from a glyph atlas
 * baked at start-up by stb_truetype (public domain, Sean Barrett) from the embedded Inter typeface (SIL Open Font License 1.1,
 * see NOTICE-frontend.txt). The tools image and the target devices need no system fonts. The atlas is baked for the layout
 * scale in use (1.0 = 1080 lines, 2.0 = 2160 lines), so text is drawn at its own pixel size, never stretched. */
#ifndef AMIGO_UI_H
#define AMIGO_UI_H
#include <math.h>
#define STB_TRUETYPE_IMPLEMENTATION
#define STBTT_STATIC
#include "stb_truetype.h"
#include "amigo_font.h"     /* amigo_font_body[] (Inter Medium) and amigo_font_bold[] (Inter Display Bold), subset to Latin-1 */

#define UI_PI 3.14159265358979f
enum { F_TITLE, F_H1, F_H2, F_BODY, F_SMALL, F_TINY, UI_NFONTS };
static const float ui_fbase[UI_NFONTS] = {92, 46, 32, 30, 24, 20};      /* pixel sizes at 1080 lines */
static const int ui_fbold[UI_NFONTS] = {1, 1, 1, 0, 0, 1};
static const int ui_extra_cp[] = {0xA9, 0xB7, 0xD7, 0x2022, 0x2026, 0x2039, 0x203A, 0x2190, 0x2191, 0x2192, 0x2193, 0x21B5};
#define UI_NEXTRA ((int)(sizeof ui_extra_cp / sizeof ui_extra_cp[0]))
#define UI_NCP (95 + UI_NEXTRA)
typedef struct { stbtt_packedchar pc[UI_NCP]; float ascent; } ui_font_t;
static ui_font_t ui_f[UI_NFONTS];
static SDL_Renderer *ui_r;
static SDL_Texture *ui_atlas, *ui_blob, *ui_tex;
static int ui_aw, ui_ah;
static float ui_baked;                  /* layout scale the atlas was baked for (0 = none yet) */
static float ui_u = 1, ui_px = 1, ui_alpha = 1, ui_ox, ui_oy;   /* layout scale, logical units per output pixel, global alpha, offset */
static int ui_measure;                  /* 1 = glyph and keycap calls only return their width */
static int ui_padstyle;                 /* 0 = Xbox letters, 1 = PlayStation shapes */

static int ui_cp_index(int cp) {
    if (cp >= 32 && cp < 127) return cp - 32;
    for (int i = 0; i < UI_NEXTRA; i++) if (ui_extra_cp[i] == cp) return 95 + i;
    return '?' - 32;
}
static void ui_init(SDL_Renderer *r, float u) {
    ui_r = r;
    if (u < 0.5f) u = 0.5f;
    if (u > 4.0f) u = 4.0f;
    if (ui_atlas && ui_baked >= u * 0.98f) return;
    Uint32 t0 = SDL_GetTicks();
    stbtt_fontinfo fi[2];
    const unsigned char *fd[2] = {amigo_font_body, amigo_font_bold};
    for (int i = 0; i < 2; i++) if (!stbtt_InitFont(&fi[i], fd[i], stbtt_GetFontOffsetForIndex(fd[i], 0))) { fprintf(stderr, "ui: embedded font %d unreadable\n", i); return; }
    static int cps[UI_NCP];
    for (int i = 0; i < 95; i++) cps[i] = 32 + i;
    for (int i = 0; i < UI_NEXTRA; i++) cps[95 + i] = ui_extra_cp[i];
    double area = 0;
    for (int f = 0; f < UI_NFONTS; f++) { double s = ui_fbase[f] * u; area += s * s * 0.62 * UI_NCP * (s < 40 ? 2 : 1); }
    area *= 1.25;
    int maxt = 4096; SDL_RendererInfo info;
    if (SDL_GetRendererInfo(r, &info) == 0 && info.max_texture_width > 0 && info.max_texture_height > 0)
        maxt = info.max_texture_width < info.max_texture_height ? info.max_texture_width : info.max_texture_height;
    static const int dims[][2] = {{1024, 1024}, {2048, 1024}, {2048, 2048}, {4096, 2048}, {4096, 4096}};
    unsigned char *px = NULL; int ok = 0, aw = 0, ah = 0;
    for (int d = 0; d < 5 && !ok; d++) {
        if (dims[d][0] > maxt || dims[d][1] > maxt) break;
        if ((double)dims[d][0] * dims[d][1] < area && d < 4) continue;
        aw = dims[d][0]; ah = dims[d][1];
        unsigned char *np = realloc(px, (size_t)aw * ah); if (!np) break; px = np;
        stbtt_pack_context pk;
        if (!stbtt_PackBegin(&pk, px, aw, ah, 0, 1, NULL)) continue;
        ok = 1;
        for (int f = 0; f < UI_NFONTS; f++) {
            float s = ui_fbase[f] * u; int b = ui_fbold[f];
            stbtt_PackSetOversampling(&pk, s < 40 ? 2 : 1, 1);
            stbtt_pack_range rg; memset(&rg, 0, sizeof rg);
            rg.font_size = s; rg.array_of_unicode_codepoints = cps; rg.num_chars = UI_NCP; rg.chardata_for_range = ui_f[f].pc;
            if (!stbtt_PackFontRanges(&pk, fd[b], 0, &rg, 1)) ok = 0;
            int asc, desc, gap; stbtt_GetFontVMetrics(&fi[b], &asc, &desc, &gap);
            ui_f[f].ascent = asc * stbtt_ScaleForPixelHeight(&fi[b], s);
        }
        stbtt_PackEnd(&pk);
    }
    if (!px || !aw) { fprintf(stderr, "ui: no glyph atlas (max texture %d)\n", maxt); free(px); return; }
    uint32_t *argb = malloc((size_t)aw * ah * 4);
    if (!argb) { free(px); return; }
    for (size_t i = 0; i < (size_t)aw * ah; i++) argb[i] = ((uint32_t)px[i] << 24) | 0x00FFFFFFu;
    if (ui_atlas) SDL_DestroyTexture(ui_atlas);
    ui_atlas = SDL_CreateTexture(r, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC, aw, ah);
    if (ui_atlas) {
        SDL_UpdateTexture(ui_atlas, NULL, argb, aw * 4);
        SDL_SetTextureBlendMode(ui_atlas, SDL_BLENDMODE_BLEND); SDL_SetTextureScaleMode(ui_atlas, SDL_ScaleModeLinear);
    }
    free(argb); free(px);
    ui_aw = aw; ui_ah = ah; ui_baked = ui_atlas ? u : 0;
    if (!ui_blob) {   /* a soft round light, for the animated background */
        static uint32_t bp[128 * 128];
        for (int y = 0; y < 128; y++) for (int x = 0; x < 128; x++) {
            float dx = (x - 63.5f) / 64.0f, dy = (y - 63.5f) / 64.0f, d = sqrtf(dx * dx + dy * dy), a = d < 1 ? (1 - d) * (1 - d) * (1 - d * 0.5f) : 0;
            bp[y * 128 + x] = ((uint32_t)(a * 255.0f) << 24) | 0x00FFFFFFu;
        }
        ui_blob = SDL_CreateTexture(r, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC, 128, 128);
        if (ui_blob) { SDL_UpdateTexture(ui_blob, NULL, bp, 128 * 4); SDL_SetTextureScaleMode(ui_blob, SDL_ScaleModeLinear); }
    }
    fprintf(stderr, "ui: glyph atlas %dx%d (scale %.2f%s) baked in %u ms\n", aw, ah, u, ok ? "" : ", incomplete", SDL_GetTicks() - t0);
}

/* ------------------------------------------------------------------ colours (0xAARRGGBB) */
static SDL_Color ui_c(uint32_t c) {
    float a = (float)(c >> 24) * (ui_alpha < 0 ? 0 : ui_alpha > 1 ? 1 : ui_alpha);
    SDL_Color k = {(Uint8)(c >> 16), (Uint8)(c >> 8), (Uint8)c, (Uint8)(a + 0.5f)}; return k;
}
static SDL_Color ui_c0(uint32_t c) { SDL_Color k = ui_c(c); k.a = 0; return k; }
static uint32_t ui_mix(uint32_t a, uint32_t b, float t) {
    if (t <= 0) return a; if (t >= 1) return b;
    uint32_t r = 0;
    for (int s = 0; s < 32; s += 8) { float x = (float)((a >> s) & 255), y = (float)((b >> s) & 255); r |= (uint32_t)(x + (y - x) * t + 0.5f) << s; }
    return r;
}
static uint32_t ui_a(uint32_t c, float a) { if (a < 0) a = 0; if (a > 1) a = 1; return (c & 0x00FFFFFFu) | ((uint32_t)((c >> 24) * a + 0.5f) << 24); }

/* ------------------------------------------------------------------ geometry batch */
#define UI_MAXV 6144
static SDL_Vertex ui_v[UI_MAXV]; static int ui_ix[UI_MAXV * 3]; static int ui_nv, ui_ni;
static void ui_flush(void) {
    if (ui_ni && ui_r) { SDL_SetRenderDrawBlendMode(ui_r, SDL_BLENDMODE_BLEND); SDL_RenderGeometry(ui_r, ui_tex, ui_v, ui_nv, ui_ix, ui_ni); }
    ui_nv = ui_ni = 0;
}
static void ui_begin(SDL_Texture *t, int nv, int ni) {
    if (t != ui_tex || ui_nv + nv > UI_MAXV || ui_ni + ni > UI_MAXV * 3) { ui_flush(); ui_tex = t; }
}
static int ui_vt(float x, float y, SDL_Color c, float s, float t) {
    SDL_Vertex *v = &ui_v[ui_nv]; v->position.x = x + ui_ox; v->position.y = y + ui_oy; v->color = c; v->tex_coord.x = s; v->tex_coord.y = t; return ui_nv++;
}
static void ui_tri(int a, int b, int c) { ui_ix[ui_ni++] = a; ui_ix[ui_ni++] = b; ui_ix[ui_ni++] = c; }
static void ui_clip(const SDL_Rect *r) {
    ui_flush();
    if (r) { SDL_Rect k = *r; k.x += (int)ui_ox; k.y += (int)ui_oy; SDL_RenderSetClipRect(ui_r, &k); } else SDL_RenderSetClipRect(ui_r, NULL);
}

/* a rounded rectangle outline at distance d outside the edge (negative = inside): points, outward normals; returns the count */
static int ui_ring(float x, float y, float w, float h, float r, float d, float *px, float *py, float *nx, float *ny) {
    if (r > w / 2) r = w / 2; if (r > h / 2) r = h / 2; if (r < 0.5f) r = 0.5f;
    int n = (int)(r / (3.0f * ui_px)); if (n < 2) n = 2; if (n > 14) n = 14;
    const float cx[4] = {x + w - r, x + w - r, x + r, x + r}, cy[4] = {y + r, y + h - r, y + h - r, y + r};
    int k = 0;
    for (int c = 0; c < 4; c++) for (int i = 0; i <= n; i++) {
        float a = (c - 1) * UI_PI / 2 + i * (UI_PI / 2) / n, cs = cosf(a), sn = sinf(a), rr = r + d;
        if (rr < 0) rr = 0;
        px[k] = cx[c] + cs * rr; py[k] = cy[c] + sn * rr; nx[k] = cs; ny[k] = sn; k++;
    }
    return k;
}
/* filled rounded rectangle, vertical gradient from ct (top) to cb (bottom) */
static void ui_rrect(float x, float y, float w, float h, float r, uint32_t ct, uint32_t cb) {
    if (w <= 0 || h <= 0 || !ui_r) return;
    float px[64], py[64], nx[64], ny[64]; int n = ui_ring(x, y, w, h, r, 0, px, py, nx, ny);
    float f = ui_px * 1.2f;
    ui_begin(NULL, 1 + 2 * n, 9 * n);
    int c0 = ui_vt(x + w / 2, y + h / 2, ui_c(ui_mix(ct, cb, 0.5f)), 0, 0), b = ui_nv;
    for (int i = 0; i < n; i++) ui_vt(px[i], py[i], ui_c(ui_mix(ct, cb, (py[i] - y) / h)), 0, 0);
    int o = ui_nv;
    for (int i = 0; i < n; i++) ui_vt(px[i] + nx[i] * f, py[i] + ny[i] * f, ui_c0(ui_mix(ct, cb, (py[i] - y) / h)), 0, 0);
    for (int i = 0; i < n; i++) { int j = (i + 1) % n; ui_tri(c0, b + i, b + j); ui_tri(b + i, o + i, o + j); ui_tri(b + i, o + j, b + j); }
}
/* rounded rectangle outline, t thick, drawn inside the edge */
static void ui_rrect_line(float x, float y, float w, float h, float r, float t, uint32_t c) {
    if (w <= 0 || h <= 0 || !ui_r) return;
    float px[64], py[64], nx[64], ny[64], qx[64], qy[64], mx[64], my[64];
    int n = ui_ring(x, y, w, h, r, 0, px, py, nx, ny); ui_ring(x, y, w, h, r, -t, qx, qy, mx, my);
    float f = ui_px * 1.2f; SDL_Color s = ui_c(c), z = ui_c0(c);
    ui_begin(NULL, 4 * n, 18 * n);
    int A = ui_nv; for (int i = 0; i < n; i++) ui_vt(px[i], py[i], s, 0, 0);
    int B = ui_nv; for (int i = 0; i < n; i++) ui_vt(qx[i], qy[i], s, 0, 0);
    int C = ui_nv; for (int i = 0; i < n; i++) ui_vt(px[i] + nx[i] * f, py[i] + ny[i] * f, z, 0, 0);
    int D = ui_nv; for (int i = 0; i < n; i++) ui_vt(qx[i] - nx[i] * f, qy[i] - ny[i] * f, z, 0, 0);
    for (int i = 0; i < n; i++) { int j = (i + 1) % n;
        ui_tri(A + i, B + i, B + j); ui_tri(A + i, B + j, A + j);
        ui_tri(A + i, C + i, C + j); ui_tri(A + i, C + j, A + j);
        ui_tri(B + i, D + i, D + j); ui_tri(B + i, D + j, B + j); }
}
static void ui_circle(float cx, float cy, float r, uint32_t c) { ui_rrect(cx - r, cy - r, 2 * r, 2 * r, r, c, c); }
static void ui_ring_line(float cx, float cy, float r, float t, uint32_t c) { ui_rrect_line(cx - r, cy - r, 2 * r, 2 * r, r, t, c); }
static void ui_line(float x0, float y0, float x1, float y1, float t, uint32_t c) {
    float dx = x1 - x0, dy = y1 - y0, l = sqrtf(dx * dx + dy * dy); if (l < 0.01f || !ui_r) return;
    float ux = -dy / l, uy = dx / l, h = t / 2, f = ui_px * 1.2f; SDL_Color s = ui_c(c), z = ui_c0(c);
    ui_begin(NULL, 8, 18);
    int a0 = ui_vt(x0 + ux * h, y0 + uy * h, s, 0, 0), a1 = ui_vt(x1 + ux * h, y1 + uy * h, s, 0, 0);
    int b0 = ui_vt(x0 - ux * h, y0 - uy * h, s, 0, 0), b1 = ui_vt(x1 - ux * h, y1 - uy * h, s, 0, 0);
    int c0 = ui_vt(x0 + ux * (h + f), y0 + uy * (h + f), z, 0, 0), c1 = ui_vt(x1 + ux * (h + f), y1 + uy * (h + f), z, 0, 0);
    int d0 = ui_vt(x0 - ux * (h + f), y0 - uy * (h + f), z, 0, 0), d1 = ui_vt(x1 - ux * (h + f), y1 - uy * (h + f), z, 0, 0);
    ui_tri(a0, a1, b1); ui_tri(a0, b1, b0); ui_tri(c0, c1, a1); ui_tri(c0, a1, a0); ui_tri(b0, b1, d1); ui_tri(b0, d1, d0);
}
/* a rectangle with a colour at each corner (top-left, top-right, bottom-right, bottom-left) */
static void ui_quad4(float x, float y, float w, float h, uint32_t tl, uint32_t tr, uint32_t br, uint32_t bl) {
    if (!ui_r) return;
    ui_begin(NULL, 4, 6);
    int a = ui_vt(x, y, ui_c(tl), 0, 0), b = ui_vt(x + w, y, ui_c(tr), 0, 0), c = ui_vt(x + w, y + h, ui_c(br), 0, 0), d = ui_vt(x, y + h, ui_c(bl), 0, 0);
    ui_tri(a, b, c); ui_tri(a, c, d);
}
static void ui_tri3(float x0, float y0, float x1, float y1, float x2, float y2, uint32_t c) {
    if (!ui_r) return;
    ui_begin(NULL, 3, 3); SDL_Color k = ui_c(c);
    int a = ui_vt(x0, y0, k, 0, 0), b = ui_vt(x1, y1, k, 0, 0), d = ui_vt(x2, y2, k, 0, 0);
    ui_tri(a, b, d);
}
/* a soft round light (additive or blended) */
static void ui_soft(float cx, float cy, float rad, uint32_t c, int additive) {
    ui_flush(); if (!ui_blob) return;
    SDL_SetTextureColorMod(ui_blob, (Uint8)(c >> 16), (Uint8)(c >> 8), (Uint8)c);
    SDL_SetTextureAlphaMod(ui_blob, ui_c(c).a);
    SDL_SetTextureBlendMode(ui_blob, additive ? SDL_BLENDMODE_ADD : SDL_BLENDMODE_BLEND);
    SDL_FRect d = {cx - rad + ui_ox, cy - rad + ui_oy, rad * 2, rad * 2};
    SDL_RenderCopyF(ui_r, ui_blob, NULL, &d);
}

/* ------------------------------------------------------------------ text */
static int ui_utf8(const char **s) {
    const unsigned char *p = (const unsigned char *)*s; int c = *p++;
    if (c >= 0xF0 && p[0] && p[1] && p[2]) { c = ((c & 7) << 18) | ((p[0] & 63) << 12) | ((p[1] & 63) << 6) | (p[2] & 63); p += 3; }
    else if (c >= 0xE0 && p[0] && p[1]) { c = ((c & 15) << 12) | ((p[0] & 63) << 6) | (p[1] & 63); p += 2; }
    else if (c >= 0xC0 && p[0]) { c = ((c & 31) << 6) | (p[0] & 63); p += 1; }
    *s = (const char *)p; return c;
}
static float ui_fpx(int f, float sc) { return ui_fbase[f] * ui_u * sc; }       /* the font's pixel height in layout units */
static float ui_text_w(int f, const char *s, float sc, float sp) {
    if (!s) return 0;
    if (!ui_atlas) return (float)strlen(s) * ui_fpx(f, sc) * 0.5f;
    float k = ui_u * sc / ui_baked, w = 0;
    while (*s) { int cp = ui_utf8(&s); w += ui_f[f].pc[ui_cp_index(cp)].xadvance * k + sp * ui_u; }
    return w;
}
/* text with its line top at y; align 0 left, 1 centre, 2 right; sc scales the font, sp adds letter spacing; returns the width */
static float ui_text(int f, float x, float y, const char *s, uint32_t col, float sc, int align, float sp) {
    if (!ui_atlas || !s || !ui_r) return 0;
    float k = ui_u * sc / ui_baked, w = ui_text_w(f, s, sc, sp);
    if (align == 1) x -= w / 2; else if (align == 2) x -= w;
    float pen = x, base = y + ui_f[f].ascent * k;
    if (k > 0.99f && k < 1.01f) base = floorf(base + 0.5f);
    SDL_Color c = ui_c(col);
    while (*s) {
        int cp = ui_utf8(&s), gi = ui_cp_index(cp);
        if (cp != ' ') {
            stbtt_aligned_quad q; float qx = 0, qy = 0;
            stbtt_GetPackedQuad(ui_f[f].pc, ui_aw, ui_ah, gi, &qx, &qy, &q, 0);
            float gx = pen; if (k > 0.99f && k < 1.01f) gx = floorf(gx + 0.5f);
            float x0 = gx + q.x0 * k, y0 = base + q.y0 * k, x1 = gx + q.x1 * k, y1 = base + q.y1 * k;
            ui_begin(ui_atlas, 4, 6);
            int a = ui_vt(x0, y0, c, q.s0, q.t0), b = ui_vt(x1, y0, c, q.s1, q.t0), d = ui_vt(x1, y1, c, q.s1, q.t1), e = ui_vt(x0, y1, c, q.s0, q.t1);
            ui_tri(a, b, d); ui_tri(a, d, e);
        }
        pen += ui_f[f].pc[gi].xadvance * k + sp * ui_u;
    }
    return w;
}
/* text centred vertically on yc (on the capital letters) */
static float ui_textc(int f, float x, float yc, const char *s, uint32_t col, float sc, int align, float sp) {
    if (!ui_atlas) return 0;
    float k = ui_u * sc / ui_baked;
    return ui_text(f, x, yc + ui_fpx(f, sc) * 0.30f - ui_f[f].ascent * k, s, col, sc, align, sp);
}
/* the largest scale (<= 1) at which s fits in width w */
static float ui_fit(int f, const char *s, float w, float sp) { float t = ui_text_w(f, s, 1, sp); return t > w && t > 0 ? w / t : 1; }

/* ------------------------------------------------------------------ controller glyphs and keycaps */
enum { GL_A, GL_B, GL_X, GL_Y, GL_LB, GL_RB, GL_LT, GL_RT, GL_BACK, GL_START, GL_DPAD, GL_DPAD_LR, GL_DPAD_UD, GL_LS, GL_RS, GL_L3, GL_R3, GL_NONE };
static void ui_ps_shape(int g, float cx, float cy, float r, uint32_t c) {
    float t = r * 0.16f;
    if (g == GL_A) { ui_line(cx - r * .45f, cy - r * .45f, cx + r * .45f, cy + r * .45f, t, c); ui_line(cx - r * .45f, cy + r * .45f, cx + r * .45f, cy - r * .45f, t, c); }
    else if (g == GL_B) ui_ring_line(cx, cy, r * .48f, t, c);
    else if (g == GL_X) ui_rrect_line(cx - r * .42f, cy - r * .42f, r * .84f, r * .84f, r * .06f, t, c);
    else { float h = r * .5f; ui_line(cx, cy - h, cx + h * .95f, cy + h * .62f, t, c); ui_line(cx + h * .95f, cy + h * .62f, cx - h * .95f, cy + h * .62f, t, c); ui_line(cx - h * .95f, cy + h * .62f, cx, cy - h, t, c); }
}
/* draws glyph g with its left edge at x, centred on yc, h high; returns its width */
static float ui_glyph(int g, float x, float yc, float h) {
    static const uint32_t xb[4] = {0xFF54B848u, 0xFFE0453Bu, 0xFF2F7FE6u, 0xFFF2B51Cu};
    static const uint32_t ps[4] = {0xFF8DB7FFu, 0xFFFF6E6Eu, 0xFFF29AD2u, 0xFF42D6B2u};
    static const char *xl[4] = {"A", "B", "X", "Y"};
    float r = h / 2, w = h;
    const uint32_t key = 0xFF323C52u, keyhi = 0xFF46526Cu, lab = 0xFFF2F5FBu;
    switch (g) {
    case GL_A: case GL_B: case GL_X: case GL_Y:
        if (ui_measure) return h;
        if (ui_padstyle == 1) { ui_circle(x + r, yc, r, 0xFF262D3Du); ui_ps_shape(g, x + r, yc, r, ps[g]); }
        else { ui_circle(x + r, yc, r, xb[g]); ui_textc(F_TINY, x + r, yc, xl[g], 0xFFFFFFFFu, h / (ui_fbase[F_TINY] * ui_u) * 0.62f, 1, 0); }
        return h;
    case GL_LB: case GL_RB: {
        const char *t = ui_padstyle == 1 ? (g == GL_LB ? "L1" : "R1") : (g == GL_LB ? "LB" : "RB");
        w = h * 1.55f; if (ui_measure) return w;
        ui_rrect(x, yc - h * .36f, w, h * .72f, h * .3f, keyhi, key);
        ui_textc(F_TINY, x + w / 2, yc, t, lab, h / (ui_fbase[F_TINY] * ui_u) * 0.5f, 1, 0); return w; }
    case GL_LT: case GL_RT: {
        const char *t = ui_padstyle == 1 ? (g == GL_LT ? "L2" : "R2") : (g == GL_LT ? "LT" : "RT");
        w = h * 1.25f; if (ui_measure) return w;
        ui_rrect(x, yc - h / 2, w, h, h * .32f, keyhi, key);
        ui_textc(F_TINY, x + w / 2, yc, t, lab, h / (ui_fbase[F_TINY] * ui_u) * 0.5f, 1, 0); return w; }
    case GL_BACK: case GL_START: {
        const char *t = ui_padstyle == 1 ? (g == GL_BACK ? "Share" : "Options") : (g == GL_BACK ? "Back" : "Start");
        float sc = h / (ui_fbase[F_TINY] * ui_u) * 0.46f; w = ui_text_w(F_TINY, t, sc, 0) + h * .6f; if (ui_measure) return w;
        ui_rrect(x, yc - h * .36f, w, h * .72f, h * .36f, keyhi, key); ui_textc(F_TINY, x + w / 2, yc, t, lab, sc, 1, 0); return w; }
    case GL_DPAD: case GL_DPAD_LR: case GL_DPAD_UD: {
        if (ui_measure) return h;
        float a = h * .34f, cx = x + r;
        uint32_t ch = g == GL_DPAD_LR ? 0xFFFFFFFFu : keyhi, cv = g == GL_DPAD_UD ? 0xFFFFFFFFu : keyhi;
        ui_rrect(cx - a / 2, yc - r, a, h, a * .25f, cv, cv); ui_rrect(x, yc - a / 2, h, a, a * .25f, ch, ch);
        ui_rrect(cx - a / 2, yc - a / 2, a, a, 0.5f, keyhi, keyhi); return h; }
    case GL_LS: case GL_RS: case GL_L3: case GL_R3: {
        if (ui_measure) return h;
        const char *t = g == GL_LS ? "L" : g == GL_RS ? "R" : g == GL_L3 ? "L3" : "R3";
        ui_circle(x + r, yc, r, key); ui_ring_line(x + r, yc, r, h * .1f, keyhi);
        ui_textc(F_TINY, x + r, yc, t, lab, h / (ui_fbase[F_TINY] * ui_u) * (t[1] ? .42f : .55f), 1, 0); return h; }
    default: return 0;
    }
}
/* a keyboard key cap with its left edge at x; returns its width */
static float ui_keycap(const char *t, float x, float yc, float h) {
    float sc = h / (ui_fbase[F_TINY] * ui_u) * 0.48f, w = ui_text_w(F_TINY, t, sc, 0) + h * .7f;
    if (w < h) w = h;
    if (ui_measure) return w;
    ui_rrect(x, yc - h / 2, w, h, h * .22f, 0xFF3E4860u, 0xFF2A3244u);
    ui_rrect_line(x, yc - h / 2, w, h, h * .22f, ui_px * 1.5f, 0x40FFFFFFu);
    ui_textc(F_TINY, x + w / 2, yc, t, 0xFFF2F5FBu, sc, 1, 0);
    return w;
}
#endif
