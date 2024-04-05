const Drawer = () => {
    return (
        <div
            id="drawer"
            className="fixed inset-0 h-0 z-40 overflow-hidden flex flex-col items-center justify-center md:hidden bg-neutral-100 dark:bg-neutral-900 transition-[height] duration-300 ease-in-out">
            <nav className="flex flex-col items-center space-y-2">
                {/* {LINKS.map((LINK) => (
                    <a
                        href={LINK.HREF}
                        className={cn(
                            "flex items-center justify-center px-3 py-1 rounded-full",
                            "text-current hover:text-black dark:hover:text-white",
                            "hover:bg-black/5 dark:hover:bg-white/20",
                            "transition-colors duration-300 ease-in-out",
                            pathname === LINK.HREF ||
                                "/" + subpath?.[0] === LINK.HREF
                                ? "pointer-events-none bg-black dark:bg-white text-white dark:text-black"
                                : ""
                        )}>
                        {LINK.TEXT}
                    </a>
                ))} */}
            </nav>

            <div className="flex gap-1 mt-5">
                <a
                    href="/search"
                    aria-label={`Search blog posts and projects on`}>
                    <svg className="size-full">
                        <use href="/ui.svg#search"></use>
                    </svg>
                </a>

                <a
                    href="/rss.xml"
                    target="_blank"
                    aria-label={`Rss feed for`}
                    className="size-9 rounded-full p-2 items-center justify-center bg-transparent hover:bg-black/5 dark:hover:bg-white/20 stroke-current hover:stroke-black hover:dark:stroke-white border border-black/10 dark:border-white/25 transition-colors duration-300 ease-in-out">
                    <svg className="size-full">
                        <use href="/ui.svg#rss"></use>
                    </svg>
                </a>

                <button
                    id="drawer-theme-button"
                    aria-label={`Toggle light and dark theme`}
                    className="size-9 rounded-full p-2 items-center justify-center bg-transparent hover:bg-black/5 dark:hover:bg-white/20 stroke-current hover:stroke-black hover:dark:stroke-white border border-black/10 dark:border-white/25 transition-colors duration-300 ease-in-out">
                    <svg className="block dark:hidden size-full">
                        <use href="/ui.svg#sun"></use>
                    </svg>
                    <svg className="hidden dark:block size-full">
                        <use href="/ui.svg#moon"></use>
                    </svg>
                </button>
            </div>
        </div>
    );
};

export default Drawer;
