import { wallets } from "@cosmos-kit/keplr-extension";
import { ChainProvider } from "@cosmos-kit/react";
import { ThemeProvider } from "@interchain-ui/react";
import "@interchain-ui/react/styles";
import { assets, chains } from "chain-registry";
import type { AppProps } from "next/app";
import Head from "next/head";
import Footer from "../components/Footer";
import Header from "../components/Header";
import { mantra } from "../lib/chainConfig";
import "../styles/globals.css";

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <ChainProvider
            chains={[...chains, mantra]}
            assetLists={assets} // supported asset lists
            wallets={wallets} // supported wallets
            // walletConnectOptions={}
        >
            <ThemeProvider>
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
            </ThemeProvider>
        </ChainProvider>
    );
}

export default MyApp;
