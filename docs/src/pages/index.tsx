import React from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";

import styles from "./index.module.css";

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <img src="img/logo.svg" alt="Priede" width="35%" />
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <br />
        <br />

        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/lejupielade">
            Lejupielādēt
          </Link>
        </div>
        <br />
        <br />
        <p className="hero__subtitle">Izmēģini pārlūkā</p>
        {process.env.NODE_ENV === "production" ? (
          <iframe
            src="https://priede-editor.vercel.app"
            frameBorder="0"></iframe>
        ) : (
          <iframe src="http://localhost:5173" frameBorder="0"></iframe>
        )}
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title}`}
      description="Priedes valodas dokumentācija">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
