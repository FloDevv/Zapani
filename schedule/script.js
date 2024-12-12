fetch("../video.json")
  .then((response) => {
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    return response.json();
  })
  .then((data) => {
    const schedule = document.getElementById("schedule");
    const currently = document.getElementById("currently");
    const days = [
      "Monday",
      "Tuesday",
      "Wednesday",
      "Thursday",
      "Friday",
      "Saturday",
      "Sunday",
    ];
    const now = new Date();
    let use24HourFormat = false;

    const currentDayIndex = now.getDay() === 0 ? 6 : now.getDay() - 1;
    const remainingDays = days.slice(currentDayIndex);

    // Filter videos for the current week
    const currentWeekVideos = data.filter((video) => {
      const start = new Date(video.start_time);
      const videoWeek = getWeekNumber(start);
      const currentWeek = getWeekNumber(now);
      return videoWeek === currentWeek;
    });

    // Find the currently playing video
    let currentVideo = null;
    for (const video of currentWeekVideos) {
      const start = new Date(video.start_time);
      const end = new Date(start.getTime() + video.duration * 1000);
      if (start <= now && now <= end) {
        currentVideo = video;
        break;
      }
    }

    if (currentVideo) {
      const start = new Date(currentVideo.start_time);
      const end = new Date(start.getTime() + currentVideo.duration * 1000);
      const progress = calculateProgress(now, start, end);
      const progressBar = createProgressBar(progress);
      currently.setAttribute("data-end-time", end.toISOString());
      currently.innerHTML = `Currently Playing: ${currentVideo.title} ${progressBar}`;

      // Mise à jour toutes les secondes
      setInterval(() => {
        const newNow = new Date();
        const newProgress = calculateProgress(newNow, start, end);
        const newProgressBar = createProgressBar(newProgress);
        currently.innerHTML = `Currently Playing: ${currentVideo.title} ${newProgressBar}`;
      }, 1000);
    }

    // Filter future videos from now
    const filteredVideos = currentWeekVideos.filter((video) => {
      const start = new Date(video.start_time);
      return start >= now;
    });

    for (const day of remainingDays) {
      const col = document.createElement("div");
      col.className = "day-column";
      const header = document.createElement("h3");
      header.textContent = day;
      col.appendChild(header);

      const videosForDay = filteredVideos.filter((video) => {
        const videoDay = new Date(video.start_time).toLocaleDateString(
          "en-US",
          { weekday: "long" }
        );
        return videoDay === day;
      });

      for (const video of videosForDay) {
        const videoDiv = document.createElement("div");
        videoDiv.className = "video-item";

        const titleDiv = document.createElement("div");
        titleDiv.className = "video-title";
        titleDiv.textContent = video.title;

        const timeDiv = document.createElement("div");
        timeDiv.className = "video-time";

        const startTime = new Date(video.start_time);
        const endTime = new Date(startTime.getTime() + video.duration * 1000);

        // Store original times as data attributes
        timeDiv.setAttribute("data-start", video.start_time);
        timeDiv.setAttribute("data-end", endTime.toISOString());

        timeDiv.textContent = `${formatTime(startTime)} - ${formatTime(
          endTime
        )}`;

        videoDiv.appendChild(titleDiv);
        videoDiv.appendChild(timeDiv);

        if (currentVideo && video.path === currentVideo.path) {
          videoDiv.classList.add("current");
        }
        col.appendChild(videoDiv);
      }

      schedule.appendChild(col);
    }

    function formatTime(date) {
      return date.toLocaleTimeString("en-US", {
        hour: "2-digit",
        minute: "2-digit",
        hour12: !use24HourFormat,
      });
    }

    function updateAllTimes() {
      const timeElements = document.querySelectorAll(".video-time");
      for (const el of timeElements) {
        const startTime = new Date(el.getAttribute("data-start"));
        const endTime = new Date(el.getAttribute("data-end"));
        el.textContent = `${formatTime(startTime)} - ${formatTime(endTime)}`;
      }
    }

    // Initialize time format toggle
    const timeFormatToggle = document.getElementById("timeFormatToggle");
    const formatLabel = document.getElementById("formatLabel");

    // Set initial state
    timeFormatToggle.checked = false;
    formatLabel.textContent = "12h";

    timeFormatToggle.addEventListener("change", (e) => {
      use24HourFormat = e.target.checked;
      formatLabel.textContent = use24HourFormat ? "24h" : "12h";
      updateAllTimes();
    });

    function calculateProgress(current, start, end) {
      const total = end.getTime() - start.getTime();
      const elapsed = current.getTime() - start.getTime();
      return Math.min(Math.max(elapsed / total, 0), 1);
    }

    function formatRemainingTime(current, end) {
      const remaining = Math.max(0, end.getTime() - current.getTime()) / 1000; // en secondes
      const hours = Math.floor(remaining / 3600);
      const minutes = Math.floor((remaining % 3600) / 60);
      const seconds = Math.floor(remaining % 60);
      return `${hours.toString().padStart(2, "0")}:${minutes
        .toString()
        .padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;
    }

    function createProgressBar(progress, length = 20) {
      const filled = Math.round(progress * length);
      const empty = length - filled;
      const now = new Date();
      const end = new Date(currently.getAttribute("data-end-time"));
      const remainingTime = formatRemainingTime(now, end);
      return `<span class="progress-bar">[${
        filled > 0 ? "█".repeat(filled) : ""
      }${empty > 0 ? "░".repeat(empty) : ""}] ${Math.round(
        progress * 100
      )}% <span class="remaining-time">(${remainingTime})</span></span>`;
    }

    function getWeekNumber(date) {
      const tempDate = new Date(date.getTime());
      tempDate.setHours(0, 0, 0, 0);
      tempDate.setDate(tempDate.getDate() + 3 - ((tempDate.getDay() + 6) % 7));
      const week1 = new Date(tempDate.getFullYear(), 0, 4);
      return (
        1 +
        Math.round(
          ((tempDate - week1) / 86400000 - 3 + ((week1.getDay() + 6) % 7)) / 7
        )
      );
    }
  })
  .catch((error) => {
    console.error("Error loading video data:", error);
    const schedule = document.getElementById("schedule");
    schedule.innerHTML =
      '<p class="error">Error loading schedule. Please try again later.</p>';
  });
