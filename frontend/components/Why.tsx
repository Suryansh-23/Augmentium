/* eslint-disable @next/next/no-img-element */
import ReactPlayer from "react-player";
import Animated from "./Animated";

const Why = () => {
    return (
        <Animated className="pt-12 md:pt-24 lg:pt-32 bg-black">
            <div
                id="why"
                className="container space-y-10 xl:space-y-16 flex flex-col items-center">
                <h1 className="lg:leading-tighter text-3xl font-bold tracking-tighter sm:text-4xl text-center md:text-5xl xl:text-[3.4rem] 2xl:text-[3.75rem] w-[40rem]">
                    Why we need <span className="line-through">Stablecoin</span>{" "}
                    Augmentium?
                </h1>
                <ReactPlayer
                    url="https://youtu.be/pGzfexGmuVw?si=jA7frCDno_l-D2DN"
                    width="100%"
                    height="80vh"
                    light={
                        <img
                            src="/thumbnail.png"
                            title="thumbnail"
                            className="object-cover rounded-xl"
                        />
                    }
                />
            </div>
        </Animated>
    );
};

export default Why;
