import type { NextPage } from "next";
import { useState } from "react";
// import { useCount } from "../api/counter";
import Header from "../components/Header";
import Hero from "../components/Hero";

const Home: NextPage = () => {
    // const { count, error, increase } = useCount();
    const [isLoading, setLoading] = useState(false);

    return (
        <>
            <Header />
            <Hero />
        </>
        // <div className={styles.container}>
        //     <Head>
        //         <title>Counter Dapp</title>
        //         <meta
        //             name="description"
        //             content="Counter dapp: an example dapp"
        //         />
        //         <link rel="icon" href="/favicon.ico" />
        //     </Head>

        //     <Drawer />

        //     <main className={styles.main}>
        //         <h1 className={styles.title}>Count</h1>

        //         {/* <p
        //             className={
        //                 isLoading
        //                     ? [styles.count, styles.pulse].join(" ")
        //                     : styles.count
        //             }>
        //             {count === undefined ? "?" : count}
        //         </p> */}
        //         {/*
        //         {error ? (
        //             <p className={styles.error}>Error: {error.message}</p>
        //         ) : (
        //             <></>
        //         )} */}

        //         <div className={styles.grid}>
        //             <a
        //                 className={styles.card}
        //                 // onClick={async () => {
        //                 //     setLoading(true);
        //                 //     await increase();
        //                 //     setLoading(false);
        //                 // }}
        //             >
        //                 <h2>＋ Increase Counter</h2>
        //             </a>
        //         </div>
        //     </main>
        // </div>
    );
};

export default Home;
