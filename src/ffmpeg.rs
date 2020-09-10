use crate::config::FFmpegConfig;
use hyper::body::Bytes;
use hyper::body::Sender;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use std::sync::Arc;

struct ArgBuilder<'a> {
    args: Vec<&'a str>,
}

impl<'a> ArgBuilder<'a> {
    fn with(mut self, name: &str, val: &'a str) -> ArgBuilder<'a> {
        let search = format!("%{}", name);
        if let Some(arg) = self.args.iter_mut().find(|a| **a == search) {
            *arg = val;
        } else {
            println!("[W]: ffmpeg argument {} could not be set.", name);
        }
        self
    }

    fn build(self) -> Vec<&'a str> {
        self.args
    }
}

pub struct FFmpeg {
    config: Arc<FFmpegConfig>,
}

impl FFmpeg {
    pub fn new(config: Arc<FFmpegConfig>) -> FFmpeg {
        FFmpeg { config }
    }

    pub async fn transcode(&self, file: &str, mut sender: Sender) {
        let args = self
            .build_args()
            .with("ss", "0")
            .with("i", file)
            .with("f", "mp4")
            .with("vcodec", "copy")
            .with("acodec", "copy")
            .build();

        let mut cmd = Command::new(&self.config.bin)
            .args(&args)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut buf: [u8; 65536] = [0; 65536];
        let mut stdout = BufReader::new(cmd.stdout.as_mut().unwrap());

        while let Ok(()) = stdout.read_exact(&mut buf) {
            let b = Bytes::copy_from_slice(&buf);
            sender.send_data(b).await.unwrap();
            buf = [0; 65536];
        }

        let status = cmd.wait();
        println!("Exited with status {:?}", status);
    }

    fn build_args(&self) -> ArgBuilder<'_> {
        let args = self
            .config
            .codecs
            .get("*")
            .unwrap()
            .args
            .iter()
            .map(|a| a.as_str())
            .collect::<Vec<&str>>();
        ArgBuilder { args }
    }
}
