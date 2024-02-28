import { useEffect, useState } from "react";
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
  const [retryCount, setRetryCount] = useState(0);

  const logMessage = (msg: string, type = "status") => {
    setLog((prevLog) => [...prevLog, { message: msg, type: type }]);
  };

  const retryConnect = () => {
    const timeoutId = setTimeout(() => {
      connect();
    }, 1000);

    return () => clearTimeout(timeoutId);
  };

  useEffect(() => {
    if (retryCount == 0) {
      return;
    }
    if (retryCount <= 3) {
      retryConnect();
    }
  }, [retryCount]);

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
      setRetryCount(0);
    };

    newSocket.onmessage = (ev: MessageEvent) => {
      logMessage("Received: " + ev.data, "message");
    };

    newSocket.onclose = () => {
      logMessage("Disconnected");
      setSocket(null);
      updateConnectionStatus();

      setRetryCount((count) => count + 1);
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

  const handleSubmit = () => {
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
      <Button
        variant="contained"
        onClick={() => {
          if (socket) {
            disconnect();
          } else {
            setRetryCount(0);
            retryConnect();
          }
        }}
      >
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
      <Input value={inputText} onChange={(e) => setInputText(e.target.value)} />
      <Button
        variant="contained"
        onClick={handleSubmit}
        disabled={socket === null}
      >
        送信する
      </Button>
    </Box>
  );
};

export default ChatApp;
