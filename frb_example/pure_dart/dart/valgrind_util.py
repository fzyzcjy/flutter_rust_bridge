#!/usr/bin/env python3
#
# Copyright (c) 2011, the Dart project authors.  Please see the AUTHORS file
# for details. All rights reserved. Use of this source code is governed by a
# BSD-style license that can be found in the LICENSE file.

# Simple wrapper for running Valgrind and checking the output on
# stderr for memory leaks.

# NOTE XXX this file is copied and modified from: https://github.com/dart-lang/sdk/blob/master/runtime/tools/valgrind.py

import re
import selectors
import subprocess
import sys

print('valgrind_util.py start')

VALGRIND_ARGUMENTS = [
    'valgrind',
    '--error-exitcode=1',
    '--leak-check=full',
    '--trace-children=yes',
    '--ignore-ranges=0x000-0xFFF',  # Used for implicit null checks.
    '--vex-iropt-level=1'  # Valgrind crashes with the default level (2).
]

# Compute the command line.
command = VALGRIND_ARGUMENTS + sys.argv[1:]

# Run Valgrind.
print('run command: ', ' '.join(command))  # NOTE XXX added by me
process = subprocess.Popen(
    command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

# NOTE XXX edited by me
# code = process.wait()
# output = process.stdout.readlines()
# errors = process.stderr.readlines()
# https://stackoverflow.com/questions/31833897/python-read-from-subprocess-stdout-and-stderr-separately-while-preserving-order
sel = selectors.DefaultSelector()
sel.register(process.stdout, selectors.EVENT_READ)
sel.register(process.stderr, selectors.EVENT_READ)
output_raw, errors_raw = '', ''
done = False
while not done:
    for key, _ in sel.select():
        data = key.fileobj.read1(10000).decode()
        if not data:
            done = True
            break
        if key.fileobj is process.stdout:
            output_raw += data
            print(data, end="")
        else:
            errors_raw += data
            print(data, end="", file=sys.stderr)
output = output_raw.split('\n')
errors = errors_raw.split('\n')
code = process.wait()
print("process code=", code)

# Always print the output, but leave out the 3 line banner printed
# by certain versions of Valgrind.
if len(output) > 0 and output[0].startswith("** VALGRIND_ROOT="):
    output = output[3:]
# sys.stdout.writelines(output) # NOTE XXX edited by me

# NOTE XXX added by me
DART_ALL_TESTS_PASSED_STR = 'All tests passed!'
if not any(DART_ALL_TESTS_PASSED_STR in line for line in output):
    print(f'error: valgrind_util does not find "{DART_ALL_TESTS_PASSED_STR}", thus dart test seems failed')
    sys.exit(1)

# NOTE XXX edited by me
# If Valgrind produced an error, we report that to the user.
# if code != 0:
#     sys.stderr.writelines(errors)
#     sys.exit(code)

# Look through the leak details and make sure that we don't have
# any definitely or indirectly lost bytes. We allow possibly lost
# bytes to lower the risk of false positives.
LEAK_RE = r"(?:definitely|indirectly) lost:"
LEAK_LINE_MATCHER = re.compile(LEAK_RE)
LEAK_OKAY_MATCHER = re.compile(r"lost: 0 bytes in 0 blocks")
leaks = []
for line in errors:
    if LEAK_LINE_MATCHER.search(line):
        leaks.append(line)
        if not LEAK_OKAY_MATCHER.search(line):
            # NOTE XXX edited by me
            print('error: valgrind_util find this line is bad:', line)
            # sys.stderr.writelines(errors)
            sys.exit(1)

# Make sure we found the right number of leak lines.
if not len(leaks) in [0, 2, 3]:
    # sys.stderr.writelines(errors) # NOTE XXX edited by me
    print('error: #### Malformed Valgrind output.\n#### Exiting.')
    sys.exit(1)

# Success.
print('valgrind_util thinks it is ok')
sys.exit(0)
