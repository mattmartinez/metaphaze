# Competitor UX Deep-Dives (Product Level)

Five competitor sites analyzed at the interaction/composition level. Brand-level competitive audit was already done in `.design/branding/metaphaze/discover/competitive-audit.md` — this chunk is strictly about UX patterns, hero composition, scroll depth, and things metaphaze's design phase needs to decide.

---

## 1. htmx.org

**Hero composition (above the fold, 1280x800):**
- Top-left: `</> htmx` logotype, small, monospace
- Top-right: nav — `docs · reference · examples · essays · talk · book · support`
- Center-left: one-sentence manifesto in ~48px monospace ("high power tools for HTML")
- Center-right: small sponsored/donate block
- Below manifesto: a 5-line HTML code example that demonstrates htmx in action
- Below code: `npm` / `unpkg` install options as plain text
- All black text on white (htmx is light-mode by default)

**How they present installation:**
- Three installation paths shown as tabs: `npm`, `Yarn`, `CDN`
- The CDN line is the dominant option — copy-paste a `<script>` tag
- No "Copy" button — the visitor is expected to select and copy themselves
- No post-install verification output

**Navigation pattern:**
- Nav uses no brackets; items separated by `·` dots
- Nav has hover underline — minimal decoration
- Clicking `docs` goes to a separate site (htmx.org/docs) with the same typography

**"What it does" explainer:**
- The code example IS the explainer
- Below the hero, a 4-paragraph essay titled "motivation" explains why htmx exists
- No feature grid, no icons, no comparison table
- First-person voice: "We believe..."

**Strengths:** single voice throughout; code example functions as both proof and demo; nav bar doesn't compete with content; refusal to use marketing illustrations.

**Weaknesses:** the first-person "we" voice occasionally reads as confessional rather than declarative; the light background reduces contrast signal compared to a dark terminal-like look; no VHS-style demo.

**Adopt for metaphaze:** the "code example IS the explainer" principle; the four-part hero (logo, manifesto, code, install); the `docs / reference / examples` nav brevity.

**Avoid for metaphaze:** the light background; the first-person "we" voice (metaphaze is imperative/declarative); the CDN tab pattern.

---

## 2. suckless.org

