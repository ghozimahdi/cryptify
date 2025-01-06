# Menggunakan image resmi Rust dengan musl
FROM rust:latest AS builder

# Tambahkan musl-tools untuk membuat binary statis
RUN apt-get update && apt-get install -y musl-tools

# Tambahkan target musl untuk Rust
RUN rustup target add x86_64-unknown-linux-musl

# Buat direktori kerja dan salin source code ke dalamnya
WORKDIR /app/cryptify
COPY . .

# Build binary untuk target Linux (x86_64 musl)
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Gunakan stage baru untuk mengambil binary saja
FROM alpine:latest AS runtime

# Salin binary dari builder stage
WORKDIR /app
COPY --from=builder /app/cryptify/target/release/cryptify .

# Tambahkan izin eksekusi (opsional)
RUN chmod +x cryptify

# Set binary sebagai default command
CMD ["./cryptify"]