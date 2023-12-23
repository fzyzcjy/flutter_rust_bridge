window.BENCHMARK_DATA = {
  "lastUpdate": 1703347349555,
  "repoUrl": "https://github.com/fzyzcjy/flutter_rust_bridge",
  "entries": {
    "Flutter Rust Bridge Benchmark": [
      {
        "commit": {
          "author": {
            "email": "5236035+fzyzcjy@users.noreply.github.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a56090f1b2fa819be05dd7a158fcd5a93d38c84b",
          "message": "Merge pull request #1477 from fzyzcjy/feat/11600\n\nAdd frbSse bench test and fix frbCstSse test forget to be sync",
          "timestamp": "2023-12-22T22:15:12+08:00",
          "tree_id": "bfcb9ce6a8f666520c62549b36c0eb1b06342e99",
          "url": "https://github.com/fzyzcjy/flutter_rust_bridge/commit/a56090f1b2fa819be05dd7a158fcd5a93d38c84b"
        },
        "date": 1703256678238,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Windows\"}",
            "value": 18.996207531287865,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Windows\"}",
            "value": 209.1869150779896,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Windows\"}",
            "value": 2235.994,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 11.535502736747395,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.55634325,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.4530914607183067,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.24198402243436748,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.016213363439199886,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 374.5057105410972,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 36.12531835341293,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 49.84672133190439,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 209.36407411284412,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.63651725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.904003797998101,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 134.392399834779,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.062320948753581025,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.298488,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 144.04389770152153,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 34.58065910505567,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 78.30742727379507,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1998.5104895104896,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.68530725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 20.359830351484426,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1962.4730134932533,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.588281045768551,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 6.6636236118839065,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 159.02122754014945,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.908904,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 23.38500491996064,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 784.637,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.4094595,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 67.4405111897762,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2014.438,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.90711875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 23.733335683316295,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 752.2965,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.837843,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 15.792596037019814,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 486.98273092369476,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.272321,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 26.900641241984474,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 897.34725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.931463,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 24.0375125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 802.06925,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.523328,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 83.15790347346143,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2823.8375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 5.702269021029213,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 95.73168858596524,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 3192.02125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 6.554383752636556,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 8.870936,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 168.98575524928313,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.3663915,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 63.83259158926729,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6596.170658682635,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.6096895,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 62.751001784199595,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 8136.8,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.702064,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 68.36821224039191,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 12796.466367713005,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.107717,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 39.79220005954153,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6486.228739002932,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.8028805,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 321.4244511407662,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 29613.422535211266,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.533304,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2166.45,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 256766.77777777778,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.333294,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2503.194,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 296555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Linux\"}",
            "value": 18.56351460100454,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Linux\"}",
            "value": 205.30306446653685,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Linux\"}",
            "value": 2066.356,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 25.98351348542327,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.3620608662079657,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.2966305764441922,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.1721286556557427,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.014908341694093719,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 180.78974961583657,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 27.355399934347304,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 29.849113485762043,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 166.52535176088585,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4269598410822187,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.5632077183961408,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 120.55252894942102,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.05266205564438636,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.2309515,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 129.12377450980392,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 27.08404089647234,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 44.87827043037293,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1743.2038327526132,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.48100175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 19.696971424228607,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1906.0869565217392,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.040977709473091,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2.8620260844926917,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 65.56162722087458,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.60520525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 18.55679915640696,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 625.73475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.98422575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 46.00303594838888,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 1786.6979010494751,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5937695,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 18.131065951472387,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 600.1475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5640625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 13.248352634060513,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 444.1811946902655,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.88814775,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 22.854683931645084,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 792.14175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.471413,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 21.696809425524595,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 684.073,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.936372,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 74.87607092304253,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2431.396,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.276308,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 88.95221027479091,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2852.285,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4936484035462373,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.823831838084081,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 121.28829546150548,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.90885575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 74.5683694991599,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 7581.812734082397,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4760156708707777,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 54.08939467573843,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5073.1525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.48185692105417105,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 60.825283494330115,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 8089.461077844311,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.89844625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 33.269869817017835,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 4999.653846153846,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.54748975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 277.0853522695628,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 29537.369863013697,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9710837644581178,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2088.838,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 223654.83333333334,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.1332358667641333,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2254.371,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 274342.75,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Macos\"}",
            "value": 45.47418293889004,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Macos\"}",
            "value": 1041.7425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Macos\"}",
            "value": 10443.07,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 40.140368482318465,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.9639035,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.53479,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.3250530679118379,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.023745469244063632,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 661.8818663136996,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 41.92838724555041,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 49.14334856749718,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 265.71436163146006,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.151478,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2.598685,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 186.03255943020997,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.07941419604702965,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 1.4391727804136099,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 181.75302752293578,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 42.953353521036014,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 67.22516890188565,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 1797.7097933513028,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.167553,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 24.940547225622193,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 2219.655,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 6.896850904175345,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 10.516926960088343,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 96.86512664051529,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.579863710068145,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 48.138741363914086,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1516.1911544227887,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.439593,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 105.32886565928379,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3318.3094405594406,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.2002455,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 36.546835625315,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1111.7635,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.2550675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 28.809363143821137,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 879.10375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.121342,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 44.10331027882901,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1380.754,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.5295852352073824,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 30.973754841858053,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 898.6275,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.247264,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 111.53057453242245,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3414.91904047976,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.875968062015969,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 112.02531078508231,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3383.9820089955024,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.037878,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 4.2966440275169795,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 280.0053385782523,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.98505375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 76.49421189959942,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 7256.165584415585,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.8692285,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 66.7330931717701,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6321.4350282485875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.9244635,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 67.34273558674751,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 10074.546610169491,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.312622,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 42.73889122217555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6931.88418079096,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.6205935,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 357.3770807779919,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 37977.05454545454,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.260524,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2603.80875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 277415.5714285714,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.8124631406526444,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2618.757,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 323018.8333333333,
            "unit": "Microseconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ch271828n@outlook.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "committer": {
            "email": "ch271828n@outlook.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "distinct": true,
          "id": "229db75f5651289bf5a7468b368e04ce4d1c0b3d",
          "message": "Merge branch 'feat/11549'",
          "timestamp": "2023-12-23T07:30:50+08:00",
          "tree_id": "d25952a1a55a3abec4cb7a7066b9f90dd86f5d33",
          "url": "https://github.com/fzyzcjy/flutter_rust_bridge/commit/229db75f5651289bf5a7468b368e04ce4d1c0b3d"
        },
        "date": 1703293575904,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Windows\"}",
            "value": 20.967930514573254,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Windows\"}",
            "value": 208.58919032156697,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Windows\"}",
            "value": 2259.378,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.01683016489060393,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Windows\"}",
            "value": 0.049196490758035095,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.027677807654027404,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 0.21862381698073738,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Windows\"}",
            "value": 1.1342995,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 12.08455486942756,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.54968175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.4494449888193764,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.25129663957047826,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.016118160346242284,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 378.1569294762715,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 35.63100603944344,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 49.20775514221041,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 215.99773218142548,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.6561435,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.8690020654989672,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 136.35612038265484,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.055283737906922994,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.3076375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 148.59346208967446,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 34.627053792482556,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 77.86708195444812,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1979.1177052423343,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.683931,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 20.32734,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1946.0014992503748,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.5206135674478043,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 6.707475350459454,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 68.35530947742575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.89200875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 24.281137148578566,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 789.27475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.4410155,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 69.679008,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2028.678,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.94371975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 26.412412700698393,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 716.294,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.92242275,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 16.281757379895065,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 476.45369237046106,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.2810355555555555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 26.60305808776765,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 906.22725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.9834265,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 24.335395655323897,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 779.578,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.292585,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 89.68861465294576,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2606.3575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.771615,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 91.91970903101839,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 3249.2413793103447,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.776762,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2.435572,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 194.0301071118402,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.204952,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 70.73177889576884,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 5972.299450549451,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.69742675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 58.90766464634748,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 8713.628,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.63091775,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 74.5092879256966,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 11231.911016949152,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.258427,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 35.311145510835914,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6817.6725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.73606075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 275.58803659394795,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 32827.82089552239,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.220974,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2418.627,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 258824.88888888888,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.303905,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2489.84,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 287158.25,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Linux\"}",
            "value": 20.55413536146616,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Linux\"}",
            "value": 205.54433371005857,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Linux\"}",
            "value": 2053.362,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.016674470891615938,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Linux\"}",
            "value": 0.04525990586422028,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.027800299895748875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 0.17146003432743875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Linux\"}",
            "value": 1.0247945,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 25.972067111653637,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.36585740977984726,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.3006786519566258,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.16642647656142232,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.014273405985726594,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 181.68650072674419,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 25.837495316960997,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 28.712903411048583,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 164.1662152179266,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4294315096454792,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.580070709964645,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 120.20303593928121,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.053214339986159424,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.228489,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 129.07868937048502,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 26.693716299183173,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 45.27114692500962,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1649.0981038746909,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.49978575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 26.026217520596695,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1914.7991666666667,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9668052608080813,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2.7386155750544985,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 62.748360784313725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.58061375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 17.140410376716986,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 564.90875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.989471,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 44.9437744749021,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 1750.6079460269866,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.604641,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 17.83981916131108,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 600.17575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.548529,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 12.731280977954622,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 393.2898435953277,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.8732215,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 23.441149073680094,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 827.225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.466761,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 28.209723902760974,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 715.95125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.936244781877609,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 74.74122774342547,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2431.936,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.260665,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 90.83054298642534,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2898.00875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.49484048244852313,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.6748294125852936,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 126.24455083784805,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.9134515,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 76.26098860441024,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 6032.593516209477,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4882791615187139,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 53.5353359726086,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5096.92,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4721464596821222,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 62.27416451670967,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 8694.395209580838,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.8804585,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 32.994271548691835,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 4760.655,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.55925525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 300.7087872185911,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 29609.652777777777,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9274780362609818,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2088.592,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 215330.1,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.076067,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2201.315,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 298992.44444444444,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Macos\"}",
            "value": 47.76309789578682,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Macos\"}",
            "value": 1075.7785,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Macos\"}",
            "value": 10515.59,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.023907066994023233,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Macos\"}",
            "value": 0.0576348648473027,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.02683212418472254,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 0.24318661439412873,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Macos\"}",
            "value": 1.383623308188346,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 42.680032436354324,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.99078425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.546634,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.3302055499696146,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.024375791804993666,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 722.721098265896,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 43.468832862421216,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 49.215315714356024,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 264.29228329809723,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1671675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2.640595,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 184.8775020754543,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.07708285192291715,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 1.4541055,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 181.4436819471132,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 42.35706722012792,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 69.21983802865647,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 1809.8065099457503,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1972825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 25.0695875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 2224.15,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 7.0053415622580975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 12.290689314828128,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 119.13789611627352,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.5896219551890225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 50.34203885635887,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1545.119940029985,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.483136,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 106.63223447106212,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3358.67916041979,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.167305,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 37.7766527918401,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1151.3145,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.2549448725275638,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 29.203121082053777,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 899.914,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.262751,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 44.022294410822354,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1363.651,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.697036,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 31.050084749576254,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 904.55675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.071912,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 111.33816556760587,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3453.991004497751,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.616759191620404,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 111.13507637417773,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3370.8455772113944,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.8698908150545925,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 4.036305472770896,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 280.5329693406132,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.62532,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 72.66228670170639,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6897.191558441558,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.80445075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 65.51312,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 8616.978813559323,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.90233975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 67.47639313738529,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 10053.733333333334,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.288273,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 42.00209995800084,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6650.5192307692305,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.422061,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 350.81982300884954,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 36693.15517241379,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.190506,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2620.5075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 279850.25,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.451718774140613,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2659.26375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 312377,
            "unit": "Microseconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "5236035+fzyzcjy@users.noreply.github.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "791551a74022a745400b92ef1380db14f498ebe5",
          "message": "Merge pull request #1479 from fzyzcjy/feat/11602\n\nAllow skip all-contributor for local script",
          "timestamp": "2023-12-23T09:18:33+08:00",
          "tree_id": "03c84e864997c99576e224a84c9d8f6364d1acba",
          "url": "https://github.com/fzyzcjy/flutter_rust_bridge/commit/791551a74022a745400b92ef1380db14f498ebe5"
        },
        "date": 1703295843786,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Windows\"}",
            "value": 20.604001516987484,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Windows\"}",
            "value": 206.71499023939177,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Windows\"}",
            "value": 2215.562,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.016624592,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Windows\"}",
            "value": 0.04717145135374243,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.026893180184005133,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 0.2127524091919229,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Windows\"}",
            "value": 1.122491,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 13.611983774807394,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.5269955,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.39772625507609,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.24546623776106674,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.016005919189977805,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 352.01566349876805,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 36.947350919973395,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 42.54553383394669,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 203.26930894308944,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.65338975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.9471535264232367,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 131.03204986760812,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.061689136349815606,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.288439,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 130.46340481803026,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 36.700798238370496,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 72.71442594343053,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 2103.4395373291272,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.68027975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 21.733066835665188,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1803.656671664168,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.844808579942677,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 6.221100697695396,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 60.56763058289175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.9149705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 23.119475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 763.58725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.584709,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 60.95015528427785,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2011.738,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.9150949235041737,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 23.971112158883127,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 760.9475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.78511975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 18.06340904028446,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 490.573441108545,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.0906807337591056,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 28.83761729906161,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 823.279,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.8075015,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 25.317616753107856,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 702.3245,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.394624,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 79.00738634119367,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2825.46625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.442065,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 97.25507297970807,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2861.92625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.828296,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2.16346,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 169.95024,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.182137,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 66.80329633160112,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 5812.142857142857,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.66686625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 57.957421729005546,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 8212.014981273409,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.62087525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 73.94046148846279,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 10792.609137055837,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.1136444431777783,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 39.71371821570446,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6112.844943820225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.7959855,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 301.00830765107867,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 28663.773333333334,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.4801875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2104.803,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 255341.88888888888,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.223925,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2420.475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 269351.28571428574,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Linux\"}",
            "value": 20.372701018391854,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Linux\"}",
            "value": 202.54908942924,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Linux\"}",
            "value": 2023.554,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.016999883889500755,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Linux\"}",
            "value": 0.04541924986374225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.02626214894141975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 0.17118245915270164,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Linux\"}",
            "value": 1.0185895,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 25.256936113250912,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.3514189557779698,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.29050475118981733,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.16168708406503327,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.01409083169715938,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 176.66928716544476,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 25.987915643394533,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 28.313528129335484,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 161.08851481958763,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4328376563655523,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.5936239531880234,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 120.6817397512941,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.09800406487340142,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.3456375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 129.76426861534586,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 25.68099230858126,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 43.62627606666085,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1725.5025862068965,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.510095,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 20.93160054719562,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1867.3988005997,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.023618653302696,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2.868406257708834,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 67.35304418103448,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.61070625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 19.026982027398272,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 628.7145,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.97837175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 45.440566992912586,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 1803.5247376311845,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.598637,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 17.83958816298582,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 601.663,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.57145725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 13.17573628345044,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 444.56164994425865,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.88104275,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 23.98721609268333,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 828.40175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.5326545,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 22.624760752392476,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 705.78325,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9311020344489829,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 74.05938774760207,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2364.639,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.276639,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 89.31142970927917,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2838.34625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.48939009836667535,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.7345271327364336,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 131.20794965220273,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.9773705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 72.0753287038669,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 6654.93006993007,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.47127521091777824,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 55.22010779892201,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5088.3675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.502014004125767,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 60.561468770624586,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 8172.7784431137725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.920194,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 32.52448041079641,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 4728.092657342658,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5977605,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 292.8510158013544,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 30975.51388888889,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9302912848543576,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2050.113,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 213952.55555555556,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.105991,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2201.1675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 257444,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Macos\"}",
            "value": 46.8761247122829,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Macos\"}",
            "value": 1094.834,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Macos\"}",
            "value": 10194.125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.022268603455462794,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Macos\"}",
            "value": 0.056427895548576835,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.026392224189891365,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 0.24246157682922656,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Macos\"}",
            "value": 1.3875975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 42.54768438743166,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.9857785,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.546743,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.33020415867918335,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.02426258380589933,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 685.0123287671233,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 43.55116172723908,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 48.69669596552312,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 269.363367003367,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.221066,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2.59898,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 185.05954128440368,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.07688125646237487,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 1.4476835,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 181.32361990950227,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 45.06717140990108,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 70.75407690402915,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 1881.4844778927563,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1993175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 26.6177,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 2331.591,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 6.823975979664603,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 11.213384241894158,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 108.27311606756172,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.6303686848156576,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 50.41450315555006,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1551.5419790104947,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.5487925,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 108.43499059392636,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3453.8140929535234,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1570735,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 38.04662562699498,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1150.9985,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.2372015,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 29.07083200417597,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 911.6715,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.104893,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 45.052977038091846,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1404.924,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.885126,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 31.391066731765708,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 914.983,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.205894345579241,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 114.54539318258522,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3557.7727272727275,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.7456779407415444,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 113.56595495758994,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3406.9775112443776,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.0062,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 4.174584,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 279.34483242182023,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.052049973975013,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 75.05319415150323,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 7021.863636363636,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.80982375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 68.14880188997637,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 8707.631355932202,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.84127275,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 68.83240184916856,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 10066.175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.212624,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 43.30544805641555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6984.89010989011,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.5951457024271487,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 367.7660280029477,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 37708.96296296296,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.446042,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2683.79375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 288294.25,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.7228089578932817,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2572.30875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 318431.14285714284,
            "unit": "Microseconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ch271828n@outlook.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "committer": {
            "email": "ch271828n@outlook.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "distinct": true,
          "id": "c03314c98c7c518dbe0c8c3f73ea313d66875172",
          "message": "chore: doc",
          "timestamp": "2023-12-23T17:09:31+08:00",
          "tree_id": "06842723846daf562c91e253cdb511e50ce232d2",
          "url": "https://github.com/fzyzcjy/flutter_rust_bridge/commit/c03314c98c7c518dbe0c8c3f73ea313d66875172"
        },
        "date": 1703325707322,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Windows\"}",
            "value": 18.86986054111567,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Windows\"}",
            "value": 227.9816808114262,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Windows\"}",
            "value": 2087.181,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.019205884375161752,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Windows\"}",
            "value": 0.04376254041247492,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.029241783970758217,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 0.19659290481549876,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Windows\"}",
            "value": 1.2436225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 11.300032205027374,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.5704845,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.44628746241628126,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.255893632,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.016320924438796534,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 372.78028326500186,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 36.54237009427757,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 49.98547935619314,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 208.91926049717986,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.66445,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.8731593134203433,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 130.01269265639166,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.055575872860877465,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.284165,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 132.08095752111245,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 41.43763726018315,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 76.62965517241379,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 2077.845275181724,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.68305825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 22.45206,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1834.839580209895,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.8848568246989656,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 6.720646793753843,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 68.82354358074396,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.90679825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 27.6236125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 713.16875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.7355111322444339,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 60.16776388320156,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2225.51,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.86154325,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 26.929379050139037,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 717.9315,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.7830333333333334,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 17.15151424242879,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 496.2839651518719,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.1006770398849033,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 29.665438838244647,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 842.67375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.950032,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 26.637597377419013,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 720.563,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.522248,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 80.44303998728039,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2943.3225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.477879,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 97.78131576539482,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2863.3575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.82713775,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2.57341375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 164.26622898104105,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.4202365,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 64.01486754653983,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6448.0467032967035,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.6391235,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 59.70852059491266,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 8831.419491525423,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.680879,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 75.40271470122978,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 11683.448430493274,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.2533315,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 36.39347521255258,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6382.662686567164,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.805781,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 274.43135090382225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 32189.89230769231,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.199515,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2395.417,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 265472.2,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.30128,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2489.326,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 292092.625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Linux\"}",
            "value": 20.593259601685098,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Linux\"}",
            "value": 205.36004919544942,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Linux\"}",
            "value": 2056.783,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.01730531997404202,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Linux\"}",
            "value": 0.046261831861214506,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.026765993435499576,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 0.17248740504045343,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Linux\"}",
            "value": 1.041982,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 25.44830833047041,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.3644270805203259,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.3098716976405472,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.16785197581081246,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.01430113998569886,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 181.1896910952079,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 26.948057722624196,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 29.337572610455904,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 162.35189544605893,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.42400648862680446,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.5495667252166374,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 122.13099893352293,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.05367328124857406,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.247322,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 127.61210235570512,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 27.080510195791696,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 44.46898345784418,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1711.234388366125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5046431647076095,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 20.434507523939807,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1888.3305847076463,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.061098131924642,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2.864869820315365,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 67.52310860538132,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.579932,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 16.301528,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 562.498,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.9938415,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 45.67321408482394,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 1757.944527736132,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.6126375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 18.063800489596083,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 599.13025,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.58143825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 13.213295076877078,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 434.93195785777,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.878697,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 23.87015421415182,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 801.29225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.5223725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 22.933221534227727,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 705.54375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9750752624623689,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 73.99037154092095,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2419.409,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.250547,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 89.76361694553222,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2855.285,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.499727171815888,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.7452416273791862,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 131.04641683103222,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.9644915,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 69.81905838542359,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5869.752913752914,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.49547792139696756,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 54.97057060330263,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5070.6825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4996176950586838,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 60.507290217194026,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 8017.416167664671,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.893156,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 32.97898893384683,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 4575.705069124424,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5704245,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 288.4731413837559,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 29550.090909090908,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9285535357232322,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2114.903,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 227969.55555555556,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.050625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2229.214,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 264832.85714285716,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Macos\"}",
            "value": 46.092228432116656,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Macos\"}",
            "value": 1031.579,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Macos\"}",
            "value": 10102.142180094786,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.021898421824812626,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Macos\"}",
            "value": 0.055030634036963906,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.027761746972238252,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 0.23741337233602977,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Macos\"}",
            "value": 1.3606665,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 39.193556605067705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.977187,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.53488975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.32361536060638774,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.02377910380976717,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 642.1534510433387,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 39.800640783267994,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 43.87071442672575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 258.0579280092891,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.120661,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2.55549125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 182.3140127388535,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.07554167379040604,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 1.394383,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 180.81257413997628,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 39.79038675791819,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 67.69227645028091,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 1768.202296819788,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1375135,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 23.913191629360686,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 2160.367,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 6.367654924734469,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 11.195255474452555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 95.47696787436155,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.495640252179874,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 48.32696244163849,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1480.9305,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.414811,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 102.84717880387434,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3326.8710644677662,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1450585,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 37.07556239550084,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1117.199,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.2450815,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 28.06224362653824,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 875.50525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.9703525148237426,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 44.63159013153701,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1375.0035,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.664336,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 30.888623390415503,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 899.25575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.24046031965476,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 112.30514267115113,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3419.935532233883,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.058738,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 113.25241490340386,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3239.1244377811095,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.21478,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 4.295296,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 281.29645587213344,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.98024875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 74.134374886722,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6747.185064935065,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.849092,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 66.93383711721225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 8604.684,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.829683,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 66.97945205479452,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 11353.242424242424,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.118728,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 42.0920645333224,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6421.535,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.422766,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 351.9894865954091,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 36863.44642857143,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.406836,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2610.427,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 279693.75,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.669755997683002,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2577.9425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 327255,
            "unit": "Microseconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "5236035+fzyzcjy@users.noreply.github.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2be4044320645c39186701ff20acbd48a5013508",
          "message": "Merge pull request #1483 from fzyzcjy/feat/11610\n\nFix benchmark dashboard",
          "timestamp": "2023-12-23T21:02:58+08:00",
          "tree_id": "3432262c19850f2145cf9940372829572098a730",
          "url": "https://github.com/fzyzcjy/flutter_rust_bridge/commit/2be4044320645c39186701ff20acbd48a5013508"
        },
        "date": 1703338975342,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Windows\"}",
            "value": 18.61845914771203,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Windows\"}",
            "value": 222.23630946265283,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Windows\"}",
            "value": 2066.115,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.019164694971252956,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Windows\"}",
            "value": 0.042278648661770814,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.02895405865978981,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 0.1917686338634037,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Windows\"}",
            "value": 1.212864,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 8.68353732399564,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.54470166834651,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.43917548536700285,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.24671657152909143,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.01625761018903396,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 390.0528471138846,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 35.888727390180875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 46.60733594332588,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 209.34404437931755,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.637291,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.856087821956089,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 128.9082490033732,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.05427676146298696,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.2662795,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 139.2124796880078,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 33.92176184257391,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 78.23505711156314,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1931.0067567567567,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.6627115,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 19.92231267976405,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1945.1724137931035,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.4328395959468216,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 6.666065613657347,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 91.44925018288222,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.84693675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 22.335074819401445,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 756.629,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.358739,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 65.15024580973464,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 1978.880059970015,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.89699525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 25.54738548937981,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 701.908,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.816784,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 15.320990074059555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 461.8340653045637,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.1838895,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 25.933011803858353,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 881.6845,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.8097555,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 22.80786149017314,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 756.949,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.223489,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 78.82630354007594,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2850.93375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.4452075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 97.59476083012522,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2844.72875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.8091595,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2.144401,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 162.26371654955898,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.2024825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 69.64472233596875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 5869.233516483517,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.6675965,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 58.112142215004496,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 8573.378277153559,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.61178525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 72.86241219642801,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 11094.622881355932,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.2610615,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 34.884823707996965,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6508.6,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.8698495090364282,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 267.98981642771,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 31375.72,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.232406,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2360.013,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 230552.66666666666,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.461711,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2233.54,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 278023.8888888889,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Linux\"}",
            "value": 20.607425098143725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Linux\"}",
            "value": 205.22608153078204,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Linux\"}",
            "value": 2054.996,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.017297882474053176,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Linux\"}",
            "value": 0.047465145857604564,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.03469762865302371,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 0.17187277017982464,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Linux\"}",
            "value": 1.0211955,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 25.213163733548484,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.36132129981555594,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.29395314941768663,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.16460209056634845,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.014299893985700106,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 178.7822472512738,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 26.18902958006521,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 29.504373995013793,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 162.24929017603634,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.43459254053637436,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.5741989629005186,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 120.10981657405748,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.053667242158318945,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.241449,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 129.23948658410734,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 26.40385757851796,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 43.72375497354729,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1708.7523484201538,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.49415375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 19.61858,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1898.9587706146926,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.0642944506659924,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2.8251101439817385,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 65.05330948121646,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5844225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 16.077928,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 560.25325,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.987735,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 45.05081879672481,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 1757.2398800599701,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.58879425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 17.324404148069927,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 596.64725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.55950675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 13.023273953452094,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 434.5434971726838,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.9298295,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 23.060637424203218,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 788.8055,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.4349135,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 30.135815641843582,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 711.6705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9108955445522278,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 74.58489160396334,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2473.378,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.26505,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 89.30744927020292,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2865.6925,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.51399475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.7377573811213094,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 124.39756835076226,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.9126905,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 70.65257937020485,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5908.083916083916,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.46945940610811876,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 53.554006783972866,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5448.99,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.47613318719785214,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 60.88105237895242,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 9318.465034965035,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.8919015,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 33.155906083455704,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 5518.76,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.554352,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 307.78670485378353,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 29783.676056338027,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9581882709058644,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2108.097,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 218558.2,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.08438,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2227.987,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 272973.3333333333,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Macos\"}",
            "value": 46.69659905951646,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Macos\"}",
            "value": 1057.35,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Macos\"}",
            "value": 10488.137362637362,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.021868918456262165,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Macos\"}",
            "value": 0.05698212936035741,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.02706283318201171,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 0.24210490143158728,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Macos\"}",
            "value": 1.351719,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 42.143158160900164,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.973239,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.63313675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.41749845570317323,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.023587255494103187,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 666.022977022977,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 43.33567342693708,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 52.438673308862086,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 266.03312051077415,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1482425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2.6137375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 185.78584678167317,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.07491786933877302,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 1.467855016072492,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 184.53527312654407,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 44.96322025134327,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 70.83541703559412,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 1777.5301953818828,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1986175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 24.697734418124654,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 2262.485,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 6.49464841238399,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 12.273954107777376,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 102.57172162675009,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.6810199094900453,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 48.978979346840774,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1514.5674662668666,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.600985,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 105.08184586308273,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3351.1079460269866,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.251261,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 36.973252213982285,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1140.569,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.3212485,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 28.825823643410853,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 896.0545,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.20642,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 44.69082327776615,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1407.746,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.905435,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 31.284328161569817,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 914.52425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.054701458973906,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 109.33637663623364,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3545.9265367316343,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.7031669726247705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 114.1706878552372,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3418.6221889055473,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.864192317903841,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 4.13187,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 290.29860812168175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.7127525,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 74.47450791771496,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 7309.811688311688,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.89462675,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 65.84200026599282,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6318.232,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.92703075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 67.89387290208033,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 10037.5875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.155967672016164,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 43.77569864720541,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 7259.225274725275,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.327749,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 365.756114852889,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 39141.269230769234,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.30659,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2665.4775,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 284131.125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.586442060168455,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2706.5575,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 332781.71428571426,
            "unit": "Microseconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "5236035+fzyzcjy@users.noreply.github.com",
            "name": "fzyzcjy",
            "username": "fzyzcjy"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "15c27036a03da4d74d25702fb14a6e391df55bee",
          "message": "Merge pull request #1480 from fzyzcjy/feat/11605\n\nFurther improve code coverage by adding tests",
          "timestamp": "2023-12-23T22:56:32+08:00",
          "tree_id": "50e413b0ec553f46c6a7cc85d13ee66001adca42",
          "url": "https://github.com/fzyzcjy/flutter_rust_bridge/commit/15c27036a03da4d74d25702fb14a6e391df55bee"
        },
        "date": 1703347341294,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Windows\"}",
            "value": 20.610023217308456,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Windows\"}",
            "value": 206.80314394328573,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Windows\"}",
            "value": 2197.971,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.016658622641718954,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Windows\"}",
            "value": 0.04666743862666049,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.02688032768415615,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 0.2111217957879166,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Windows\"}",
            "value": 1.119745,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 8.283609660331095,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.52471625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.46329174902207265,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.2524040539693429,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Windows\"}",
            "value": 0.014562980534602993,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Windows\"}",
            "value": 402.5377339504931,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 36.3188784775187,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 37.4639318160532,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 215.73562722467912,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.57106375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2.0103604948197527,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 128.8735342828751,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.048360354166834915,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 1.3398195,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 129.223191315806,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 36.5903951701427,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 73.61819051825677,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 2015.0654582074521,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.6427955,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 20.4927411885146,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 1856.804347826087,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.4701044879694365,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 7.453471818493508,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 68.14108548260707,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.84546725,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 22.216158838411616,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 741.34425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.4720425,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 59.49156028797871,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2091.867,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.84529325,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 26.631569671119884,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 696.52375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.82625975,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 15.348077313458806,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 490.5945,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.0870705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 28.6749923500612,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 866.5545,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.724927,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 25.996146027941297,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 709.61025,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.4146170484455216,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 78.76293272627365,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2873.67125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.455612,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Windows\"}",
            "value": 96.93996974012104,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Windows\"}",
            "value": 2849.165,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.79998825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2.84365875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 165.4274786260687,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.2516535,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 66.74751698902247,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 5824.876373626374,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.673573,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 57.82962356313434,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 8436.887640449439,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.63669625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 75.19858764519536,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 10878.944915254237,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 1.213913,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 34.31128450972392,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 6521.5225,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 0.767371,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 267.3104186170924,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 30941.493333333332,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.201413,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2256.858,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 229358.1,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Windows\"}",
            "value": 2.494385,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Windows\"}",
            "value": 2234.148,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Windows\"}",
            "value": 303253.875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Linux\"}",
            "value": 20.15289816214108,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Linux\"}",
            "value": 204.96966280619043,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Linux\"}",
            "value": 2042.254,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.016659056475011414,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Linux\"}",
            "value": 0.044922541865232375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.026280818191200386,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 0.16760095697883565,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Linux\"}",
            "value": 0.9947585,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 24.884015776442336,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.34872579002787973,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.28591025802836534,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.16318074064325186,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Linux\"}",
            "value": 0.013861777430691113,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Linux\"}",
            "value": 169.75768120862332,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 24.749928844559392,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 27.844294704015148,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 158.85219601302518,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4139062948874964,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.540844,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 114.00307985216709,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.05189334090322658,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.209657,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 122.0871667699938,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 25.915735869593387,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 42.43042620446783,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1674.789958158996,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.47899234628556137,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 20.02909826721386,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 1872.7016491754123,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9298983715517541,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2.708252311492192,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 63.222784169932986,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5583435,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 16.006719983200043,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 551.067,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.043757,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 47.8888044459222,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 1811.8793103448277,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.57803,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 17.97610519115847,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 582.835,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.5300073188582581,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 12.764034502853363,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 378.84586466165416,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.849116,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 22.50527820777434,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 780.1005,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.37178075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 28.57751551360262,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 680.099,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.8671300664349668,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 71.34019319613607,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2334.722,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.241278,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Linux\"}",
            "value": 87.74160577189899,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Linux\"}",
            "value": 2804.365,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4878853649785281,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 1.6284569357715322,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 130.49006150857883,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.87813625,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 77.0466098686546,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 6394.4058441558445,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.4515100483386774,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 51.353322896677106,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 4954.305,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.45801854476122505,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 59.70819583608328,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 7631.508982035928,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.8856266666666667,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 32.212976,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 4651.764411027569,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 0.55067375,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 287.83390641351474,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 29604.833333333332,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 1.9411850294074853,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2055.489,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 215146.8,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Linux\"}",
            "value": 2.132113,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Linux\"}",
            "value": 2211.659,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Linux\"}",
            "value": 265555.55555555556,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"90000049\",\"platform\":\"Macos\"}",
            "value": 47.42255260532941,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"9000000001\",\"platform\":\"Macos\"}",
            "value": 1050.779,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"PrimeNumber\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"900000000013\",\"platform\":\"Macos\"}",
            "value": 10286.77,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.02189396595621207,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"IntParse\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"1000000000\",\"platform\":\"Macos\"}",
            "value": 0.055864108882717825,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.026475301688915206,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 0.24151294067896475,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Base64Encode\",\"approach\":\"Na\",\"direction\":null,\"asynchronous\":false,\"arg\":\"100\",\"platform\":\"Macos\"}",
            "value": 1.3545715,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 38.63218790442526,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Frb\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.9617505,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.54424175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"FrbCstSse\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.3228677564845468,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":false,\"platform\":\"Macos\"}",
            "value": 0.023639977744090004,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"VoidFunction\",\"approach\":\"Raw\",\"direction\":null,\"asynchronous\":true,\"platform\":\"Macos\"}",
            "value": 650.8188028627196,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 41.24327222485719,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 49.52778465653014,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 256.7284045693749,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1307115,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2.65669125,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 183.72971481140755,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.07477418530812933,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 1.406186,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 182.36651583710406,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 43.430544396430044,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 68.25342115141794,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 1864.7213420316868,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.1891875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 25.02594379244966,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 2195.242,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 6.618300225352672,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 12.321504700649342,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Bytes\",\"approach\":\"Raw\",\"direction\":\"Output\",\"asynchronous\":true,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 101.32377526723745,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.668130415934792,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 49.23181414548684,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1533.2016491754123,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.53025,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 106.92071352502663,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3293.422788605697,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.221942,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 36.88318093455252,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1134.6075,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.2653135,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 28.819781691746467,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 886.85175,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.176665,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 44.727152116229504,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 1385.263,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.669199,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 30.754750358373744,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 907.1186666666666,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.283018,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 113.01657019634548,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3433.8125937031487,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.709249968062524,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"5\",\"platform\":\"Macos\"}",
            "value": 111.63176795580111,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"BinaryTree\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10\",\"platform\":\"Macos\"}",
            "value": 3316.8275862068967,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.9540775229612386,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 3.9853350109987415,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 283.2555941626129,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.8781010609494695,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 73.03070642527511,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Frb\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6785.172077922078,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.867911,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 66.54239388965193,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6200.943502824859,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 0.87242875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 68.18900539345415,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"FrbSse\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 9719.24152542373,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 2.113705,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 43.176298589611285,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 6998.826923076923,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 1.412547,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 358.08024036762106,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Protobuf\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 37924.236363636366,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 4.33152,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2644.07875,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Input\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 286510.75,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"0\",\"platform\":\"Macos\"}",
            "value": 3.588699558475331,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"10000\",\"platform\":\"Macos\"}",
            "value": 2665.8025,
            "unit": "Microseconds"
          },
          {
            "name": "{\"area\":\"PureDart\",\"task\":\"Blob\",\"approach\":\"Json\",\"direction\":\"Output\",\"asynchronous\":false,\"arg\":\"1000000\",\"platform\":\"Macos\"}",
            "value": 349613.5714285714,
            "unit": "Microseconds"
          }
        ]
      }
    ]
  }
}