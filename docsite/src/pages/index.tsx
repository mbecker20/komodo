import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';
import SummaryImg from '../components/SummaryImg';
import MonitorLogo from '../components/MonitorLogo';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <div style={{ display: "flex", gap: "1rem", justifyContent: "center" }}>
          <div style={{ position: "relative" }}>
            <MonitorLogo width="600px" />
            <h1 className="hero__title" style={{ margin: 0, position: "absolute", top: "40%", left: "50%", transform: "translate(-50%, -50%)" }}>
              monitor
            </h1>
          </div>
        </div>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link className="button button--secondary button--lg" to="/intro">
            docs
          </Link>
          <Link
            className="button button--secondary button--lg"
            to="https://github.com/mbecker20/monitor"
          >
            github
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout title="monitor docs" description={siteConfig.tagline}>
      {/* <SummaryImg /> */}
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        {/* <SummaryImg /> */}
      </main>
    </Layout>
  );
}
