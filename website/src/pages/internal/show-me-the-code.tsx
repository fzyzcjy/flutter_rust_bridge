import React from 'react';
import Layout from '@theme/Layout';
import ShowMeTheCode from "@site/src/components/ShowMeTheCode";

/**
 * How to take screenshot:
 * 1. Open Chrome devtools
 * 2. Click "toggle device toolbar"
 * 3. Choose "Dimensions: Responsive"
 * 4. Adjust dimension to appropreate (e.g. 7884x6758 seems good)
 * 5. Click "capture screenshot" (inside right-top menu)
 * rough ref: https://davidaugustat.com/web/take-ultra-high-resolution-screenshots-in-chrome
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