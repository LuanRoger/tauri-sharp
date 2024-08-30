# TauriSharp

![screenshot](https://github.com/LuanRoger/tauri-sharp/blob/main/images/screenshot.png)

It's a simple template that uses Tauri with a C# (gRPC) backend.

## Techs

- [Tauri 2.0](https://v2.tauri.app)
- [Rust](https://rust-lang.org)
- [Tonic](https://docs.rs/tonic/latest/tonic)
- [Vite](https://vitejs.dev)
- [React](https://react.dev)
- [TypeScript](https://typescriptlang.org)
- [C#](https://docs.microsoft.com/en-us/dotnet/csharp)
- [gRPC](https://grpc.io)

## How it works?

A simple Tauri application but it starts with a gRPC server at `localhost:5030`, made in C#, and the Rust layer uses Tonic to communicate with it.

## Motivation

The C# layer was introduced to extends the Rust backend with more Windows native capabilities, or just to use some C# library.

So you can build desktop applications using web technologies that has no limitation when comes to native integration, in addition to having a wide range of libraries available to be used, together with the .NET library.

## How to run?

### Pre-requisites

- [.NET 8](https://dotnet.microsoft.com/download/dotnet/5.0)
- [Node.js](https://nodejs.org)
- [NPM](https://npmjs.com)
- [Rust](https://rust-lang.org)

### Steps

1. Clone the repository (recomended to use SSH)

```bash
git clone git@github.com:LuanRoger/tauri-sharp.git
```

2. Install the dependencies

```bash
cd tauri-sharp
npm install
```

3. Run the application

```bash
npm run tauri dev
```

> The C# server will start automatically when you run the Tauri application. You can check this configuration in the `tauri.conf.json` file.

## License

This project is under the MIT license. See the [LICENSE](https://github.com/LuanRoger/tauri-sharp/blob/main/LICENSE) file for more details.