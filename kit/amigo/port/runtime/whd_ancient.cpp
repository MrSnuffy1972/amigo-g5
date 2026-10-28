// whd_decrunch for runtime/whd.c, using temisu/ancient (third_party/ancient, BSD 2-Clause).
// Returns the unpacked length and a malloc'd buffer in *out, or -1 if the data is not a format ancient can unpack.
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <cstdio>
#include <vector>
#include "ancient.hpp"

extern "C" long whd_decrunch(const uint8_t *src, size_t srclen, uint8_t **out) {
    try {
        if (!ancient::Decompressor::detect(src, srclen)) return -1;
        ancient::Decompressor d(src, srclen, false, true);
        std::vector<uint8_t> raw = d.decompress(true);
        uint8_t *o = (uint8_t *)malloc(raw.size() ? raw.size() : 1);
        if (!o) return -1;
        memcpy(o, raw.data(), raw.size());
        *out = o;
        return (long)raw.size();
    } catch (const std::exception &e) {
        fprintf(stderr, "whd_decrunch: %s\n", e.what());
        return -1;
    } catch (...) {
        return -1;
    }
}
