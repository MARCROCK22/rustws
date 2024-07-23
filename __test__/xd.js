formatMemoryUsage();

const { createConnection } = require("../index");

formatMemoryUsage();

const socket = createConnection({
    onMessage: (...message) => {
        // console.log("message", ...message); // buildea. q deberia dar?
    },
    onClose: () => {
        console.log("close");
    },
    onError: (error) => {
        console.log("error", error);
    },
    onOpen: (...args) => {
        console.log("open", ...args);
    },
    url: 'xd'
});

formatMemoryUsage();

// socket.send("hola");

// formatMemoryUsage();

function formatMemoryUsage() {
    const used = process.memoryUsage();
    for (let key in used) {
        console.log(`${key} ${Math.round(used[key] / 1024 / 1024 * 100) / 100} MB`);
    }
}