use regex::Regex;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Emitter; // 使用 Emitter trait 以使用 emit 方法
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use tokio::sync::oneshot;

// 错误处理
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("转换失败: {0}")]
    ConversionError(String),
    #[error(transparent)]
    TauriError(#[from] tauri::Error),
}

// 实现序列化以便在前端显示错误
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// 进度结构体
#[derive(Clone, serde::Serialize)]
struct ProgressInfo {
    progress: usize,
    output_path: Option<String>,
}

// 检查ffmpeg是否可用
async fn check_ffmpeg_available() -> bool {
    match tokio::process::Command::new("ffmpeg")
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

// 解析ffmpeg输出以获取进度
fn parse_progress(line: &str) -> Option<usize> {
    // 尝试匹配 "time=HH:MM:SS.MS" 格式
    let re = Regex::new(r"time=(\d+):(\d+):(\d+\.\d+)").ok()?;
    if let Some(caps) = re.captures(line) {
        let hours: usize = caps.get(1)?.as_str().parse().ok()?;
        let minutes: usize = caps.get(2)?.as_str().parse().ok()?;
        let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;

        let total_seconds = (hours * 3600) as f64 + (minutes * 60) as f64 + seconds;

        // 假设转换通常不会超过10分钟，将进度映射到0-100
        // 这是一个简化的估计，实际应用中可能需要更复杂的逻辑
        let progress = (total_seconds / 600.0 * 100.0).min(99.0) as usize;
        return Some(progress);
    }

    // 尝试匹配百分比格式（如果ffmpeg输出包含它）
    let re = Regex::new(r"(\d+)%").ok()?;
    if let Some(caps) = re.captures(line) {
        return caps.get(1)?.as_str().parse().ok();
    }

    None
}

// 音频转换命令
#[tauri::command]
async fn convert_audio(
    window: tauri::Window,
    input_path: String,
    output_format: String,
) -> Result<String, Error> {
    // 检查ffmpeg是否可用
    if !check_ffmpeg_available().await {
        return Err(Error::ConversionError(
            "FFmpeg not found. Please install FFmpeg to use this feature.".to_string(),
        ));
    }

    // 获取输出路径
    let input_path = Path::new(&input_path);
    let file_stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let output_path = input_path.with_file_name(format!("{}.{}", file_stem, output_format));
    let output_path_str = output_path.to_string_lossy().to_string();

    // 创建ffmpeg命令
    let command = window
        .shell()
        .command("ffmpeg")
        .args(["-y", "-i", &input_path.to_string_lossy()])
        .args(["-progress", "pipe:1"])
        .arg(&output_path_str);

    // 执行命令
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| Error::ConversionError(e.to_string()))?;

    // 创建进度跟踪器
    let progress = Arc::new(AtomicUsize::new(0));
    let progress_clone = progress.clone();

    // 监控命令输出以获取进度信息
    let window_clone = window.clone();

    // 创建一个通道来接收命令完成的信号
    let (tx, rx_done) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // 处理命令输出和监听命令完成事件
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    // 将 Vec<u8> 转换为 String
                    let line_str = String::from_utf8_lossy(&line);
                    if let Some(prog) = parse_progress(&line_str) {
                        progress_clone.store(prog, Ordering::Relaxed);
                        let _ = window_clone.emit(
                            "conversion_progress",
                            ProgressInfo {
                                progress: prog,
                                output_path: None,
                            },
                        );
                    }
                }
                CommandEvent::Stderr(line) => {
                    // 将 Vec<u8> 转换为 String
                    let line_str = String::from_utf8_lossy(&line);
                    if let Some(prog) = parse_progress(&line_str) {
                        progress_clone.store(prog, Ordering::Relaxed);
                        let _ = window_clone.emit(
                            "conversion_progress",
                            ProgressInfo {
                                progress: prog,
                                output_path: None,
                            },
                        );
                    }
                }
                CommandEvent::Terminated(status) => {
                    // 在Unix系统中，退出码为0表示成功
                    let success = status.code.map_or(false, |code| code == 0);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(success);
                    }
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待命令完成
    let success = rx_done.await.unwrap_or(false);

    if success {
        // 转换成功
        window
            .emit(
                "conversion_progress",
                ProgressInfo {
                    progress: 100,
                    output_path: Some(output_path_str.clone()),
                },
            )
            .map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(output_path_str)
    } else {
        Err(Error::ConversionError(
            "FFmpeg conversion failed".to_string(),
        ))
    }
}

