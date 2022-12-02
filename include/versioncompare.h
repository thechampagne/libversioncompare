#ifndef __VERSION_COMPARE_H__
#DEFINE __VERSION_COMPARE_H__

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CMP_EQ,
    CMP_NE,
    CMP_LT,
    CMP_LE,
    CMP_GE,
    CMP_GT
} version_compare_cmp;

typedef struct {
  size_t max_depth;
  int ignore_text;
} version_compare_manifest;

extern int version_compare_compare(version_compare_cmp* cmp, const char* a, const char* b);

extern int version_compare_compare_to( const char* a, const char* b, version_compare_cmp operator);

extern version_compare_manifest version_compare_manifest_default();

extern int version_compare_manifest_has_max_depth(version_compare_manifest* manifest);
  
#ifdef __cplusplus
}
#endif

#endif // __VERSION_COMPARE_H__
