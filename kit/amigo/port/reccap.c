/* reccap.c - screenshots and video capture for every Ami-Go port (Cowork 17 Oct 2026, Karl: "have a setup menu that lets you set
 * options. One option should be to set the hotkey or hot button to screen record or screenshot. It should default to the triggers on a
 * controller, like the Xbox controller.")
 *
 * Defaults: left trigger = screenshot (PNG), right trigger = video (AVI: Motion-JPEG picture + 16-bit PCM stereo sound, which Windows,
 * VLC, Kodi and Android players open). Both are ordinary mappable actions in the controller section of the menu, so any button can carry
 * them. Keyboard: F12 screenshot, Shift+F12 video. The menu's Capture section chooses what the video button does:
 *   "Start / stop recording"      press to start, press again to stop (a long recording stops itself after 10 minutes)
 *   "Save the last N seconds"     the last 10/20/30/60 seconds are always kept in memory; a press writes them out (instant replay)
 * Files go to <the game's folder>/captures (Windows: next to the .exe when it is writable, else the user's AmiGo folder; Android: the
 * app's external files folder, /sdcard/Android/data/<package>/files/captures, readable over USB or adb without root).
 * Self-contained: the image coding is stb_image_write (public domain, Sean Barrett), compiled in; nothing external is needed.
 * The writers below (PNG, JPEG, AVI) do not use SDL, so they can be tested headless (RECCAP_TEST). */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#ifdef _WIN32
#include <direct.h>
#define rc_mkdir(p) _mkdir(p)
#else
#define rc_mkdir(p) mkdir(p, 0755)
#endif
#define STB_IMAGE_WRITE_IMPLEMENTATION
#define STB_IMAGE_WRITE_STATIC
#define STBI_WRITE_NO_STDIO_DUMMY_UNUSED
#include "stb_image_write.h"

#define RC_FPS 50
#define RC_RATE 44100
#define RC_MAX_REC_FRAMES (RC_FPS * 600)   /* a start/stop recording stops itself after 10 minutes */

/* ------------------------------------------------------------------ growable byte buffer */
typedef struct { uint8_t *p; size_t n, cap; } rc_buf;
static void rc_put(rc_buf *b, const void *d, size_t n) {
    if (b->n + n > b->cap) { size_t c = b->cap ? b->cap * 2 : 65536; while (c < b->n + n) c *= 2; b->p = realloc(b->p, c); b->cap = c; }
    memcpy(b->p + b->n, d, n); b->n += n;
}
static void rc_jpg_cb(void *ctx, void *data, int size) { rc_put((rc_buf *)ctx, data, (size_t)size); }

/* framebuf pixels are 0x00RRGGBB; the coders want RGB bytes */
static void rc_rgb(const uint32_t *px, int w, int h, uint8_t *out) {
    for (int i = 0; i < w * h; i++) { uint32_t c = px[i]; out[i * 3] = (uint8_t)(c >> 16); out[i * 3 + 1] = (uint8_t)(c >> 8); out[i * 3 + 2] = (uint8_t)c; }
}
static int rc_jpeg(const uint32_t *px, int w, int h, rc_buf *out) {
    static uint8_t *rgb; static int rgbn;
    if (rgbn < w * h * 3) { rgb = realloc(rgb, (size_t)w * h * 3); rgbn = w * h * 3; }
    rc_rgb(px, w, h, rgb); out->n = 0;
    return stbi_write_jpg_to_func(rc_jpg_cb, out, w, h, 3, rgb, 90);
}
static int rc_png(const char *path, const uint32_t *px, int w, int h) {
    uint8_t *rgb = malloc((size_t)w * h * 3); if (!rgb) return 0;
    rc_rgb(px, w, h, rgb); int ok = stbi_write_png(path, w, h, 3, rgb, w * 3); free(rgb); return ok;
}

