# 🦀 Simple Rust Web Server

Welcome to a **Simple Web Server written in Rust**, developed for learning purposes. This project is perfect for anyone who wants to understand how web servers work under the hood using Rust.

---

## 🌟 What is this Project?

This is a basic HTTP web server built from scratch using **Rust**. It’s designed to help you:

- Learn the basics of HTTP and networking
- Understand low-level server behavior
- Experiment with multi-threading in Rust
- Serve static and dynamic content

---

## ✨ Features

- ⚡ Lightweight and fast
- 🦀 Built from scratch using Rust
- 📄 Handles basic GET requests
- 🧵 Multi-threaded request handling
- 🌐 Serves static files like HTML and CSS

---

## 📁 Project Structure

```text
.
├── src/
│   └── main.rs        # The entrypoint file
├── public/            # Static files (HTML, CSS, etc.)
│   ├── index.html
│   └── style.css
└── README.md
```

---

## 🛠 Prerequisites

### ✅ Install Rust

You need to have Rust installed on your machine.

To install Rust, run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal and verify the installation:

```bash
rustc --version
```

You should see something like:

```bash
rustc X.XX.X (your version)
```

---

## 🗂 Serving Static Files

This server reads your static files (HTML, CSS, etc.) from a folder defined by the `ROOT` environment variable.

By default, you can use the `public/` directory included in this project.

### 📌 Set the `ROOT` Environment Variable

Before running the server, make sure the `ROOT` variable points to the path of your static files.

#### 🐧 Linux / macOS

```bash
export ROOT=$(pwd)/public
```

#### 🪟 Windows (PowerShell)

```powershell
$env:ROOT = "$(Get-Location)\public"
```

---

## ▶️ Running the Server

Once `ROOT` is set, run the server using Cargo:

```bash
cargo run
```

Then, open your browser and go to:  
👉 [http://localhost:8080](http://localhost:8080)

---

#### 🎯 Project Goals

- Learn the basics of HTTP and networking
- Understand low-level server behavior
- Experiment with multi-threading in Rust
- Serve static and dynamic content

#### ✨ Features

- Lightweight and fast
- Built from scratch using Rust
- Handles GET requests
- Multi-threaded request handling

---

## ❤️ Made with Rust

This project was built for fun and learning. Feel free to explore, modify, and expand it!

---

## 📄 License

This project is licensed under the [GNU License](LICENSE).
