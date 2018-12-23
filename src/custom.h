#ifndef __REDIS_CUSTOM_H_
#define __REDIS_CUSTOM_H_

#ifdef __cplusplus
extern "C" {
#endif

extern RedisModuleString *RedisModule_ZstdCompressStr(RedisModuleCtx *ctx, const char *val, size_t size_of_data ,int complevel);
extern char *Custom_RedisModule_StringDMAZstdDecompress(RedisModuleKey *key, size_t len, int mode);

#ifdef __cplusplus
}
#endif

#endif
