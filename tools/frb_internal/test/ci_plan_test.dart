import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/ci_plan.dart';
import 'package:test/test.dart';

void main() {
  group('CI job matrix snapshot', () {
    test('kCiJobs stays in sync with ci.yaml matrices', () {
      expect(_prettyJson(_ciJobsSnapshot()), _readSnapshot('full'));
    });
  });

  group('CI filter planning', () {
    test('full-like filters enable every job and full matrices', () {
      for (final filter in ['full', '*', '', '  ']) {
        final plan = buildCiPlan(filter: filter, automaticCiDisabled: false);

        expect(
          plan.enabledJobs,
          kCiJobs.map((job) => job.id).toSet(),
          reason: filter,
        );
        expect(plan.matrixByJob['test_dart_web'], {
          'include': [
            {'package': 'frb_dart'},
            {'package': 'frb_example--dart_minimal'},
            {'package': 'frb_example--pure_dart'},
            {'package': 'frb_example--pure_dart_pde'},
          ],
        }, reason: filter);
      }
    });

    test('automatic disable suppresses every job regardless of filter', () {
      for (final filter in [
        'full',
        'test_dart_web[package=frb_dart]',
        'lint_rust_primary',
      ]) {
        final plan = buildCiPlan(filter: filter, automaticCiDisabled: true);

        expect(plan.enabledJobs, isEmpty, reason: filter);
        expect(plan.matrixByJob['test_dart_web'], {'include': []});
      }
    });

    test('single non-matrix job filter enables just that job', () {
      final plan = buildCiPlan(
        filter: 'lint_rust_primary',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'lint_rust_primary'});
      expect(plan.matrixByJob.values, everyElement({'include': []}));
    });

    test('comma-separated job filters combine matrix and non-matrix jobs', () {
      final plan = buildCiPlan(
        filter:
            'lint_dart_primary,test_dart_web[package=frb_example--pure_dart]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'lint_dart_primary', 'test_dart_web'});
      expect(plan.matrixByJob['test_dart_web'], {
        'include': [
          {'package': 'frb_example--pure_dart'},
        ],
      });
    });

    test('matrix job without brackets enables its full matrix', () {
      final plan = buildCiPlan(
        filter: 'test_dart_valgrind',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_dart_valgrind'});
      expect(plan.matrixByJob['test_dart_valgrind'], {
        'include': [
          {'package': 'frb_example--dart_minimal'},
          {'package': 'frb_example--pure_dart'},
          {'package': 'frb_example--pure_dart_pde'},
        ],
      });
    });

    test('single package filter selects one dart web entry', () {
      final plan = buildCiPlan(
        filter: 'test_dart_web[package=frb_example--pure_dart_pde]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_dart_web'});
      expect(plan.matrixByJob['test_dart_web'], {
        'include': [
          {'package': 'frb_example--pure_dart_pde'},
        ],
      });
    });

    test('same dimension OR filter selects multiple entries', () {
      final plan = buildCiPlan(
        filter: 'test_dart_web[package=frb_dart|frb_example--pure_dart_pde]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_dart_web'});
      expect(plan.matrixByJob['test_dart_web'], {
        'include': [
          {'package': 'frb_dart'},
          {'package': 'frb_example--pure_dart_pde'},
        ],
      });
    });

    test('multiple dimensions are ANDed for dart native', () {
      final plan = buildCiPlan(
        filter:
            'test_dart_native[image=ubuntu-24.04,package=tools--frb_internal]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_dart_native'});
      expect(plan.matrixByJob['test_dart_native'], {
        'include': [
          {'image': 'ubuntu-24.04', 'package': 'tools--frb_internal'},
        ],
      });
    });

    test('excluded matrix combinations stay excluded before filtering', () {
      expect(
        () => buildCiPlan(
          filter:
              'test_dart_native[image=windows-2025,package=tools--frb_internal]',
          automaticCiDisabled: false,
        ),
        throwsFormatException,
      );
    });

    test('nested info dimensions can be filtered by platform and package', () {
      final plan = buildCiPlan(
        filter:
            'test_flutter_native_desktop[platform=linux,package=frb_example--gallery|frb_example--flutter_via_integrate]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_flutter_native_desktop'});
      expect(plan.matrixByJob['test_flutter_native_desktop'], {
        'include': [
          {
            'info': {
              'image': 'ubuntu-latest',
              'platform': 'linux',
              'package': 'frb_example--flutter_via_integrate',
            },
          },
          {
            'info': {
              'image': 'ubuntu-latest',
              'platform': 'linux',
              'package': 'frb_example--gallery',
            },
          },
        ],
      });
    });

    test('nested info dimensions can be filtered by image and version', () {
      final plan = buildCiPlan(
        filter: 'test_rust[image=ubuntu-latest,version=nightly|1.85.0]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_rust'});
      expect(plan.matrixByJob['test_rust'], {
        'include': [
          {
            'info': {'image': 'ubuntu-latest', 'version': 'nightly'},
          },
          {
            'info': {'image': 'ubuntu-latest', 'version': '1.85.0'},
          },
        ],
      });
    });

    test('numeric matrix values are matched as strings', () {
      final plan = buildCiPlan(
        filter:
            'test_flutter_native_android[package=frb_example--flutter_via_create,device=pixel,api-level=35]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_flutter_native_android'});
      expect(plan.matrixByJob['test_flutter_native_android'], {
        'include': [
          {
            'package': 'frb_example--flutter_via_create',
            'device': 'pixel',
            'api-level': 35,
          },
        ],
      });
    });

    test('values containing spaces are accepted verbatim', () {
      final plan = buildCiPlan(
        filter:
            'test_flutter_native_ios[device=iPhone 16 Pro Max Simulator (18.6),package=frb_example--rust_ui_counter--ui]',
        automaticCiDisabled: false,
      );

      expect(plan.enabledJobs, {'test_flutter_native_ios'});
      expect(plan.matrixByJob['test_flutter_native_ios'], {
        'include': [
          {
            'package': 'frb_example--rust_ui_counter--ui',
            'device': 'iPhone 16 Pro Max Simulator (18.6)',
          },
        ],
      });
    });

    test('duplicate filters merge values for the same dimension', () {
      final plan = buildCiPlan(
        filter:
            'test_dart_web[package=frb_dart,package=frb_example--pure_dart]',
        automaticCiDisabled: false,
      );

      expect(plan.matrixByJob['test_dart_web'], {
        'include': [
          {'package': 'frb_dart'},
          {'package': 'frb_example--pure_dart'},
        ],
      });
    });
  });

  group('CI filter validation', () {
    test('rejects unknown jobs', () {
      expect(
        () => buildCiPlan(filter: 'does_not_exist', automaticCiDisabled: false),
        throwsFormatException,
      );
    });

    test('rejects filters for non-matrix jobs', () {
      expect(
        () => buildCiPlan(
          filter: 'lint_rust_primary[package=frb_dart]',
          automaticCiDisabled: false,
        ),
        throwsFormatException,
      );
    });

    test('rejects unknown matrix dimensions', () {
      expect(
        () => buildCiPlan(
          filter: 'test_dart_web[image=ubuntu-latest]',
          automaticCiDisabled: false,
        ),
        throwsFormatException,
      );
    });

    test('rejects filters that match no matrix entries', () {
      expect(
        () => buildCiPlan(
          filter: 'test_dart_web[package=not_a_package]',
          automaticCiDisabled: false,
        ),
        throwsFormatException,
      );
    });

    test('rejects malformed bracket syntax', () {
      for (final filter in [
        'test_dart_web]',
        'test_dart_web[package=frb_dart',
        '[package=frb_dart]',
      ]) {
        expect(
          () => buildCiPlan(filter: filter, automaticCiDisabled: false),
          throwsFormatException,
          reason: filter,
        );
      }
    });

    test('rejects malformed conditions', () {
      for (final filter in [
        'test_dart_web[package]',
        'test_dart_web[=frb_dart]',
        'test_dart_web[package=]',
      ]) {
        expect(
          () => buildCiPlan(filter: filter, automaticCiDisabled: false),
          throwsFormatException,
          reason: filter,
        );
      }
    });
  });
}

Object _ciJobsSnapshot() => {
  'jobs': [
    for (final job in kCiJobs)
      {'id': job.id, if (job.matrix != null) 'matrix': job.matrix!.entries},
  ],
};

String _prettyJson(Object value) =>
    const JsonEncoder.withIndent('  ').convert(value);

String _readSnapshot(String name) {
  final snapshotPath = 'test/ci_plan_test_snapshot_$name.json';
  return File(snapshotPath).readAsStringSync().trim();
}
