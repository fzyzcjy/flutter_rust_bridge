#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "dart_api_dl.h"

intptr_t init_frb_dart_api_dl(void *data);

uintptr_t naive_NewPersistentHandle(Dart_Handle non_persistent_handle);

uintptr_t naive_HandleFromPersistent(uintptr_t persistent_handle);
