"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[5785],{15680:(e,r,t)=>{t.d(r,{xA:()=>u,yg:()=>d});var a=t(96540);function n(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function i(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);r&&(a=a.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,a)}return t}function l(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?i(Object(t),!0).forEach((function(r){n(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):i(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function o(e,r){if(null==e)return{};var t,a,n=function(e,r){if(null==e)return{};var t,a,n={},i=Object.keys(e);for(a=0;a<i.length;a++)t=i[a],r.indexOf(t)>=0||(n[t]=e[t]);return n}(e,r);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)t=i[a],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(n[t]=e[t])}return n}var p=a.createContext({}),s=function(e){var r=a.useContext(p),t=r;return e&&(t="function"==typeof e?e(r):l(l({},r),e)),t},u=function(e){var r=s(e.components);return a.createElement(p.Provider,{value:r},e.children)},y={inlineCode:"code",wrapper:function(e){var r=e.children;return a.createElement(a.Fragment,{},r)}},c=a.forwardRef((function(e,r){var t=e.components,n=e.mdxType,i=e.originalType,p=e.parentName,u=o(e,["components","mdxType","originalType","parentName"]),c=s(t),d=n,g=c["".concat(p,".").concat(d)]||c[d]||y[d]||i;return t?a.createElement(g,l(l({ref:r},u),{},{components:t})):a.createElement(g,l({ref:r},u))}));function d(e,r){var t=arguments,n=r&&r.mdxType;if("string"==typeof e||n){var i=t.length,l=new Array(i);l[0]=c;var o={};for(var p in r)hasOwnProperty.call(r,p)&&(o[p]=r[p]);o.originalType=e,o.mdxType="string"==typeof e?e:n,l[1]=o;for(var s=2;s<i;s++)l[s]=t[s];return a.createElement.apply(null,l)}return a.createElement.apply(null,t)}c.displayName="MDXCreateElement"},95787:(e,r,t)=>{t.r(r),t.d(r,{assets:()=>p,contentTitle:()=>l,default:()=>y,frontMatter:()=>i,metadata:()=>o,toc:()=>s});var a=t(58168),n=(t(96540),t(15680));const i={},l="Flutter wrapper",o={unversionedId:"manual/integrate/library/creating-libraries/flutter-wrapper",id:"manual/integrate/library/creating-libraries/flutter-wrapper",title:"Flutter wrapper",description:"On this page, we will start creating the Flutter wrapper around our Dart-only library package.",source:"@site/docs/manual/integrate/07-library/02-creating-libraries/02-flutter-wrapper.md",sourceDirName:"manual/integrate/07-library/02-creating-libraries",slug:"/manual/integrate/library/creating-libraries/flutter-wrapper",permalink:"/flutter_rust_bridge/manual/integrate/library/creating-libraries/flutter-wrapper",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/manual/integrate/07-library/02-creating-libraries/02-flutter-wrapper.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Dart-only base",permalink:"/flutter_rust_bridge/manual/integrate/library/creating-libraries/dart-only"},next:{title:"Platform setup",permalink:"/flutter_rust_bridge/manual/integrate/library/platform-setup/"}},p={},s=[{value:"Additional setup steps",id:"additional-setup-steps",level:2}],u={toc:s};function y(e){let{components:r,...t}=e;return(0,n.yg)("wrapper",(0,a.A)({},u,t,{components:r,mdxType:"MDXLayout"}),(0,n.yg)("h1",{id:"flutter-wrapper"},"Flutter wrapper"),(0,n.yg)("p",null,"On this page, we will start creating the Flutter wrapper around our Dart-only library package.\nWe start with the plugin_ffi Flutter template since it is somewhat similar to what we need,\nbut we will need to modify it significantly in the coming steps.\nConfiguring the build processes for each supported platform is also a bit involved,\nso those are covered individually in the coming pages."),(0,n.yg)("p",null,"Run ",(0,n.yg)("inlineCode",{parentName:"p"},"flutter create --help")," to see all the available options; you may want to set some (like ",(0,n.yg)("inlineCode",{parentName:"p"},"--org"),")."),(0,n.yg)("p",null,"Finally, in the ",(0,n.yg)("inlineCode",{parentName:"p"},"packages")," folder, run the following, adding any other options you choose\nand replacing ",(0,n.yg)("inlineCode",{parentName:"p"},"library_name")," with your library name:"),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-bash"},"flutter create --template=plugin_ffi --platforms=android,ios,macos,linux,windows --org=com.example flutter_library_name\n")),(0,n.yg)("h2",{id:"additional-setup-steps"},"Additional setup steps"),(0,n.yg)("ol",null,(0,n.yg)("li",{parentName:"ol"},"Add your Dart-only base as a dependency in your new Flutter package's ",(0,n.yg)("inlineCode",{parentName:"li"},"pubspec.yaml"),".\n",(0,n.yg)("em",{parentName:"li"},"Use the version syntax, e.g. ",(0,n.yg)("inlineCode",{parentName:"em"},"^1.0.0")),". Melos will take care of the dependency resolution for us."),(0,n.yg)("li",{parentName:"ol"},"If you choose to have integration testing in CI (recommended),\nadd an ",(0,n.yg)("inlineCode",{parentName:"li"},"integration_test")," folder to your Flutter package's and/or Flutter example package's root directory,\nthen add the following to the  ",(0,n.yg)("inlineCode",{parentName:"li"},"pubspec.yaml")," of the applicable package(s):")),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-yaml"},"dev_dependencies:\n  flutter_test:\n    sdk: flutter\n  integration_test:\n    sdk: flutter\n")),(0,n.yg)("ol",{start:3},(0,n.yg)("li",{parentName:"ol"},"In ",(0,n.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/lib/flutter_library_name.dart"),",\nadd the following near the top of the file, replacing ",(0,n.yg)("inlineCode",{parentName:"li"},"library_name")," with your Dart-only package's name:")),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-dart"},"export 'package:library_name/library_name.dart';\n")),(0,n.yg)("p",null,"This re-exports your Dart-only package to users of your Flutter package,\nso they only need to do one ",(0,n.yg)("inlineCode",{parentName:"p"},"flutter pub add"),"."),(0,n.yg)("ol",{start:4},(0,n.yg)("li",{parentName:"ol"},"Finally, we will need to write some code to be able to handle FFI in Flutter.\nModify the following as needed (replacing ",(0,n.yg)("inlineCode",{parentName:"li"},"library_name")," and ",(0,n.yg)("inlineCode",{parentName:"li"},"LibraryName")," with your library name).")),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-dart"},"// lib/src/ffi/stub.dart\nObject createLibraryImpl() => throw UnimplementedError();\n")),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-dart"},"// lib/src/ffi/io.dart\nimport 'dart:ffi';\nimport 'dart:io';\n\nDynamicLibrary createLibraryImpl() {\n  const base = 'library_name';\n\n  if (Platform.isIOS || Platform.isMacOS) {\n    return DynamicLibrary.open('$base.framework/$base');\n  } else if (Platform.isWindows) {\n    return DynamicLibrary.open('$base.dll');\n  } else {\n    return DynamicLibrary.open('lib$base.so');\n  }\n}\n")),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-dart"},"// lib/src/ffi/web.dart\nimport 'package:library_name/library_name.dart';\n\nWasmModule createLibraryImpl() {\n  // TODO add web support. See:\n  // https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter/lib/ffi.web.dart\n  throw UnsupportedError('Web support is not provided yet.');\n}\n")),(0,n.yg)("pre",null,(0,n.yg)("code",{parentName:"pre",className:"language-dart"},"// lib/src/ffi.dart\nimport 'package:library_name/library_name.dart';\nimport 'ffi/stub.dart'\n    if (dart.library.io) 'ffi/io.dart'\n    if (dart.library.js_interop) 'ffi/web.dart';\n\nLibraryName createLib() =>\n    createWrapper(createLibraryImpl());\n")),(0,n.yg)("ol",{start:5},(0,n.yg)("li",{parentName:"ol"},"Run ",(0,n.yg)("inlineCode",{parentName:"li"},"melos bs"))),(0,n.yg)("p",null,"Now, inside your Flutter library, you can call ",(0,n.yg)("inlineCode",{parentName:"p"},"createLib()")," to get an instance of the FRB-generated Dart class!\nHowever, it won't work just yet; we will wire up our Flutter package to use our Rust binaries in the next subsection."))}y.isMDXComponent=!0}}]);