# Proxy Checker

[![License](https://img.shields.io/github/license/dry-com/proxy-checker)](https://github.com/dry-com/proxy-checker/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues/dry-com/proxy-checker)](https://github.com/dry-com/proxy-checker/issues)
[![Stars](https://img.shields.io/github/stars/dry-com/proxy-checker)](https://github.com/dry-com/proxy-checker/stargazers)

Proxy Checker is a powerful and efficient application for testing the availability and speed of proxies. Built using Tauri, React, Vite, and styled with shadcn, this tool offers a modern and performant UI, while being lightweight and secure.

## Features

- **Cross-platform**: Runs on Windows, macOS, and Linux.
- **Efficient Testing**: Quickly test multiple proxies for connectivity and speed.
- **User-Friendly Interface**: Built with React and styled using shadcn for a clean and intuitive UI.
- **Lightweight & Secure**: Developed using Tauri to ensure a small footprint and enhanced security.
- **Real-time Feedback**: Get instant results on the status of each proxy.

## Technologies Used

- **[Tauri](https://tauri.app/)**: Framework for building tiny, fast, and secure desktop applications.
- **[React](https://reactjs.org/)**: JavaScript library for building user interfaces.
- **[Vite](https://vitejs.dev/)**: Next-generation frontend tooling.
- **[shadcn](https://shadcn.dev/)**: UI components and styling based on Radix UI primitives.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Node.js**: Version 14.x or later
- **Rust**: Required for Tauri development
- **yarn** or **npm**: For package management

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/dry-com/proxy-checker.git
   cd proxy-checker
   ```

2. **Install dependencies**:

   If you use `yarn`:

   ```bash
   yarn install
   ```

   Or if you use `npm`:

   ```bash
   npm install
   ```

3. **Build the application**:

   For development:

   ```bash
   npm tauri dev
   ```

   For production:

   ```bash
   npm tauri build
   ```

## Usage

After following the installation steps, you can start using Proxy Checker:

1. **Run the application**:

   ```bash
   cargo tauri dev
   ```

2. **Load Proxies**: Add the list of proxies you want to test.
3. **Start Checking**: Click the "Check" button to start testing proxies.
4. **View Results**: The status and speed of each proxy will be displayed in real-time.

## Contributing

Contributions are always welcome!

To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature-name`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature/your-feature-name`).
6. Open a pull request.

Please make sure your code adheres to the project's coding standards.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/dry-com/proxy-checker/blob/main/LICENSE) file for more details.

## Acknowledgements

- Thanks to the Tauri, React, Vite, and shadcn teams for their incredible tools and frameworks.

