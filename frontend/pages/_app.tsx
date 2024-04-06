import { wallets } from "@cosmos-kit/keplr-extension";
import { ChainProvider } from "@cosmos-kit/react";
import { assets, chains } from "chain-registry";
import type { AppProps } from "next/app";
import { mantra } from "../lib/chainConfig";
import "../styles/globals.css";

import "@interchain-ui/react/styles";
import Header from "../components/Header";
import Footer from "../components/Footer";
import Head from "next/head";

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <ChainProvider
            chains={[...chains, mantra]}
            assetLists={assets} // supported asset lists
            wallets={wallets} // supported wallets
            // walletConnectOptions={}
        >
            <Head>
                <title>Augmentium</title>
                <meta
                    name="description"
                    content="The stablecoin of the future."
                />
            </Head>
            <Header />
            <Component {...pageProps} />
            <Footer />
        </ChainProvider>
    );
}

export default MyApp;
