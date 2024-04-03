const { fontFamily } = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.rs"],
    theme: {
        extend: { fontFamily: { sans: ["Inter var", ...fontFamily.sans] } },
    },
    plugins: [],
};
