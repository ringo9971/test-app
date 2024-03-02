import { useEffect, useState } from "react";
import { Input, Box, Button } from "@mui/material";
import ApiClient from "../ApiClient";

interface LogEntry {
  message: string;
  type: string;
}

interface Messages {
  messages: Message[];
}

interface Message {
  message: string;
}

const ChatApp = (): JSX.Element => {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [status, setStatus] = useState("disconnected");
  const [log, setLog] = useState<LogEntry[]>([]);
  const [inputText, setInputText] = useState("");
  const [retryCount, setRetryCount] = useState(0);

  const [gameId, setGameId] = useState("");

  const apiClient = new ApiClient();

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
    const retryConnectCallback = () => {
      if (retryCount <= 0) {
        return;
      }
      if (retryCount <= 3) {
        retryConnect();
      }
    };

    retryConnectCallback();
  }, [retryCount]);

  const connect = () => {
    if (gameId === "") {
      return;
    }
    disconnect();

    const proto = window.location.protocol.startsWith("https") ? "wss" : "ws";
    const host = import.meta.env.VITE_API_HOST;
    const wsUri = `${proto}://${host}/games/${gameId}/ws`;

    logMessage("Connecting...");
    const newSocket = new WebSocket(wsUri);

    newSocket.onopen = () => {
      logMessage("Connected");
      setSocket(newSocket);
      setStatus("connected");
      handleOpen();
      setRetryCount(0);
    };

    newSocket.onmessage = (ev: MessageEvent) => {
      let message: Message;
      try {
        message = JSON.parse(ev.data) as Message;
      } catch (e) {
        message = { message: ev.data };
      }

      logMessage(message.message);
    };

    newSocket.onclose = () => {
      logMessage("Disconnected");
      setSocket(null);
      setStatus("disconnected");

      setRetryCount((count) => count + 1);
    };
  };

  const disconnect = () => {
    if (socket) {
      logMessage("Disconnecting...");
      socket.close();
      setSocket(null);
      setStatus("disconnected");
    }
  };

  const handleOpen = async () => {
    const res = await apiClient.get<Messages>(`games/${gameId}`);
    setLog(res.messages.map((m) => ({ message: m.message, type: "status" })));
  };

  const handleSubmit = () => {
    if (!socket) {
      return;
    }

    const text = inputText.trim();

    if (text === "") {
      return;
    }

    logMessage(text);

    const message: Message = { message: text };
    socket.send(JSON.stringify(message));
    apiClient.create(`games/${gameId}`, message);

    setInputText("");
  };

  return (
    <Box>
      <Input
        value={gameId}
        onChange={(e) => setGameId(e.target.value)}
        disabled={status === "connected"}
      />
      <Button
        variant="contained"
        onClick={() => {
          if (socket) {
            setRetryCount(-1);
            disconnect();
          } else {
            setRetryCount(0);
            connect();
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
