// Copyright (c) 2023, the Dart project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

#include "use_rust.h"

int32_t add(int32_t a, int32_t b) { return a + b; }

intptr_t InitDartApiDL(void *data) { return Dart_InitializeApiDL(data); }

void *NewPersistentHandle(Dart_Handle non_persistent_handle) {
  return Dart_NewPersistentHandle_DL(non_persistent_handle);
}

Dart_Handle HandleFromPersistent(void *persistent_handle) {
  return Dart_HandleFromPersistent_DL(persistent_handle);
}
