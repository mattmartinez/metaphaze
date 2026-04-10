# Content Strategy — Microcopy for metaphaze

How to write the words on the landing page so they read like a README, not a landing page. The Senior Operator is allergic to landing-page voice; every word needs to earn its place.

## The "README IS the Marketing" Principle

The single most important content decision for metaphaze: **the website and the README should share a voice, and ideally share copy.** The README is the source of truth. The landing page is a formatted, typeset version of the README — not a rewrite with more adjectives.

Why:
- The Senior Operator reads the README before installing. If the landing page tells a different story than the README, trust drops.
- Maintaining two voices is expensive. Pick one.
- The "one voice" discipline is its own positioning move — htmx, suckless, and Berkeley Graphics all do this.

What this means practically:
- The hero manifesto = the first paragraph of the README
- The "what it does" section = the README's description section
- The install block = the README's install section, verbatim
- The refusals list = the README's non-goals section, verbatim

The website should add nothing the README doesn't say. It should only present it better typographically.

## Headlines

Rules for every headline on the page:

1. **Lowercase.** No exceptions except `[BADGE]` codes. The brand is lowercase; the voice is lowercase; headlines are lowercase. "metaphaze orchestrates claude code" not "Metaphaze Orchestrates Claude Code."
2. **Short.** 2-6 words. "what it does" not "what metaphaze can do for you."
3. **Concrete.** A headline must name a thing. "install" not "get started." "refusals" not "our philosophy." "why it's different" not "the metaphaze difference."
4. **No questions.** "what is metaphaze" is bad; "metaphaze" as a heading followed by a sentence is good.
5. **No "introducing."** Ever. If a headline begins with "Introducing," rewrite it.
6. **No colons in headlines.** "metaphaze: the orchestrator" is bad; "metaphaze" alone is fine.

Examples of good headlines for this page:
- `metaphaze` (the hero, just the name)
- `what it does` (section)
- `why it's different` (section)
- `install` (section)
- `refusals` (footer section)

Examples of bad headlines:
- "Welcome to Metaphaze!"
- "Unleash the Power of Claude Code"
- "The Ultimate AI Harness"
- "Introducing metaphaze: a new way to..."

## The Manifesto (Hero Copy)

The manifesto is the single sentence below the logo. It must answer "what is this" in under 10 words.

Drafts:

- `a rust cli that orchestrates claude code` — pure description, 7 words
- `orchestrate claude code from your terminal` — imperative, 6 words
- `claude code, at terminal speed` — evocative, 5 words
- `a single binary that runs claude code sessions` — specific, 9 words

Recommended direction: pure description or imperative. Pure description is most honest; imperative is slightly more propulsive. Avoid evocative — it's the start of the slide toward marketing voice.

The manifesto should NOT include any of: "powerful," "modern," "built for," "designed to," "we believe," "the future of," "finally," "at last."

## Install Command Context

An install command needs three pieces of context:

1. **A prerequisite** — what the user needs before running this
2. **The command** — the literal `cargo install metaphaze` block
3. **The post-install verification** — how the user knows it worked

Draft:

```
requires rust 1.76+ and cargo.

    cargo install metaphaze

verify:

    metaphaze --version
    metaphaze 0.1.0
```

That's the entire install section. No "pro tip," no "troubleshooting link," no "join our discord for help." The Senior Operator can figure out troubleshooting from the GitHub issues tab.

If the command requires additional flags or configuration, put them AFTER the copy block, not before. The visitor should be able to copy the command without reading past the first block.

## The Refusals List

The refusals list is metaphaze's strongest positioning asset. It's a public statement of what the product will NOT do. Examples of well-written refusals:

```
metaphaze refuses:
  - opt-in telemetry
  - background daemons
  - config files in ~/.config (use XDG or stdin)
  - plugin architectures
  - web dashboards
  - "agentic" anything
  - any dependency not in the rust stdlib or tokio
```

Rules for writing refusals:
1. **Be specific.** "no telemetry" is vague; "no opt-in or opt-out telemetry, ever" is specific.
2. **Lowercase.** Match the voice.
3. **One verb per line.** "refuses" at the top, then a bulleted list.
4. **No justifications in the list itself.** Don't explain why. The refusal IS the explanation.
5. **Keep it short.** 5-10 items max. A list of 50 reads as defensive; a list of 7 reads as deliberate.

The refusals list is the footer's main content. It should be the last thing the visitor reads before leaving — a final positioning anchor.

## Banned Words and Phrases

Explicit banlist for every word on this page:

