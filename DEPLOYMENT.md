fly deploy# Deployment Guide for AurumTabula on Fly.io

## Prerequisites

- Fly.io CLI installed (`flyctl` or `fly`)
- You must be logged in: `fly auth login`

## Setup Steps

### 1. Set up your database secret on Fly.io

Your application needs a `DATABASE_URL` environment variable. Set it as a secret:

```bash
fly secrets set DATABASE_URL="postgresql://user:password@host:port/database"
```

Replace with your actual PostgreSQL connection string. If you don't have a database yet, you can create one on Fly.io:

```bash
fly postgres create
```

Then attach it to your app:

```bash
fly postgres attach your-postgres-app-name
```

### 2. Deploy the application

```bash
fly deploy
```

This will:

- Build your Rust application in a Docker container
- Deploy it to Fly.io
- Make it available at https://aurumtabula.fly.dev/

### 3. Check deployment status

```bash
fly status
```

### 4. View logs (if there are issues)

```bash
fly logs
```

## Troubleshooting

### If the site shows nothing:

1. Check the logs: `fly logs`
2. Verify the DATABASE_URL is set: `fly secrets list`
3. Make sure your database is accessible from Fly.io
4. Check that the app is running: `fly status`

### If you get connection errors:

- The app binds to `0.0.0.0` on port 8080 (configured in fly.toml)
- Make sure your DATABASE_URL is correct and the database accepts connections

### Common commands:

- `fly deploy` - Deploy/redeploy your app
- `fly logs` - View application logs
- `fly ssh console` - SSH into your app
- `fly secrets list` - List all secrets (values hidden)
- `fly status` - Check app status
- `fly scale count 1` - Scale to 1 instance if auto-scaled to 0

## What was fixed:

1. ✅ Created `Dockerfile` for building the Rust app
2. ✅ Created `fly.toml` configuration file
3. ✅ Created `.flyignore` and `.dockerignore` files
4. ✅ Fixed main.rs to have proper homepage function
5. ✅ App already configured to bind to `0.0.0.0:PORT` for Fly.io

Your application should now work when you redeploy!

