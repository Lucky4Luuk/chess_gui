use std::sync::mpsc;
use std::thread;
use std::process::Command;

pub enum EngineOrPlayer {
    Player,
    Engine(Engine),
}

pub struct Engine {
    pub path: String,
    pub locked_in: bool,
    handles: Vec<thread::JoinHandle<()>>,
}

impl Engine {
    pub fn empty() -> Self {
        Self {
            path: String::new(),
            locked_in: false,
            handles: Vec::new(),
        }
    }

    // Spawns a thread, which calls upon the exe at self.path, and returns a MovePromise.
    // The MovePromise can be polled to see if a move has been returned already, so the
    // main thread doesn't have to block.
    pub fn gen_move(&mut self, board: &chess::Board) -> MovePromise {
        let fen = format!("{}", board);
        let (sender, receiver) = mpsc::channel();
        let path = self.path.clone();
        // TODO: Push to buffer of handles, to cancel any stuck threads
        thread::spawn(move || {
            match Command::new(&format!("./{}", path)).args(["--board", &fen]).output() {
                Ok(output) => {
                    // TODO: Check output status
                    println!("output: {:?}", output);
                    sender.send(Ok(String::from_utf8(output.stdout).unwrap()));
                },
                Err(err) => { sender.send(Err(format!("Error: {err}"))); },
            };
        });
        MovePromise::new(receiver)
    }
}

pub struct MovePromise {
    receiver: mpsc::Receiver<Result<String, String>>,
}

impl MovePromise {
    pub fn new(receiver: mpsc::Receiver<Result<String, String>>) -> Self {
        Self {
            receiver,
        }
    }

    pub fn poll_recv(self) -> Result<Result<String, String>, Self> {
        self.receiver.try_recv().map_err(|_| self)
    }
}
