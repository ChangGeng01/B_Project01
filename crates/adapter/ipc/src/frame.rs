//! 帧编解码：4 字节大端长度前缀加 JSON 体，单帧上限由调用方给。
//!
//! 长度前缀先于解析被判上限：先分配再判长度，等于把「对端声称的长度」
//! 当成可信输入，那是最典型的一条拒绝服务路径。

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// 协议允许的单帧上限，配置项 `ipc.max_frame_bytes` 的默认值。
pub const DEFAULT_MAX_FRAME_BYTES: u32 = 1_048_576;

#[derive(Debug)]
pub enum FrameError {
    /// 对端正常关闭，且不在一帧的中间。
    Closed,
    TooLarge {
        declared: u32,
        limit: u32,
    },
    Io(std::io::Error),
}

impl std::fmt::Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameError::Closed => f.write_str("对端已关闭连接"),
            FrameError::TooLarge { declared, limit } => {
                write!(f, "帧长 {declared} 超过上限 {limit}")
            }
            FrameError::Io(e) => write!(f, "IO 错误：{e}"),
        }
    }
}

impl std::error::Error for FrameError {}

impl From<std::io::Error> for FrameError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
            FrameError::Closed
        } else {
            FrameError::Io(e)
        }
    }
}

pub async fn write_frame<W: AsyncWrite + Unpin>(
    w: &mut W,
    body: &[u8],
    max: u32,
) -> Result<(), FrameError> {
    let len = u32::try_from(body.len()).unwrap_or(u32::MAX);
    if len > max {
        return Err(FrameError::TooLarge {
            declared: len,
            limit: max,
        });
    }
    w.write_all(&len.to_be_bytes()).await?;
    w.write_all(body).await?;
    w.flush().await?;
    Ok(())
}

pub async fn read_frame<R: AsyncRead + Unpin>(r: &mut R, max: u32) -> Result<Vec<u8>, FrameError> {
    let mut head = [0u8; 4];
    match r.read_exact(&mut head).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Err(FrameError::Closed),
        Err(e) => return Err(FrameError::from(e)),
    }
    let declared = u32::from_be_bytes(head);
    if declared > max {
        // 不读取正文：超长帧一律断开，读完再拒等于替对端付了内存。
        return Err(FrameError::TooLarge {
            declared,
            limit: max,
        });
    }
    let mut body = vec![0u8; declared as usize];
    r.read_exact(&mut body).await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn round_trip_preserves_bytes() {
        let mut buf = Vec::new();
        write_frame(&mut buf, b"{\"v\":1}", DEFAULT_MAX_FRAME_BYTES)
            .await
            .unwrap();
        let mut cursor = Cursor::new(buf);
        let body = read_frame(&mut cursor, DEFAULT_MAX_FRAME_BYTES)
            .await
            .unwrap();
        assert_eq!(body, b"{\"v\":1}");
    }

    #[tokio::test]
    async fn empty_frame_round_trips() {
        let mut buf = Vec::new();
        write_frame(&mut buf, b"", 16).await.unwrap();
        let mut cursor = Cursor::new(buf);
        assert!(read_frame(&mut cursor, 16).await.unwrap().is_empty());
    }

    // 负样例断言的是上限这条规则本身：声明超长的帧必须在读正文之前被拒。
    #[tokio::test]
    async fn oversized_declared_length_is_rejected_before_reading_the_body() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&2_000_000u32.to_be_bytes());
        // 正文一个字节都不给：若实现先读正文，这里会挂住而不是立刻报错。
        let mut cursor = Cursor::new(buf);
        let err = read_frame(&mut cursor, DEFAULT_MAX_FRAME_BYTES)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            FrameError::TooLarge {
                declared: 2_000_000,
                limit: 1_048_576
            }
        ));
    }

    #[tokio::test]
    async fn oversized_write_is_rejected() {
        let mut buf = Vec::new();
        let err = write_frame(&mut buf, &[0u8; 32], 16).await.unwrap_err();
        assert!(matches!(
            err,
            FrameError::TooLarge {
                declared: 32,
                limit: 16
            }
        ));
        assert!(buf.is_empty(), "被拒的帧不得写出半截");
    }

    #[tokio::test]
    async fn clean_close_is_distinguished_from_a_truncated_frame() {
        let mut empty = Cursor::new(Vec::new());
        assert!(matches!(
            read_frame(&mut empty, 16).await.unwrap_err(),
            FrameError::Closed
        ));

        let mut half = Cursor::new(4u32.to_be_bytes().to_vec());
        assert!(matches!(
            read_frame(&mut half, 16).await.unwrap_err(),
            FrameError::Closed
        ));
    }
}
