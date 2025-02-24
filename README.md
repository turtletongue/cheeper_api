# How to start the App

Build executable:

```bash
cargo build --release
```

Setup environment variables:

```dotenv
IP_ADDRESS=127.0.0.1
PORT=3030
SESSION_KEY=<64_byte_secret>
MONGO_CONNECTION=mongodb://<username>:<password>@localhost:27017
```

Run:

```bash
cargo run --release
```

You also can run executable directly.