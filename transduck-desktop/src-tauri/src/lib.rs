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

// 通用 FFmpeg 命令执行函数
async fn execute_ffmpeg_command(
    window: tauri::Window,
    input_path_str: &str,
    output_path_str: &str,
    ffmpeg_args: Vec<String>,
    error_message: &str,
) -> Result<String, Error> {
    // 检查ffmpeg是否可用
    if !check_ffmpeg_available().await {
        return Err(Error::ConversionError(
            "FFmpeg not found. Please install FFmpeg to use this feature.".to_string(),
        ));
    }

    // 创建ffmpeg命令
    let mut command_builder = window.shell().command("ffmpeg");
    command_builder = command_builder.args(["-y", "-i", input_path_str]);
    for arg in ffmpeg_args {
        command_builder = command_builder.arg(arg);
    }
    command_builder = command_builder.args(["-progress", "pipe:1"]);
    command_builder = command_builder.arg(output_path_str);

    // 执行命令
    let (mut rx, _child) = command_builder
        .spawn()
        .map_err(|e| Error::ConversionError(e.to_string()))?;

    // 创建进度跟踪器
    let progress = Arc::new(AtomicUsize::new(0));
    let progress_clone = progress.clone();

    // 监控命令输出以获取进度信息
    let window_clone = window.clone();

    // 创建一个通道来接收命令完成的信号
    let (tx, rx_done) = oneshot::channel();
    let tx_arc = Arc::new(Mutex::new(Some(tx))); // Renamed to avoid conflict in spawn closure

    // 处理命令输出和监听命令完成事件
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
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
                    let success = status.code.map_or(false, |code| code == 0);
                    if let Some(tx_channel) = tx_arc.lock().unwrap().take() { // Use renamed tx_arc
                        let _ = tx_channel.send(success);
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
        window
            .emit(
                "conversion_progress",
                ProgressInfo {
                    progress: 100,
                    output_path: Some(output_path_str.to_string()),
                },
            )
            .map_err(|e| Error::ConversionError(e.to_string()))?;
        Ok(output_path_str.to_string())
    } else {
        Err(Error::ConversionError(error_message.to_string()))
    }
}

// 音频转换命令
#[tauri::command]
async fn convert_audio(
    window: tauri::Window,
    input_path: String,
    output_format: String,
) -> Result<String, Error> {
    let p = Path::new(&input_path);
    let file_stem = p.file_stem().unwrap_or_default().to_string_lossy();
    let output_file_name = format!("{}.{}", file_stem, output_format);
    let output_path = p.with_file_name(output_file_name);
    let output_path_str = output_path.to_string_lossy().to_string();

    execute_ffmpeg_command(
        window,
        &input_path,
        &output_path_str,
        vec![], // No specific additional args for audio conversion
        "FFmpeg audio conversion failed",
    )
    .await
}

// 视频转换命令
#[tauri::command]
async fn convert_video(
    window: tauri::Window,
    input_path: String,
    output_format: String,
) -> Result<String, Error> {
    let p = Path::new(&input_path);
    let file_stem = p.file_stem().unwrap_or_default().to_string_lossy();
    let output_file_name = format!("{}.{}", file_stem, output_format);
    let output_path = p.with_file_name(output_file_name);
    let output_path_str = output_path.to_string_lossy().to_string();

    execute_ffmpeg_command(
        window,
        &input_path,
        &output_path_str,
        vec![], // No specific additional args for video conversion
        "FFmpeg video conversion failed",
    )
    .await
}

// 从视频中提取音频
#[tauri::command]
async fn extract_audio_from_video(
    window: tauri::Window,
    input_path: String,
    output_format: String,
) -> Result<String, Error> {
    let p = Path::new(&input_path);
    let file_stem = p.file_stem().unwrap_or_default().to_string_lossy();
    let output_file_name = format!("{}_audio.{}", file_stem, output_format);
    let output_path = p.with_file_name(output_file_name);
    let output_path_str = output_path.to_string_lossy().to_string();

    execute_ffmpeg_command(
        window,
        &input_path,
        &output_path_str,
        vec!["-vn".to_string()], // Specific arg to extract audio (no video)
        "FFmpeg audio extraction failed",
    )
    .await
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
