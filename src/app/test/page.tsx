'use client'

import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

export default function Test() {
    const [simConnectData, setSimConnectData] = useState(null);

    useEffect(() => {
        // Start the SimConnect listener when the component mounts
        invoke('start_simconnect_listener')
            .catch(console.error);

        // Listen for SimConnect data events
        const unlisten = listen('simconnect-data', (event) => {
            console.log('Received data from SimConnect:', event.payload);
            // Update the state with the new data
            setSimConnectData(event.payload);
        });

        // Cleanup: stop listening to the event when the component unmounts
        return () => {
            unlisten.then((unlistenFn) => unlistenFn());
        };
    }, []);

    return (
        <div>
            <h1>Test</h1>
            <div>
                <h2>SimConnect Data:</h2>
                {simConnectData ? (
                    <pre>{JSON.stringify(simConnectData, null, 2)}</pre>
                ) : (
                    <p>No data received yet.</p>
                )}
            </div>
        </div>
    );
}
