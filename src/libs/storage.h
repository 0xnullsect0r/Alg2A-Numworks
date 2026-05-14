#ifndef STORAGE_H
#define STORAGE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

bool storage_record_name_is_equal_to(const char * name, const char * baseName, const char * extension);
size_t storage_record_size(const char * fullName);
bool storage_record_exists(const char * fullName);
int storage_record_read(const char * fullName, uint8_t * buffer, size_t bufferSize);
int storage_record_write(const char * fullName, const uint8_t * data, size_t size);
void storage_record_destroy(const char * fullName);

#ifdef __cplusplus
}
#endif

#endif // STORAGE_H
