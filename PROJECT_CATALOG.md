# Project catalog

The curated project metadata is stored in [`assets/data/projects.json`](assets/data/projects.json).
GitHub Pages publishes it at:

```text
https://egonik-unlp.github.io/assets/data/projects.json
```

## How hosting works

The site is built by Trunk in `.github/workflows/gh-pages-deploy.yml`. The root
`index.html` contains a Trunk `copy-dir` directive for `assets`, so the complete
`assets/` directory is copied into `dist/`. The workflow deploys `dist/` as the
GitHub Pages artifact whenever a commit is pushed to `main`.

To publish an update:

1. Edit `assets/data/projects.json`.
2. Validate it locally:

   ```sh
   jq empty assets/data/projects.json
   ```

3. Commit and push the change to `main`.
4. Wait for the **Release to Github Pages** workflow to finish.
5. Verify the deployed file:

   ```sh
   curl --fail https://egonik-unlp.github.io/assets/data/projects.json
   ```

If GitHub Pages is not enabled yet, open the repository's **Settings → Pages**
and select **GitHub Actions** as the source. The existing workflow performs the
build and deployment.

## Merge behavior

The catalog is intentionally a sparse set of curated overrides. Fetch the live
repository list from the GitHub API, look up each repository name in
`repositories`, and let curated values override GitHub values. Repositories not
listed in the catalog still appear using their GitHub metadata.

```ts
type GitHubRepository = {
  name: string;
  description: string | null;
  language: string | null;
};

type CatalogEntry = {
  title?: string;
  description?: string;
  languages?: string[];
  categories?: string[];
  projectTypes?: string[];
  domains?: string[];
  featured?: boolean;
};

const entry: CatalogEntry | undefined = catalog.repositories[repository.name];

const project = {
  ...repository,
  ...entry,
  title: entry?.title ?? repository.name,
  description:
    entry?.description ??
    repository.description ??
    "No project description is available.",
  languages: entry?.languages ?? (repository.language ? [repository.language] : []),
  categories: entry?.categories ?? [],
  projectTypes: entry?.projectTypes ?? [],
  domains: entry?.domains ?? [],
  featured: entry?.featured ?? false,
};
```

Use the structured arrays as independent filters rather than flattening them
into one tag collection:

- `languages`: implementation languages and important runtimes
- `categories`: broad technical areas
- `projectTypes`: library, CLI, web app, notebook, simulation, and similar forms
- `domains`: subject matter such as spectroscopy, music, or materials science
- `status`: `active`, `paused`, or `completed`
- `maturity`: `experimental`, `prototype`, or `usable`
- `role`: the relationship to the work, such as `creator`, `port-author`, or
  `reproduction`
- `display.priority`: ascending manual order for cards

New optional fields can be added without changing consumers. Components should
ignore fields they do not understand and should not assume every entry has
`highlights`, `links`, or display-specific metadata.
