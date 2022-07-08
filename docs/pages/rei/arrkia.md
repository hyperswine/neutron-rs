---
layout: default
title: Arrkia
parent: Rei
---

Arrkia is an extension to Phanta Rei. It is like TS + NextJS + Chakra UI + Qt Designer + Flutter

```rust
// ensures this is seen/compiled as an arrkia component with FFX and arrkia hooks
@export-default, component 
ExampleComponent() {
    @state
    clicks = state::Count(0)

    return (
        <>
            <Box bgColor="cyan" textColor="white">
                <H1>This is an Arrkia Component!</H1>
            </Box>
        </>
    )
}
```
