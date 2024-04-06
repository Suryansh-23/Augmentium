"use client";
import Animated from "./Animated";
import { BackgroundBeams } from "./ui/BackgroundBeams";

export function TradeBanner() {
    return (
        <div
            id="trade"
            className="h-[40rem] w-full rounded-md bg-black relative flex flex-col items-center justify-center antialiased bg-transparent border-transparent">
            <Animated className="max-w-2xl flex flex-col items-center mx-auto p-4 z-10">
                <h1 className="relative z-10 text-lg md:text-7xl  bg-clip-text text-transparent bg-gradient-to-b from-neutral-200 to-neutral-600  text-center font-sans font-bold">
                    Save for the future
                </h1>
                <p></p>
                <p className="text-neutral-500 max-w-lg mx-auto my-2 text-sm text-center relative z-10">
                    Start trading with Augmentium and save for the future. Join
                    the RWA revolution now. The world is changing, and so should
                    your investments. Currency of the future awaits you.
                </p>
                <a
                    href="/trade"
                    className="py-2 mt-2 px-4 rounded-xl truncate text-xs md:text-sm lg:text-base bg-black dark:bg-white text-white dark:text-black hover:opacity-75 blend">
                    Start Trading
                </a>
            </Animated>
            <BackgroundBeams />
        </div>
    );
}
