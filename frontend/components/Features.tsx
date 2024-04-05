import Link from "next/link";
import Image from "next/image";
import { motion } from "framer-motion";

export default function Features() {
    return (
        <motion.section
            initial={{ opacity: 0, transform: "translateY(25px)" }}
            whileInView={{ opacity: 1, transform: "translateY(0)" }}
            transition={{ duration: 1 }}
            viewport={{ once: true }}
            className="pt-12 md:pt-24 lg:pt-32 bg-black">
            <div className="container space-y-10 xl:space-y-16">
                <div className="grid gap-4 px-10 md:grid-cols-2 md:gap-16">
                    <div>
                        <h1 className="lg:leading-tighter text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl xl:text-[3.4rem] 2xl:text-[3.75rem]">
                            The Future of Stability: GoldCoin
                        </h1>
                    </div>
                    <div className="flex flex-col items-start space-y-4">
                        <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl/relaxed dark:text-gray-400">
                            Real Stability. Transparent. Secure.
                        </p>
                    </div>
                </div>
                <div className="grid gap-4 px-4 md:grid-cols-3 md:gap-8">
                    <div className="flex flex-col items-start space-y-2">
                        <Link
                            className="inline-flex flex-1 flex-col rounded-lg overflow-hidden border border-gray-200 shadow-sm hover:shadow transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950"
                            href="#">
                            <Image
                                alt="Image"
                                className="aspect-square object-cover object-center"
                                height="400"
                                src="/gc.png"
                                width="400"
                            />
                            <div className="p-4 flex-1">
                                <h3 className="font-bold leading-tight">
                                    Buttons
                                </h3>
                                <p className="text-sm text-gray-500 dark:text-gray-400">
                                    A collection of buttons with multiple states
                                    and sizes.
                                </p>
                            </div>
                        </Link>
                    </div>
                    <div className="flex flex-col items-start space-y-2">
                        <Link
                            className="inline-flex flex-1 flex-col rounded-lg overflow-hidden border border-gray-200 shadow-sm hover:shadow transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950"
                            href="#">
                            <Image
                                alt="Image"
                                className="aspect-[16/9] object-cover object-center"
                                height="250"
                                src="/placeholder.svg"
                                width="400"
                            />
                            <div className="p-4 flex-1">
                                <h3 className="font-bold leading-tight">
                                    Buttons
                                </h3>
                                <p className="text-sm text-gray-500 dark:text-gray-400">
                                    A collection of buttons with multiple states
                                    and sizes.
                                </p>
                            </div>
                        </Link>
                    </div>
                    <div className="flex flex-col items-start space-y-2">
                        <Link
                            className="inline-flex flex-1 flex-col rounded-lg overflow-hidden border border-gray-200 shadow-sm hover:shadow transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950"
                            href="#">
                            <Image
                                alt="Image"
                                className="aspect-[16/9] object-cover object-center"
                                height="250"
                                src="/placeholder.svg"
                                width="400"
                            />
                            <div className="p-4 flex-1">
                                <h3 className="font-bold leading-tight">
                                    Buttons
                                </h3>
                                <p className="text-sm text-gray-500 dark:text-gray-400">
                                    A collection of buttons with multiple states
                                    and sizes.
                                </p>
                            </div>
                        </Link>
                    </div>
                </div>
            </div>
        </motion.section>
    );
}
