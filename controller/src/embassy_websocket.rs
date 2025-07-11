use base64ct::{Base64, Encoding};
use core::fmt::Write;
use core::net::Ipv4Addr;
use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Instant};
use esp_hal::rng::Rng;
use heapless::String;

const REQUEST_BUFFER_SIZE: usize = 256;
const SOCKET_TIMEOUT_SECONDS: u64 = 20;
const HANDSHAKE_BUFFER_SIZE: usize = 512;
const PATH_BUFFER_SIZE: usize = 64;

pub struct EmbassyWebSocket<'a> {
    socket: Mutex<CriticalSectionRawMutex, Option<TcpSocket<'a>>>,
    rng: Mutex<CriticalSectionRawMutex, Option<Rng>>,
    ip: Ipv4Addr,
    port: u16,
    path: heapless::String<PATH_BUFFER_SIZE>,
    is_connected: Mutex<CriticalSectionRawMutex, bool>,
}

impl<'a> EmbassyWebSocket<'a> {
    pub fn new(
        ip: Ipv4Addr,
        port: u16,
        path: heapless::String<PATH_BUFFER_SIZE>,
        rng: Rng,
    ) -> Result<Self, EmbassyWebSocketError> {
        Ok(Self {
            socket: Mutex::new(None),
            rng: Mutex::new(Some(rng)),
            ip,
            port,
            path,
            is_connected: Mutex::new(false),
        })
    }

    pub async fn is_connected(&self) -> bool {
        let is_connected = self.is_connected.lock().await;
        *is_connected
    }

    pub async fn connect<'b>(
        &self,
        stack: &'b Stack<'b>,
        rx_buffer: &'b mut [u8],
        tx_buffer: &'b mut [u8],
    ) -> Result<(), EmbassyWebSocketError>
    where
        'b: 'a,
    {
        let mut socket = TcpSocket::new(*stack, rx_buffer, tx_buffer);

        socket
            .connect((self.ip, self.port))
            .await
            .map_err(|_e| EmbassyWebSocketError::ConnectionFailed)?;

        let websocket_key = {
            let mut rng_guard = self.rng.lock().await;
            generate_websocket_key(rng_guard.as_mut().unwrap())?
        };

        let upgrade_request =
            build_upgrade_request(self.ip, self.port, self.path.clone(), websocket_key)?;

        socket
            .write(upgrade_request.as_bytes())
            .await
            .map_err(|_e| EmbassyWebSocketError::HandshakeFailed)?;
        socket
            .flush()
            .await
            .map_err(|_e| EmbassyWebSocketError::HandshakeFailed)?;

        wait_for_handshake_response(&mut socket).await?;

        *self.socket.lock().await = Some(socket);
        *self.is_connected.lock().await = true;

        Ok(())
    }

    pub async fn write_text<const N: usize>(
        &self,
        text: String<N>,
    ) -> Result<(), EmbassyWebSocketError>
    where
        [u8; N + 6]: Sized,
    {
        let masking_key = {
            let mut rng_guard = self.rng.lock().await;
            if let Some(rng) = rng_guard.as_mut() {
                let mut key = [0u8; 4];
                rng.read(&mut key);
                key
            } else {
                return Err(EmbassyWebSocketError::ConnectionClosed);
            }
        };

        let frame = bytes_to_websocket_frame(text, masking_key)
            .map_err(|_| EmbassyWebSocketError::FrameCreationFailed)?;

        let mut socket_guard = self.socket.lock().await;
        if let Some(socket) = socket_guard.as_mut() {
            socket
                .write(&frame)
                .await
                .map_err(|_e| EmbassyWebSocketError::SendFailed)?;
            socket
                .flush()
                .await
                .map_err(|_e| EmbassyWebSocketError::SendFailed)?;
            Ok(())
        } else {
            Err(EmbassyWebSocketError::ConnectionClosed)
        }
    }

    pub async fn read(&self, buffer: &mut [u8]) -> Result<usize, EmbassyWebSocketError> {
        let mut socket_guard = self.socket.lock().await;

        if let Some(socket) = socket_guard.as_mut() {
            socket
                .read(buffer)
                .await
                .map_err(|_e| EmbassyWebSocketError::ReadError)
        } else {
            Err(EmbassyWebSocketError::ConnectionClosed)
        }
    }

    pub async fn read_with_timeout(
        &self,
        buffer: &mut [u8],
        timeout: Duration,
    ) -> Result<usize, EmbassyWebSocketError> {
        let mut socket_guard = self.socket.lock().await;

        if let Some(socket) = socket_guard.as_mut() {
            match embassy_time::with_timeout(timeout, socket.read(buffer)).await {
                Ok(result) => result.map_err(|_e| EmbassyWebSocketError::ReadError),
                Err(_) => Err(EmbassyWebSocketError::ReadError),
            }
        } else {
            Err(EmbassyWebSocketError::ConnectionClosed)
        }
    }

    pub async fn disconnect(self) -> Result<(), EmbassyWebSocketError> {
        if let Some(mut socket) = self.socket.lock().await.take() {
            socket.close();
            *self.is_connected.lock().await = false;
        }
        Ok(())
    }
}

