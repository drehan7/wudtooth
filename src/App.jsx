import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import MainHeader from "./Components/Header/MainHeader";
import Device from "./Components/Device/Device";
import "./App.css";

function App() {
    const [ greetMsg, setGreetMsg ] = useState("");
    const [ name, setName ] = useState("");
    const [ devices, setDevices ] = useState([]);

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet", { name }));
    }

    async function getDevices() {
        let dev_ret = await invoke("get_devices", {});

        setDevices(dev_ret.filter( d => d?.name.length ));
    }

    return (
        <div className="container">
            <MainHeader />
            <div className="main-devices-container">
                {devices && devices.map(d => (
                    <Device device={d} />
                ))}
                <button onClick={getDevices}>BUTTON</button>
            </div>
        </div>
    );
}

export default App;
