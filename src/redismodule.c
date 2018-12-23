#include <string.h>
#include "redismodule.h"
#include <zstd.h>

#include "custom.h"


// RedisModule_Init is defined as a static function and so won't be exported as
// a symbol. Export a version under a slightly different name so that we can
// get access to it from Rust.
int Export_RedisModule_Init(RedisModuleCtx *ctx, const char *name, int ver, int apiver) {
    return RedisModule_Init(ctx, name, ver, apiver);
}

RedisModuleString *Custom_RedisModule_ZstdCompressStr(RedisModuleCtx *ctx, const char *val, size_t size_of_data, int complevel){
    return RedisModule_ZstdCompressStr(ctx, val, size_of_data, complevel);
}

char *Custom_RedisModule_StringDMAZstdDecompress(RedisModuleKey *key, size_t len, int mode){
    return RedisModule_StringDMAZstdDecompress(key, len, mode);
}
