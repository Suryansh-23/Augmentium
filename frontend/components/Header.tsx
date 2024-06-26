import { usePathname } from "next/navigation";
import { useEffect } from "react";
import { LINKS, SITE } from "../lib/consts";
import { cn } from "../lib/utils";
import Container from "./Container"; // Import Container component from your project
import Logo from "./Logo";
import { useChain, useWallet } from "@cosmos-kit/react";

const Header = () => {
    const pathname = usePathname();
    const subpath = pathname.match(/[^/]+/g);

    const { username, connect, disconnect, wallet, openView } =
        useChain("mantrachaintestnet");
    const { status, mainWallet } = useWallet("keplr-extension");

    useEffect(() => {
        if (status === "Connected") {
            console.log("Connected to Keplr wallet");
            console.log("username:", username);
            console.log("wallet:", mainWallet);
        }
    }, [status]);

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
        const onScroll = () => {
            const header = document.getElementById("header");
            if (!header) return;

            if (window.scrollY > 0) {
                header.classList.add("scrolled");
            } else {
                header.classList.remove("scrolled");
            }
        };

        document.addEventListener("scroll", onScroll);
        initializeDrawerButton();

        return () => {
            removeEventListener("scroll", onScroll);
        };
    }, []);

    return (
        <header id="header" className="fixed top-0 w-full h-16 z-50 ">
            <Container size="md">
                <div className="relative h-full w-full">
                    <div className="absolute left-0 top-1/2 -translate-y-1/2 flex gap-1 font-semibold">
                        <Logo />
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
                        <button
                            onClick={async () => {
                                // openView();
                                if (status !== "Connected") {
                                    openView();
                                } else {
                                    disconnect();
                                    console.log(
                                        "Disconnected from Keplr wallet"
                                    );
                                }
                            }}
                            title={
                                status !== "Connected"
                                    ? "Connect Wallet"
                                    : "Connected @ " + username
                            }
                            className={cn(
                                "hidden md:flex",
                                "rounded-full p-2 px-4 items-center justify-center",
                                "bg-transparent hover:bg-black/5 dark:hover:bg-white/20",
                                "stroke-current hover:stroke-black hover:dark:stroke-white",
                                "border border-black/10 dark:border-white/25",
                                "transition-colors duration-300 ease-in-out",
                                pathname === "/search" ||
                                    "/" + subpath?.[0] === "/search"
                                    ? "pointer-events-none bg-black dark:bg-white text-white dark:text-black"
                                    : ""
                            )}>
                            {status !== "Connected"
                                ? "Connect Wallet"
                                : "Disconnect"}
                        </button>
                    </div>
                </div>
            </Container>
        </header>
    );
};

export default Header;
