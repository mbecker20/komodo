import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: "automated builds 🛠️",
    description: (
      <>
        build auto versioned docker images from github repos, trigger builds on
        git push
      </>
    ),
  },
  {
    title: "deploy docker containers 🚀",
    description: (
      <>
        deploy your builds (or any docker image), see uptime and logs across all
        your servers
      </>
    ),
  },
  {
    title: "powered by Rust 🦀",
    description: <>The core API and periphery client are written in Rust</>,
  },
];

function Feature({title, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