**Marketing verbs:**
- agentic (forbidden by brand)
- empower, empowers, empowerment
- transform, transforms, transformation
- unlock, unlocks
- leverage, leverages
- enable, enables (unless technical: "enable the feature")
- journey (as a metaphor for workflow)
- ecosystem (as a metaphor for a product suite)
- revolutionize, revolutionary
- innovative, innovation
- seamless, seamlessly
- robust (unless describing error handling)
- powerful, power (unless referring to electricity)
- intelligent, smart (unless describing specific algorithms)
- solution (as a product synonym)
- next-generation, next-gen
- cutting-edge, bleeding-edge
- game-changing, game-changer
- paradigm (shift)
- synergy, synergistic
- holistic
- streamline, streamlined

**Marketing nouns:**
- workflow (unless literally referring to GitHub Actions)
- productivity (the metric, not the idea)
- efficiency (when used vaguely)
- experience (as in "developer experience" — spell out what you mean)

**Marketing phrases:**
- "built for developers" (redundant on a developer tool)
- "designed with X in mind"
- "we believe that"
- "at scale"
- "best-in-class"
- "world-class"
- "state-of-the-art"
- "future-proof"
- "out of the box"
- "just works" (cliché, but arguably acceptable — be careful)

**Allowed but with caution:**
- "simple" — only if the thing actually is simple
- "fast" — only with a concrete benchmark
- "small" — only with a specific binary size
- "single binary" — fine, factual

## Tone Calibration Examples

Wrong (landing page voice):
> Metaphaze transforms the way you work with AI agents, empowering developers to unlock the full potential of Claude Code through our innovative orchestration platform.

Right (README voice):
> metaphaze runs claude code sessions from the command line. one binary. mit licensed.

Wrong:
> Get started in minutes with our simple installation process!

Right:
> install:
>
>     cargo install metaphaze

Wrong:
> Built by developers, for developers — metaphaze is the modern way to orchestrate AI agents.

Right:
> built in rust. uses tokio. mit license. source on github.

## Link Text Conventions

Consistent link text patterns across the page:

- **Internal nav**: `[/docs]`, `[/examples]`, `[/changelog]` — bracketed with leading slash, like a file path
- **External links**: plain URL text, e.g. `github.com/metaphaze/metaphaze` — no "click here," no "learn more"
- **Action buttons**: bracketed verb, e.g. `[ copy ]`, `[ install ]`, `[ view source ]` — lowercase verb in brackets
- **GitHub link**: always `github.com/metaphaze/metaphaze`, never "GitHub" or "Our Repo"
- **License**: always `MIT` in caps (it's an acronym), never "mit license"

The brackets on nav items and action buttons are a visual pattern, not just decoration — they signal "this is a command you can issue to the site." The pattern is consistent with Berkeley Graphics and reinforces the terminal metaphor.

## Information Density

How much explainer text should precede the install command?

Answer: one paragraph, 2-3 sentences, under 40 words.

Example:

> metaphaze runs claude code sessions from the command line. one binary. no config file.

That's 16 words. Then the install command. That's the correct ratio for the Senior Operator persona — enough context to confirm they're on the right page, not enough to bury the command.

If the explainer grows beyond 3 sentences, move the extra sentences to the "what it does" section below the install block. The hero should be ruthless about word count.

## Section Copy Drafts

Drafts for each section's body copy, to be refined in the design phase:

**Hero:**
```
mz▌

metaphaze runs claude code sessions from the command line.
one binary. no config file. mit licensed.

    cargo install metaphaze

[ copy ]
```

**What it does:**
```
what it does

metaphaze wraps the claude code cli and orchestrates
multi-session workflows. you write a .tape-style script;
metaphaze runs it and logs every command claude executed.

┌──────────┐   ┌───────────────┐   ┌──────────┐
│ you      │──▶│   metaphaze   │──▶│ claude   │
└──────────┘   └───────────────┘   └──────────┘
                       │
                       ▼
                ┌──────────┐
                │   logs   │
                └──────────┘
```

**Why it's different:**
```
why it's different

              metaphaze    [ harness A ]    [ harness B ]
telemetry     [NO]         [OPT-OUT]        [ALWAYS]
config file   [NONE]       [YAML]           [JSON]
binary size   [3MB]        [28MB]           [450MB]
daemon        [NO]         [YES]            [YES]
license       [MIT]        [APACHE]         [PROPRIETARY]
```

**Install:**
```
install

requires rust 1.76+ and cargo.

    cargo install metaphaze

verify:

    metaphaze --version
    metaphaze 0.1.0
```

**Footer:**
```
mit · github.com/metaphaze/metaphaze

refusals:
  - opt-in telemetry
  - background daemons
  - web dashboards
  - "agentic" anything
  - plugin architectures
  - config files (use flags)
```

These are drafts — the design phase will refine them. But the voice is set.
