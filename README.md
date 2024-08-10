# Calculator

## objective
I want to create a simple calculator

![Rust](img/Rust.png)

## Step by step
1. Create a repository on GitHub.
2. Create a folder for the project.
   ```bash
   mkdir calculator
   ```
4. Create a new Rust project.
   ```bash
   Cargo new app```
6. Link Git and GitHub.
   ```bash
   git init
   git remote add origin
   git add .
   git commit -m "Initial commit"
   git push -u origin main ```
7. Create a new branch.
   ```bash
   git checkout -b create_dockerfile
   git push origin create_dockerfile```
8. Create a dockerfile to run the project.
   ```docker
   FROM rust:latest
   WORKDIR /app
   COPY . .
   RUN cargo build --release && cargo run
   CMD ["./target/release/app"]```
9. Run dockerfile.
   ```bash
   docker build -t app .
   docker run --rm -it app```
    