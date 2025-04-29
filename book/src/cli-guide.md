# Walking the Trails (CLI Guide)

🌲 RangerKit is not just a book — it is a trail that can be walked.

Every spirit you meet, every waystone you mark, begins with the CLI:  
your staff, your compass, your voice on the trail.

This guide will teach you how to commune with the spirits,  
how to summon them,  
and how to hear their whispers.

---

# RangerKit CLI Structure

RangerKit CLI commands all begin with the word:

```bash
rangerkit
```

and branch into subcommands like trails branching from a clearing.

---

# Communing with the Spirits

To see the spirits who walk beside you, use:

```bash
rangerkit spirits commune
```

🌿 This gathers all known spirits into a circle:
- Their glyphs
- Their names
- Their abilities
- Their whispers

You may also choose how you listen:

| Mode | Command |
|:---|:---|
| Friendly human text (default) | `rangerkit spirits commune` |
| Structured JSON for shells like Nushell | `rangerkit spirits commune --format json` |

---

# Summoning a Spirit

To call a specific spirit to your side, use:

```bash
rangerkit spirits summon "Spirit Name"
```

For example:

```bash
rangerkit spirits summon "Lantern Spirit"
```

🌟 If the spirit knows your voice, it will answer —  
showing its glyph, name, and abilities.

🌫️ If the spirit does not know your voice,  
the forest will remain silent, and you will be gently told.

---

# Spirit Commands Summary

| Command | Description |
|:---|:---|
| `rangerkit spirits commune` | Gather all known spirits around you |
| `rangerkit spirits commune --format json` | Gather spirits and output structured JSON |
| `rangerkit spirits summon <spirit-name>` | Summon a specific spirit to your trail |

---

# Closing Blessing

> *To call the spirits is not merely to issue commands.  
> It is to speak, and to listen, and to remember that the forest listens too.*



  
