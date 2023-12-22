import React from 'react';
import CodeBlock from '@theme/CodeBlock';

// noinspection JSUnusedGlobalSymbols
export default function ShowMeTheCode() {
    return (
        <div>
            <SectionTitle color="#4caf50">1. Simple</SectionTitle>
            <p style={{
                margin: '4px 0px',
            }}>Simple Rust...</p>
            <CodeBlock language="rust">
                {"fn f(a: String, b: Vec<String>) -> MyStruct { ... }"}
            </CodeBlock>
            <p style={{
                margin: '4px', marginTop: '-12px', marginLeft: '0px',
            }}>
                ...called from Dart, without manual intervention.</p>
            <CodeBlock language="dart">
                {"print(f(a: 'Hello', b: ['World']));"}
            </CodeBlock>

            {/*<div style={{*/}
            {/*    borderTop: '1px solid #ddd',*/}
            {/*    marginLeft: '-28px',*/}
            {/*    marginTop: '24px',*/}
            {/*    marginBottom: '20px',*/}
            {/*}}></div>*/}
            <SectionTitle color="#2196f3">2. Fancy</SectionTitle>
            <p style={{
                margin: '4px 0px',
            }}>Let's see how fancy we can support:</p>
            <CodeBlock language="rust">
                {" ".repeat(0)}<Comment color="#388e3c">Arbitrarily fancy Rust types</Comment>
                {"\n"}
                <b>struct</b>
                {" Garden "}
                <MyMark color="#CBF6D3">
                    {"{ land: whatever::fancy::Land }"}
                </MyMark>
                {"\n\n"}
                {" ".repeat(0)}<Comment color="#1976d2">Complex but auto-translatable</Comment>
                {"\n"}
                <b>enum</b>
                {" Tree "}
                <MyMark color="#cfeeff">
                    {"{ A { name: (String, i32), children: Option<Vec<Tree>> }, B }"}
                </MyMark>
                {"\n\n"}
                {" ".repeat(0)}<Comment color="#e65100">Support functions & methods</Comment>
                {"\n"}
                <MyMark color="#ffe0b2">
                    <b>impl</b>
                </MyMark>
                {" Garden {\n"}
                {" ".repeat(4)}<Comment color="#c2185b">Allow async & sync Rust</Comment>
                {"\n    "}
                <MyMark color="#ffe0eb">
                    <b>async</b>
                </MyMark>
                {" "}
                <b>fn</b>
                {" plant(\n"}
                {" ".repeat(8)}<Comment color="#fbc02d">Support T/&T/&mut T</Comment>
                {"\n        "}
                <MyMark color="#fff9c4">
                    <b>&mut</b>
                </MyMark>
                {" self,\n        tree: Tree,\n"}
                {" ".repeat(8)}<Comment color="#689f38">Rust can also call Dart</Comment>
                {"\n        chooser: "}
                <MyMark color="#dcedc8">
                    <b>impl Fn</b>
                    {"(String) -> bool"}
                </MyMark>
                {",\n"}
                {" ".repeat(8)}<Comment color="#0288d1">Error translation ; zero copy</Comment>
                {"\n    ) -> "}
                <MyMark color="#b2ebf2">
                    {"Result"}
                </MyMark>
                {"<"}
                <MyMark color="#b2ebf2">
                    {"Vec<u8>"}
                </MyMark>
                {", FancyError> {\n        ...\n    }\n}"}
            </CodeBlock>
            <p style={{
                margin: '4px', marginTop: '-12px', marginLeft: '0px',
            }}>Still seamlessly call in Dart:</p>
            <CodeBlock>
                <b>var</b>
                {" tree = Tree.a(('x', 42), [Tree.b()]);\n"}
                {" ".repeat(0)}<Comment color="#7b1fa2">Async & sync Dart</Comment>
                {"\n"}
                <b>print</b>
                {"("}
                <MyMark color="#fcd2ff">
                    <b>await</b>
                </MyMark>
                {" garden.plant(tree, (a) => "}
                <b>true</b>
                {"));"}
            </CodeBlock>
        </div>
    )
        ;
}

const SectionTitle = ({children, color}) => {
    return <h2>{children}</h2>
    // return <div style={{
    //     // position: 'absolute',
    //     // left: '0px',
    //     // padding: '0px 8px',
    //     marginBottom: '8px',
    //     display: 'flex',
    //     justifyContent: 'center',
    // }}>
    //     <span style={{
    //         padding: '2px 8px',
    //         border: `2px solid ${color}`,
    //         borderRadius: '8px',
    //         background: 'white',
    //         lineHeight: 'normal',
    //     }}>{children}</span>
    // </div>
}

const Comment = ({children, color}) => {
    return (
        <span style={{
            // color: color,
            color: '#888',
        }}>
                {"// "}
            <b><span style={{color: color}}>↱</span></b>
            {" "}
            {/*<span style={{*/}
            {/*    height: '10px',*/}
            {/*    width: '10px',*/}
            {/*    backgroundColor: color,*/}
            {/*    borderRadius: '50%',*/}
            {/*    display: 'inline-block',*/}
            {/*    marginRight: '8px',*/}
            {/*}}/>*/}
            {children}
        </span>
    )
}

// const Item = ({children, color}) => {
//     return (
//         <div>
//             <span style={{
//                 height: '10px',
//                 width: '10px',
//                 backgroundColor: color,
//                 borderRadius: '50%',
//                 display: 'inline-block',
//                 marginRight: '8px',
//             }}/>
//             <span style={{
//                 // fontFamily: '"Open Sans", sans-serif',
//             }}>{children}</span>
//         </div>
//     )
// }

const MyMark = ({
                    children, color,
                    ...rest
                }) => {
    return (
        <>
            <span style={{background: color}}>{children}</span>
            {/*{hint && <div style={{position: 'absolute', left: '0px', marginTop: '-22px'}}>*/}
            {/*    <Item color={hintColor}>{hint}</Item>*/}
            {/*</div>}*/}
        </>
    )
}
