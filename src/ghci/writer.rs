use async_dup::Arc;
use async_dup::Mutex;
use std::fmt::Debug;
use std::io;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::fs::File;
use tokio::io::AsyncWrite;
use tokio::io::DuplexStream;
use tokio::io::Sink;
use tokio::io::Stderr;
use tokio::io::Stdout;
use tokio_util::compat::Compat;
use tokio_util::compat::FuturesAsyncWriteCompatExt;
use tokio_util::compat::TokioAsyncWriteCompatExt;

/// A dynamically reconfigurable sink for `ghci` process output. Built for use in `GhciOpts`, but
/// usable as a general purpose clonable [`AsyncWrite`]r.
#[derive(Debug)]
pub struct GhciWriter(Kind);

#[derive(Debug)]
enum Kind {
    Stdout(Stdout),
    Stderr(Stderr),
    DuplexStream(Compat<Arc<Mutex<Compat<DuplexStream>>>>),
    Sink(Sink),
    Tee(Box<GhciWriter>, Arc<Mutex<File>>),
}

impl GhciWriter {
    /// Write to `stdout`.
    pub fn stdout() -> Self {
        Self(Kind::Stdout(tokio::io::stdout()))
    }

    /// Write to `stderr`.
    pub fn stderr() -> Self {
        Self(Kind::Stderr(tokio::io::stderr()))
    }

    /// Write to an in-memory buffer.
    pub fn duplex_stream(duplex_stream: DuplexStream) -> Self {
        Self(Kind::DuplexStream(
            Arc::new(Mutex::new(duplex_stream.compat_write())).compat_write(),
        ))
    }

    /// Write to the void.
    pub fn sink() -> Self {
        Self(Kind::Sink(tokio::io::sink()))
    }

    /// Write to both the given writer and a file (duplicate output).
    pub fn tee(writer: Self, file: File) -> Self {
        Self(Kind::Tee(Box::new(writer), Arc::new(Mutex::new(file))))
    }
}

