/** @type {import('tailwindcss').Config} */

import defaultTheme from "tailwindcss/defaultTheme";

import svgToDataUri from "mini-svg-data-uri";
import { default as flattenColorPalette } from "tailwindcss/lib/util/flattenColorPalette";

module.exports = {
    content: [
        "./app/**/*.{js,ts,jsx,tsx,mdx}",
        "./pages/**/*.{js,ts,jsx,tsx,mdx}",
        "./components/**/*.{js,ts,jsx,tsx,mdx}",
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ["Atkinson", ...defaultTheme.fontFamily.sans],
            },
            typography: {
                DEFAULT: {
                    css: {
                        maxWidth: "full",
                    },
                },
            },
            rotate: {
                45: "45deg",
                135: "135deg",
                225: "225deg",
                315: "315deg",
            },
            animation: {
                twinkle: "twinkle 2s ease-in-out forwards",
                meteor: "meteor 3s ease-in-out forwards",
            },
            keyframes: {
                twinkle: {
                    "0%": {
                        opacity: 0,
                        transform: "rotate(0deg)",
                    },
                    "50%": {
                        opacity: 1,
                        transform: "rotate(180deg)",
                    },
                    "100%": {
                        opacity: 0,
                        transform: "rotate(360deg)",
                    },
                },
                meteor: {
                    "0%": {
                        opacity: 0,
                        transform: "translateY(200%)",
                    },
                    "50%": {
                        opacity: 1,
                    },
                    "100%": {
                        opacity: 0,
                        transform: "translateY(0)",
                    },
                },
            },
        },
    },
    plugins: [
        function ({ matchUtilities, theme }: any) {
            matchUtilities(
                {
                    "bg-grid": (value: any) => ({
                        backgroundImage: `url("${svgToDataUri(
                            `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="32" height="32" fill="none" stroke="${value}"><path d="M0 .5H31.5V32"/></svg>`
                        )}")`,
                    }),
                    "bg-grid-small": (value: any) => ({
                        backgroundImage: `url("${svgToDataUri(
                            `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="8" height="8" fill="none" stroke="${value}"><path d="M0 .5H31.5V32"/></svg>`
                        )}")`,
                    }),
                    "bg-dot": (value: any) => ({
                        backgroundImage: `url("${svgToDataUri(
                            `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="16" height="16" fill="none"><circle fill="${value}" id="pattern-circle" cx="10" cy="10" r="1.6257413380501518"></circle></svg>`
                        )}")`,
                    }),
                },
                {
                    values: flattenColorPalette(theme("backgroundColor")),
                    type: "color",
                }
            );
        },
    ],
};

