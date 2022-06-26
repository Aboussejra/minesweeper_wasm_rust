# rust_wasm_vue_minesweeper

This is an implementation of a Minesweeper (see rules here : https://en.wikipedia.org/wiki/Minesweeper_(video_game))

Whole logic is written in Rust and compiled using Web Assembly, really basic frontend logic is written using vue.js along with TypeScript (built with vite https://vitejs.dev/#browser-support)

As a rust enthusiast and software engineer developing in rust professionaly, goal was discovering Web Assembly to get more insight on what the future of web could be like.

While doing that, I tried to put that in a Vue.js project to understand a bit more frontend dev (being mainly a back-end software engineer), because more knowledge is always good to take.

# Build

## Building wasm

After install wasm-pack (see https://rustwasm.github.io/wasm-pack/installer/) do in root directory:

``` 
wasm-pack build --target web
```

## Running webapp

Go into `UI-minesweeper` directory (`cd UI-minesweeper`)

Install frontend dependencies (if needed install yarn here https://yarnpkg.com/, one could use npm)

```
yarn install
```

And then run 

``` 
yarn run dev
```

