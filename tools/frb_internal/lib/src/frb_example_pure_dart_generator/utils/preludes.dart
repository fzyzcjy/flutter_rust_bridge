final kDirectSourcesPrelude = computeDuplicatorPrelude('');

String computeDuplicatorPrelude(String origin) =>
    '// NOTE: This file is mimicking how a human developer writes tests, \n'
    '// and is auto-generated$origin by frb_internal\n'
    '// Please do not modify manually, but modify the origin and re-run frb_internal generator\n\n';
