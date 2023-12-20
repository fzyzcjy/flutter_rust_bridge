import React, {useEffect, useState} from 'react';
import Translate, {translate} from '@docusaurus/Translate';
import {PageMetadata} from '@docusaurus/theme-common';
import Layout from '@theme/Layout';

// noinspection JSUnusedGlobalSymbols
export default function NotFound() {
  const [pathname, setPathname] = useState('');
  useEffect(() => setPathname(window.location.pathname));

  const v1Url = pathname.replace(/^\/flutter_rust_bridge/,"/flutter_rust_bridge/v1");

  return (
    <>
      <PageMetadata
        title={translate({
          id: 'theme.NotFound.title',
          message: 'Page Not Found',
        })}
      />
      <Layout>
        <main className="container margin-vert--xl">
          <div className="row">
            <div className="col col--6 col--offset-3">
              <h1 className="hero__title">
                <Translate
                  id="theme.NotFound.title"
                  description="The title of the 404 page">
                  Page Not Found
                </Translate>
              </h1>
              <p>
                If you are using flutter_rust_bridge v1.x, try to visit:&nbsp;
                <a href={v1Url}>{v1Url}</a>
              </p>
            </div>
          </div>
        </main>
      </Layout>
    </>
  );
}
