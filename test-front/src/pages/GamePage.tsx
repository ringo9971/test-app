import { useState } from "react";
import { Input, Box, Button } from "@mui/material";

interface LogEntry {
  message: string;
  type: string;
}

const ChatApp = (): JSX.Element => {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [status, setStatus] = useState("disconnected");
  const [log, setLog] = useState<LogEntry[]>([]);
  const [inputText, setInputText] = useState("");

  const logMessage = (msg: string, type = "status") => {
    setLog((prevLog) => [...prevLog, { message: msg, type: type }]);
  };

  const connect = () => {
    disconnect();

    const proto = window.location.protocol.startsWith("https") ? "wss" : "ws";
    const host = import.meta.env.VITE_API_HOST;
    const wsUri = `${proto}://${host}/ws`;

    logMessage("Connecting...");
    const newSocket = new WebSocket(wsUri);

    newSocket.onopen = () => {
      logMessage("Connected");
      setSocket(newSocket);
      updateConnectionStatus();
    };

    newSocket.onmessage = (ev: MessageEvent) => {
      logMessage("Received: " + ev.data, "message");
    };

    newSocket.onclose = () => {
      logMessage("Disconnected");
      setSocket(null);
      updateConnectionStatus();
    };
  };

  const disconnect = () => {
    if (socket) {
      logMessage("Disconnecting...");
      socket.close();
      setSocket(null);
      updateConnectionStatus();
    }
  };

  const updateConnectionStatus = () => {
    if (socket) {
      setStatus("connected");
    } else {
      setStatus("disconnected");
    }
  };

  const handleSubmit = (ev: React.FormEvent) => {
    ev.preventDefault();

    if (!socket) {
      return;
    }

    const text = inputText.trim();

    if (text === "") {
      return;
    }

    logMessage("Sending: " + text);
    socket.send(text);

    setInputText("");
  };

  return (
    <Box>
      <Button variant="contained" onClick={socket ? disconnect : connect}>
        {socket ? "切断" : "接続"}
      </Button>
      <Box display="flex">
        Status:
        <Box
          style={{
            backgroundColor: status === "connected" ? "transparent" : "red",
            color: status === "connected" ? "green" : "white",
          }}
        >
          {status}
        </Box>
      </Box>
      <Box
        id="log"
        style={{
          width: "30em",
          height: "20em",
          overflow: "auto",
          margin: "0.5em 0",
          border: "1px solid black",
        }}
      >
        {log.map((entry, index) => (
          <Box key={index} className={`msg msg--${entry.type}`}>
            {entry.message}
          </Box>
        ))}
      </Box>
      <Box>
        <Input
          type="text"
          id="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
        />
        <Button variant="contained" onSubmit={handleSubmit}>
          送信する
        </Button>
      </Box>
    </Box>
  );
};

export default ChatApp;
