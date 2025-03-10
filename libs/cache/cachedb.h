#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Cache Cache;

typedef struct FileAttr FileAttr;

typedef struct Option_OsString Option_OsString;

typedef struct Result_Cache__Error Result_Cache__Error;

typedef struct Result_bool__Error Result_bool__Error;

typedef struct String String;

struct Result_Cache__Error new(const struct String *p, uint64_t life);

bool is_exist(const struct Cache *self, const struct String *path);

struct Option_OsString get_full_path(const struct Cache *self, const OsString *name);

const struct FileAttr *get_attr(const struct Cache *self, const OsString *path);

void refresh(struct Cache *self);

struct Result_bool__Error set_path(struct Cache *self, const struct String *path, bool create);
