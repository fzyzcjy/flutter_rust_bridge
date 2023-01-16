# [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge): High-level memory-safe binding generator for Flutter/Dart <-> Rust

[![Rust Package](https://img.shields.io/crates/v/flutter_rust_bridge.svg)](https://crates.io/crates/flutter_rust_bridge)
[![Flutter Package](https://img.shields.io/pub/v/flutter_rust_bridge.svg)](https://pub.dev/packages/flutter_rust_bridge)
[![Stars](https://img.shields.io/github/stars/fzyzcjy/flutter_rust_bridge)](https://github.com/fzyzcjy/flutter_rust_bridge)
[![CI](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/ci.yaml)
[![Example](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/post_release.yaml/badge.svg)](https://github.com/fzyzcjy/flutter_rust_bridge/actions/workflows/post_release.yaml)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6afbdad19e7245adbf9e9771777be3d7)](https://app.codacy.com/gh/fzyzcjy/flutter_rust_bridge?utm_source=github.com&utm_medium=referral&utm_content=fzyzcjy/flutter_rust_bridge&utm_campaign=Badge_Grade_Settings)

![Logo](https://github.com/fzyzcjy/flutter_rust_bridge/raw/master/book/logo.png)

Want to combine the best between [Flutter](https://flutter.dev/), a cross-platform hot-reload rapid-development UI toolkit, and [Rust](https://www.rust-lang.org/), a language empowering everyone to build reliable and efficient software? Here it comes!

## 🚀 Advantages

* **Memory-safe**: Never need to think about malloc/free.
* **Feature-rich**: `enum`s with values, platform-optimized `Vec`, possibly recursive `struct`, zero-copy big arrays, opaque types on arbitrary structs/classes, `Stream` (iterator) abstraction, error (`Result`) handling, cancellable tasks, concurrency control, and more. See full features [here](https://fzyzcjy.github.io/flutter_rust_bridge/feature.html).
* **Async programming**: Rust code will never block the Flutter. Call Rust naturally from Flutter's main isolate (thread); sync mode also equally supported.
* **Lightweight**: This is not a huge framework that includes everything, so you are free to use your favorite Flutter and Rust libraries. <sub>For example, state-management with Flutter library (e.g. MobX) can be elegant and simple (contrary to implementing in Rust); implementing a photo manipulation algorithm in Rust will be fast and safe (countrary to implementing in Flutter).</sub>
* **Cross-platform**: Android, iOS, Windows, Linux, MacOS, and Web.
* **Easy to code-review & convince yourself**: This package simply simulates how humans  write boilerplate code. If you want to convince yourself (or your team) that it is safe, there is not much code to look at. No magic at all! ([More about](https://fzyzcjy.github.io/flutter_rust_bridge/safety.html) safety concerns.)
* **Fast**: It is only a thin (though feature-rich) wrapper, without overhead such as protobuf serialization, thus performant. (More [benchmarks](https://github.com/fzyzcjy/flutter_rust_bridge/issues/318#issuecomment-1034536815) later) <small>(Throw away components like thread-pool to make it even faster)</small>
* **Pure-Dart compatible:** Despite the name, this package is 100% compatible with [pure](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/README.md) Dart.

## 💡 User Guide

Check out [the user guide](https://fzyzcjy.github.io/flutter_rust_bridge/) for [show-me-the-code](https://fzyzcjy.github.io/flutter_rust_bridge/quickstart.html), [tutorials](https://fzyzcjy.github.io/flutter_rust_bridge/tutorial_with_flutter.html), [features](https://fzyzcjy.github.io/flutter_rust_bridge/feature.html) and much more.

## 📎 P.S. Achieve ~60 FPS, no matter how janky the Flutter app was due to build/layout

Here is my another open-source library :) https://github.com/fzyzcjy/flutter_smooth.

## ✨ Contributors

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-46-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key) following [all-contributors](https://github.com/all-contributors/all-contributors) specification):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/fzyzcjy"><img src="https://avatars.githubusercontent.com/u/5236035?v=4?s=100" width="100px;" alt="fzyzcjy"/><br /><sub><b>fzyzcjy</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=fzyzcjy" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=fzyzcjy" title="Documentation">📖</a> <a href="#example-fzyzcjy" title="Examples">💡</a> <a href="#ideas-fzyzcjy" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-fzyzcjy" title="Maintenance">🚧</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Desdaemon"><img src="https://avatars.githubusercontent.com/u/36768030?v=4?s=100" width="100px;" alt="Viet Dinh"/><br /><sub><b>Viet Dinh</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Desdaemon" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Desdaemon" title="Tests">⚠️</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Desdaemon" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/SecondFlight"><img src="https://avatars.githubusercontent.com/u/6700184?v=4?s=100" width="100px;" alt="Joshua Wade"/><br /><sub><b>Joshua Wade</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=SecondFlight" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/smw-wagnerma"><img src="https://avatars.githubusercontent.com/u/66412697?v=4?s=100" width="100px;" alt="Marcel"/><br /><sub><b>Marcel</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=smw-wagnerma" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/rustui"><img src="https://avatars.githubusercontent.com/u/90625190?v=4?s=100" width="100px;" alt="rustui"/><br /><sub><b>rustui</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=rustui" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://adventures.michaelfbryan.com/"><img src="https://avatars.githubusercontent.com/u/17380079?v=4?s=100" width="100px;" alt="Michael Bryan"/><br /><sub><b>Michael Bryan</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Michael-F-Bryan" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://bus710.net"><img src="https://avatars.githubusercontent.com/u/8920680?v=4?s=100" width="100px;" alt="bus710"/><br /><sub><b>bus710</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=bus710" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://scholar.google.com/citations?user=RbAto7EAAAAJ"><img src="https://avatars.githubusercontent.com/u/1213857?v=4?s=100" width="100px;" alt="Sebastian Urban"/><br /><sub><b>Sebastian Urban</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=surban" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trobanga"><img src="https://avatars.githubusercontent.com/u/8888869?v=4?s=100" width="100px;" alt="Daniel"/><br /><sub><b>Daniel</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=trobanga" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/AlienKevin"><img src="https://avatars.githubusercontent.com/u/22850071?v=4?s=100" width="100px;" alt="Kevin Li"/><br /><sub><b>Kevin Li</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=AlienKevin" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=AlienKevin" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://valeth.me"><img src="https://avatars.githubusercontent.com/u/3198362?v=4?s=100" width="100px;" alt="Patrick Auernig"/><br /><sub><b>Patrick Auernig</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=valeth" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://antonok.com"><img src="https://avatars.githubusercontent.com/u/22821309?v=4?s=100" width="100px;" alt="Anton Lazarev"/><br /><sub><b>Anton Lazarev</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=antonok-edm" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Unoqwy"><img src="https://avatars.githubusercontent.com/u/65187632?v=4?s=100" width="100px;" alt="Unoqwy"/><br /><sub><b>Unoqwy</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Unoqwy" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://feber.dev"><img src="https://avatars.githubusercontent.com/u/1727318?v=4?s=100" width="100px;" alt="Febrian Setianto"/><br /><sub><b>Febrian Setianto</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=feber" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Syndim"><img src="https://avatars.githubusercontent.com/u/835035?v=4?s=100" width="100px;" alt="syndim"/><br /><sub><b>syndim</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=syndim" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/sagudev"><img src="https://avatars.githubusercontent.com/u/16504129?v=4?s=100" width="100px;" alt="sagu"/><br /><sub><b>sagu</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=sagudev" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=sagudev" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://bandism.net/"><img src="https://avatars.githubusercontent.com/u/22633385?v=4?s=100" width="100px;" alt="Ikko Ashimine"/><br /><sub><b>Ikko Ashimine</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=eltociear" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/alanlzhang"><img src="https://avatars.githubusercontent.com/u/59032810?v=4?s=100" width="100px;" alt="alanlzhang"/><br /><sub><b>alanlzhang</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=alanlzhang" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=alanlzhang" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/sccheruku"><img src="https://avatars.githubusercontent.com/u/5800058?v=4?s=100" width="100px;" alt="Sai Chaitanya"/><br /><sub><b>Sai Chaitanya</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=sccheruku" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://ares.zone (国内)"><img src="https://avatars.githubusercontent.com/u/40336192?v=4?s=100" width="100px;" alt="Ares Andrew"/><br /><sub><b>Ares Andrew</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=TENX-S" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/raphaelrobert"><img src="https://avatars.githubusercontent.com/u/9882746?v=4?s=100" width="100px;" alt="raphaelrobert"/><br /><sub><b>raphaelrobert</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=raphaelrobert" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/thomas725"><img src="https://avatars.githubusercontent.com/u/68635351?v=4?s=100" width="100px;" alt="thomas725"/><br /><sub><b>thomas725</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=thomas725" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://dport.me"><img src="https://avatars.githubusercontent.com/u/7816187?v=4?s=100" width="100px;" alt="Daniel Porteous (dport)"/><br /><sub><b>Daniel Porteous (dport)</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=banool" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/w-ensink"><img src="https://avatars.githubusercontent.com/u/46427708?v=4?s=100" width="100px;" alt="Wouter Ensink"/><br /><sub><b>Wouter Ensink</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=w-ensink" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/dbsxdbsx"><img src="https://avatars.githubusercontent.com/u/17372655?v=4?s=100" width="100px;" alt="老董"/><br /><sub><b>老董</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=dbsxdbsx" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=dbsxdbsx" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/lattice0"><img src="https://avatars.githubusercontent.com/u/6632321?v=4?s=100" width="100px;" alt="Lattice 0"/><br /><sub><b>Lattice 0</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=lattice0" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=lattice0" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://soeur.dev"><img src="https://avatars.githubusercontent.com/u/26034975?v=4?s=100" width="100px;" alt="orange soeur"/><br /><sub><b>orange soeur</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=juzi5201314" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Roms1383"><img src="https://avatars.githubusercontent.com/u/21016014?v=4?s=100" width="100px;" alt="Rom's"/><br /><sub><b>Rom's</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Roms1383" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Roms1383" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Cupnfish"><img src="https://avatars.githubusercontent.com/u/40173605?v=4?s=100" width="100px;" alt="Cupnfish"/><br /><sub><b>Cupnfish</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Cupnfish" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/SoLongAndThanksForAllThePizza"><img src="https://avatars.githubusercontent.com/u/103753680?v=4?s=100" width="100px;" alt="SoLongAndThanksForAllThePizza"/><br /><sub><b>SoLongAndThanksForAllThePizza</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=SoLongAndThanksForAllThePizza" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=SoLongAndThanksForAllThePizza" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Stonks3141"><img src="https://avatars.githubusercontent.com/u/82178396?v=4?s=100" width="100px;" alt="Sam Nystrom"/><br /><sub><b>Sam Nystrom</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Stonks3141" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://i.hsfzxjy.site"><img src="https://avatars.githubusercontent.com/u/4702188?v=4?s=100" width="100px;" alt="hsfzxjy"/><br /><sub><b>hsfzxjy</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=hsfzxjy" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://tmpfs.org"><img src="https://avatars.githubusercontent.com/u/238069?v=4?s=100" width="100px;" alt="muji"/><br /><sub><b>muji</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=tmpfs" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Zaitam"><img src="https://avatars.githubusercontent.com/u/71014214?v=4?s=100" width="100px;" alt="Zaitam"/><br /><sub><b>Zaitam</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=Zaitam" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/alexthe2"><img src="https://avatars.githubusercontent.com/u/33789063?v=4?s=100" width="100px;" alt="Alex Procelewski"/><br /><sub><b>Alex Procelewski</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=alexthe2" title="Documentation">📖</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://matrix.to/#/@vincentherl:matrix.org"><img src="https://avatars.githubusercontent.com/u/5569193?v=4?s=100" width="100px;" alt="Vincent Herlemont"/><br /><sub><b>Vincent Herlemont</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=vincent-herlemont" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/CicadaCinema"><img src="https://avatars.githubusercontent.com/u/52425971?v=4?s=100" width="100px;" alt="CicadaCinema"/><br /><sub><b>CicadaCinema</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=CicadaCinema" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=CicadaCinema" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/rogurotus"><img src="https://avatars.githubusercontent.com/u/61418195?v=4?s=100" width="100px;" alt="rogurotus"/><br /><sub><b>rogurotus</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=rogurotus" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=rogurotus" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/huang12zheng"><img src="https://avatars.githubusercontent.com/u/28038074?v=4?s=100" width="100px;" alt="huang12zheng"/><br /><sub><b>huang12zheng</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=huang12zheng" title="Code">💻</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=huang12zheng" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://svenstaro.org"><img src="https://avatars.githubusercontent.com/u/1664?v=4?s=100" width="100px;" alt="Sven-Hendrik Haase"/><br /><sub><b>Sven-Hendrik Haase</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=svenstaro" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ngasull"><img src="https://avatars.githubusercontent.com/u/912991?v=4?s=100" width="100px;" alt="Nicolas Gasull"/><br /><sub><b>Nicolas Gasull</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=ngasull" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/codercengiz"><img src="https://avatars.githubusercontent.com/u/45819755?v=4?s=100" width="100px;" alt="codercengiz"/><br /><sub><b>codercengiz</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=codercengiz" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://www.floeschner.de/"><img src="https://avatars.githubusercontent.com/u/12967904?v=4?s=100" width="100px;" alt="Fabian Löschner"/><br /><sub><b>Fabian Löschner</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=w1th0utnam3" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://gsconrad.com"><img src="https://avatars.githubusercontent.com/u/15874617?v=4?s=100" width="100px;" alt="Gregory Conrad"/><br /><sub><b>Gregory Conrad</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=GregoryConrad" title="Documentation">📖</a> <a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=GregoryConrad" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://www.zaynetro.com/"><img src="https://avatars.githubusercontent.com/u/627197?v=4?s=100" width="100px;" alt="Roman Zaynetdinov"/><br /><sub><b>Roman Zaynetdinov</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=zaynetro" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/jsonmona"><img src="https://avatars.githubusercontent.com/u/105187344?v=4?s=100" width="100px;" alt="jsonmona"/><br /><sub><b>jsonmona</b></sub></a><br /><a href="https://github.com/fzyzcjy/flutter_rust_bridge/commits?author=jsonmona" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

More specifically, thanks for all these contributions:

* [Desdaemon](https://github.com/Desdaemon): Support not only simple enums but also enums with fields which gets translated to native enum or freezed class in Dart. Support the Option type as nullable types in Dart. Support Vec of Strings type. Support comments in code. Add marker attributes for future usage. Add Linux and Windows support for with-flutter example, and make CI works for that. Avoid parameter collision. Overhaul the documentation and add several chapters to demonstrate configuring a Flutter+Rust project in all five platforms. Refactor command module. Precompiled binary CI workflow. Fix bugs. Add support for the Web platform, parallel to the existing mobile/desktop platforms, via WASM and JavaScript as intermediate values. GitHub retry actions. Implement draft of opaque types. Refactor Boxed and Option.
* [rogurotus](https://github.com/rogurotus): Add Rust opaque types, enabling arbitrary Rust structs to be used as opaque Dart objects by generating wrappers and raw Arc pointers, as well as Dart opaque types, allowing to use any Dart objects in Rust code. Extend `SyncReturn` for more types. Fix generation bug. Fix SyncReturn. Update CI.
* [ngasull](https://github.com/ngasull): Make sync mode support whatever types that classical async mode supports. Bump sdk.
* [SecondFlight](https://github.com/SecondFlight): Allow structs and enums to be imported from other files within the crate by creating source graph. Auto-create relavent dir. Fix `store_dart_post_cobject` error with ffigen 6.0.
* [Unoqwy](https://github.com/Unoqwy): Add struct mirrors, such that types in the external crates can be imported and used without redefining and copying.
* [antonok-edm](https://github.com/antonok-edm): Avoid converting syn types to strings before parsing to improve code and be more robust.
* [lattice0](https://github.com/lattice0): Support methods, such that Rust struct impls can be converted to Dart class methods. StreamSink at any argument.
* [sagudev](https://github.com/sagudev): Make code generator a `lib`. Add error types. Depend on `cbindgen`. Fix LLVM paths. Update deps. Fix CI errors.
* [surban](https://github.com/surban): Support unit return type. Skip unresolvable modules. Ignore prefer_const_constructors. Non-final Dart fields.
* [Roms1383](https://github.com/Roms1383): Fix build_runner calling bug. Remove global `ffigen` dependency. Improve version check. Fix enum name-variant conflicts. Support Chrono date time and UUID types. Migrate to Rust 1.64 workspace. Update and refactor CI. Update header comments. Code cleanup.
* [GregoryConrad](https://github.com/GregoryConrad): Add doc to setup frb inside a Dart/Flutter library.
* [trobanga](https://github.com/trobanga): Add support for `[T;N]` structs. Add `usize` support. Add a cmd argument. Separate dart tests.
* [dbsxdbsx](https://github.com/dbsxdbsx): Allow generating multiple Rust and Dart files. Fix lint. Update doc.
* [SoLongAndThanksForAllThePizza](https://github.com/SoLongAndThanksForAllThePizza): Refactor and enhance SyncReturn to support more types.
* [huang12zheng](https://github.com/huang12zheng): Support type aliases and nested ones. Tweak code generation.
* [hsfzxjy](https://github.com/hsfzxjy): Fix SyncReturn use-after-free bug.
* [Cupnfish](https://github.com/Cupnfish): Support arrays as function paramters. Allow multi mirror.
* [alanlzhang](https://github.com/alanlzhang): Add generation for Dart metadata. Enhance module parser.
* [Zaitam](https://github.com/Zaitam): Fix when method return struct.
* [AlienKevin](https://github.com/AlienKevin): Add flutter example for macOS. Add doc for Android NDK bug.
* [banool](https://github.com/banool): Fix pubspec parsing. Fix symbol-stripping doc.
* [efc-mw](https://github.com/efc-mw): Improve Windows encoding handling.
* [valeth](https://github.com/valeth): Rename callFfi's port.
* [sccheruku](https://github.com/sccheruku): Prevent double-generating utility.
* [jsonmona](https://github.com/jsonmona): Add import.
* [w-ensink](https://github.com/w-ensink): Improve doc. Fix CI. Refactor. Add tests.
* [codercengiz](https://github.com/codercengiz): Fix mirroring bug.
* [Michael-F-Bryan](https://github.com/Michael-F-Bryan): Detect broken bindings.
* [bus710](https://github.com/bus710): Add a case in troubleshooting.
* [Syndim](https://github.com/Syndim): Add a bracket to box.
* [TENX-S](https://github.com/TENX-S): Improve doc. Reproduce a bug.
* [CicadaCinema](https://github.com/CicadaCinema): Bump version. Improve doc.
* [w1th0utnam3](https://github.com/w1th0utnam3): Improve message.
* [vincent-herlemont](https://github.com/vincent-herlemont): Loosen version.
* [zaynetro](https://github.com/zaynetro): Improve doc.
* [raphaelrobert](https://github.com/raphaelrobert): Remove oudated doc.
* [alexthe2](https://github.com/alexthe2): Improve doc.
* [tmpfs](https://github.com/tmpfs): Improve doc.
* [thomas725](https://github.com/thomas725): Improve doc.
* [juzi5201314](https://github.com/juzi5201314): Improve doc.
* [svenstaro](https://github.com/svenstaro): Improve doc.
* [Stonks3141](https://github.com/Stonks3141): Fix doc credit.
* [feber](https://github.com/feber): Fix doc link.
* [rustui](https://github.com/rustui): Fix a typo.
* [eltociear](https://github.com/eltociear): Fix a typo.
