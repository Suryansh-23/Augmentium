import Link from "next/link";
import Image from "next/image";
import { SITE } from "../lib/consts";

const Logo = () => {
    return (
        <Link
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
        </Link>
    );
};

export default Logo;
