# Pressent

Create good looking presentations with markdown.

<<>>
## Building
```shell
make
```
or
```shell
cargo build --release
```
that should create a `target/release/pressent` binary.
There is also a symlink `pressent` that points to `target/release/pressent`.

<<>>
## Usage
Pressent syntax is standard markdown syntax + the `<<>>` symbol for separating slides,
for instance:

`presentation.md`:
```markdown
# My Presentation

<<>>
## Hello

<<>>
## World
```
Running the presentation is done with: `pressent presentation.md`.
You can specify the port with `-p`.
The presentation will be served on `localhost:8000` (or any other port that was chosen).
