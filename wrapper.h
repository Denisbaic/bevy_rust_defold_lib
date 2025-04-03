#pragma once
#include <stdint.h>

typedef uint64_t dmhash_t;

void PostMessage(const char* receiver_url, const char* message_name, const char* message_data_as_json, const size_t message_data_len);

/*# get string value from hash
 *
 * Returns the original string used to produce a hash.
 * Always returns a null terminated string. Returns "<unknown>" if the original string wasn't found.
 * @name dmHashReverseSafe64
 * @param hash [type:uint64_t] hash value
 * @return [type:const char*] Original string value or "<unknown>" if it wasn't found.
 * @note Do not store this pointer
 */
const char* dmHashReverseSafe64(uint64_t hash);

uint64_t dmHashString64(const char* string);