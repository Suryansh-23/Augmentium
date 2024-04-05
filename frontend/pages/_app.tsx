import { wallets } from "@cosmos-kit/keplr-extension";
import { ChainProvider } from "@cosmos-kit/react";
import { assets, chains } from "chain-registry";
import type { AppProps } from "next/app";
import { mantra } from "../lib/chainConfig";
import "../styles/globals.css";

import "@interchain-ui/react/styles";

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <ChainProvider
            chains={[...chains, mantra]}
            assetLists={assets} // supported asset lists
            wallets={wallets} // supported wallets
            // walletConnectOptions={}
        >
            <Component {...pageProps} />
        </ChainProvider>
    );
}

export default MyApp;
