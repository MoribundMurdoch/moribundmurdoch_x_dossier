# Murdo, Remember: Don’t Share Your API Keys With LLMs

This repository is public.

Do not commit:

- `.env`
- API keys
- bearer tokens
- OAuth secrets
- downloaded private archives
- raw account exports
- anything copied from a dashboard that says “secret”

Public viewers should never need an X API token.

Maintainer-only imports use local secrets, ignored by Git.
The public app reads generated datasets, not live private credentials.

If a machine asks for your token, bonk the machine.
