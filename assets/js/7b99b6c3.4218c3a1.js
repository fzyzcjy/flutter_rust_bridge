"use strict";(self.webpackChunkflutter_rust_bridge=self.webpackChunkflutter_rust_bridge||[]).push([[928],{15680:(e,t,n)=>{n.d(t,{xA:()=>u,yg:()=>y});var a=n(96540);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),d=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},u=function(e){var t=d(e.components);return a.createElement(p.Provider,{value:t},e.children)},s={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,p=e.parentName,u=o(e,["components","mdxType","originalType","parentName"]),m=d(n),y=r,c=m["".concat(p,".").concat(y)]||m[y]||s[y]||i;return n?a.createElement(c,l(l({ref:t},u),{},{components:n})):a.createElement(c,l({ref:t},u))}));function y(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,l=new Array(i);l[0]=m;var o={};for(var p in t)hasOwnProperty.call(t,p)&&(o[p]=t[p]);o.originalType=e,o.mdxType="string"==typeof e?e:r,l[1]=o;for(var d=2;d<i;d++)l[d]=n[d];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},82168:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>l,default:()=>s,frontMatter:()=>i,metadata:()=>o,toc:()=>d});var a=n(58168),r=(n(96540),n(15680));const i={},l="DateTime (Chrono)",o={unversionedId:"guides/types/translatable/detailed/chrono",id:"guides/types/translatable/detailed/chrono",title:"DateTime (Chrono)",description:"Codegen optionally support chrono crate with feature chrono.",source:"@site/docs/guides/types/translatable/detailed/chrono.md",sourceDirName:"guides/types/translatable/detailed",slug:"/guides/types/translatable/detailed/chrono",permalink:"/flutter_rust_bridge/guides/types/translatable/detailed/chrono",draft:!1,editUrl:"https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/website/docs/guides/types/translatable/detailed/chrono.md",tags:[],version:"current",frontMatter:{},sidebar:"tutorialSidebar",previous:{title:"Map and Set",permalink:"/flutter_rust_bridge/guides/types/translatable/detailed/map_set"},next:{title:"UUID",permalink:"/flutter_rust_bridge/guides/types/translatable/detailed/uuid"}},p={},d=[],u={toc:d};function s(e){let{components:t,...n}=e;return(0,r.yg)("wrapper",(0,a.A)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,r.yg)("h1",{id:"datetime-chrono"},"DateTime (Chrono)"),(0,r.yg)("p",null,"Codegen optionally support ",(0,r.yg)("a",{parentName:"p",href:"https://docs.rs/chrono"},"chrono crate")," with feature ",(0,r.yg)("inlineCode",{parentName:"p"},"chrono"),"."),(0,r.yg)("table",null,(0,r.yg)("thead",{parentName:"table"},(0,r.yg)("tr",{parentName:"thead"},(0,r.yg)("th",{parentName:"tr",align:null},"\ud83e\udd80 Rust"),(0,r.yg)("th",{parentName:"tr",align:null},"\ud83c\udfaf Dart"))),(0,r.yg)("tbody",{parentName:"table"},(0,r.yg)("tr",{parentName:"tbody"},(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"DateTime<Utc>")),(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"DateTime")," ",(0,r.yg)("em",{parentName:"td"},"utc"))),(0,r.yg)("tr",{parentName:"tbody"},(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"DateTime<Local>")),(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"DateTime")," ",(0,r.yg)("em",{parentName:"td"},"local timezone"))),(0,r.yg)("tr",{parentName:"tbody"},(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"NaiveDateTime")),(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"DateTime")," ",(0,r.yg)("em",{parentName:"td"},"utc assumed"))),(0,r.yg)("tr",{parentName:"tbody"},(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"Duration")),(0,r.yg)("td",{parentName:"tr",align:null},(0,r.yg)("inlineCode",{parentName:"td"},"Duration"))))),(0,r.yg)("p",null,"You can also use nullable values through ",(0,r.yg)("inlineCode",{parentName:"p"},"Option"),", for example: ",(0,r.yg)("inlineCode",{parentName:"p"},"Option<NaiveDateTime>"),"."),(0,r.yg)("p",null,"\u26a0\ufe0f Please note that:"),(0,r.yg)("ul",null,(0,r.yg)("li",{parentName:"ul"},"on native platforms, ",(0,r.yg)("em",{parentName:"li"},"microseconds")," unit is used."),(0,r.yg)("li",{parentName:"ul"},"on web platform, ",(0,r.yg)("em",{parentName:"li"},"milliseconds")," unit is used (due to JS limitation with dates).")),(0,r.yg)("p",null,"\ud83d\udca1 Also a ",(0,r.yg)("inlineCode",{parentName:"p"},"DateTime<Local>")," will always be translated into local time of the device, which might not be what you want if you expect them to be sent ",(0,r.yg)("em",{parentName:"p"},"as-is"),"."),(0,r.yg)("blockquote",null,(0,r.yg)("p",{parentName:"blockquote"},"In that case, you could implement it in your codebase by sending a ",(0,r.yg)("inlineCode",{parentName:"p"},"u32")," (timezone offset) alongside the ",(0,r.yg)("inlineCode",{parentName:"p"},"i64")," (timestamp) over the wire, or open a issue / PR here to further discuss it. The reason why this choice was originally made is to have all ",(0,r.yg)("inlineCode",{parentName:"p"},"DateTime<Utc>"),", ",(0,r.yg)("inlineCode",{parentName:"p"},"DateTime<Local>")," and ",(0,r.yg)("inlineCode",{parentName:"p"},"NaiveDateTime")," been represented by a single ",(0,r.yg)("inlineCode",{parentName:"p"},"i64"),".")))}s.isMDXComponent=!0}}]);