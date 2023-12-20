import React from 'react';
import Layout from '@theme/Layout';
import ShowMeTheCode from "@site/src/components/ShowMeTheCode";

/**
 * How to take screenshot:
 * 1. Follow https://davidaugustat.com/web/take-ultra-high-resolution-screenshots-in-chrome
 *    to create a super large device (e.g. 3950x3475)
 * 2. Chrome's "capture screenshot"
 */
export default class Page extends React.Component {
    componentDidMount() {
        setInterval(() => {
            document.body.style.overflow = "hidden";
        }, 1000);
    }

    render() {
        return (
            <div className="my-hide-nav">
                <Layout>
                    <div style={{
                        padding: '16px',
                        scale: '4.0',
                        transformOrigin: 'top left',
                    }}>
                        <div style={{
                            width: '650px',
                        }}>
                            <ShowMeTheCode/>
                        </div>
                    </div>
                </Layout>
            </div>
        );
    }
}