# REST Client 🚀

[![CI](https://github.com/doani/zed-restclient/actions/workflows/ci.yml/badge.svg)](https://github.com/doani/zed-restclient/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A REST Client extension for the [Zed editor](https://zed.dev/), bringing the powerful and intuitive workflow of `vscode-restclient` to the Zed ecosystem.

This extension allows you to send HTTP requests directly from your `.http` or `.rest` files, supporting a fast, text-based API development experience within Zed.

## ✨ Key Features (Planned & In Progress)
- [x] **HTTP Syntax Highlighting**: Full support for `.http` and `.rest` files.
- [x] **In-Editor Requests**: Send requests directly from the editor using Code Lenses.
- [ ] **Response Preview**: View beautiful, formatted JSON, XML, or HTML responses.
- [ ] **Variables & Environments**: Manage dynamic data across multiple requests.
- [ ] **Sidecar Architecture**: Leveraging a Rust-based native backend for high performance and reliability.

## 🚀 Getting Started

### Installation
*Instructions on how to install from the Zed Extension Store will be added here once released.*

### Usage
Create a file ending in `.http` and write your request:

```http
GET https://api.github.com/repos/doani/zed-restclient
Accept: application/json
```

Then click the **Send Request** button (Code Lens) that appears above the request.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

**Important Rule: No Feature Without an Issue.**
Before starting work on a new feature or bug fix, **please open an issue first** to discuss the idea.

1. Open an Issue
2. Fork the Project
3. Create your Feature Branch (`git checkout -b feature/issue-123-AmazingFeature`)
4. Commit your Changes (`git commit -m 'feat: Add some AmazingFeature'`)
5. Push to the Branch (`git push origin feature/issue-123-AmazingFeature`)
6. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## ☕️ Support My Work

**Fueling Code, Infrastructure, and Open Source ☕️💻**

Hi, I'm so glad you stopped by!

I’ve been a professional developer since 2016. Currently, I work as a DevOps Engineer, where I'm heavily involved in shaping the future of tech through application analysis for AI Agents at **click solutions GmbH**. I love the challenge of bridging the gap between complex software and scalable infrastructure.

While I love my day job, I am just now starting an exciting new chapter: building my own private and open-source projects. My current focus is **zed-restclient**—my first major project for the Zed ecosystem.

Every coffee you buy or sponsorship you provide goes directly toward fueling my weekend coding sessions, covering server/hosting costs, and keeping the momentum going for tools like this.

- **GitHub Sponsors:** [github.com/sponsors/doani](https://github.com/sponsors/doani)
- **Buy Me a Coffee:** [buymeacoffee.com/doani](https://buymeacoffee.com/doani)

Thank you for being part of my coding journey!

## 📜 License
Distributed under the MIT License. See `LICENSE` for more information.
