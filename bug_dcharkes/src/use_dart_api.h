// Copyright (c) 2023, the Dart project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

#include <stdint.h>

#include "dart_api_dl.h"

#if _WIN32
#define MYLIB_EXPORT __declspec(dllexport)
#else
#define MYLIB_EXPORT
#endif

MYLIB_EXPORT int32_t add(int32_t a, int32_t b);

MYLIB_EXPORT intptr_t InitDartApiDL(void *data);

MYLIB_EXPORT void *NewPersistentHandle(Dart_Handle non_persistent_handle);

MYLIB_EXPORT Dart_Handle HandleFromPersistent(void *persistent_handle);