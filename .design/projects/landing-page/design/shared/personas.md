# Personas

> Phase: design | Project: landing-page | Generated: 2026-04-08

---

## Primary Persona — the Senior Operator

**Name:** Ren  
**Demographics:** 35, staff-level Rust/Go/Python engineer. 10 years in the industry. Runs a home lab. Reads HN daily. Has opinions about software.

**Context of arrival:** Ren clicked a link from a Hacker News thread titled "I let Claude Code build a Rust project for three hours while I slept." Someone mentioned `mz` in the comments. Ren opened the tab in the background, finished reading the thread, then clicked over.

**Goals:**
- Understand in 5 seconds whether this is worth 2 minutes of reading
- Know what `mz` actually does — not marketing language, real function description
- Find the install command without hunting for it
- Evaluate whether the author is someone who writes real code, or someone selling vibes

**Pain points:**
- Tired of AI tooling landing pages that promise autonomy and deliver chat interfaces
- Allergic to the "agentic" / "empower" / "transform your workflow" voice
- Has been burned by tools that look good on the landing page and fall apart on real projects
- Does not want to sign up for anything, give an email, accept cookies

**Usage context:** Desktop, dark mode, latest Firefox or Chrome. Monospace font rendering is something Ren cares about. Will open DevTools to check bundle size if the page feels bloated.

**What converts Ren:** A real terminal recording showing the tool doing something a senior dev would actually do. The install command above the fold. Technical honesty. A refusals list that signals the maintainer has taste.

**What loses Ren instantly:** Gradient backgrounds, hero illustrations, "trusted by X companies," any hint of telemetry, typewriter reveal animations, rounded buttons.

---

## Secondary Persona — the Curious Observer

**Name:** Zara  
**Demographics:** 28, ML engineer at a mid-size company. Writes Python primarily, dabbles in Rust. Follows Ren on X.

**Context of arrival:** Ren retweeted the page. Zara isn't a daily terminal user but is interested in AI coding workflows.

**Goals:**
- Understand what this is at a conceptual level — does it fit their workflow?
- See if there's a docs link to read more before committing to install

**Pain points:**
- Less familiar with Rust/CLI tooling; might not know `cargo install`
- Doesn't use the terminal as their primary interface

**Usage context:** Desktop, may be on light mode, Chrome.

**What converts Zara:** Clear one-sentence explanation of what `mz` does, a visible docs link, and seeing that install is one command.

**Note:** Zara is secondary. The page optimizes for Ren. Zara is served by the existing layout — if Ren's conversion path is clear, Zara's is too.

---

## Related

- [navigation.md](./navigation.md)
- [information-architecture.md](./information-architecture.md)
- [../screen-01-landing.md](../screen-01-landing.md)
