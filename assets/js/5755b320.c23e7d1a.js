"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[5146],{15680:(e,t,r)=>{r.d(t,{xA:()=>p,yg:()=>f});var n=r(96540);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function l(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function o(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var u=n.createContext({}),s=function(e){var t=n.useContext(u),r=t;return e&&(r="function"==typeof e?e(t):l(l({},t),e)),r},p=function(e){var t=s(e.components);return n.createElement(u.Provider,{value:t},e.children)},c={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,u=e.parentName,p=o(e,["components","mdxType","originalType","parentName"]),m=s(r),f=a,d=m["".concat(u,".").concat(f)]||m[f]||c[f]||i;return r?n.createElement(d,l(l({ref:t},p),{},{components:r})):n.createElement(d,l({ref:t},p))}));function f(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,l=new Array(i);l[0]=m;var o={};for(var u in t)hasOwnProperty.call(t,u)&&(o[u]=t[u]);o.originalType=e,o.mdxType="string"==typeof e?e:a,l[1]=o;for(var s=2;s<i;s++)l[s]=r[s];return n.createElement.apply(null,l)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},18424:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>u,contentTitle:()=>l,default:()=>c,frontMatter:()=>i,metadata:()=>o,toc:()=>s});var n=r(58168),a=(r(96540),r(15680));const i={},l="Full list of attributes",o={unversionedId:"guides/custom/attributes/full-list",id:"guides/custom/attributes/full-list",title:"Full list of attributes",description:"The following are by alphabetical order instead of importance.",source:"@site/docs/guides/custom/attributes/full-list.md",sourceDirName:"guides/custom/attributes",slug:"/guides/custom/attributes/full-list",permalink:"/flutter_rust_bridge/guides/custom/attributes/full-list",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/custom/attributes/full-list.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Specify attributes using comments",permalink:"/flutter_rust_bridge/guides/custom/attributes/comments"},next:{title:"Cross-platform utils",permalink:"/flutter_rust_bridge/guides/cross-platform/"}},u={},s=[],p={toc:s};function c(e){let{components:t,...r}=e;return(0,a.yg)("wrapper",(0,n.A)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,a.yg)("h1",{id:"full-list-of-attributes"},"Full list of attributes"),(0,a.yg)("p",null,"The following are by alphabetical order instead of importance.\nFor example, seldomly used feature may appear near the top."),(0,a.yg)("ul",null,(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(dart2rust(..))]"),": Custom encoders/decoders."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(dart_code = ..)]"),": Inject extra Dart code."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(default = ..)]"),": Set default parameters."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(external)]"),": Mark external methods."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(getter)]"),": Mark function as Dart getter."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(ignore)]"),": Ignore the object annotated."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(init)]"),": Mark function to be executed at startup."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(mirror)]"),": Manually mirror external types (can use auto mode instead)."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(name)]"),": Rename the object."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(non_eq)]"),": Disable generating ",(0,a.yg)("inlineCode",{parentName:"li"},"equals"),"."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(non_hash)]"),": Disable generating ",(0,a.yg)("inlineCode",{parentName:"li"},"hashCode"),"."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(non_opaque)]"),": Mark object as non-opaque."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(opaque)]"),": Mark object as opaque."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(positional)]"),": Generate positional instead of keyword arguments."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(proxy)]"),": Enable proxy feature."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(rust2dart)]"),": Custom encoders/decoders."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(setter)]"),": Mark function as Dart setter."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(serialize)]"),": Use SSE codec."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(stream_dart_await)]"),": Await stream execution before returning."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(sync)]"),": Generate synchronous function in Dart."),(0,a.yg)("li",{parentName:"ul"},(0,a.yg)("inlineCode",{parentName:"li"},"#[frb(type_64bit_int)]"),": Change how 64-bit integers are translated.")),(0,a.yg)("p",null,"For a up-to-date full list of supported attributes, please refer to the ",(0,a.yg)("inlineCode",{parentName:"p"},"FrbAttribute"),"\nof ",(0,a.yg)("a",{parentName:"p",href:"https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/library/codegen/parser/mir/parser/attribute.rs"},"this file"),"\n(except that it is pascal case instead of snake case).\nTo know more details of each attribute, the search bar at top-right may be useful."))}c.isMDXComponent=!0}}]);