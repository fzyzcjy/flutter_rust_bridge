#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "dart_api_dl.h"

intptr_t InitializeApiDL(void *data);

void *NewPersistentHandle(Dart_Handle non_persistent_handle);

uintptr_t HandleFromPersistent(void *persistent_handle);
