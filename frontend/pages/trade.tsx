"use client";
import Animated from "../components/Animated";
import TradeBox from "../components/TradeBox";
import { BackgroundBeams } from "../components/ui/BackgroundBeams";

const Trade = () => {
    return (
        <>
            <Animated className="h-[60vh] mt-40 w-[40vw] m-auto relative z-10">
                <TradeBox />
            </Animated>
            <BackgroundBeams />
        </>
    );
};

export default Trade;
