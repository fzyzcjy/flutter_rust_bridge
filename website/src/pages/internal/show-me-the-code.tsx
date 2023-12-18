import React from 'react';
import Layout from '@theme/Layout';
import ShowMeTheCode from "@site/src/components/ShowMeTheCode";

export default class Page extends React.Component {
    render() {
        return (
            <div className="my-hide-nav">
                <Layout>
                    <div style={{
                        padding: '32px',
                        scale: '8.0',
                        transformOrigin: 'top left',
                    }}>
                        <ShowMeTheCode/>
                    </div>
                </Layout>
            </div>
        );
    }
}