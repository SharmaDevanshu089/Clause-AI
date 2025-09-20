lause AI 📜🤖
Clause AI is a state-of-the-art legal assistant, built for the Google Gen AI Exchange Hackathon, that leverages generative AI to translate complex legal documents into simple, understandable English, democratizing legal literacy for everyone.

### ✨ A Note on Human-Crafted Code ✨
In an era of AI-driven development, we believe in the irreplaceable value of human craftsmanship, especially when building secure and reliable systems. The core backend of Clause AI, written in Rust, is 100% human-authored code. This project is a testament to a developer's deep understanding of Rust's powerful concurrency, memory safety, and performance features. By writing the code by hand, we ensure that every line is deliberate, every dependency is understood, and the architecture is thoughtfully designed for robustness and security—a level of quality and assurance that only human expertise can provide.

## 🚀 The Problem
Legal documents—from apartment leases to terms of service—are a part of everyday life. However, they are often filled with dense jargon and complex clauses that are intimidating and nearly impossible for the average person to understand. This information gap creates a power imbalance, leaving people vulnerable to signing unfair or predatory agreements simply because they can't comprehend the text.

## ✨ Our Solution
Clause AI bridges this gap by acting as your personal legal translator. By pasting any legal text into our simple interface, users get an instant, AI-powered breakdown of the document. Our tool empowers users by providing clarity and confidence, transforming a stressful process into a simple check for understanding. It's not just a summarizer; it's a context-aware assistant that ensures it has the full story before providing an analysis.

## 🎯 Key Features
Color-Coded Flag System: Instantly get a high-level sense of a document's fairness with our RED (Restrictive), BLUE (Balanced), and GREEN (Standard) flags.

Two-Step Context System: Clause AI ensures it always has the full picture by first asking for your role in the agreement (e.g., tenant or landlord) before providing a detailed analysis.

Plain English Summary: Translates walls of legalese into a concise, bullet-point summary tailored to your perspective.

Key Information Extraction: Automatically pulls out and displays the most critical details, such as names, dates, and monetary amounts.

Jargon Buster: Identifies and defines confusing legal terms in simple, one-sentence explanations.

## 🛠️ Tech Stack & How It Works
This project was built with a focus on performance and learning. Here’s a breakdown of the core components and how they fit together.

Backend Language: Rust
We chose Rust for its incredible performance and safety guarantees. As a systems language, it's a great choice for building a reliable and efficient server, which was a key learning goal for this hackathon.

Web Server: actix-web Crate
We're using the popular actix-web crate to create the HTTP server. This is the core of our backend; it listens for requests from the user's browser, handles routing (like sending users to the main page or the /analyze API endpoint), and sends back responses.

AI Communication: google-generative-ai-rs Crate
This community-built crate is the bridge to the AI. It simplifies the process of sending our detailed system prompt and the user's legal text to the Google Gemini Pro API. It handles the complex parts of making the web request and parsing the response from Google.

API Key Management: dotenvy Crate
To keep our secret Google AI API key safe and out of the source code, we use the dotenvy crate. It loads the key from a .env file (which is ignored by Git) into the application's environment at startup.

Frontend: HTML, CSS, & JavaScript
The user interface is a simple, single web page. We kept it basic to focus on the core functionality. The JavaScript on the page uses the fetch() API to send the legal text to our Rust backend and then displays the AI's response.

### 🛡️ A Note on Security
A key part of this project's design is security. By using a Rust backend, we get memory safety right out of the box. More importantly, this architecture ensures our Google Gemini API key lives only on the server. It's loaded from the .env file and is never sent to the user's browser, which prevents it from being stolen.

## 🎬 Demo Video
Watch our 3-minute video to see Clause AI in action and understand the impact it can have.

(Link to your YouTube/Vimeo video will go here)

## 🔧 How to Run Locally
To get Clause AI running on your local machine, follow these simple steps.

### Prerequisites
Rust and Cargo installed

A modern web browser

A Google AI API key

### Backend (Rust Server)
Clone the repository:

Set up your environment variables:
Create a file named .env in the project root and add your API key and Google Cloud Project ID:

Run the server:

The server will be running at http://127.0.0.1:8080.

### Frontend (Web Interface)
Open the index.html file in your web browser.

You can now paste text and interact with the application, which will communicate with your local Rust server.

## 🏆 Google Gen AI Exchange Hackathon Submission
Project Name: Clause AI

Creator: Devanshu Sharma ([Your GitHub Profile Link Here])

Focus Area: Social Good

Project Alignment: Clause AI represents a paradigm shift in user empowerment by leveraging state-of-the-art generative AI to promote digital equity. Our solution synergizes technology and social impact, providing a tool that makes complex legal information accessible to all. With a scalable and robust backend built in Rust and powered by Google's Gemini API, Clause AI is a practical, high-impact application designed to solve a pressing real-world problem.

## 📜 License
This project is licensed under the MIT License. See the  file for details.
