# Deploying Revi GitHub Pages

This guide covers deploying the Revi landing site and interactive demo to GitHub Pages.

**What gets deployed:**

| URL | Source | Content |
|-----|--------|---------|
| `https://yorha-agents.github.io/Revi/` | `site/index.html` | Landing page |
| `https://yorha-agents.github.io/Revi/demo/` | `site/demo/index.html` | Interactive demo |
| `https://yorha-agents.github.io/Revi/guide/` | `site/guide/index.html` | Styled user guide |

---

## Prerequisites

- Push access to the `YoRHa-Agents/Revi` repository
- The `docs/readme-and-pages` branch merged into `main` (PR #1)

---

## Step 1 — Enable GitHub Pages

1. Go to **https://github.com/YoRHa-Agents/Revi/settings/pages**
2. Under **Build and deployment > Source**, select **GitHub Actions**
3. Click **Save**

That's it. No branch or folder selection needed — the workflow handles everything.

> If you see "GitHub Pages is currently disabled" after saving, that's normal. It activates on first deployment.

---

## Step 2 — Trigger the first deployment

The workflow runs automatically when files in `site/` change on `main`. To trigger it immediately:

### Option A — Manual dispatch (no code change needed)

1. Go to **https://github.com/YoRHa-Agents/Revi/actions/workflows/pages.yml**
2. Click **Run workflow** → select `main` → click **Run workflow**

### Option B — Push any change to `site/`

```bash
# Any edit to site/ on main triggers deployment
git checkout main
echo "<!-- deployed $(date) -->" >> site/index.html
git add site/index.html
git commit -m "Trigger pages deployment"
git push origin main
```

---

## Step 3 — Verify deployment

1. Go to **https://github.com/YoRHa-Agents/Revi/actions** and watch the "Deploy to GitHub Pages" workflow
2. Once the job shows a green checkmark, visit:
   - **https://yorha-agents.github.io/Revi/** — landing page
   - **https://yorha-agents.github.io/Revi/demo/** — interactive demo
   - **https://yorha-agents.github.io/Revi/guide/** — styled user guide
3. The deployment URL also appears in the workflow run summary under **Environments**

---

## How the workflow works

The deployment is defined in `.github/workflows/pages.yml`:

```
push to main (site/** changed)  →  checkout repo
       or                           →  upload site/ as artifact
manual workflow_dispatch            →  deploy to GitHub Pages
```

**Triggers:**
- Automatic on push to `main` when any file under `site/` changes
- Manual via the Actions tab (workflow_dispatch)

**Permissions required:**
- `contents: read` — to checkout the repo
- `pages: write` — to deploy to GitHub Pages
- `id-token: write` — for the OIDC token used by the deploy action

**Concurrency:** Only one deployment runs at a time. New pushes queue (they don't cancel in-progress deployments).

---

## Updating the site

Edit files directly in `site/`:

```
site/
  index.html          ← Landing page
  demo/
    index.html        ← Interactive demo
  guide/
    index.html        ← Styled user guide
```

Push to `main` and the workflow deploys automatically. No build step — these are static HTML files served as-is.

```bash
# Edit, commit, push — deployed in ~30 seconds
vim site/index.html
git add site/
git commit -m "Update landing page copy"
git push origin main
```

---

## Custom domain (optional)

To serve from a custom domain instead of `yorha-agents.github.io/Revi`:

1. Go to **Settings > Pages > Custom domain**
2. Enter your domain (e.g. `revi.example.com`)
3. Click **Save**
4. Add a DNS record at your registrar:

| Type | Name | Value |
|------|------|-------|
| `CNAME` | `revi` | `yorha-agents.github.io` |

5. Wait for DNS propagation (up to 24 hours, usually minutes)
6. Check **Enforce HTTPS** once the certificate is provisioned

Then add a `CNAME` file to the site directory so deployments preserve the setting:

```bash
echo "revi.example.com" > site/CNAME
git add site/CNAME
git commit -m "Add custom domain CNAME"
git push origin main
```

---

## Troubleshooting

### "Pages is not enabled" or 404 after deploy

- Confirm **Settings > Pages > Source** is set to **GitHub Actions** (not "Deploy from a branch")
- Check the Actions tab for failed workflow runs

### Workflow not triggering on push

- The path filter only watches `site/**`. Changes to `README.md` or other files won't trigger it
- Use manual dispatch from the Actions tab to force a run

### Stale content after deployment

- GitHub Pages CDN may cache for up to 10 minutes
- Hard-refresh with `Ctrl+Shift+R` / `Cmd+Shift+R`
- Append `?v=2` to the URL to bust cache during testing

### Permission errors in workflow

- Ensure the repository has **Settings > Actions > General > Workflow permissions** set to **Read and write permissions**
- The `pages: write` and `id-token: write` permissions in the workflow file must match what the repo allows
