# Zapani 📺

<div align="center">

![GitHub Stars](https://img.shields.io/github/stars/FloDevv/Zapani?style=for-the-badge)
![GitHub Forks](https://img.shields.io/github/forks/FloDevv/Zapani?style=for-the-badge)
![GitHub Issues](https://img.shields.io/github/issues/FloDevv/Zapani?style=for-the-badge)
![GitHub PRs](https://img.shields.io/github/issues-pr/FloDevv/Zapani?style=for-the-badge)

</div>

## 🚧 WORK IN PROGRESS, not working yet !

## 📝 Overview

**Zapani** is a high-performance Rust application that creates an m3u8 url from your local video files. This url can be use like on VLC or any other player that supports HLS protocol. It is designed to be the more lightweight and efficient.

## ✨ Features

- 📺 **Video Streaming**

  - HLS (HTTP Live Streaming) protocol support
  - Automatic detection of GPU capabilities (NVIDIA, AMD, Intel, Apple)
  - Auto-cleanup of old segments

- 🔄 **Playlist Management**
  - Automatic video file discovery with multiple directories
  - Random playlist generation
  - Supports multiple video formats (MP4, MKV, AVI)
  - Continuous playback with loop functionality
  - Real TV channel-like schedule system with 7-day programming

## 🚀 Coming Soon

- Better random playlist generation like choose time of day to play specific shows
- More settings for the user to configure
- Add idle stream for eco-friendly (if you keep the program open but nobody is watching, no need to use resources)