fn generate_websocket_key(rng: &mut Rng) -> Result<String<24>, EmbassyWebSocketError> {
    let mut random_buffer: [u8; 16] = [0; 16];
    rng.read(&mut random_buffer);

    let mut encode_buffer: [u8; 24] = [0; 24];
    let random_str = Base64::encode(&random_buffer, &mut encode_buffer)
        .map_err(|_| EmbassyWebSocketError::KeyGenerationFailed)?;

    let mut key = String::new();
    key.push_str(random_str)
        .map_err(|_| EmbassyWebSocketError::KeyGenerationFailed)?;
    Ok(key)
}

fn build_upgrade_request(
    ip: Ipv4Addr,
    port: u16,
    path: String<PATH_BUFFER_SIZE>,
    websocket_key: String<24>,
) -> Result<String<REQUEST_BUFFER_SIZE>, EmbassyWebSocketError> {
    let mut upgrade_request: String<REQUEST_BUFFER_SIZE> = String::new();

    write!(
        &mut upgrade_request,
        "GET {} HTTP/1.1\r\nHost: {}:{}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n\r\n",
        path,
        ip,
        port,
        websocket_key,
    ).map_err(|_e| {
        EmbassyWebSocketError::HandshakeFailed
    })?;

    Ok(upgrade_request)
}

async fn wait_for_handshake_response(
    socket: &mut TcpSocket<'_>,
) -> Result<(), EmbassyWebSocketError> {
    let deadline = Instant::now() + Duration::from_secs(SOCKET_TIMEOUT_SECONDS);
    let mut buffer = [0u8; HANDSHAKE_BUFFER_SIZE];
    let mut total_bytes = 0;

    while Instant::now() < deadline {
        match socket.read(&mut buffer).await {
            Ok(len) => {
                if len > 0 {
                    total_bytes += len;
                    let response = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };

                    if response.contains("Sec-WebSocket-Accept")
                        || response.contains("sec-websocket-accept")
                    {
                        return Ok(());
                    }
                } else {
                    return Err(EmbassyWebSocketError::ConnectionClosed);
                }
            }
            Err(_e) => {
                return Err(EmbassyWebSocketError::ReadError);
            }
        }
    }

    if total_bytes == 0 {
        Err(EmbassyWebSocketError::HandshakeTimeout)
    } else {
        Err(EmbassyWebSocketError::HandshakeFailed)
    }
}

pub fn bytes_to_websocket_frame<const N: usize>(
    text: String<N>,
    masking_key: [u8; 4],
) -> Result<[u8; N + 6], &'static str>
where
    [u8; N + 6]: Sized,
{
    let mut frame = [0; N + 6];

    // FIN + Opcode
    frame[0] = 0x81;

    // MASK + Payload Length
    if N > 125 {
        return Err("Payload length is too long");
    }

    frame[1] = 0x80 | (N as u8); // 0x80 for MASK bit

    // Masking Key
    frame[2..6].copy_from_slice(&masking_key);

    // Masked Payload
    let bytes = text.as_bytes();
    for i in 0..N {
        frame[6 + i] = bytes[i] ^ masking_key[i % 4];
    }

    Ok(frame)
}

#[derive(Debug)]
pub enum EmbassyWebSocketError {
    ConnectionFailed,
    HandshakeFailed,
    HandshakeTimeout,
    ConnectionClosed,
    ReadError,
    SendFailed,
    KeyGenerationFailed,
    FrameCreationFailed,
    MessageTooLarge,
}
