# Delta Spec: replace-portfolio-with-profile-page

**Capability:** profile-page
**Change:** replace-portfolio-with-profile-page

## Purpose

Present Eduardo Gonik's professional profile (engineering, research and teaching) as a plain,
bilingual static page that recruiters and technical leads can read and verify, with the CV available
for download, while keeping the existing static subsites served at their URLs.

## ADDED Requirements

### Requirement: Bilingual profile page
The site SHALL serve the profile page in English at `/` and in Spanish at `/es/`, with the same sections, entries and links in both languages, and each version MUST link to the other.

#### Scenario: English page at the root
- **GIVEN** the site is deployed
- **WHEN** a visitor opens `/`
- **THEN** the profile page is shown in English with `lang="en"`
- **AND** it contains a link to the Spanish version at `/es/`

#### Scenario: Spanish page mirrors the English one
- **GIVEN** the site is deployed
- **WHEN** a visitor opens `/es/`
- **THEN** the profile page is shown in Spanish with `lang="es"`
- **AND** it has the same sections, entries and outbound links as `/`
- **AND** it contains a link back to `/`

### Requirement: Header with identity, contact and CV
The page header SHALL show the name "Eduardo Gonik", the headline "Data engineer · La Plata, Argentina" (translated on `/es/`), links to email, GitHub, LinkedIn and Google Scholar, and a "CV (PDF)" link that MUST download the CV served at `/cv.pdf`.

#### Scenario: Recruiter downloads the CV
- **GIVEN** a visitor is on `/` or `/es/`
- **WHEN** they follow the "CV (PDF)" link in the header
- **THEN** the browser receives the PDF served at `/cv.pdf`

#### Scenario: Contact links are present
- **GIVEN** a visitor is on the page
- **WHEN** they read the header
- **THEN** they find working links to `mailto:eduardogonik@gmail.com`, the GitHub profile, the LinkedIn profile and the Google Scholar profile

### Requirement: Section order follows the CV
The page SHALL present, after the header, a profile paragraph followed by the sections Stack, Work, Projects and Research, in that order.

#### Scenario: Reading top to bottom
- **GIVEN** a visitor opens the page
- **WHEN** they scroll from top to bottom
- **THEN** they encounter the header, the profile paragraph, Stack, Work, Projects and Research in that order

### Requirement: Stack section
The page SHALL include an always-visible Stack section, with a heading like the other sections, that lists the tools in five groups (Languages, ML & AI, Data, Systems & Web, Infra & Platforms); each group MUST show the tools used regularly first as filled tags and the remaining tools after them as outlined, lighter tags, with a legend naming both kinds and with each kind exposed to assistive technology as its own labelled list, exactly as agreed in this change's design.md.

#### Scenario: Stack is visible on load
- **GIVEN** a visitor opens the page
- **WHEN** they reach the Stack section, between the profile and Work
- **THEN** the five groups are shown without any interaction, each with filled tags for regular tools followed by outlined tags for tools also used
- **AND** a legend explains the two styles

#### Scenario: Screen reader distinguishes the two kinds
- **GIVEN** a screen reader user reaches a stack group
- **WHEN** they move through its lists
- **THEN** the regular tools and the tools also used are announced as two separately labelled lists

### Requirement: Content comes only from the agreed copy
All visible text SHALL come from the agreed copy recorded in this change's `context.md` ("Texto acordado") or, for the Stack section, the stack list agreed in `design.md`, with the Spanish incorporating the owner's corrections; the page MUST NOT add claims, metrics, taglines or descriptions that are not in that copy.

#### Scenario: No invented text
- **GIVEN** the agreed copy in `context.md`
- **WHEN** the rendered page is compared against it
- **THEN** every sentence on the page is present in the agreed copy for that language
- **AND** no section, tagline or call to action exists that the copy does not contain

### Requirement: Work entries
The Work section SHALL list Snappler (Data engineer, 2025–2026), Goodvibes Movement (Software and data engineer, 2021–2026) and Universidad Nacional de La Plata (teaching, 2016–present) with their agreed descriptions, and MUST NOT name the client of the Snappler verification work.

#### Scenario: Client stays anonymous
- **GIVEN** the Snappler entry describes the layered verification of a client's refactor
- **WHEN** a visitor reads it
- **THEN** the client is referred to only as "a client"
- **AND** no link to that client's code or name appears anywhere on the page

### Requirement: Selected projects with links
The Projects section SHALL show exactly Lensing (with Infinite Playlist nested under it as built with Lensing), lvv and convert-invert, each with its agreed description and its links (Lensing: repository and `/lensing-presentation/`; Infinite Playlist: live demo and repository; lvv and convert-invert: repository), followed by a "More on GitHub" link to the GitHub profile.