**Hero composition:**
- Top: plain text header "suckless.org · software that sucks less"
- Left sidebar: category list (dwm, st, slock, tabbed, dmenu, ...)
- Right content: news feed, release notes, mailing list announcements
- All black Helvetica (!) on white — yes, suckless is NOT monospace on their landing page
- No images, no icons, no CTAs, no install command above the fold
- Footer: mirror links, mailing list signup (one of the few dev sites that DOES have a mailing list, and it's plain text)

**How they present installation:**
- Installation is on each project's individual page, not on the landing
- On project pages: a single paragraph ("get the sources"), then a git clone URL, then `make install`
- No package manager path shown first — they assume you compile from source

**Navigation pattern:**
- Left sidebar nav, sticky
- No brackets, no decoration, plain underlined links
- "hosted by nixos.org" footer link — signals allegiance

**"What it does" explainer:**
- The tagline "software that sucks less" is the entire pitch
- The news feed implicitly signals "this is an active project"
- Individual project pages have one paragraph of description each

**Strengths:** absolute refusal to sell; every pixel is functional; the design is the message; no wasted motion.

**Weaknesses:** genuinely hostile to newcomers — if you don't already know what dwm is, the site will not explain it to you; news-feed-as-landing-page is dated; sidebar nav feels 2008.

**Adopt for metaphaze:** the refusal to sell; the single-voice principle; the "every pixel is functional" discipline.

**Avoid for metaphaze:** the Helvetica (metaphaze is monospace-only); the sidebar nav; the news feed as landing content; the hostility to newcomers (metaphaze can be terse without being hostile).

---

## 3. charm.sh (and charm.land)

**Hero composition:**
- Top: charm logo (pink heart) + nav (products, blog, about, community)
- Center: animated TUI demo (NOT a video — it's an actual running TUI embedded via `charm.sh` custom tech)
- Below demo: tagline "The Charm'd Life — tools to make your command line glamorous"
- Further down: product grid of 12+ tools (gum, bubbletea, lipgloss, glamour, skate, pop, vhs, charmtone, mods, bubbles, soft-serve, wish)
- Footer: newsletter signup (yes, charm has a newsletter), social links, merch store

**How they present installation:**
- Per-product pages each have `brew install`, `go install`, and `apt` options
- No install command on the landing itself — the landing is a portfolio, not a product page
- Install commands appear as plain monospace blocks, no copy button

**Navigation pattern:**
- Top nav with underline-on-hover
- Product grid acts as secondary nav
- No brackets — charm uses brand color (pink) as the accent

**"What it does" explainer:**
- The animated TUI demos ARE the explainer
- Each product card has a 1-sentence description
- Blog posts linked prominently for long-form explainers

**Strengths:** charm.sh is the gold standard for "polish is OK if the polish is terminal polish"; the animated TUI demos are the single best thing on any dev-tool landing page; the product grid makes a portfolio site scannable.

**Weaknesses:** metaphaze is one tool, not a portfolio; the pink-and-rainbow brand is the opposite of metaphaze's `#5fb878`-on-black restraint; newsletter signup violates metaphaze rules; too much polish for metaphaze's Sage archetype.

**Adopt for metaphaze:** the animated TUI demo as hero asset (VHS is made by charm — they invented this pattern); the "per-product 1-sentence description" discipline.

**Avoid for metaphaze:** the portfolio structure; the pink accent; the newsletter signup; the merch store; the maximalist product grid.

---

## 4. oxide.computer

**Hero composition:**
- Full-width dark hero with a rendered 3D rack image (Oxide's actual product)
- Tagline: "On-premises cloud, as it should be"
- Two CTAs: "Request a quote" and "Read the RFDs"
- Below hero: horizontal band with three specs ("12x Cooling Efficiency," "2-hour setup," "$X/kWh")
- Further down: modular sections — API/CLI/Console trinity, customer quotes, RFD links, podcast teasers

**How they present installation:**
- Oxide is a hardware product — there's no `cargo install`
- The "CLI" section shows a terminal with the oxide CLI running commands
- The CLI screen appears ~60% down the page — it's proof of product depth, not the entry point

**Navigation pattern:**
- Top nav: Product, Cloud, Docs, Company, Events, Blog, Customers, RFDs
- Standard sans-serif nav, no brackets
- "Request a quote" as a button in the nav — enterprise sales move

**"What it does" explainer:**
- The hero image does half the work
- The concrete spec band below the hero does the other half
- Modular sections expand each claim with supporting evidence
- RFD links signal engineering rigor (RFDs are Oxide's internal design docs, published publicly)

**Strengths:** oxide.computer is the canonical example of "avoid hype, show concrete specs, publish your work." The RFD link pattern is brilliant — it turns internal engineering docs into public proof of seriousness. The modular storytelling lets each section be its own argument.

**Weaknesses:** the page is too long (5-6 viewports); the hardware-rack hero image is specific to their product and doesn't generalize; the enterprise sales framing ("Request a quote") is opposite to metaphaze's MIT-license positioning.

**Adopt for metaphaze:** the concrete-specs-not-hype principle; the "show your work" move — the refusals list is metaphaze's version of Oxide's RFDs; modular sections instead of one long wall of text.

**Avoid for metaphaze:** the multi-viewport length; the enterprise CTAs; the customer logo wall; the sans-serif typography.

---

## 5. tailscale.com

**Hero composition:**
- Top: Tailscale logo + nav (Product, Use cases, Customers, Pricing, Docs, Blog, Contact sales)
- Center: tabbed hero with 5 rotating use cases (each tab shows a different screenshot of the Tailscale admin UI)
- Left: tagline "Simply secure connectivity" + two CTAs ("Start connecting devices" primary, "Contact sales" secondary)
- Right: the active tab's screenshot
- Below hero: "Developer approved" section with individual developer testimonials (not company logos)
- Further down: business metrics, cross-platform badges, feature sections, pricing teaser, footer

**How they present installation:**
- Install is buried ~3 viewports down
- The "Start connecting devices" button goes to a signup page, not an install command
- Actual install commands are in the docs, not the landing
- Platform badges (Linux, macOS, Windows, iOS, Android) signal cross-platform without showing commands

**Navigation pattern:**
- Standard SaaS top nav, heavy on CTAs
- No brackets, sans-serif
- "Contact sales" as a persistent CTA — enterprise move

**"What it does" explainer:**
- The tabbed hero does the work — each tab shows a use case
- GUI screenshots (not terminal recordings) dominate
- Business metrics ("20,000 businesses") substitute for concrete technical specs

**Strengths:** tailscale is the canonical "Rust CLI tool that grew into a SaaS company" example; the tabbed hero handles multi-audience messaging well; the "Developer approved" section is a softer version of the logo wall.

**Weaknesses:** everything about this site is wrong for metaphaze; it's a SaaS landing page, not a tool landing page; the tabbed hero breaks the single-voice principle; the GUI screenshots hide the fact that Tailscale is also a CLI tool.

**Adopt for metaphaze:** nothing, directly. But the discipline of "one tab per use case" could translate to metaphaze's "one section per page element" — modular, scannable.

**Avoid for metaphaze:** the tabbed hero; the GUI screenshots; the "Developer approved" testimonials; the enterprise CTAs; the business metrics.

---

## Pattern Comparison Matrix

| Feature | htmx | suckless | charm.sh | oxide | tailscale | metaphaze (target) |
|---|---|---|---|---|---|---|
| Hero: install command | yes | no | no | no | no | **yes** |
| Hero: code/terminal demo | yes (HTML) | no | yes (TUI) | no | no (GUI) | **yes (VHS)** |
| Hero: brand illustration | no | no | yes (heart) | yes (rack) | yes (diagram) | **no** |
| Nav: bracketed | no | no | no | no | no | **yes** |
| Nav: monospace | yes | no | no | no | no | **yes** |
| Typography: 100% monospace | yes | no | no | no | no | **yes** |
| Page length (viewports) | 3-4 | 2 | 6-8 | 5-6 | 8-10 | **3-4** |
| Dark background | no | no | yes | yes | partial | **yes** |
| Social proof on landing | no | no | no | yes (logos) | yes (testimonials) | **no** |
| License on landing | yes (BSD) | yes | yes | N/A | no | **yes (MIT)** |
| Newsletter signup | no | partial (ML) | yes | no | no | **no** |
| Marketing illustrations | no | no | yes (mascot) | yes (rack) | yes (diagrams) | **no** |
| Concrete specs | no | no | no | yes | partial | **yes** |
| Terminal recording | no | no | yes (TUI) | no | no | **yes (VHS)** |

---

## What metaphaze Will Do Differently

1. **Monospace everywhere, not just in code blocks.** None of the reference sites commit to monospace for headlines AND body AND nav. Berkeley Graphics is the only site that commits this hard, and that's the visual reference point.
2. **Install command in the first viewport.** Only suckless does this, and suckless doesn't have an install command on the landing at all — they link to project pages. metaphaze will put `cargo install metaphaze` visible without scrolling, which none of the reference sites do.
3. **VHS recording as the primary demo, not a static screenshot.** charm.sh uses TUI demos; nobody else does. metaphaze should adopt this because it IS the tool running, not a mockup.
4. **Refusals list as positioning asset.** No reference site does this — oxide's RFDs are the closest analog. A public "things we refused to build" list is unusual enough to be memorable and precise enough to self-select the right audience.
5. **Single-page, one scroll.** htmx is 3-4 viewports; metaphaze targets 3-4 as well, but with harder section boundaries. No infinite scroll. No "see more" buttons.
6. **Bracketed nav as brand signature.** Berkeley Graphics is the closest precedent; metaphaze pushes this further with `[/docs]` and `[ copy ]` as consistent patterns, not occasional flourishes.
