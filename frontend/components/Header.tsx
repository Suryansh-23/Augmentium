import Image from "next/image";
import { usePathname } from "next/navigation";
import { useEffect } from "react";
import { cn } from "../lib/utils";
import Container from "./Container"; // Import Container component from your project

const SITE = {
    TITLE: "GoldCoin",
    AUTHOR: "Suryansh Chauhan",
};

const LINKS = [
    {
        TEXT: "Home",
        HREF: "/",
    },
    {
        TEXT: "Work",
        HREF: "#work",
    },
    {
        TEXT: "Blog",
        HREF: "/blog",
    },
    {
        TEXT: "Projects",
        HREF: "/projects",
    },
];

const Header = () => {
    const pathname = usePathname();
    const subpath = pathname.match(/[^/]+/g);

    useEffect(() => {
        const initializeDrawerButton = () => {
            const drawerButton = document.getElementById(
                "header-drawer-button"
            );
            drawerButton?.addEventListener("click", toggleDrawer);
        };

        const toggleDrawer = () => {
            const drawer = document.getElementById("drawer");
            const drawerButton = document.getElementById(
                "header-drawer-button"
            );
            drawer?.classList.toggle("open");
            drawerButton?.classList.toggle("open");
        };

        initializeDrawerButton();
    }, []);

    return (
        <header id="header" className="fixed top-0 w-full h-16 z-50 ">
            <Container size="md">
                <div className="relative h-full w-full">
                    <div className="absolute left-0 top-1/2 -translate-y-1/2 flex gap-1 font-semibold">
                        <a
                            href="/"
                            className="flex gap-1 text-current hover:text-black dark:hover:text-white transition-colors duration-300 ease-in-out">
                            <Image
                                alt="GC"
                                width="24"
                                height="24"
                                className="size-6 fill-current mx-2"
                                title="GC"
                                src="/gc.png"
                            />
                            <div>{SITE.TITLE}</div>
                        </a>
                    </div>

                    <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
                        <nav className="hidden md:flex items-center justify-center text-sm gap-1">
                            {LINKS.map((LINK, index) => (
                                <a
                                    key={index}
                                    href={LINK.HREF}
                                    className={cn(
                                        "h-8 rounded-full px-3 text-current",
                                        "flex items-center justify-center",
                                        "transition-colors duration-300 ease-in-out",
                                        pathname === LINK.HREF ||
                                            "/" + subpath?.[0] === LINK.HREF
                                            ? "bg-black dark:bg-white text-white dark:text-black"
                                            : "hover:bg-black/5 dark:hover:bg-white/20 hover:text-black dark:hover:text-white"
                                    )}>
                                    {LINK.TEXT}
                                </a>
                            ))}
                        </nav>
                    </div>

                    <div className="buttons absolute right-0 top-1/2 -translate-y-1/2 flex gap-1">
                        <a
                            href="/search"
                            aria-label={`Search blog posts and projects on ${SITE.TITLE}`}
                            className={cn(
                                "hidden md:flex",
                                "size-9 rounded-full p-2 items-center justify-center",
                                "bg-transparent hover:bg-black/5 dark:hover:bg-white/20",
                                "stroke-current hover:stroke-black hover:dark:stroke-white",
                                "border border-black/10 dark:border-white/25",
                                "transition-colors duration-300 ease-in-out",
                                pathname === "/search" ||
                                    "/" + subpath?.[0] === "/search"
                                    ? "pointer-events-none bg-black dark:bg-white text-white dark:text-black"
                                    : ""
                            )}>
                            <svg className="size-full">
                                <use href="/ui.svg#search"></use>
                            </svg>
                        </a>

                        <button
                            id="header-drawer-button"
                            aria-label={`Toggle drawer open and closed`}
                            className={cn(
                                "flex md:hidden",
                                "size-9 rounded-full p-2 items-center justify-center",
                                "bg-transparent hover:bg-black/5 dark:hover:bg-white/20",
                                "stroke-current hover:stroke-black hover:dark:stroke-white",
                                "border border-black/10 dark:border-white/25",
                                "transition-colors duration-300 ease-in-out"
                            )}>
                            <svg id="drawer-open" className="size-full">
                                <use href="/ui.svg#menu"></use>
                            </svg>
                            <svg id="drawer-close" className="size-full">
                                <use href="/ui.svg#x"></use>
                            </svg>
                        </button>
                    </div>
                </div>
            </Container>
        </header>
    );
};

export default Header;
