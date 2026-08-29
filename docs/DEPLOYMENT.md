# Deployment & Infrastructure Runbook

## 1. Hosting Architecture
- **Primary Hosting**: GitHub Pages (`https://fthtrading.github.io/time/`)
- **DNS & Edge CDN**: Cloudflare (`time.unykorn.ai`)
- **Build Output**: Static assets compiled from `apps/web/`

---

## 2. GitHub Actions Deployment
The deployment workflow `.github/workflows/deploy-pages.yml` triggers on every push to `main`:
1. Checks out repository.
2. Configures GitHub Pages environment.
3. Uploads `apps/web/` directory as the deployment artifact.
4. Deploys directly to the GitHub Pages edge.

---

## 3. Local Development
To serve locally for testing:
```bash
python -m http.server 8080 --directory apps/web
```
Navigate to `http://localhost:8080/`.
