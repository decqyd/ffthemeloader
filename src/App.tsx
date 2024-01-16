import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";

function handleFirefoxReload() {
  invoke("restart_firefox")
}

function App() {
  return (
    <div class="bg-[#131313] flex flex-col justify-center h-screen items-center text-white">
      <h1 class="text-5xl font-raleway pb-20">FIREFOX THEME LOADER</h1>
      
      <Button variant="secondary" onClick={() => handleFirefoxReload()}>Reload Firefox</Button>
    </div>
  );
}

export default App;
