import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "dist/static/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "/static/js/",
        }),

        is_watch && serve({
            contentBase: "dist",
            open: true,
            historyApiFallback: true
        }),

        is_watch && livereload("dist"),

        !is_watch && terser(),
    ],
};
