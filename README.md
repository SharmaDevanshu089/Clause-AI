# Clause AI — Demystifying Legal Jargon with Gen AI 📜🤖

### My Submission for the Google Gen AI Exchange Hackathon


***
> ![Status](https://img.shields.io/badge/status-paused-orange) **Notice:** This project is currently paused due to pending updates required by the Gemini API Rust crate. Development will resume once the library stabilizes.
## Introduction

Hey there! I'm **Devanshu**, a developer passionate about leveraging technology for social good.
This is **Clause AI**, an intelligent legal co-pilot engineered to tackle a universal problem:
**the intimidating complexity of legal documents**.

Clause AI is my effort to harness the power of **generative AI** to empower everyday users by bringing clarity to contracts, agreements, and the dreaded wall of fine print.

***

## ✨ A Commitment to Craftsmanship ✨

In an era where AI-generated code is increasingly common, I chose a different path.

- The **entire backend of Clause AI** is written in **Rust**, completely handcrafted by me.
- This decision ensures a solid, secure foundation without reliance on "black-box" AI-generated code for a mission-critical task.
- For me, programming is an art form — this project was about precision, robustness, and transparency.

***

## 🚀 The Problem

We all know the feeling:

- Signing a **lease contract** without fully grasping the legal obligations.
- Struggling through a dense, **terms of service** agreement.
- Feeling pressured to sign **freelance contracts** full of jargon.

These documents are often designed to overwhelm and intimidate, creating an **unfair information asymmetry**.
This problem demands a technological solution.

***

## ✨ The Solution: An Intelligent Legal Co-Pilot

Clause AI serves as a **first-pass tool** to help users cut through legal complexity.

- You paste any legal text.
- A **custom-configured Google Gemini model** processes it.
- You get a **clear, contextual, and actionable summary** — in plain English.

The goal isn’t to replace professional lawyers but to **empower** individuals to feel informed, confident, and secure.

***

## 🎯 Core Features

- **At-a-Glance Clause Flagging**
    - Documents are flagged with:
        - 🔴 **Red** = Highly Restrictive / Warning
        - 🔵 **Blue** = Fair \& Balanced
        - 🟢 **Green** = Standard / Benign
- **Context-Aware Analysis**
    - Tailored insights based on your role (e.g., *Tenant* vs *Landlord*).
- **Executive Summaries**
    - The entire document distilled into **concise, human-readable bullet points**.
- **Key Entity Extraction**
    - Automatically pulls out **names, dates, monetary values, and locations**.
- **Jargon Buster**
    - Translates intimidating legal terms into **plain English**.

***

## 🛠️ Architecture \& Tech Stack

- **Backend Language:** Rust — chosen for performance, memory safety, and fearless concurrency.
- **Web Framework:** Actix-web — a high-performance, pragmatic Rust web framework.
- **AI Integration:** [`google-generative-ai-rs`](https://crates.io/) — Rust crate bridging the app with Google Gemini Pro API.
- **Secret Management:** [`dotenvy`](https://crates.io/crates/dotenvy) — securely loads API keys from `.env`.
- **Frontend:** Simple **vanilla HTML, CSS, and JavaScript** — lightweight and focused on usability.

***

## 🛡️ Security-First Architecture

Security was non-negotiable in Clause AI’s design.

- **Rust memory safety** eliminates many common vulnerabilities.
- Credentials (**API keys and Project IDs**) remain exclusively on the server.
- No secrets are exposed to the browser, mitigating the risk of client-side key theft.

***

## 🔧 Run It Yourself

### Prerequisites

- Rust + Cargo installed
- A Google AI **API Key** and **Project ID**
- A modern web browser


### 1. Backend (Rust Server)

```bash
# Clone the repository
git clone https://github.com/SharmaDevanshu089/Clause-AI.git
cd Clause-AI

# Create your .env file
echo 'GEMINI_API_KEY="YOUR_GOOGLE_AI_API_KEY"' > .env
echo 'PROJECT_ID="YOUR_GOOGLE_PROJECT_ID"' >> .env

# Run the server
cargo run --release
```

The server runs on:
[http://127.0.0.1:8000](http://127.0.0.1:8000)

***

### 2. Frontend (Webpage)

- Open `index.html` in your web browser.
- It will automatically connect to your running local server.

***

## 🚧 Current Status \& Future Roadmap

- ✅ **Core Functionality:** Fully operational text analysis engine.
- ⚠️ **Current API:** Uses a simple text-based exchange. Planned migration to structured **JSON API** once the Rust Gemini crate fully supports it.

**Planned Roadmap:**

- Migrate to **JSON-native API** for seamless data flow.
- Build a **video demo** showcasing Clause AI capabilities.
- Expand context-awareness beyond tenant/landlord to multiple legal roles.

***

## 🏆 Hackathon Submission Details

- **Project Name:** Clause AI
- **Creator:** Devanshu Sharma ([GitHub Profile](https://github.com/SharmaDevanshu089/))
- **Focus Area:** Social Good
- **Goal:** To use **technology as a democratizing force** by addressing legal information asymmetry and promoting fairness, equity, and user confidence.

***

## 📜 License

This project is licensed under the **MIT License**.
You’re free to explore, use, and build on it.

***