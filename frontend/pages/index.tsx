import type { NextPage } from "next";
import { useEffect, useState } from "react";
// import { useCount } from "../api/counter";
import Features from "../components/Features";
import Footer from "../components/Footer";
import Header from "../components/Header";
import Hero from "../components/Hero";
import { TradeBanner } from "../components/TradeBanner";

const Home: NextPage = () => {
    // const { count, error, increase } = useCount();
    const [isLoading, setLoading] = useState(false);
    useEffect(() => {
        const animate = () => {
            const animateElements = document.querySelectorAll(".animate");

            animateElements.forEach((element, index) => {
                setTimeout(() => {
                    element.classList.add("show");
                }, index * 150);
            });
        };

        document.addEventListener("DOMContentLoaded", animate);

        return () => {
            document.removeEventListener("DOMContentLoaded", animate);
        };
    });

    return (
        <>
            <Header />
            <Hero />
            <div className="relative w-full bg-white dark:bg-black">
                <div className="mx-auto sm:w-full md:w-4/5 lg:w-3/4 p-5 space-y-24 pb-16">
                    <Features />
                </div>
                <TradeBanner />
            </div>
            <Footer />
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
