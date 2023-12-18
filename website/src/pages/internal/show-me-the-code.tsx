import React from 'react';
import Layout from '@theme/Layout';
import ShowMeTheCode from "@site/src/components/ShowMeTheCode";

/**
 * How to take screenshot:
 * 1. Follow https://davidaugustat.com/web/take-ultra-high-resolution-screenshots-in-chrome
 *    to create a super large device (e.g. 8000x8000)
 * 2. Chrome's "capture screenshot"
 */
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