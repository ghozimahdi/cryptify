# Menggunakan image resmi Rust sebagai tahap builder
FROM rust:latest AS builder

# Tambahkan musl-tools untuk mendukung binary statis
RUN apt-get update && apt-get install -y musl-tools

# Tambahkan target musl untuk Rust
RUN rustup target add x86_64-unknown-linux-musl

# Buat direktori kerja di dalam container
WORKDIR /app/cryptify

# Salin semua file proyek ke dalam container
COPY . .

# Build binary untuk target Linux (x86_64 musl)
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Gunakan stage baru untuk runtime
FROM alpine:latest AS runtime

# Salin binary dari tahap builder
WORKDIR /app
COPY --from=builder /app/cryptify/target/x86_64-unknown-linux-musl/release/cryptify .

# Tambahkan izin eksekusi untuk binary (opsional)
RUN chmod +x cryptify

# Set binary sebagai default command
CMD ["./cryptify"]