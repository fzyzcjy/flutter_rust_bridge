"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[8056],{3905:(e,t,r)=>{r.d(t,{Zo:()=>c,kt:()=>d});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var u=n.createContext({}),l=function(e){var t=n.useContext(u),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},c=function(e){var t=l(e.components);return n.createElement(u.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,u=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),f=l(r),d=o,b=f["".concat(u,".").concat(d)]||f[d]||p[d]||a;return r?n.createElement(b,i(i({ref:t},c),{},{components:r})):n.createElement(b,i({ref:t},c))}));function d(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=f;var s={};for(var u in t)hasOwnProperty.call(t,u)&&(s[u]=t[u]);s.originalType=e,s.mdxType="string"==typeof e?e:o,i[1]=s;for(var l=2;l<a;l++)i[l]=r[l];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},7139:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>u,contentTitle:()=>i,default:()=>p,frontMatter:()=>a,metadata:()=>s,toc:()=>l});var n=r(7462),o=(r(7294),r(3905));const a={},i="Protobuf / JSON / etc",s={unversionedId:"guides/how-to/protobuf-json",id:"guides/how-to/protobuf-json",title:"Protobuf / JSON / etc",description:"If you want to use Protobuf, JSON, or whatever serialization methods, it is also quite easy:",source:"@site/docs/guides/how-to/protobuf-json.md",sourceDirName:"guides/how-to",slug:"/guides/how-to/protobuf-json",permalink:"/flutter_rust_bridge/guides/how-to/protobuf-json",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/how-to/protobuf-json.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Stateful Rust",permalink:"/flutter_rust_bridge/guides/how-to/stateful-rust"},next:{title:"Android NDK Init",permalink:"/flutter_rust_bridge/guides/how-to/ndk-init"}},u={},l=[{value:"Drawbacks",id:"drawbacks",level:2}],c={toc:l};function p(e){let{components:t,...r}=e;return(0,o.kt)("wrapper",(0,n.Z)({},c,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"protobuf--json--etc"},"Protobuf / JSON / etc"),(0,o.kt)("p",null,"If you want to use Protobuf, JSON, or whatever serialization methods, it is also quite easy:\nSince ",(0,o.kt)("inlineCode",{parentName:"p"},"flutter_rust_bridge")," supports complicated types, it surely supports the simple ",(0,o.kt)("inlineCode",{parentName:"p"},"Vec<u8>"),"/",(0,o.kt)("inlineCode",{parentName:"p"},"String"),"."),(0,o.kt)("p",null,"For example, the code below uses JSON to serialize the arguments/results:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-dart"},'// Dart\nvar result = jsonDecode(await api.f(a: jsonEncode({"x": [100, 200, "what"], "y": "hello"})));\n')),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'// Rust\npub fn f(a: String) -> Result<String> {\n    let arg = serde_json::from_str(&a)?;\n    Ok(json!({"some": "result", "is": [42]}).to_string())\n}\n')),(0,o.kt)("p",null,"If you want to even automatically generate those serialization calls (",(0,o.kt)("inlineCode",{parentName:"p"},"jsonEncode"),", ",(0,o.kt)("inlineCode",{parentName:"p"},"jsonDecode"),", etc),\nfeel free to create an issue to tell me! (I usually reply quickly within hours)"),(0,o.kt)("h2",{id:"drawbacks"},"Drawbacks"),(0,o.kt)("p",null,"However, it is generally preferred to avoid Protobuf/JSON,\nand use the flutter_rust_bridge features directly."),(0,o.kt)("p",null,"On one hand, by doing so, we can use all features that flutter_rust_bridge provides,\nwhich is a long list ;)"),(0,o.kt)("p",null,"On the other hand, for example,\nplease refer to ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/flutter/flutter/issues/60758"},"https://github.com/flutter/flutter/issues/60758"),"\nto see why the official Flutter chooses to write a custom serialization protocol,\ninstead of using Protobuf.\n(P.S. In flutter_rust_bridge, the SSE codec is serialization-based which is like the Flutter protocol in that text,\nand the CST/DCO codecs mimic how humans write C glue code in the standard way.\nBoth can be freely configured and used.)"))}p.isMDXComponent=!0}}]);