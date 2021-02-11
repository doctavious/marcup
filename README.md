# Markus

what is this about

markup to AST

- Markdown using commonmark as the spec
- Asciidoc
- reStructuredText (future)


## Motivation

- Comrak doesnt support front matter.
- would like it to be nicer to work with getting things from AST as well as change portions about the AST
- pulldown-cmark does appear to provide an AST so modifying markdown might be more challenging


## Features 

Support AST and streaming.

https://www.reddit.com/r/rust/comments/7a80l5/parsing_with_nom_keeping_track_of_position_in/
https://github.com/fflorent/nom_locate

> You can return custom types with nom, which means you can in theory take in a (&str, Loc) and return a (&str, Loc). If anyone has a better suggestion I would be interested in hearing it though.

> It would sure be nice if nom supported something like this by default...

> from my point of view, nom supports it: it provides a way to define your own input type and work with it. I try to keep the library small, because a lot of use cases will be very specific. Input handling is often specific to the file/IO code the program is using.

### AST 
Follow [Markdown Abstract Syntax Tree (mdast)](https://github.com/syntax-tree/mdast) specification which is used by unified/unifyjs which power  @remarkjs, @rehypejs, @retextjs, and @redotjs, used to build things like @mdx-js, @prettier, @gatsbyjs

https://github.com/syntax-tree/unist

https://unifiedjs.com/explore/package/remark-parse/ 
is build on top of 
https://github.com/micromark/micromark
https://github.com/syntax-tree/mdast-util-from-markdown


pub type MarkdownText = Vec<MarkdownInline>;

#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownInline {
Link(String, String), --> PhrasingContent
Image(String, String),  --> StaticPhrasingContent
InlineCode(String), --> StaticPhrasingContent
Bold(String), --> StaticPhrasingContent
Italic(String), --> StaticPhrasingContent
Plaintext(String), --> StaticPhrasingContent
}


## Name

Word play on Conrad Marcus the host of the Venom symbiote after Eddie Brock. Close to Markup

## Research 

- https://users.rust-lang.org/t/is-there-a-better-way-to-represent-an-abstract-syntax-tree/9549/4
- https://michael-f-bryan.github.io/static-analyser-in-rust/book/parse/ast.html
- https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/associated-constants.html
- https://amedee.me/post/2019-09-01-tsplib-nom-parser/
- https://www.leonrische.me/pages/parsing_scheme_with_nom.html

related to content model

- https://stackoverflow.com/questions/40776020/is-there-any-way-to-restrict-a-generic-type-to-one-of-several-types
- https://stackoverflow.com/questions/52240099/should-i-use-enums-or-boxed-trait-objects-to-emulate-polymorphism

# Features
- Lossless representation - Unified/remark doesnt appear to be lossless. Example of this is with new lines.

### Nom 

look at https://github.com/HGHimself/prose

https://github.com/Geal/nom/issues/14 - this issue includes nom implementations for a variety of formats 

### Pest


## Remark 

```shell
remark input.md --tree-out
```

## Run

Test with print
```shell
 RUST_BACKTRACE=1 cargo test -- --nocapture
```
