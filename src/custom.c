#include <string.h>
#include "redismodule.h"
#include <zstd.h>
#include "custom.h"

static int _RM_CompressStr(RedisModuleCtx *ctx, RedisModuleString **zval, const char *val, size_t size_of_data ,int complevel);
static int _RM_DecompressStr(RedisModuleKey *key, char **dst, size_t len_compress, int mode);

RedisModuleString *RedisModule_ZstdCompressStr(RedisModuleCtx *ctx, const char *val, size_t size_of_data ,int complevel)
{

    RedisModuleString *zval= NULL;
    if(_RM_CompressStr(ctx, &zval, val, size_of_data, complevel) == REDISMODULE_OK){
        return zval;
    }
   
    return NULL;

}

static int _RM_CompressStr(RedisModuleCtx *ctx, RedisModuleString **zval, const char *val, size_t size_of_data ,int complevel)
{
    int rc = REDISMODULE_OK;

    size_t bound = ZSTD_compressBound(size_of_data);
    void* compressed = RedisModule_Alloc(bound + 1);
    
    size_t actual = ZSTD_compress(compressed, bound ,val, size_of_data, complevel);
    if(ZSTD_isError(actual)){
        rc = REDISMODULE_ERR;
        goto finally;
    }

    *zval =  RedisModule_CreateString(ctx, compressed, actual);


  finally:
    if (compressed != NULL){
        RedisModule_Free(compressed); compressed=NULL;
    }

    return rc;
    
}

char *RedisModule_StringDMAZstdDecompress(RedisModuleKey *key, size_t *len, int mode)
{
    char *val = NULL;
    if(_RM_DecompressStr(key, &val, len, mode) == REDISMODULE_OK){
        return val;
    }
    return NULL;

}


static int _RM_DecompressStr(RedisModuleKey *key, char **dst, size_t len_compress, int mode)
{
    int rc = REDISMODULE_OK;
    const char *compressed = NULL; 
    unsigned long long len_decompress;
    
    compressed = RedisModule_StringDMA(key, &len_compress, mode); 

   // if (len_compress == 0){
   //     rc = REDISMODULE_ERR;
   //     goto finally;
   // }


   len_decompress = ZSTD_getDecompressedSize(compressed, len_compress);
    void* const decompress = RedisModule_Alloc((size_t)len_decompress + 1);

    size_t d_size = ZSTD_decompress(decompress, len_decompress, compressed, len_compress);
    if(ZSTD_isError(d_size)){
        rc = REDISMODULE_ERR;
        goto finally;
    }

   *dst = RedisModule_Strdup((char*)decompress);



finally:
//    if( decompress != NULL ){
//        RedisModule_Free(decompress); decompress=NULL;
//    }

  return rc;
}