/* ------------------------------------------------------------------ AVI (RIFF) writer: Motion-JPEG video + 16-bit PCM stereo */
#define RC_MOVI 322                         /* file offset of the 'movi' fourcc (the header below is a fixed 326 bytes) */
#define RC_MAX_BYTES (1000L * 1000L * 1000L)  /* stop a recording at 1 GB, well inside what every AVI player reads */
typedef struct { FILE *f; rc_buf idx; uint32_t nvid, nau; long bytes; } rc_avi;
static void rc_le32(uint8_t *p, uint32_t v) { p[0] = (uint8_t)v; p[1] = (uint8_t)(v >> 8); p[2] = (uint8_t)(v >> 16); p[3] = (uint8_t)(v >> 24); }
static void rc_le16(uint8_t *p, uint32_t v) { p[0] = (uint8_t)v; p[1] = (uint8_t)(v >> 8); }
static void rc_patch32(FILE *f, long at, uint32_t v) { uint8_t b[4]; rc_le32(b, v); fseek(f, at, SEEK_SET); fwrite(b, 1, 4, f); }
static int rc_avi_open(rc_avi *a, const char *path, int w, int h) {
    a->f = fopen(path, "wb"); a->idx.n = 0; a->nvid = a->nau = 0; a->bytes = 0;
    if (!a->f) return 0;
    uint8_t H[326]; memset(H, 0, sizeof H);
    memcpy(H, "RIFF", 4); memcpy(H + 8, "AVI LIST", 8); rc_le32(H + 16, 294); memcpy(H + 20, "hdrlavih", 8); rc_le32(H + 28, 56);
    uint8_t *m = H + 32;                                            /* avih */
    rc_le32(m, 1000000 / RC_FPS); rc_le32(m + 4, 4u << 20); rc_le32(m + 12, 0x10); rc_le32(m + 24, 2); rc_le32(m + 28, 1u << 20);
    rc_le32(m + 32, (uint32_t)w); rc_le32(m + 36, (uint32_t)h);
    memcpy(H + 88, "LIST", 4); rc_le32(H + 92, 116); memcpy(H + 96, "strlstrh", 8); rc_le32(H + 104, 56);
    m = H + 108;                                                    /* video stream header */
    memcpy(m, "vidsMJPG", 8); rc_le32(m + 20, 1); rc_le32(m + 24, RC_FPS); rc_le32(m + 36, 1u << 20); rc_le32(m + 40, 0xFFFFFFFFu);
    rc_le16(m + 52, (uint32_t)w); rc_le16(m + 54, (uint32_t)h);
    memcpy(H + 164, "strf", 4); rc_le32(H + 168, 40);
    m = H + 172;                                                    /* BITMAPINFOHEADER */
    rc_le32(m, 40); rc_le32(m + 4, (uint32_t)w); rc_le32(m + 8, (uint32_t)h); rc_le16(m + 12, 1); rc_le16(m + 14, 24); memcpy(m + 16, "MJPG", 4);
    rc_le32(m + 20, (uint32_t)(w * h * 3));
    memcpy(H + 212, "LIST", 4); rc_le32(H + 216, 94); memcpy(H + 220, "strlstrh", 8); rc_le32(H + 228, 56);
    m = H + 232;                                                    /* audio stream header: 4-byte blocks, 44100 a second */
    memcpy(m, "auds", 4); rc_le32(m + 20, 4); rc_le32(m + 24, RC_RATE * 4); rc_le32(m + 36, 16384); rc_le32(m + 40, 0xFFFFFFFFu); rc_le32(m + 44, 4);
    memcpy(H + 288, "strf", 4); rc_le32(H + 292, 18);
    m = H + 296;                                                    /* WAVEFORMATEX: PCM, 2 channels, 16 bit */
    rc_le16(m, 1); rc_le16(m + 2, 2); rc_le32(m + 4, RC_RATE); rc_le32(m + 8, RC_RATE * 4); rc_le16(m + 12, 4); rc_le16(m + 14, 16);
    memcpy(H + 314, "LIST", 4); memcpy(H + 322, "movi", 4);
    fwrite(H, 1, sizeof H, a->f); a->bytes = sizeof H;
    return 1;
}
static void rc_avi_chunk(rc_avi *a, const char *id, const void *d, uint32_t n) {
    uint8_t h[8], e[16];
    memcpy(h, id, 4); rc_le32(h + 4, n);
    memcpy(e, id, 4); rc_le32(e + 4, 0x10); rc_le32(e + 8, (uint32_t)(a->bytes - RC_MOVI)); rc_le32(e + 12, n);
    rc_put(&a->idx, e, 16);
    fwrite(h, 1, 8, a->f); fwrite(d, 1, n, a->f); if (n & 1) fputc(0, a->f);
    a->bytes += 8 + (long)n + (long)(n & 1);
}
static void rc_avi_frame(rc_avi *a, const uint8_t *jpg, uint32_t n, const int16_t *au, int ns) {
    rc_avi_chunk(a, "00dc", jpg, n); a->nvid++;
    if (ns > 0) { rc_avi_chunk(a, "01wb", au, (uint32_t)ns * 4); a->nau += (uint32_t)ns; }
}
static void rc_avi_close(rc_avi *a) {
    if (!a->f) return;
    uint8_t h[8]; memcpy(h, "idx1", 4); rc_le32(h + 4, (uint32_t)a->idx.n);
    fwrite(h, 1, 8, a->f); if (a->idx.n) fwrite(a->idx.p, 1, a->idx.n, a->f);
    long end = a->bytes + 8 + (long)a->idx.n;
    rc_patch32(a->f, 4, (uint32_t)(end - 8)); rc_patch32(a->f, 48, a->nvid); rc_patch32(a->f, 140, a->nvid); rc_patch32(a->f, 264, a->nau);
    rc_patch32(a->f, 318, (uint32_t)(a->bytes - RC_MOVI));
    fclose(a->f); a->f = NULL; a->idx.n = 0;
}

