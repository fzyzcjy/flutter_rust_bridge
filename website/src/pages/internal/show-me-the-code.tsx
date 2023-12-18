import React from 'react';
import Layout from '@theme/Layout';
import ShowMeTheCode from "@site/src/components/ShowMeTheCode";

export default class Page extends React.Component {
    render() {
        return (
            <Layout>
                <div style={{margin: '32px'}}>
                    <ShowMeTheCode/>
                </div>
            </Layout>
        );
    }
}