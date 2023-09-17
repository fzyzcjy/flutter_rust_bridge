#!/bin/bash
set -eux

(cd rust && cargo build)

(cd dart && dart main.dart)