#ifndef RECCAP_TEST
/* ------------------------------------------------------------------ the capture worker (a thread, so the game never waits on a coder or the disk) */
enum { J_SHOT, J_FRAME, J_START, J_STOP, J_REPLAY, J_QUIT };
typedef struct rc_job { int type, replay, nau, want; uint32_t *px; int16_t *au; char path[1024]; struct rc_job *next; } rc_job;
static SDL_mutex *rc_mx; static SDL_cond *rc_cv; static SDL_Thread *rc_th;
static rc_job *rc_head, *rc_tail; static volatile int rc_qlen;
static char rc_dirp[1024]; static int rc_ready, rc_shot_pending;
static volatile int rc_rec;                 /* recording (main thread; the worker clears it if the file fills up) */
static int rc_rec_frames; static Uint32 rc_flash_t;
static char rc_msg[300]; static volatile int rc_msg_new;      /* a message for the screen, from the worker (under rc_mx) */
static char rc_toast[300]; static float rc_toast_t;
static uint32_t *rc_pool[8]; static int rc_npool;
typedef struct { rc_buf j; int16_t *au; int nau, aucap; } rc_slot;
static rc_slot *rc_ring; static int rc_ring_head, rc_ring_n;
#define RC_RING_CAP (60 * RC_FPS)
#ifdef _WIN32
#define RC_SEP "\\"
#else
#define RC_SEP "/"
#endif

