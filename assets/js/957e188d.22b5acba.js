"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[5418],{15680:(e,t,r)=>{r.d(t,{xA:()=>p,yg:()=>d});var a=r(96540);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function n(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function l(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?n(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):n(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function o(e,t){if(null==e)return{};var r,a,i=function(e,t){if(null==e)return{};var r,a,i={},n=Object.keys(e);for(a=0;a<n.length;a++)r=n[a],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);for(a=0;a<n.length;a++)r=n[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var s=a.createContext({}),u=function(e){var t=a.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):l(l({},t),e)),r},p=function(e){var t=u(e.components);return a.createElement(s.Provider,{value:t},e.children)},m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},y=a.forwardRef((function(e,t){var r=e.components,i=e.mdxType,n=e.originalType,s=e.parentName,p=o(e,["components","mdxType","originalType","parentName"]),y=u(r),d=i,c=y["".concat(s,".").concat(d)]||y[d]||m[d]||n;return r?a.createElement(c,l(l({ref:t},p),{},{components:r})):a.createElement(c,l({ref:t},p))}));function d(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var n=r.length,l=new Array(n);l[0]=y;var o={};for(var s in t)hasOwnProperty.call(t,s)&&(o[s]=t[s]);o.originalType=e,o.mdxType="string"==typeof e?e:i,l[1]=o;for(var u=2;u<n;u++)l[u]=r[u];return a.createElement.apply(null,l)}return a.createElement.apply(null,r)}y.displayName="MDXCreateElement"},96405:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>l,default:()=>m,frontMatter:()=>n,metadata:()=>o,toc:()=>u});var a=r(58168),i=(r(96540),r(15680));const n={},l="Platform setup",o={unversionedId:"manual/integrate/library/platform-setup/index",id:"manual/integrate/library/platform-setup/index",title:"Platform setup",description:"In this subsection, we will be exploring how to set up our Flutter wrapper package",source:"@site/docs/manual/integrate/07-library/03-platform-setup/index.md",sourceDirName:"manual/integrate/07-library/03-platform-setup",slug:"/manual/integrate/library/platform-setup/",permalink:"/flutter_rust_bridge/manual/integrate/library/platform-setup/",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/manual/integrate/07-library/03-platform-setup/index.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Flutter wrapper",permalink:"/flutter_rust_bridge/manual/integrate/library/creating-libraries/flutter-wrapper"},next:{title:"Windows & Linux",permalink:"/flutter_rust_bridge/manual/integrate/library/platform-setup/windows-and-linux"}},s={},u=[{value:"Binary distribution",id:"binary-distribution",level:2},{value:"How it works",id:"how-it-works",level:2}],p={toc:u};function m(e){let{components:t,...r}=e;return(0,i.yg)("wrapper",(0,a.A)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,i.yg)("h1",{id:"platform-setup"},"Platform setup"),(0,i.yg)("p",null,"In this subsection, we will be exploring how to set up our Flutter wrapper package\nto bundle the platform-specific Rust binaries so users of the library will be able\nto actually use the library.\nAfter creating the Flutter wrapper on the previous page, you may have noticed that\nyou get a runtime error when trying to use it as-is because of ",(0,i.yg)("inlineCode",{parentName:"p"},"DynamicLibrary"),";\nthis is because the binaries are not yet distributed along with the package!"),(0,i.yg)("h2",{id:"binary-distribution"},"Binary distribution"),(0,i.yg)("p",null,"Unfortunately, at the time of writing, pub.dev has a hard 100 MB upload limit and\ndiscourages distributing platform-specific binaries through pub.dev directly.\nIn the future, hopefully with Native Assets, there will be a way to distribute\nour Rust binaries through pub.dev, or something similar, which will make\ndistribution as a library author ",(0,i.yg)("em",{parentName:"p"},"much")," more convenient."),(0,i.yg)("p",null,"In the meantime, however, we will need to work around these limitations. There are\nmany ways to distribute the binaries ourselves, outside of pub.dev, but in this\nsubsection, we will cover using GitHub releases because it easily integrates with\nour CI/CD solution, GitHub Actions (more on this ",(0,i.yg)("a",{parentName:"p",href:"ci"},"later"),")."),(0,i.yg)("h2",{id:"how-it-works"},"How it works"),(0,i.yg)("p",null,"If you look in your Flutter wrapper's pubspec (",(0,i.yg)("inlineCode",{parentName:"p"},"/packages/flutter_library_name/pubspec.yaml"),"),\nyou should notice the following section near the bottom\n(if you don't see this, or it is incomplete, add it now!):"),(0,i.yg)("pre",null,(0,i.yg)("code",{parentName:"pre",className:"language-yaml"},"flutter:\n  plugin:\n    platforms:\n      ios:\n        ffiPlugin: true\n      macos:\n        ffiPlugin: true\n      android:\n        ffiPlugin: true\n      linux:\n        ffiPlugin: true\n      windows:\n        ffiPlugin: true\n")),(0,i.yg)("p",null,"This section of the pubspec tells Flutter that our package is using the newer\nffi plugins format instead of the older platform channels Flutter has.\nThis makes the work on our end much simpler, because instead of having to\nspecify platform channels for each platform supported, we now only have to\nbundle the binaries with our package."),(0,i.yg)("p",null,"But, the key here is that we still must bundle our binaries along with our package.\nTo do so, we have to follow a certain procedure (",(0,i.yg)("em",{parentName:"p"},"read this; it is important"),"):"),(0,i.yg)("ol",null,(0,i.yg)("li",{parentName:"ol"},"We have a series of build scripts (",(0,i.yg)("inlineCode",{parentName:"li"},"/scripts/build-*.sh"),") that build all of our\nplatform specific binaries into ",(0,i.yg)("inlineCode",{parentName:"li"},"/platform-build")," and package them up appropriately,\nbased on the target platform.\nExample: on iOS/macOS, this bundle is an XCFramework, on Windows/Linux, it is a ",(0,i.yg)("inlineCode",{parentName:"li"},".tar.gz"),"."),(0,i.yg)("li",{parentName:"ol"},"These binaries are uploaded to somewhere online; as mentioned previously, we will use\nGitHub releases in this guide (which is ",(0,i.yg)("a",{parentName:"li",href:"ci"},"automated in ci"),")."),(0,i.yg)("li",{parentName:"ol"},"When the Dart tooling builds our library (such as when an application consuming\nour library is built), it invokes the platform specific build process.\nWe hijack this build process by downloading a copy of the binaries for the needed platform,\n",(0,i.yg)("em",{parentName:"li"},"if not already present on the filesystem"),". This last part is the key; it allows us to run\nintegration tests locally and in CI by providing our own copy of the binaries instead of\nforcing our build process to always fetch the binaries from GitHub releases."),(0,i.yg)("li",{parentName:"ol"},"After the binaries are stored locally (either by being copied to the proper folder(s)\nor by fetching them from online), we extract them and place them in the needed locations.")),(0,i.yg)("p",null,"Here are the relevant directories per platform.\nThis is helpful for if you want to test your library on a real device or emulator locally.\nAlso note: replace ",(0,i.yg)("inlineCode",{parentName:"p"},"library_name")," below with your library name, and replace ",(0,i.yg)("inlineCode",{parentName:"p"},"library_tag")," below with\n",(0,i.yg)("inlineCode",{parentName:"p"},"library_name-vVERSION")," where ",(0,i.yg)("inlineCode",{parentName:"p"},"VERSION")," is the current version in your Dart-only ",(0,i.yg)("inlineCode",{parentName:"p"},"pubspec.yaml"),".\nThis setup is a bit of a pain to test locally with but I am not sure there is a better way at the moment\n(other than creating a melos script to copy over all the binaries for you).\nThe idea here is that you will do most of your integration tests in CI."),(0,i.yg)("ul",null,(0,i.yg)("li",{parentName:"ul"},"Binary archive locations (copy created archives from ",(0,i.yg)("inlineCode",{parentName:"li"},"/platform-build")," to these locations to test locally)",(0,i.yg)("ul",{parentName:"li"},(0,i.yg)("li",{parentName:"ul"},"iOS (",(0,i.yg)("inlineCode",{parentName:"li"},"/plaform-build/LibraryName.xcframework.zip"),"): ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/ios/Frameworks/library_tag.zip")," "),(0,i.yg)("li",{parentName:"ul"},"macOS (",(0,i.yg)("inlineCode",{parentName:"li"},"/platform-build/LibraryName.xcframework.zip"),"): ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/macos/Frameworks/library_tag.zip")," "),(0,i.yg)("li",{parentName:"ul"},"Android (",(0,i.yg)("inlineCode",{parentName:"li"},"/platform-build/android.tar.gz"),"): ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/android/library_tag.tar.gz")," "),(0,i.yg)("li",{parentName:"ul"},"Windows (",(0,i.yg)("inlineCode",{parentName:"li"},"/platform-build/other.tar.gz"),"): ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/windows/library_tag.tar.gz")," "),(0,i.yg)("li",{parentName:"ul"},"Linux (",(0,i.yg)("inlineCode",{parentName:"li"},"/platform-build/other.tar.gz"),"): ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/linux/library_tag.tar.gz")," "))),(0,i.yg)("li",{parentName:"ul"},"Extracted binary locations (not as important to know, but helps understand the build process)",(0,i.yg)("ul",{parentName:"li"},(0,i.yg)("li",{parentName:"ul"},"iOS: ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/ios/Frameworks/")," "),(0,i.yg)("li",{parentName:"ul"},"macOS: ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/macos/Frameworks/")," "),(0,i.yg)("li",{parentName:"ul"},"Android: ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/android/src/main/jniLibs/"),(0,i.yg)("ul",{parentName:"li"},(0,i.yg)("li",{parentName:"ul"},"If you know what an ",(0,i.yg)("inlineCode",{parentName:"li"},"aar")," is, Flutter does something similar for android plug-ins under the hood"))),(0,i.yg)("li",{parentName:"ul"},"Windows: ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/windows/")," "),(0,i.yg)("li",{parentName:"ul"},"Linux: ",(0,i.yg)("inlineCode",{parentName:"li"},"/packages/flutter_library_name/linux/")," ")))),(0,i.yg)("p",null,"Always use melos to build the latest version(s) of the binaries (e.g. ",(0,i.yg)("inlineCode",{parentName:"p"},"melos run build:android"),")\n",(0,i.yg)("em",{parentName:"p"},"before copying the binary archives from ",(0,i.yg)("inlineCode",{parentName:"em"},"/platform-build/")," and testing locally"),"!\nAlso, ",(0,i.yg)("em",{parentName:"p"},"do not check the ",(0,i.yg)("inlineCode",{parentName:"em"},"/platform-build/")," or ",(0,i.yg)("inlineCode",{parentName:"em"},"/target/")," directories into source control"),"!"))}m.isMDXComponent=!0}}]);