# Crab Url 🦀

Simple temporal url shortener built with Rust.

## Usage

### Development

#### Configure environment variables

```env
REDIS_URL=<redis_url>
```

#### Frontend

```bash
cd frontend
pnpm install
pnpm run dev
```

#### Backend

See all log configurations

```bash
cargo run -- -h
```

Run the server

```bash
cargo run
```

### Production

The app is ready to be deployed to [fly.io](https://fly.io/) with a managed Redis instance.

Follow [fly.io's documentation](https://fly.io/docs/apps/deploy/) to deploy the app.
