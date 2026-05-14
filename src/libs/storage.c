#include "storage.h"
#include <string.h>

// Minimal storage stubs for the EADK app
// These are placeholders; actual storage operations are handled by the OS

bool storage_record_name_is_equal_to(const char * name, const char * baseName, const char * extension) {
    if (!name || !baseName) return false;
    size_t base_len = strlen(baseName);
    size_t ext_len = extension ? strlen(extension) : 0;
    size_t name_len = strlen(name);
    if (ext_len == 0) {
        return strncmp(name, baseName, name_len) == 0 && base_len == name_len;
    }
    if (name_len != base_len + 1 + ext_len) return false;
    if (strncmp(name, baseName, base_len) != 0) return false;
    if (name[base_len] != '.') return false;
    return strcmp(name + base_len + 1, extension) == 0;
}

size_t storage_record_size(const char * fullName) {
    (void)fullName;
    return 0;
}

bool storage_record_exists(const char * fullName) {
    (void)fullName;
    return false;
}

int storage_record_read(const char * fullName, uint8_t * buffer, size_t bufferSize) {
    (void)fullName;
    (void)buffer;
    (void)bufferSize;
    return -1;
}

int storage_record_write(const char * fullName, const uint8_t * data, size_t size) {
    (void)fullName;
    (void)data;
    (void)size;
    return -1;
}

void storage_record_destroy(const char * fullName) {
    (void)fullName;
}