#### Scenario: Visitor opens a live demo
- **GIVEN** a visitor reads the Infinite Playlist entry
- **WHEN** they follow its "live demo" link
- **THEN** they reach the running Infinite Playlist demo

#### Scenario: Only the selected projects appear
- **GIVEN** the page is rendered
- **WHEN** a visitor reads the Projects section
- **THEN** it contains Lensing, Infinite Playlist, lvv and convert-invert and no other project entries
- **AND** it ends with a link to the GitHub profile

### Requirement: Research section with Scholar as source
The Research section SHALL show the agreed PhD paragraph (including that the thesis is being written), the 2023 undergraduate thesis co-direction without naming the student, the eight agreed publications with year, title and venue (marking first authorship and submitted status where agreed), a link to Google Scholar as the full list, and the five lab software entries linked to their repositories.

#### Scenario: Publications point to Scholar
- **GIVEN** a visitor reads the publications list
- **WHEN** they want the complete record
- **THEN** a link to the Google Scholar profile is shown next to the list

#### Scenario: Lab software is linked
- **GIVEN** a visitor reads "Software from the lab"
- **WHEN** they follow any of the five entries
- **THEN** they reach that tool's public repository

### Requirement: Images are optional
The page SHALL show the photo and project thumbnails when their image files exist, and when an image is missing the page MUST render that entry without an empty frame, placeholder text or broken image.

#### Scenario: Thumbnail not yet available
- **GIVEN** no image file exists for the lvv thumbnail
- **WHEN** the page is rendered
- **THEN** the lvv entry shows its text and links with no empty box or broken image

#### Scenario: Thumbnail available
- **GIVEN** an image file exists for Infinite Playlist
- **WHEN** the page is rendered
- **THEN** the Infinite Playlist entry shows that image with descriptive alt text

### Requirement: Plain page without scripts or motion
The page SHALL be fully readable and navigable with JavaScript disabled, MUST NOT include animations or decorative canvases, and MUST NOT require loading font files beyond the system fonts.

#### Scenario: JavaScript disabled
- **GIVEN** a browser with JavaScript disabled
- **WHEN** the visitor opens `/` or `/es/`
- **THEN** all sections, text and links are visible and usable

### Requirement: Readable at phone width
The page SHALL lay out without horizontal scrolling at a viewport width of 360 CSS pixels, and its interactive links in the header MUST have touch targets of at least 44 CSS pixels in height.

#### Scenario: Phone visitor
- **GIVEN** a viewport 360 pixels wide
- **WHEN** the page is rendered
- **THEN** no content overflows horizontally and the header's CV link is at least 44 pixels tall

### Requirement: Existing static subsites keep their URLs
The site SHALL keep serving `/lensing-presentation/` and `/snappec-verify/` (including `install.sh`, `latest/` and `versions/0.1.0/`) at the same URLs with unchanged content, and the snappec-verify publish script MUST write new releases where the built site serves them.

#### Scenario: Install script still works
- **GIVEN** the new site is deployed
- **WHEN** a user runs `curl -fsSL https://egonik-unlp.github.io/snappec-verify/install.sh | sh`
- **THEN** the script downloads the release from `/snappec-verify/latest/` as before

#### Scenario: Deck still loads
- **GIVEN** the new site is deployed
- **WHEN** a visitor opens `/lensing-presentation/`
- **THEN** the deck loads with all its styles, scripts and assets

#### Scenario: Publishing a new snappec-verify release
- **GIVEN** the publish script is run with a snappec-verify checkout
- **WHEN** the site is built and deployed afterwards
- **THEN** the new release is served under `/snappec-verify/versions/<version>/` and `/snappec-verify/latest/`

### Requirement: Indexing rules
The site SHALL allow search engines to index `/lensing-presentation/`, and the snappec-verify page MUST remain marked `noindex` and MUST NOT be linked from the profile page.

#### Scenario: Deck is indexable
- **GIVEN** the deployed `robots.txt`
- **WHEN** a crawler reads it
- **THEN** no rule disallows `/lensing-presentation/`

### Requirement: Static deployment to GitHub Pages
The site SHALL be produced by a static Zola build and deployed to GitHub Pages by the repository workflow, without running a server process, and the legacy endpoints under `/assets/data/` MUST no longer be served.

#### Scenario: Push to main deploys the site
- **GIVEN** a commit is pushed to `main`
- **WHEN** the workflow runs
- **THEN** it builds the site with Zola and deploys the output to GitHub Pages

#### Scenario: Legacy JSON is gone
- **GIVEN** the new site is deployed
- **WHEN** a client requests `/assets/data/projects.json`
- **THEN** the response is a 404
