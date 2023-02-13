import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import React from "react";

export default function down() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout title={`${siteConfig.title}`} description="Lejupielādēt PRIEDI">
      <main className="download">
        <h1>Lejupielādēt</h1>
        <a
          className="button button--secondary button--lg"
          href="releases/priede.exe">
          64 bitu Windows
        </a>
        <div className="flex-sepperator"></div>
        <h2>Kā izmantot:</h2>
        <code>priede.exe priedes-koda-fails.pr ast</code>
        <p>Galā pievienojot "ast" tiks izprintēts abstraktais sintakses koks</p>
      </main>
    </Layout>
  );
}
