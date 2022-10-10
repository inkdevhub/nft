import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

const Home: NextPage = () => {
  return (
    <div className={styles.container}>
      <Head>
        <title>Wasm Contracts</title>
        <meta name="description" content="Wasm Contracts" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>
          Wasm Contracts
        </h1>

        <div className={styles.grid}>
          <a href="/flipper" className={styles.card}>
            <h2>Flipper &rarr;</h2>
            <p>Flips a boolean value.</p>
          </a>
        </div>
      </main>
    </div>
  )
}

export default Home
