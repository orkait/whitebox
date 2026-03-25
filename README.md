# whitebox

Whitebox is the surface layer.

It is the visible output plane of the system.

## Role In The Architecture

This layer exists so the agent can give life which humans can see with visuals.

The architecture currently describes this surface as:

- Canvas
- Terminal
- NxN pixel Asciii Box

It outputs to screen.

## Visibility Function

Whitebox is where the system becomes visible to humans.
It is the presentation surface for agent output and visual presence.

## Related Interfaces

The architecture also mentions:

- text to speech
- speed to text

Those interaction modes sit close to the visible surface layer even if they are
not yet split into a separate repo in this layout.
