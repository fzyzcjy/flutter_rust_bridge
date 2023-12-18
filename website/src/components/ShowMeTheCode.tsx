import React from 'react';
import CodeBlock from '@theme/CodeBlock';

// noinspection JSUnusedGlobalSymbols
export default function ShowMeTheCode() {
    return (
        <div style={{display: 'flex', flexDirection: 'row', position: 'relative'}}>
            <div style={{paddingRight: '28px', width: '300px', borderRight: '1px solid #ddd'}}>
            </div>
            <div style={{paddingLeft: '28px'}}>
                <SectionTitle color="#4caf50">1. Simple</SectionTitle>
                <p style={{margin: '4px', marginLeft: '0px'}}>Simple Rust...</p>
                <CodeBlock language="rust">
                    {"fn f(a: String, b: Vec<String>) -> MyStruct { ... }"}
                </CodeBlock>
                <p style={{margin: '4px', marginTop: '-12px', marginLeft: '0px'}}>
                    ...called from Dart, without manual intervention.</p>
                <CodeBlock language="dart">
                    {"print(f(a: 'Hello', b: ['Tom']));"}
                </CodeBlock>

                <div style={{
                    borderTop: '1px solid #ddd',
                    marginLeft: '-28px',
                    marginTop: '24px',
                    marginBottom: '20px',
                }}></div>
                <SectionTitle color="#2196f3">2. Fancy</SectionTitle>
                <p>Let's see how fancy we can support:</p>
                <CodeBlock language="rust">
                    <b>struct</b>
                    {" Garden "}
                    <MyMark
                        color="#CBF6D3"
                        hintColor="#4caf50"
                        hint="Arbitrarily fancy Rust types">
                        {"{ land: whatever::fancy::Land }"}
                    </MyMark>
                    {"\n\n"}
                    <b>enum</b>
                    {" Tree "}
                    <MyMark
                        color="#cfeeff"
                        hintColor="#03a9f4"
                        hint="Complex but auto-translatable">
                        {"{ A {name:(String,i32),children:Option<Vec<Tree>>}, B }"}
                    </MyMark>
                    {"\n\n"}
                    <MyMark
                        color="#ffecb3"
                        hintColor="#ffc107"
                        hint="Support functions & methods">
                        <b>impl</b>
                    </MyMark>
                    {" Garden {\n    "}
                    <MyMark
                        color="#bbdefb"
                        hintColor="#03a9f4"
                        hint="Allow async & sync Rust">
                        <b>async</b>
                    </MyMark>
                    {" "}
                    <b>fn</b>
                    {" plant(\n        "}
                    <MyMark
                        color="#dcedc8"
                        hintColor="#8bc34a"
                        hint="Support T/&T/&mut T">
                        <b>&mut</b>
                    </MyMark>
                    {" self,\n        tree: Tree,\n        chooser: "}
                    <MyMark
                        color="#ffe0eb"
                        hintColor="#e91e63"
                        hint="Rust can also call Dart">
                        <b>impl Fn</b>
                        {"(String) -> bool"}
                    </MyMark>
                    {",\n    ) -> "}
                    <MyMark
                        color="#b9f6ca"
                        hintColor="#00bcd4"
                        hint="Error translation ; zero copy">
                        {"Result"}
                    </MyMark>
                    {"<"}
                    <MyMark
                        color="#b2ebf2"
                        hintColor={null}
                        hint={null}>
                        {"Vec<u8>"}
                    </MyMark>
                    {", FancyError> {\n        ...\n    }\n}"}
                </CodeBlock>
                <p>Still seamlessly call in Dart:</p>
                <CodeBlock>
                    <b>var</b>
                    {" tree = Tree.a(('x', 42), [Tree.b()]);\n"}
                    <b>print</b>
                    {"("}
                    <MyMark
                        color="#fcd2ff"
                        hintColor="#9c27b0"
                        hint="Async & sync Dart">
                        <b>await</b>
                    </MyMark>
                    {" garden.plant(tree, (a) => "}
                    <b>true</b>
                    {"));"}
                </CodeBlock>
            </div>
        </div>
    );
}

const SectionTitle = ({children, color}) => {
    return <div style={{
        // position: 'absolute',
        // left: '0px',
        // padding: '0px 8px',
        marginBottom: '8px',
        display: 'flex',
        justifyContent: 'center',
    }}>
        <span style={{
            padding: '2px 8px',
            border: `2px solid ${color}`,
            borderRadius: '8px',
            background: 'white',
            lineHeight: 'normal',
        }}>{children}</span>
    </div>
}

const Item = ({children, color}) => {
    return (
        <div>
            <span style={{
                height: '10px',
                width: '10px',
                backgroundColor: color,
                borderRadius: '50%',
                display: 'inline-block',
                marginRight: '8px',
            }}/>
            <span style={{
                // fontFamily: '"Open Sans", sans-serif',
            }}>{children}</span>
        </div>
    )
}

const MyMark = ({children, color, hintColor, hint}) => {
    return (
        <>
            <span style={{background: color}}>{children}</span>
            {hint && <div style={{position: 'absolute', left: '0px', marginTop: '-22px'}}>
                <Item color={hintColor}>{hint}</Item>
            </div>}
        </>
    )
}
