"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[6546],{15680:(e,t,n)=>{n.d(t,{xA:()=>l,yg:()=>d});var r=n(96540);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var p=r.createContext({}),c=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},l=function(e){var t=c(e.components);return r.createElement(p.Provider,{value:t},e.children)},g={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,p=e.parentName,l=s(e,["components","mdxType","originalType","parentName"]),u=c(n),d=i,m=u["".concat(p,".").concat(d)]||u[d]||g[d]||a;return n?r.createElement(m,o(o({ref:t},l),{},{components:n})):r.createElement(m,o({ref:t},l))}));function d(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,o=new Array(a);o[0]=u;var s={};for(var p in t)hasOwnProperty.call(t,p)&&(s[p]=t[p]);s.originalType=e,s.mdxType="string"==typeof e?e:i,o[1]=s;for(var c=2;c<a;c++)o[c]=n[c];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},82968:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>o,default:()=>g,frontMatter:()=>a,metadata:()=>s,toc:()=>c});var r=n(58168),i=(n(96540),n(15680));const a={},o="Linking the project",s={unversionedId:"manual/integrate/existing/ios/linking",id:"manual/integrate/existing/ios/linking",title:"Linking the project",description:"Open ios/Runner.xcodeproj in Xcode, then add $crate/$crate.xcodeproj as a subproject",source:"@site/docs/manual/integrate/06-existing/04-ios/02-linking.md",sourceDirName:"manual/integrate/06-existing/04-ios",slug:"/manual/integrate/existing/ios/linking",permalink:"/flutter_rust_bridge/manual/integrate/existing/ios/linking",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/manual/integrate/06-existing/04-ios/02-linking.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Creating the Rust project",permalink:"/flutter_rust_bridge/manual/integrate/existing/ios/proj"},next:{title:"Generating bindings",permalink:"/flutter_rust_bridge/manual/integrate/existing/ios/gen"}},p={},c=[],l={toc:c};function g(e){let{components:t,...a}=e;return(0,i.yg)("wrapper",(0,r.A)({},l,a,{components:t,mdxType:"MDXLayout"}),(0,i.yg)("h1",{id:"linking-the-project"},"Linking the project"),(0,i.yg)("p",null,"Open ",(0,i.yg)("inlineCode",{parentName:"p"},"ios/Runner.xcodeproj")," in Xcode, then add ",(0,i.yg)("inlineCode",{parentName:"p"},"$crate/$crate.xcodeproj")," as a ",(0,i.yg)("em",{parentName:"p"},"subproject"),"\nof the Runner project. It should look like this:"),(0,i.yg)("p",null,(0,i.yg)("img",{alt:"proj-tree",src:n(83813).A,width:"273",height:"343"})),(0,i.yg)("p",null,"Click on the ",(0,i.yg)("inlineCode",{parentName:"p"},"Runner")," root project, then go to the ",(0,i.yg)("strong",{parentName:"p"},"Build Phases")," tab.\nFirst, expand the ",(0,i.yg)("strong",{parentName:"p"},"Dependencies")," phase, and add ",(0,i.yg)("strong",{parentName:"p"},"$crate-staticlib"),"\nfor iOS, or ",(0,i.yg)("strong",{parentName:"p"},"$crate-cdylib")," for MacOS."),(0,i.yg)("p",null,(0,i.yg)("img",{alt:"dep-phase",src:n(36230).A,width:"428",height:"490"})),(0,i.yg)("p",null,"Then, expand the ",(0,i.yg)("strong",{parentName:"p"},"Link Binary With Libraries")," phase, and add ",(0,i.yg)("strong",{parentName:"p"},"lib$crate_static.a"),"\nfor iOS, or ",(0,i.yg)("strong",{parentName:"p"},"$crate.dylib")," for MacOS."),(0,i.yg)("p",null,(0,i.yg)("img",{alt:"link-phase",src:n(21857).A,width:"428",height:"490"})))}g.isMDXComponent=!0},36230:(e,t,n)=>{n.d(t,{A:()=>r});const r=n.p+"assets/images/ios-dep-phase-13624f2d3952ef2833626f0c6ed6bc0c.png"},21857:(e,t,n)=>{n.d(t,{A:()=>r});const r=n.p+"assets/images/ios-link-phase-170207e34ca42569f74d4bb02d841bb9.png"},83813:(e,t,n)=>{n.d(t,{A:()=>r});const r=n.p+"assets/images/ios-proj-tree-937667a355756f50108c289f4345b189.png"}}]);