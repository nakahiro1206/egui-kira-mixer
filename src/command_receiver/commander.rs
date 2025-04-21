use crate::audio_handle::types::AudioHandle;

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

#[derive(Debug)]
pub enum AudioCommand {
    AddAudio(String),
    ChangeCutoff(String),
    SetVolume(String, f64),
}

pub struct Commander {
    pub sender: Sender<AudioCommand>,
}

impl Commander {
    pub fn new() -> Self {
        let (sender, receiver): (Sender<AudioCommand>, Receiver<AudioCommand>) = mpsc::channel();
        let audio_handle = Arc::new(Mutex::new(AudioHandle::new()));

        let __thread = {
            let audio_handle = Arc::clone(&audio_handle);
            thread::spawn(move || {
                while let Ok(cmd) = receiver.recv() {
                    let mut audio = audio_handle.lock().unwrap();
                    match cmd {
                        AudioCommand::AddAudio(name) => {
                            audio.add_audio();
                        }
                        AudioCommand::ChangeCutoff(name) => {
                            audio.change_cutoff();
                        }
                        AudioCommand::SetVolume(name, val) => {}
                    }
                }
            })
        };

        Self { sender }
    }

    pub fn send(&self, command: AudioCommand) {
        let _ = self.sender.send(command);
    }
}