// 视频转换命令
#[tauri::command]
async fn convert_video(
    window: tauri::Window,
    input_path: String,
    output_format: String,
) -> Result<String, Error> {
    // 检查ffmpeg是否可用
    if !check_ffmpeg_available().await {
        return Err(Error::ConversionError(
            "FFmpeg not found. Please install FFmpeg to use this feature.".to_string(),
        ));
    }

    // 获取输出路径
    let input_path = Path::new(&input_path);
    let file_stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let output_path = input_path.with_file_name(format!("{}.{}", file_stem, output_format));
    let output_path_str = output_path.to_string_lossy().to_string();

    // 创建ffmpeg命令
    let command = window
        .shell()
        .command("ffmpeg")
        .args(["-y", "-i", &input_path.to_string_lossy()])
        .args(["-progress", "pipe:1"])
        .arg(&output_path_str);

    // 执行命令
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| Error::ConversionError(e.to_string()))?;

    // 创建进度跟踪器
    let progress = Arc::new(AtomicUsize::new(0));
    let progress_clone = progress.clone();

    // 监控命令输出以获取进度信息
    let window_clone = window.clone();

    // 创建一个通道来接收命令完成的信号
    let (tx, rx_done) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // 处理命令输出和监听命令完成事件
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    // 将 Vec<u8> 转换为 String
                    let line_str = String::from_utf8_lossy(&line);
                    if let Some(prog) = parse_progress(&line_str) {
                        progress_clone.store(prog, Ordering::Relaxed);
                        let _ = window_clone.emit(
                            "conversion_progress",
                            ProgressInfo {
                                progress: prog,
                                output_path: None,
                            },
                        );
                    }
                }
                CommandEvent::Stderr(line) => {
                    // 将 Vec<u8> 转换为 String
                    let line_str = String::from_utf8_lossy(&line);
                    if let Some(prog) = parse_progress(&line_str) {
                        progress_clone.store(prog, Ordering::Relaxed);
                        let _ = window_clone.emit(
                            "conversion_progress",
                            ProgressInfo {
                                progress: prog,
                                output_path: None,
                            },
                        );
                    }
                }
                CommandEvent::Terminated(status) => {
                    // 在Unix系统中，退出码为0表示成功
                    let success = status.code.map_or(false, |code| code == 0);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(success);
                    }
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待命令完成
    let success = rx_done.await.unwrap_or(false);

    if success {
        // 转换成功
        window
            .emit(
                "conversion_progress",
                ProgressInfo {
                    progress: 100,
                    output_path: Some(output_path_str.clone()),
                },
            )
            .map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(output_path_str)
    } else {
        Err(Error::ConversionError(
            "FFmpeg conversion failed".to_string(),
        ))
    }
}

// 从视频中提取音频
#[tauri::command]
async fn extract_audio_from_video(
    window: tauri::Window,
    input_path: String,
    output_format: String,
) -> Result<String, Error> {
    // 检查ffmpeg是否可用
    if !check_ffmpeg_available().await {
        return Err(Error::ConversionError(
            "FFmpeg not found. Please install FFmpeg to use this feature.".to_string(),
        ));
    }

    // 获取输出路径
    let input_path = Path::new(&input_path);
    let file_stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let output_path = input_path.with_file_name(format!("{}_audio.{}", file_stem, output_format));
    let output_path_str = output_path.to_string_lossy().to_string();

    // 创建ffmpeg命令，只提取音频
    let command = window
        .shell()
        .command("ffmpeg")
        .args(["-y", "-i", &input_path.to_string_lossy()])
        .args(["-vn"]) // 不包含视频
        .args(["-progress", "pipe:1"])
        .arg(&output_path_str);

    // 执行命令
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| Error::ConversionError(e.to_string()))?;

    // 创建进度跟踪器
    let progress = Arc::new(AtomicUsize::new(0));
    let progress_clone = progress.clone();

    // 监控命令输出以获取进度信息
    let window_clone = window.clone();

    // 创建一个通道来接收命令完成的信号
    let (tx, rx_done) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    // 处理命令输出和监听命令完成事件
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    // 将 Vec<u8> 转换为 String
                    let line_str = String::from_utf8_lossy(&line);
                    if let Some(prog) = parse_progress(&line_str) {
                        progress_clone.store(prog, Ordering::Relaxed);
                        let _ = window_clone.emit(
                            "conversion_progress",
                            ProgressInfo {
                                progress: prog,
                                output_path: None,
                            },
                        );
                    }
                }
                CommandEvent::Stderr(line) => {
                    // 将 Vec<u8> 转换为 String
                    let line_str = String::from_utf8_lossy(&line);
                    if let Some(prog) = parse_progress(&line_str) {
                        progress_clone.store(prog, Ordering::Relaxed);
                        let _ = window_clone.emit(
                            "conversion_progress",
                            ProgressInfo {
                                progress: prog,
                                output_path: None,
                            },
                        );
                    }
                }
                CommandEvent::Terminated(status) => {
                    // 在Unix系统中，退出码为0表示成功
                    let success = status.code.map_or(false, |code| code == 0);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(success);
                    }
                    break;
                }
                _ => {}
            }
        }
    });

    // 等待命令完成
    let success = rx_done.await.unwrap_or(false);

    if success {
        // 转换成功
        window
            .emit(
                "conversion_progress",
                ProgressInfo {
                    progress: 100,
                    output_path: Some(output_path_str.clone()),
                },
            )
            .map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(output_path_str)
    } else {
        Err(Error::ConversionError(
            "FFmpeg extraction failed".to_string(),
        ))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            convert_audio,
            convert_video,
            extract_audio_from_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
