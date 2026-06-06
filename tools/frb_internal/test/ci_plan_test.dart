import 'dart:convert';

import 'package:flutter_rust_bridge_internal/src/makefile_dart/ci_plan.dart';
import 'package:test/test.dart';

void main() {
  group('CI job matrix snapshot', () {
    test('kCiJobs stays in sync with ci.yaml matrices', () {
      expect(_prettyJson(_ciJobsSnapshot()), _expectedCiJobsSnapshot.trim());
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

const _expectedCiJobsSnapshot = r'''
{
  "jobs": [
    {
      "id": "deploy_website"
    },
    {
      "id": "lint_rust_primary"
    },
    {
      "id": "lint_dart_primary"
    },
    {
      "id": "lint_rust_feature_flag"
    },
    {
      "id": "generate_run_frb_codegen_command_generate",
      "matrix": [
        {
          "image": "windows-2025",
          "package": "frb_example--dart_minimal"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--pure_dart"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--dart_build_rs"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--flutter_via_create"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--flutter_package"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--rust_ui_counter--ui"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--rust_ui_todo_list--ui"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--dart_minimal"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--pure_dart"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--dart_build_rs"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--flutter_via_create"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--flutter_package"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--rust_ui_counter--ui"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--rust_ui_todo_list--ui"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--dart_minimal"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--pure_dart"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--dart_build_rs"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--deliberate_bad"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--integrate_third_party"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_via_create"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_via_integrate"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_package"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--rust_ui_counter--ui"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--rust_ui_todo_list--ui"
        }
      ]
    },
    {
      "id": "generate_run_frb_codegen_command_integrate",
      "matrix": [
        {
          "image": "macos-15-intel",
          "package": "frb_example--flutter_via_create",
          "platforms": "default"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--flutter_via_integrate",
          "platforms": "default"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--flutter_package",
          "platforms": "default"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--flutter_via_create",
          "platforms": "default"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--flutter_via_integrate",
          "platforms": "default"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--flutter_package",
          "platforms": "default"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_via_create",
          "platforms": "default"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_via_integrate",
          "platforms": "default"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_package",
          "platforms": "default"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--flutter_via_create",
          "platforms": "ohos"
        }
      ]
    },
    {
      "id": "generate_internal"
    },
    {
      "id": "bench_dart_native",
      "matrix": [
        {
          "image": "windows-2025"
        },
        {
          "image": "macos-15-intel"
        },
        {
          "image": "ubuntu-24.04"
        }
      ]
    },
    {
      "id": "bench_upload"
    },
    {
      "id": "build_flutter",
      "matrix": [
        {
          "info": {
            "image": "windows-2025",
            "target": "windows"
          }
        },
        {
          "info": {
            "image": "windows-11-arm",
            "target": "windows"
          }
        },
        {
          "info": {
            "image": "macos-15-intel",
            "target": "macos"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "target": "linux"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "target": "android-aab"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "target": "android-apk"
          }
        },
        {
          "info": {
            "image": "macos-15-intel",
            "target": "ios"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "target": "ohos"
          }
        }
      ]
    },
    {
      "id": "test_mimic_quickstart",
      "matrix": [
        {
          "image": "windows-2025"
        },
        {
          "image": "macos-15-intel"
        },
        {
          "image": "ubuntu-latest"
        }
      ]
    },
    {
      "id": "test_rust",
      "matrix": [
        {
          "info": {
            "image": "macos-15-intel",
            "version": ""
          }
        },
        {
          "info": {
            "image": "windows-2025",
            "version": ""
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "version": ""
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "version": "nightly"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "version": "1.85.0"
          }
        }
      ]
    },
    {
      "id": "test_dart_native",
      "matrix": [
        {
          "image": "windows-2025",
          "package": "frb_dart"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--dart_minimal"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--pure_dart"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "image": "windows-2025",
          "package": "frb_example--dart_build_rs"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_dart"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--dart_minimal"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--pure_dart"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "image": "macos-15-intel",
          "package": "frb_example--dart_build_rs"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_dart"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_utils"
        },
        {
          "image": "ubuntu-24.04",
          "package": "tools--frb_internal"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--dart_minimal"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--pure_dart"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "image": "ubuntu-24.04",
          "package": "frb_example--dart_build_rs"
        }
      ]
    },
    {
      "id": "test_dart_web",
      "matrix": [
        {
          "package": "frb_dart"
        },
        {
          "package": "frb_example--dart_minimal"
        },
        {
          "package": "frb_example--pure_dart"
        },
        {
          "package": "frb_example--pure_dart_pde"
        }
      ]
    },
    {
      "id": "test_dart_valgrind",
      "matrix": [
        {
          "package": "frb_example--dart_minimal"
        },
        {
          "package": "frb_example--pure_dart"
        },
        {
          "package": "frb_example--pure_dart_pde"
        }
      ]
    },
    {
      "id": "test_dart_sanitizer",
      "matrix": [
        {
          "sanitizer": "asan",
          "package": "frb_example--dart_minimal"
        },
        {
          "sanitizer": "asan",
          "package": "frb_example--pure_dart"
        },
        {
          "sanitizer": "asan",
          "package": "frb_example--pure_dart_pde"
        },
        {
          "sanitizer": "lsan",
          "package": "frb_example--dart_minimal"
        },
        {
          "sanitizer": "lsan",
          "package": "frb_example--pure_dart"
        },
        {
          "sanitizer": "lsan",
          "package": "frb_example--pure_dart_pde"
        }
      ]
    },
    {
      "id": "test_flutter_native_android",
      "matrix": [
        {
          "package": "frb_example--flutter_via_create",
          "device": "pixel",
          "api-level": 35
        },
        {
          "package": "frb_example--flutter_via_create",
          "device": "Nexus 6",
          "api-level": 35
        },
        {
          "package": "frb_example--flutter_package--example",
          "device": "pixel",
          "api-level": 35
        },
        {
          "package": "frb_example--flutter_package--example",
          "device": "Nexus 6",
          "api-level": 35
        },
        {
          "package": "frb_example--rust_ui_counter--ui",
          "device": "pixel",
          "api-level": 35
        },
        {
          "package": "frb_example--rust_ui_counter--ui",
          "device": "Nexus 6",
          "api-level": 35
        },
        {
          "package": "frb_example--rust_ui_todo_list--ui",
          "device": "pixel",
          "api-level": 35
        },
        {
          "package": "frb_example--rust_ui_todo_list--ui",
          "device": "Nexus 6",
          "api-level": 35
        }
      ]
    },
    {
      "id": "test_flutter_native_ios",
      "matrix": [
        {
          "package": "frb_example--flutter_via_create",
          "device": "iPad (10th generation) Simulator (18.6)"
        },
        {
          "package": "frb_example--flutter_via_create",
          "device": "iPhone 16 Pro Max Simulator (18.6)"
        },
        {
          "package": "frb_example--flutter_package--example",
          "device": "iPad (10th generation) Simulator (18.6)"
        },
        {
          "package": "frb_example--flutter_package--example",
          "device": "iPhone 16 Pro Max Simulator (18.6)"
        },
        {
          "package": "frb_example--rust_ui_counter--ui",
          "device": "iPad (10th generation) Simulator (18.6)"
        },
        {
          "package": "frb_example--rust_ui_counter--ui",
          "device": "iPhone 16 Pro Max Simulator (18.6)"
        },
        {
          "package": "frb_example--rust_ui_todo_list--ui",
          "device": "iPad (10th generation) Simulator (18.6)"
        },
        {
          "package": "frb_example--rust_ui_todo_list--ui",
          "device": "iPhone 16 Pro Max Simulator (18.6)"
        }
      ]
    },
    {
      "id": "test_flutter_native_desktop",
      "matrix": [
        {
          "info": {
            "image": "windows-2025",
            "platform": "windows",
            "package": "frb_example--flutter_via_create"
          }
        },
        {
          "info": {
            "image": "macos-15-intel",
            "platform": "macos",
            "package": "frb_example--flutter_via_create"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--flutter_via_create"
          }
        },
        {
          "info": {
            "image": "windows-2025",
            "platform": "windows",
            "package": "frb_example--flutter_package--example"
          }
        },
        {
          "info": {
            "image": "macos-15-intel",
            "platform": "macos",
            "package": "frb_example--flutter_package--example"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--flutter_package--example"
          }
        },
        {
          "info": {
            "image": "windows-2025",
            "platform": "windows",
            "package": "frb_example--rust_ui_counter--ui"
          }
        },
        {
          "info": {
            "image": "macos-15-intel",
            "platform": "macos",
            "package": "frb_example--rust_ui_counter--ui"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--rust_ui_counter--ui"
          }
        },
        {
          "info": {
            "image": "windows-2025",
            "platform": "windows",
            "package": "frb_example--rust_ui_todo_list--ui"
          }
        },
        {
          "info": {
            "image": "macos-15-intel",
            "platform": "macos",
            "package": "frb_example--rust_ui_todo_list--ui"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--rust_ui_todo_list--ui"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--flutter_via_integrate"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--gallery"
          }
        },
        {
          "info": {
            "image": "ubuntu-latest",
            "platform": "linux",
            "package": "frb_example--integrate_third_party"
          }
        }
      ]
    },
    {
      "id": "test_flutter_web",
      "matrix": [
        {
          "package": "frb_example--flutter_via_create"
        },
        {
          "package": "frb_example--gallery"
        }
      ]
    },
    {
      "id": "misc_codecov"
    }
  ]
}
''';