static void rc_note(const char *fmt, const char *a, long b) {
    SDL_LockMutex(rc_mx); snprintf(rc_msg, sizeof rc_msg, fmt, a, b); rc_msg_new = 1; SDL_UnlockMutex(rc_mx);
}
static uint32_t *rc_px_get(void) {
    uint32_t *p = NULL; SDL_LockMutex(rc_mx); if (rc_npool) p = rc_pool[--rc_npool]; SDL_UnlockMutex(rc_mx);
    return p ? p : (uint32_t *)malloc((size_t)SCR_W * SCR_H * 4);
}
static void rc_px_put(uint32_t *p) {
    if (!p) return;
    SDL_LockMutex(rc_mx); if (rc_npool < 8) { rc_pool[rc_npool++] = p; p = NULL; } SDL_UnlockMutex(rc_mx);
    free(p);
}
static void rc_push(rc_job *j) {
    j->next = NULL;
    SDL_LockMutex(rc_mx); if (rc_tail) rc_tail->next = j; else rc_head = j; rc_tail = j; rc_qlen++; SDL_CondSignal(rc_cv); SDL_UnlockMutex(rc_mx);
}
static const char *rc_base(const char *p) { const char *b = p; for (; *p; p++) if (*p == '/' || *p == '\\') b = p + 1; return b; }
static void rc_ring_add(const rc_buf *jpg, const int16_t *au, int nau) {
    if (!rc_ring) { rc_ring = (rc_slot *)calloc(RC_RING_CAP, sizeof *rc_ring); if (!rc_ring) return; }
    rc_slot *s = &rc_ring[rc_ring_head];
    s->j.n = 0; rc_put(&s->j, jpg->p, jpg->n);
    if (nau > s->aucap) { s->au = (int16_t *)realloc(s->au, (size_t)nau * 4); s->aucap = s->au ? nau : 0; }
    s->nau = s->au ? nau : 0; if (s->nau) memcpy(s->au, au, (size_t)nau * 4);
    rc_ring_head = (rc_ring_head + 1) % RC_RING_CAP; if (rc_ring_n < RC_RING_CAP) rc_ring_n++;
}
static int rc_worker(void *arg) {
    (void)arg;
    static rc_avi av; static rc_buf jpg; static char avpath[1024];
    for (;;) {
        SDL_LockMutex(rc_mx);
        while (!rc_head) SDL_CondWait(rc_cv, rc_mx);
        rc_job *j = rc_head; rc_head = j->next; if (!rc_head) rc_tail = NULL; rc_qlen--;
        SDL_UnlockMutex(rc_mx);
        switch (j->type) {
        case J_QUIT: if (av.f) rc_avi_close(&av); free(j); return 0;
        case J_SHOT:
            if (rc_png(j->path, j->px, SCR_W, SCR_H)) rc_note("Screenshot saved  %s", rc_base(j->path), 0);
            else rc_note("Could not save the screenshot in %s", rc_dirp, 0);
            break;
        case J_START:
            if (av.f) rc_avi_close(&av);
            snprintf(avpath, sizeof avpath, "%s", j->path);
            if (!rc_avi_open(&av, j->path, SCR_W, SCR_H)) { rc_rec = 0; rc_note("Could not start a video in %s", rc_dirp, 0); }
            break;
        case J_FRAME: {
            int have = 0;
            if (j->px) { jpg.n = 0; have = rc_jpeg(j->px, SCR_W, SCR_H, &jpg); } else have = jpg.n > 0;   /* NULL = the queue was long: repeat the last picture */
            if (have && av.f) {
                rc_avi_frame(&av, jpg.p, (uint32_t)jpg.n, j->au, j->nau);
                if (av.bytes > RC_MAX_BYTES) { long s = (long)(av.nvid / RC_FPS); rc_avi_close(&av); rc_rec = 0; rc_note("Video saved (%s%ld s, the 1 GB limit)", "", s); }
            }
            if (have && j->replay) rc_ring_add(&jpg, j->au, j->nau);
            break; }
        case J_STOP:
            if (av.f) { long s = (long)(av.nvid / RC_FPS); rc_avi_close(&av); rc_note("Video saved  %s  (%ld s)", rc_base(avpath), s); }
            break;
        case J_REPLAY: {
            int n = j->want < rc_ring_n ? j->want : rc_ring_n;
            rc_avi ra; memset(&ra, 0, sizeof ra);
            if (n <= 0) rc_note("Nothing to save yet%s", "", 0);
            else if (!rc_avi_open(&ra, j->path, SCR_W, SCR_H)) rc_note("Could not save the replay in %s", rc_dirp, 0);
            else {
                for (int i = 0; i < n; i++) { rc_slot *s = &rc_ring[(rc_ring_head - n + i + RC_RING_CAP) % RC_RING_CAP]; rc_avi_frame(&ra, s->j.p, (uint32_t)s->j.n, s->au, s->nau); }
                rc_avi_close(&ra); free(ra.idx.p);
                rc_note("Replay saved  %s  (%ld s)", rc_base(j->path), (long)(n / RC_FPS));
            }
            break; }
        }
        rc_px_put(j->px); free(j->au); free(j);
    }
}
static int SDLCALL rc_watch(void *ud, SDL_Event *e) {   /* the app going to the background or ending: finish the video file */
    (void)ud;
    if ((e->type == SDL_APP_WILLENTERBACKGROUND || e->type == SDL_APP_TERMINATING) && rc_rec && rc_ready) {
        rc_job *j = (rc_job *)calloc(1, sizeof *j); if (j) { j->type = J_STOP; rc_push(j); } rc_rec = 0;
    }
    return 0;
}
static void rc_init(void) {
    if (rc_ready) return;
    rc_ready = 1;
    rc_dirp[0] = 0;
#ifdef __ANDROID__
    const char *ext = SDL_AndroidGetExternalStoragePath();          /* /storage/emulated/0/Android/data/<package>/files */
    if (ext) snprintf(rc_dirp, sizeof rc_dirp, "%s/captures", ext);
#elif defined(_WIN32)
    char *b = SDL_GetBasePath();                                     /* next to the .exe, when that folder can be written */
    if (b) { snprintf(rc_dirp, sizeof rc_dirp, "%scaptures", b); SDL_free(b); }
#endif
    FILE *t = NULL; char probe[1100];
    if (rc_dirp[0]) { rc_mkdir(rc_dirp); snprintf(probe, sizeof probe, "%s" RC_SEP "README.txt", rc_dirp); t = fopen(probe, "a"); }
    if (!t) {
        char *p = SDL_GetPrefPath("AmiGo", game_title);
        snprintf(rc_dirp, sizeof rc_dirp, "%scaptures", p ? p : "./"); if (p) SDL_free(p);
        rc_mkdir(rc_dirp); snprintf(probe, sizeof probe, "%s" RC_SEP "README.txt", rc_dirp); t = fopen(probe, "a");
    }
    if (t) { if (ftell(t) == 0) fprintf(t, "Ami-Go captures: screenshots (PNG) and videos (AVI, Motion-JPEG picture with PCM sound).\n"); fclose(t); }
    rc_mx = SDL_CreateMutex(); rc_cv = SDL_CreateCond();
    rc_th = SDL_CreateThread(rc_worker, "reccap", NULL);
    SDL_AddEventWatch(rc_watch, NULL);
    fprintf(stderr, "reccap: captures go to %s\n", rc_dirp);
}
static void rc_name(char *out, size_t n, const char *ext) {
    char t[96]; int k = 0;
    for (const char *s = game_title; *s && k < 60; s++) { char c = *s; t[k++] = (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == ' ' || c == '-' ? c : '_'; }
    t[k] = 0; if (!k) snprintf(t, sizeof t, "game");
    time_t now = time(NULL); struct tm *lt = localtime(&now); char ts[32] = "capture";
    if (lt) strftime(ts, sizeof ts, "%Y-%m-%d %H-%M-%S", lt);
    for (int i = 0; i < 100; i++) {
        if (i) snprintf(out, n, "%s" RC_SEP "%s %s (%d)%s", rc_dirp, t, ts, i + 1, ext); else snprintf(out, n, "%s" RC_SEP "%s %s%s", rc_dirp, t, ts, ext);
        FILE *f = fopen(out, "rb"); if (!f) return; fclose(f);
    }
}

/* ------------------------------------------------------------------ what the frontend calls */
static const char *reccap_dir(void) { rc_init(); return rc_dirp; }
static void reccap_shot(void) { rc_init(); rc_shot_pending = 1; rc_flash_t = SDL_GetTicks(); fprintf(stderr, "reccap: screenshot\n"); }
static void reccap_button(void) {
    rc_init();
    rc_job *j = (rc_job *)calloc(1, sizeof *j); if (!j) return;
    if (S_.rec_mode == 1) {
        j->type = J_REPLAY; j->want = replay_secs[S_.replay_len & 3] * RC_FPS; rc_name(j->path, sizeof j->path, ".avi");
        snprintf(rc_toast, sizeof rc_toast, "Saving the last %d seconds...", replay_secs[S_.replay_len & 3]); rc_toast_t = 2.0f;
    } else if (!rc_rec) { j->type = J_START; rc_name(j->path, sizeof j->path, ".avi"); rc_rec = 1; rc_rec_frames = 0; }
    else { j->type = J_STOP; rc_rec = 0; }
    fprintf(stderr, "reccap: video button (%s)\n", j->type == J_REPLAY ? "replay" : j->type == J_START ? "start" : "stop");
    rc_push(j);
}
static void reccap_frame(const uint32_t *fb, const int16_t *au, int ns) {
    int replay = S_.rec_mode == 1;
    if (!rc_ready) { if (!replay && !rc_shot_pending) return; rc_init(); }
    if (rc_shot_pending) {
        rc_shot_pending = 0;
        rc_job *j = (rc_job *)calloc(1, sizeof *j);
        if (j && (j->px = rc_px_get())) { memcpy(j->px, fb, (size_t)SCR_W * SCR_H * 4); j->type = J_SHOT; rc_name(j->path, sizeof j->path, ".png"); rc_push(j); } else free(j);
    }
    if (!(rc_rec || replay)) return;
    rc_job *j = (rc_job *)calloc(1, sizeof *j); if (!j) return;
    j->type = J_FRAME; j->replay = replay;
    if (rc_qlen < 100 && (j->px = rc_px_get())) memcpy(j->px, fb, (size_t)SCR_W * SCR_H * 4);
    if (ns > 0 && (j->au = (int16_t *)malloc((size_t)ns * 4))) { memcpy(j->au, au, (size_t)ns * 4); j->nau = ns; }
    rc_push(j);
    if (rc_rec && ++rc_rec_frames >= RC_MAX_REC_FRAMES) {
        rc_job *k = (rc_job *)calloc(1, sizeof *k); if (k) { k->type = J_STOP; rc_push(k); } rc_rec = 0;
    }
}
static int reccap_wants_draw(void) {
    Uint32 now = SDL_GetTicks();
    return rc_rec || rc_msg_new || rc_toast_t > 0 || (rc_flash_t && now - rc_flash_t < 200);
}
/* the recording indicator, the screenshot flash and messages, in screen pixels (called from ui_overlay in menu.c) */
static void reccap_draw(int ow, int oh, float dt) {
    Uint32 now = SDL_GetTicks(); float u = ui_u;
    if (rc_mx && rc_msg_new) { SDL_LockMutex(rc_mx); snprintf(rc_toast, sizeof rc_toast, "%s", rc_msg); rc_msg_new = 0; SDL_UnlockMutex(rc_mx); rc_toast_t = 3.0f; }
    if (rc_flash_t && now - rc_flash_t < 200) { float a = 1 - (now - rc_flash_t) / 200.0f; ui_quad4(0, 0, (float)ow, (float)oh, ui_a(0xFFFFFFFFu, .35f * a), ui_a(0xFFFFFFFFu, .35f * a), ui_a(0xFFFFFFFFu, .35f * a), ui_a(0xFFFFFFFFu, .35f * a)); }
    if (rc_rec && S_.cap_indicator) {
        char b[32]; int s = rc_rec_frames / RC_FPS; snprintf(b, sizeof b, "REC  %d:%02d", s / 60, s % 60);
        float h = 50 * u, w = ui_text_w(F_SMALL, b, 1, 1) + 70 * u, x = ow - w - 30 * u, y = 28 * u;
        ui_rrect(x, y, w, h, h / 2, 0xC8101522u, 0xC8101522u); ui_rrect_line(x, y, w, h, h / 2, 1.5f, 0x40FFFFFFu);
        ui_circle(x + 27 * u, y + h / 2, 9 * u, ui_a(0xFFFF3B3Bu, 0.55f + 0.45f * sinf(now / 1000.0f * 5.0f)));
        ui_textc(F_SMALL, x + 46 * u, y + h / 2, b, 0xFFFFFFFFu, 1, 0, 1);
    }
    if (rc_toast_t > 0) {
        rc_toast_t -= dt;
        float a = rc_toast_t < 0.4f ? rc_toast_t / 0.4f : 1, h = 56 * u, sc = ui_fit(F_SMALL, rc_toast, ow * 0.8f, 0), w = ui_text_w(F_SMALL, rc_toast, sc, 0) + 80 * u;
        float x = (ow - w) / 2, y = 30 * u - (1 - a) * 20 * u;
        float sa = ui_alpha; ui_alpha = a;
        ui_rrect(x, y, w, h, h / 2, 0xE6151B2Du, 0xE6151B2Du); ui_rrect_line(x, y, w, h, h / 2, 1.5f, 0x40FFFFFFu);
        ui_circle(x + 30 * u, y + h / 2, 10 * u, 0xFF54D17Au);
        ui_textc(F_SMALL, x + 52 * u, y + h / 2, rc_toast, 0xFFFFFFFFu, sc, 0, 0);
        ui_alpha = sa;
    }
}
static void reccap_close(void) {   /* at exit: finish any video and let the worker write what it holds */
    if (!rc_ready || !rc_th) return;
    if (rc_rec) { rc_job *j = (rc_job *)calloc(1, sizeof *j); if (j) { j->type = J_STOP; rc_push(j); } rc_rec = 0; }
    rc_job *q = (rc_job *)calloc(1, sizeof *q); if (q) { q->type = J_QUIT; rc_push(q); SDL_WaitThread(rc_th, NULL); rc_th = NULL; }
    SDL_DelEventWatch(rc_watch, NULL);
}
#endif
