# Scientific works catalog

Curated scientific-work metadata is stored in
[`assets/data/works.json`](assets/data/works.json) and published at:

```text
https://egonik-unlp.github.io/assets/data/works.json
```

The catalog is anchored to Eduardo Gonik's Google Scholar profile:

```text
https://scholar.google.com/citations?user=0CAay5kAAAAJ
```

Google Scholar does not provide a stable public API and may rate-limit automated
requests. Stable bibliographic fields are therefore cross-checked against
Crossref, PubMed, OpenAlex, publisher pages, and SEDICI-UNLP. The Scholar profile
is the identity and discovery source, not the sole runtime dependency of the
landing page.

## Hosting and updates

The existing Trunk build copies the complete `assets/` directory into `dist/`,
and the GitHub Pages workflow deploys `dist/` after every push to `main`.

To publish an update:

1. Edit `assets/data/works.json`.
2. Update the top-level `retrievedAt` date.
3. Refresh each `citations` object or remove it if a current count cannot be
   verified. Citation counts are optional and must include `source` and `asOf`.
4. Validate and inspect the catalog:

   ```sh
   jq empty assets/data/works.json
   jq '{works: (.works | length), featured: ([.works[] | select(.featured)] | length)}' \
     assets/data/works.json
   ```

5. Commit and push to `main`, then wait for **Release to Github Pages**.
6. Verify the public response:

   ```sh
   curl --fail https://egonik-unlp.github.io/assets/data/works.json | jq .schemaVersion
   ```

## Component usage

The file is self-contained, so the landing site does not need to query Google
Scholar at request time:

```ts
type ScientificWork = {
  id: string;
  featured: boolean;
  title: string;
  description: string;
  authors: string[];
  year: number;
  type: "journal-article" | "book-chapter" | "conference-abstract";
  venue: string;
  identifiers: Record<string, string>;
  urls: Record<string, string>;
  categories: string[];
  domains: string[];
  methods: string[];
  keywords: string[];
  role: "first-author" | "co-author";
  citations?: {
    count: number;
    source: string;
    asOf: string;
  };
  display: { priority: number };
};

const response = await fetch(
  "https://egonik-unlp.github.io/assets/data/works.json",
);
const catalog = await response.json();

const featuredWorks = catalog.works
  .filter((work: ScientificWork) => work.featured)
  .sort(
    (left: ScientificWork, right: ScientificWork) =>
      left.display.priority - right.display.priority,
  );
```

Use the facets independently:

- `categories`: broad disciplines such as photochemistry or bioinformatics
- `domains`: the scientific system or application being studied
- `methods`: experimental or computational approaches
- `keywords`: detailed search and display terms
- `type`: journal article, book chapter, or conference abstract
- `role`: first-author versus co-author work
- `featured`: whether the work belongs on the main landing page
- `display.priority`: ascending curated card order

Descriptions are original landing-page summaries, not copied abstracts. Links
and identifiers remain the authoritative route to each publication.