impl AsyncWrite for GhciWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match Pin::into_inner(self).0 {
            Kind::Stdout(ref mut x) => Pin::new(x).poll_write(cx, buf),
            Kind::Stderr(ref mut x) => Pin::new(x).poll_write(cx, buf),
            Kind::DuplexStream(ref mut x) => Pin::new(x).poll_write(cx, buf),
            Kind::Sink(ref mut x) => Pin::new(x).poll_write(cx, buf),
            Kind::Tee(ref mut writer, ref mut file) => {
                // Write to the primary writer first
                let primary_result = Pin::new(writer.as_mut()).poll_write(cx, buf);

                // Try to write to the file as well (best-effort)
                if let Some(mut guard) = file.try_lock() {
                    // If we can lock the file, try to write to it
                    match Pin::new(&mut *guard).poll_write(cx, buf) {
                        Poll::Ready(Ok(_)) => {}
                        Poll::Ready(Err(e)) => {
                            tracing::warn!("Failed to write to output file: {e}");
                        }
                        Poll::Pending => {}
                    }
                }

                primary_result
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::into_inner(self).0 {
            Kind::Stdout(ref mut x) => Pin::new(x).poll_flush(cx),
            Kind::Stderr(ref mut x) => Pin::new(x).poll_flush(cx),
            Kind::DuplexStream(ref mut x) => Pin::new(x).poll_flush(cx),
            Kind::Sink(ref mut x) => Pin::new(x).poll_flush(cx),
            Kind::Tee(ref mut writer, ref mut file) => {
                // Flush both the primary writer and the file
                let primary_result = Pin::new(writer.as_mut()).poll_flush(cx);

                if let Some(mut guard) = file.try_lock() {
                    match Pin::new(&mut *guard).poll_flush(cx) {
                        Poll::Ready(Ok(_)) => {}
                        Poll::Ready(Err(e)) => {
                            tracing::warn!("Failed to flush output file: {e}");
                        }
                        Poll::Pending => {}
                    }
                }

                primary_result
            }
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match Pin::into_inner(self).0 {
            Kind::Stdout(ref mut x) => Pin::new(x).poll_shutdown(cx),
            Kind::Stderr(ref mut x) => Pin::new(x).poll_shutdown(cx),
            Kind::DuplexStream(ref mut x) => Pin::new(x).poll_shutdown(cx),
            Kind::Sink(ref mut x) => Pin::new(x).poll_shutdown(cx),
            Kind::Tee(ref mut writer, ref mut file) => {
                // Shutdown both the primary writer and the file
                let primary_result = Pin::new(writer.as_mut()).poll_shutdown(cx);

                if let Some(mut guard) = file.try_lock() {
                    match Pin::new(&mut *guard).poll_shutdown(cx) {
                        Poll::Ready(Ok(_)) => {}
                        Poll::Ready(Err(e)) => {
                            tracing::warn!("Failed to shutdown output file: {e}");
                        }
                        Poll::Pending => {}
                    }
                }

                primary_result
            }
        }
    }
}

impl Clone for GhciWriter {
    fn clone(&self) -> Self {
        match &self.0 {
            Kind::Stdout(_) => Self::stdout(),
            Kind::Stderr(_) => Self::stderr(),
            Kind::DuplexStream(x) => Self(Kind::DuplexStream(x.clone())),
            Kind::Sink(_) => Self::sink(),
            Kind::Tee(writer, file) => Self(Kind::Tee(writer.clone(), file.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_tee_writes_to_both_destinations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("output.txt");

        // Create a duplex stream to simulate console output
        let (console_writer, mut console_reader) = tokio::io::duplex(1024);

        // Create a file for tee output
        let output_file = tokio::fs::File::create(&output_path).await.unwrap();

        // Create a tee writer that writes to both
        let console_ghci = GhciWriter::duplex_stream(console_writer);
        let mut tee_writer = GhciWriter::tee(console_ghci, output_file);

        // Write some test data
        let test_data = b"Hello from GHCi!\n";
        tee_writer.write_all(test_data).await.unwrap();
        tee_writer.flush().await.unwrap();

        // Verify the data was written to the console (duplex stream)
        let mut console_buffer = vec![0u8; test_data.len()];
        console_reader
            .read_exact(&mut console_buffer)
            .await
            .unwrap();
        assert_eq!(&console_buffer, test_data);

        // Close the writer to ensure file is flushed
        drop(tee_writer);

        // Verify the data was written to the file
        let file_contents = tokio::fs::read_to_string(&output_path).await.unwrap();
        assert_eq!(file_contents.as_bytes(), test_data);
    }

    #[tokio::test]
    async fn test_tee_can_be_cloned() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("output.txt");

        let (console_writer, _console_reader) = tokio::io::duplex(1024);
        let output_file = tokio::fs::File::create(&output_path).await.unwrap();

        let console_ghci = GhciWriter::duplex_stream(console_writer);
        let tee_writer = GhciWriter::tee(console_ghci, output_file);

        // Clone should work
        let _cloned_writer = tee_writer.clone();

        // Both writers should share the same underlying file
        // (this is ensured by Arc<Mutex<File>>)
    }

    #[tokio::test]
    async fn test_tee_with_stdout() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("output.txt");

        let output_file = tokio::fs::File::create(&output_path).await.unwrap();
        let mut tee_writer = GhciWriter::tee(GhciWriter::stdout(), output_file);

        // Write some test data
        let test_data = b"Testing stdout tee\n";
        tee_writer.write_all(test_data).await.unwrap();
        tee_writer.flush().await.unwrap();

        // Close the writer to ensure file is flushed
        drop(tee_writer);

        // Verify the data was written to the file
        let file_contents = tokio::fs::read_to_string(&output_path).await.unwrap();
        assert_eq!(file_contents.as_bytes(), test_data);
    }
}
