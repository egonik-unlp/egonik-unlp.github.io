# egonik-unlp.github.io

Personal profile page of Eduardo Gonik, in English (`/`) and Spanish (`/es/`). It is a static site
built with [Zola](https://www.getzola.org/) 0.22.1 and deployed to GitHub Pages by
`.github/workflows/gh-pages-deploy.yml` on every push to `main`.

## Editing the content

| What | Where |
|---|---|
| Profile paragraph | body of `content/_index.md` (English) and `content/_index.es.md` (Spanish) |
| Work, projects, research, lab software, links | `[extra]` front matter of the same two files |
| Publications (shared by both languages) | `data/publications.toml` — Google Scholar is the source of truth |
| Fixed labels ("Work", "CV (PDF)", …) | `[translations]` and `[languages.es.translations]` in `config.toml` |
| Layout | `templates/index.html`, `templates/macros.html` |
| Styles | `static/style.css` |
| CV | `static/cv.pdf`, served at `/cv.pdf` |

Both language files have the same structure; change them together.

### Images

Images live in `static/img/` and are referenced from the front matter (`photo`, `image`,
`image_alt`). Zola resizes them at build time. A referenced image that does not exist is simply not
shown, so an entry can declare its image before the file is added. Currently missing:
`img/photo.jpg`, `img/convert-invert.png`.

## Running it

```sh
zola serve   # http://127.0.0.1:1111 with live reload
zola build   # writes the site to public/
```

Use Zola 0.22.x: 0.23 switched to Tera 2 and the templates use Tera 1 syntax.

## Other content served from `static/`

- `static/lensing-presentation/` — the Lensing slide deck, at `/lensing-presentation/`.
- `static/snappec-verify/` — an unlisted download page (`noindex`), its `install.sh` and release
  tarballs. Publish a new version with:

  ```sh
  scripts/publish-snappec-verify.sh <snappec-testing-harness checkout>
  ```

  The script writes `versions/<version>/` and replaces `latest/` under `static/snappec-verify/`.
