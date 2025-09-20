# Clause AI — My Submission for the Google Gen AI Exchange Hackathon 📜🤖

Hey there! I'm Devanshu, and this is Clause AI. I built this project for the Google Gen AI Exchange Hackathon to tackle a problem I think we've all faced: staring at a legal document and feeling compleatly overwhelmed. This tool is my attempt at using the power of generative AI to make that experience a little less stressful for everyone.

### ✨ A Quick Note on the Code ✨

In a world where you can ask an AI to write code for you, I wanted to make something with my own two hands. The entire Rust backend for Clause AI is 100% human-written code—by ME. I believe there's a certain craft to programming, and building this project was my way of diving deep into Rust's powerful features. I wanted to make sure every line was deliberate, and the whole thing was built on a solid, secure foundation that I understood completely.

## 🚀 The Problem

We've all been there, right? You're about to sign a lease for a new apartment or click "I agree" on a new app, and you're faced with a wall of text that makes no sense. It's intimidating, and it often feels like you have no choice but to sign and hope for the best. This isnt fair, and it's a problem I wanted to try and solve.

## ✨ My Solution

I built Clause AI to be a friendly translator for that confusing legal language. You can paste any legal text into the app, and it uses Google's powerful AI to give you a simple, easy-to-understand breakdown. The goal isn't to replace lawyers, but to give regular people a first-pass tool to feel more confident and informed about what they're signing. Its a REALLY useful tool.

## 🎯 Here's What It Can Do

Color-Coded Flags: It gives you an instant vibe-check on the document with a RED (Heads up, this is restrictive!), BLUE (Looks fair and balanced!), or GREEN (Pretty standard stuff!) flag.

Smart Context: The app is designed to get the full story. It will ask if you're the "tenant" or the "landlord" before giving an analysis, so teh advice is tailored to you.

Simple Summaries: It boils down the entire document into a few simple bullet points.

Key Info Extraction: It pulls out the important stuff like names, dates, and dollar amounts so you can see them at a glance.

Jargon Buster: Finds those fancy legal words and tells you what they actually mean in plain English.

## 🛠️ My Tech Stack & How It Works

I wanted to use this hackathon to learn and build with powerful tools. Here’s a peek under the hood:

Backend Language: Rust
I went with Rust because I love its focus on performance and safety. Building the server with it was a great challenge and ensures the app is fast and reliable.

Web Server: actix-web Crate
To get the server up and running, I used the popular actix-web crate. It's the backbone of the project, listening for requests from the browser and sending back the AI's analysis.

Talking to the AI: google-generative-ai-rs Crate
This crate was the key to connecting my app to the Google Gemini Pro API. It makes it really straightforward to send my detailed prompt and the user's text to the AI.

Keeping Secerts Safe: dotenvy Crate
To make sure my Google AI API key wasn't just sitting in the code, I used the dotenvy crate. It loads the key from a local .env file that I've kept out of version control.

Frontend: Simple HTML, CSS, & JavaScript
I kept the frontend simple to focus on the core AI features. A basic webpage sends the user's text to my Rust backend and displays the results.

### 🛡️ A Quick Note on Security

A big part of this project was making sure it was secure. Rust helps a lot with its memory safety features, which is awesome. But the most important security decision was the architecture: the Google Gemini API key stays on my Rust server and is never, ever sent to the user's browser. This means no one can steal it from the frontend code.

## 🎬 Demo Video

Check out the 3-minute video I made to see Clause AI in action!

(I will add Later)

## 🔧 Want to Run It Yourself?

If you want to get this running on your own machine, here’s how.

### You'll Need:

Rust and Cargo

A web browser

A Google AI API key

### Backend (The Rust Server)

Clone the repo:

Set up your secrets:
Create a file named .env in the main folder and add your API key and Project ID:

Run the server:

It should now be running at http://127.0.0.1:8080.

### Frontend (The Webpage)

Just open the index.html file in your browser.

It will automatically connect to your local server. Have fun!

## 🏆 My Submission for the Google Gen AI Exchange Hackathon

Project Name: Clause AI

Creator: Devanshu Sharma ([https://github.com/SharmaDevanshu089/])

Focus Area: Social Good

My Goal for This Project: I believe that technology should empower people. My goal with Clause AI was to use the incredible power of Google's generative AI to tackle a real-world problem and promote a bit more fairness and digital equity. I wanted to build something practical that could genuinely help someone feel less anxious about a major life decision.

## 📜 License

This project is licensed under the MIT License. Feel free to check out the code!
