document.addEventListener("DOMContentLoaded", () => {
  const analyzeButton = document.getElementById("analyze-button");
  const docInput = document.getElementById("doc-input");
  const resultsContainer = document.getElementById("results-container");
  const buttonText = document.querySelector(".button-text");
  const spinner = document.querySelector(".spinner");

  let currentConversation = [];

  analyzeButton.addEventListener("click", async () => {
    const inputText = docInput.value;
    if (!inputText.trim()) {
      alert("Please paste some text to analyze.");
      return;
    }
    currentConversation.push({ role: "user", parts: [{ text: inputText }] });

    setLoadingState(true);

    try {
      const response = await fetch("/analyze", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ text: inputText }),
      });

      if (!response.ok) {
        throw new Error(`Server responded with status: ${response.status}`);
      }
      const rawResponse = await response.text();
      try {
        const data = JSON.parse(rawResponse);
        displayResults(data);
        currentConversation = [];
      } catch (e) {
        displayInterimQuestion(rawResponse);
        currentConversation.push({
          role: "model",
          parts: [{ text: rawResponse }],
        });
      }
    } catch (error) {
      displayError(error);
      currentConversation = [];
    } finally {
      setLoadingState(false);
    }
  });

  function setLoadingState(isLoading) {
    if (isLoading) {
      buttonText.style.display = "none";
      spinner.style.display = "block";
      analyzeButton.disabled = true;
    } else {
      buttonText.style.display = "inline";
      spinner.style.display = "none";
      analyzeButton.disabled = false;
    }
    docInput.value = "";
  }

  function displayInterimQuestion(text) {
    resultsContainer.style.display = "block";
    resultsContainer.innerHTML = `<div class="card"><p>${text.replace(
      /\n/g,
      "<br>"
    )}</p></div>`;
    docInput.placeholder = "Provide your context here...";
  }

  function displayResults(data) {
    resultsContainer.style.display = "block";
    let html = "";

    // Flag
    const flagText = data.flag || "[GREEN 🟢]: Standard Document";
    const flagColor = flagText.includes("RED")
      ? "red"
      : flagText.includes("BLUE")
      ? "blue"
      : "green";
    html += `<div class="result-flag ${flagColor}">${flagText}</div>`;

    // Summary
    if (data.summary && data.summary.length > 0) {
      html += `<div class="card"><h3>📝 Summary</h3><ul>`;
      data.summary.forEach((point) => (html += `<li>${point}</li>`));
      html += `</ul></div>`;
    }

    // Key Info
    if (data.key_info) {
      html += `<div class="card"><h3>🔑 Key Information</h3><ul>`;
      for (const [key, value] of Object.entries(data.key_info)) {
        if (value && value.length > 0) {
          const formattedKey = key
            .replace(/_/g, " ")
            .replace(/\b\w/g, (l) => l.toUpperCase());
          html += `<li><strong>${formattedKey}:</strong> ${
            Array.isArray(value) ? value.join(", ") : value
          }</li>`;
        }
      }
      html += `</ul></div>`;
    }
    if (data.jargon_buster && data.jargon_buster.length > 0) {
      html += `<div class="card"><h3>🧐 Jargon Buster</h3><ul>`;
      data.jargon_buster.forEach((item) => {
        html += `<li><span class="jargon-term">${item.term}:</span> ${item.explanation}</li>`;
      });
      html += `</ul></div>`;
    }

    resultsContainer.innerHTML = html;
    docInput.placeholder = "e.g., Paste your entire rental agreement here...";
  }

  function displayError(error) {
    resultsContainer.style.display = "block";
    resultsContainer.innerHTML = `<div class="card"><h3 style="color:var(--flag-red-text)">❌ Analysis Error</h3><p>Sorry, an error occurred. Please try again.</p><p><small>Details: ${error.message}</small></p></div>`;
  }
});
