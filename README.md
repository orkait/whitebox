# whitebox

Whitebox is the expression and rendering layer.

It is the layer the agent uses when it wants to express itself in a visible way.

## Role In The Architecture

Whitebox is not only a generic output plane.

It is the renderer/state layer for visible expression.

If the agent wants to speak, text to speech is how it does that.
If the agent wants to express through rendered output, Whitebox is the layer it
uses.

## Current Surface

Whitebox currently runs as a Tauri desktop surface.

The Rust crate in this repo is the embodiment core.
The Tauri shell renders that state in a desktop window and sends actions back into the core.

## Why It Exists

Whitebox exists so the agent can:

- render visible state
- present screen-based output
- express through visual surfaces
- give life that humans can see

## Related Interfaces

- text to speech handles spoken output
- speech to text handles spoken input
- Whitebox handles visible and rendered expression

## Development

Install JavaScript dependencies and run the desktop shell:

```bash
npm install
npm run tauri dev
```
