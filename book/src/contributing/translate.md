# Translation

For more information, please refer to

- <https://github.com/google/comprehensive-rust/blob/main/TRANSLATIONS.md>
- <https://github.com/google/mdbook-i18n-helpers>

## Prerequests

- Install [gnu gettext](https://www.gnu.org/software/gettext/)
- Install [mdbook](https://github.com/rust-lang/mdBook): `cargo install mdbook`
- Install [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid): `cargo install mdbook-mermaid`
- Install [mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers): `cargo install mdbook-i18n-helpers`

## Init

1. init po

    ```bash
    just book_init xx
    ```

2. add language select button

    file: _./book/theme/index.hbs_

    ```diff
     <ul id="language-list" class="theme-popup" aria-label="Languages" role="menu">
         <li role="none"><button role="menuitem" class="theme"><a id="en">English</a></button></li>
    +    <li role="none"><button role="menuitem" class="theme"><a id="xx">Some Language</a></button></li>
     </ul>
    ```

3. update github action

    file: _.github/workflows/gh-pages.yaml_

    ```diff
     env:
         CARGO_TERM_COLOR: always
         # Update the language picker in index.hbs to link new languages.
    -    LANGUAGES: zh
    +    LANGUAGES: zh xx
    ```

## Build

```bash
just book_build xx
```

## Serve

```bash
just book_serve xx
```

## Update po

```bash
just book_update xx
```

## Serve all languages

1. build all languages

    ```bash
    just book_build_all
    ```

2. you may need a http server like [TheWaWaR/simple-http-server](https://github.com/TheWaWaR/simple-http-server)

    ```bash
    simple-http-server -- ./book/book
    ```
