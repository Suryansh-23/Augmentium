import { motion } from "framer-motion";
import { BarChart, PenLine } from "lucide-react";
import Image from "next/image";
import { cn } from "../lib/utils";
import { Card } from "./ui/Card";

const Bars = () => {
    const variants = {
        initial: {
            height: 0,
        },
        animate: {
            height: "100%",
            transition: {
                duration: 0.2,
            },
        },
        hover: {
            height: ["0%", "100%"],
            transition: {
                duration: 2,
            },
        },
    };
    const arr = new Array(8).fill(0);
    return (
        <motion.div
            initial="initial"
            animate="animate"
            whileHover="hover"
            className="flex flex-row gap-x-4 items-end w-full h-full min-h-[7rem] dark:bg-dot-white/[0.2] bg-dot-black/[0.2] space-y-2">
            {arr.map((_, i) => (
                <motion.div
                    key={"skelenton-two" + i}
                    variants={variants}
                    style={{
                        maxHeight: i * (Math.random() + 1) * 1 + 2 + "rem",
                    }}
                    className={cn(
                        `rounded-xl border border-neutral-100 dark:border-white/[0.2] p-3 items-end space-x-2 bg-neutral-100 dark:bg-black`,
                        i === 0 ? "ml-3" : ""
                    )}
                />
            ))}
        </motion.div>
    );
};

const Coin = () => {
    return (
        <div
            // initial="initial"
            // animate="animate"
            // variants={variants}
            // transition={{
            //     duration: 5,
            //     repeat: Infinity,
            //     repeatType: "reverse",
            // }}
            className="flex flex-1 w-full h-full min-h-[6rem] dark:bg-dot-white/[0.2] rounded-lg bg-dot-black/[0.2] flex-col space-y-2 bg-red-500">
            <div className="h-full w-full dark:bg-black bg-white  dark:bg-dot-white/[0.4] relative flex items-center justify-center">
                {/* Radial gradient for the container to give a faded look */}
                <div className="absolute pointer-events-none inset-0 flex items-center justify-center dark:bg-black bg-white [mask-image:radial-gradient(ellipse_at_center,transparent_20%,black)]"></div>
                <Image
                    alt="Image"
                    className="aspect-square object-cover object-center"
                    height="250"
                    src="/gc_3d.png"
                    width="250"
                />
            </div>
        </div>
    );
};

export default function Features() {
    const feats = [
        {
            title: "Contextual Suggestions",
            description: (
                <span className="text-sm">
                    Get AI-powered suggestions based on your writing context.
                </span>
            ),
            header: <Coin />,
            className: "md:col-span-1",
            icon: <PenLine className="h-4 w-4 text-neutral-500" />,
        },
        {
            title: "Automated Proofreading",
            description: (
                <span className="text-sm">
                    Let AI handle the proofreading of your documents.
                </span>
            ),
            header: <Bars />,
            className: "md:col-span-1",
            icon: <BarChart className="h-4 w-4 text-neutral-500" />,
        },
    ];

    return (
        <motion.section
            id="why"
            initial={{ opacity: 0, transform: "translateY(25px)" }}
            whileInView={{ opacity: 1, transform: "translateY(0)" }}
            transition={{ duration: 1 }}
            viewport={{ once: true }}
            className="pt-12 md:pt-24 lg:pt-32 bg-black">
            <div className="container space-y-10 xl:space-y-16">
                <div className="grid gap-4 px-10 md:grid-cols-2 md:gap-16">
                    <div>
                        <h1 className="lg:leading-tighter text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl xl:text-[3.4rem] 2xl:text-[3.75rem]">
                            The Future of Stability: Augmentium
                        </h1>
                    </div>
                    <div className="flex flex-col items-start space-y-4">
                        <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl/relaxed dark:text-gray-400">
                            Real Stability. Transparent. Secure.
                        </p>
                    </div>
                </div>
                <div className="grid gap-4 px-4 md:grid-cols-3 md:gap-8">
                    <Card
                        title={feats[0].title}
                        description={feats[0].description}
                        header={feats[0].header}
                        icon={feats[0].icon}
                        className={feats[0].className}
                    />
                    <Card
                        title={feats[1].title}
                        description={feats[1].description}
                        header={feats[1].header}
                        icon={feats[1].icon}
                        className={feats[1].className}
                    />
                </div>
            </div>
        </motion.section>
    );
}
