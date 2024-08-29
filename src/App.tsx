import { useState } from "react";
import reactLogo from "./assets/react.svg";
import csharpLogo from "./assets/csharp.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [executionTime, setExecutionTime] = useState<number | undefined>(
    undefined
  );

  async function greet() {
    setGreetMsg("Carregando...");
    setIsLoading(true);
    const start = performance.now();

    const result = await invoke("greet", { name });

    const end = performance.now();
    setIsLoading(false);
    setGreetMsg(result as string);
    setExecutionTime(end - start);
  }

  return (
    <div className="container">
      <h1>Welcome to TauriSharp!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
        <a href="https://dotnet.microsoft.com" target="_blank">
          <img src={csharpLogo} className="logo csharp" alt="C# logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, React and C# logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit" disabled={isLoading}>
          Greet
        </button>
      </form>

      <p>{greetMsg}</p>
      {executionTime && <small>Execution time: {executionTime.toFixed(2)}ms</small>}
    </div>
  );
}

export default App;
