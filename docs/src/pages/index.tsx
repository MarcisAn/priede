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

        <div className={styles.buttons} style={{ display: "flex", gap: "7px" }}>
          <Link
            className="button button--secondary button--lg"
            to="/lejupielade">
            Lejupielādēt
          </Link>
          <Link
            className="button button--secondary button--lg"
            to="https://priede-editor.vercel.app">
            Izmēģināt
          </Link>
        </div>
        <Link
          className="button button--primary button--lg"
          style={{ marginTop: "3em" }}
          to="/docs/kas-ir-priede">
          DOKUMENTĀCIJA
        </Link>
        <br />
        <br />
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
      {/*<main>
        
        <HomepageFeatures />
      </main>*/}
    </Layout>
  );
}
