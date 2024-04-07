import { Link } from "lucide-react";
import { useEffect } from "react";
import { SITE, SOCIALS } from "../lib/consts";
import Animated from "./Animated";
import Container from "./Container";
import Logo from "./Logo";

const Footer = () => {
    const goBackToTop = (event: MouseEvent) => {
        event.preventDefault();
        window.scrollTo({
            top: 0,
            behavior: "smooth",
        });
    };

    const initializeBackToTop = () => {
        const backToTop = document.getElementById("back-to-top");
        backToTop?.addEventListener("click", goBackToTop);
    };

    useEffect(() => {
        initializeBackToTop();
        document.addEventListener("astro:after-swap", initializeBackToTop);
        return () => {
            document.removeEventListener(
                "astro:after-swap",
                initializeBackToTop
            );
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    return (
        <footer className="relative bg-white dark:bg-black">
            <Animated>
                <section className="py-5">
                    <Container size="md">
                        <div className="flex items-center justify-center sm:justify-end">
                            <button
                                id="back-to-top"
                                aria-label="Back to top of page"
                                className="group flex w-fit p-1.5 gap-1.5 text-sm items-center border rounded hover:bg-black/5 hover:dark:bg-white/10 border-black/15 dark:border-white/20 transition-colors duration-300 ease-in-out">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="20"
                                    height="20"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    strokeWidth="2.5"
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    className="stroke-current group-hover:stroke-black group-hover:dark:stroke-white rotate-90">
                                    <line
                                        x1="19"
                                        y1="12"
                                        x2="5"
                                        y2="12"
                                        className="scale-x-0 group-hover:scale-x-100 translate-x-3 group-hover:translate-x-0 transition-all duration-300 ease-in-out"
                                    />
                                    <polyline
                                        points="12 19 5 12 12 5"
                                        className="translate-x-1 group-hover:translate-x-0 transition-all duration-300 ease-in-out"
                                    />
                                </svg>
                                <div className="w-full group-hover:text-black group-hover:dark:text-white transition-colors duration-300 ease-in-out">
                                    Back to top
                                </div>
                            </button>
                        </div>
                    </Container>
                </section>

                <section className="py-5 overflow-hidden whitespace-nowrap border-t border-black/10 dark:border-white/25">
                    <Container size="md">
                        <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <div className="flex flex-col items-center sm:items-start">
                                <Logo />
                            </div>
                            <div className="flex gap-2 justify-center sm:justify-end items-center">
                                <span className="relative flex h-3 w-3">
                                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-300"></span>
                                    <span className="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
                                </span>
                                Smart Contract Status
                            </div>
                        </div>
                    </Container>
                </section>

                <section className="py-5 overflow-hidden whitespace-nowrap border-t border-black/10 dark:border-white/25">
                    <Container size="md">
                        <div className="h-full grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <div className="order-2 sm:order-1 flex flex-col items-center justify-center sm:items-start">
                                <div className="backlinks flex flex-row gap-x-2">
                                    <a
                                        href="/#hero"
                                        className="text-current hover:text-black dark:hover:text-white transition-colors duration-300 ease-in-out">
                                        Home
                                    </a>
                                    |
                                    <a
                                        href="/#why"
                                        className="text-current hover:text-black dark:hover:text-white transition-colors duration-300 ease-in-out">
                                        Why?
                                    </a>
                                    |
                                    <a
                                        href="/#learn"
                                        className="text-current hover:text-black dark:hover:text-white transition-colors duration-300 ease-in-out">
                                        Learn
                                    </a>
                                    |
                                    <a
                                        href="/#trade"
                                        className="text-current hover:text-black dark:hover:text-white transition-colors duration-300 ease-in-out">
                                        Trade
                                    </a>
                                </div>
                                <div className="text-sm mt-2">
                                    &copy; 2024 | All rights reserved
                                </div>
                            </div>

                            <div className="order-1 sm:order-2 flex justify-center sm:justify-end">
                                <div className="flex flex-wrap gap-1 items-center justify-center">
                                    {SOCIALS.map((SOCIAL, index) => (
                                        <a
                                            key={index}
                                            href={SOCIAL.HREF}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            aria-label={`${SITE.TITLE} on ${SOCIAL.NAME}`}
                                            className="group size-10 rounded-full p-2 items-center justify-center hover:bg-black/5 dark:hover:bg-white/20  blend">
                                            <svg className="size-full fill-current group-hover:fill-black group-hover:dark:fill-white blend">
                                                <use
                                                    href={`/social.svg#${SOCIAL.ICON}`}
                                                />
                                            </svg>
                                        </a>
                                    ))}
                                </div>
                            </div>
                        </div>
                    </Container>
                </section>
            </Animated>
        </footer>
    );
};

export default Footer;
