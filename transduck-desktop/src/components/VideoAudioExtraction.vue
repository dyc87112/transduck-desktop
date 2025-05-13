<template>
  <div class="conversion-container">
    <div class="card main-card">
      <!-- 页面标题 -->
      <div class="card-header">
        <div class="header-content">
          <i class="bi bi-film header-icon"></i>
          <div>
            <h2>{{ $t('videoAudio.title') }}</h2>
            <p class="text-muted mb-0">{{ $t('videoAudio.description') }}</p>
          </div>
        </div>
      </div>
      
      <div class="card-body">
        <!-- 文件选择 -->
        <div class="form-group">
          <div class="file-selector">
            <div class="file-input-wrapper">
              <div class="file-status-icon">
                <i v-if="selectedFile" class="bi bi-check-circle-fill text-success"></i>
                <i v-else class="bi bi-film text-muted"></i>
              </div>
              <input 
                type="text" 
                readonly 
                :value="selectedFile || ''" 
                class="form-control" 
                :placeholder="$t('common.selectFile')" 
              />
            </div>
            <button @click="openFileDialog" class="btn btn-primary select-btn">
              <i class="bi bi-folder2-open"></i> {{ $t('common.open') }}
            </button>
          </div>
        </div>
        
        <!-- 输出格式选择 -->
        <div class="form-group">
          <label class="form-label">
            <i class="bi bi-arrow-left-right"></i> {{ $t('common.outputFormat') }}
          </label>
          <div class="format-selector">
            <div v-for="(name, format) in audioFormats" :key="format" class="format-option">
              <input 
                type="radio" 
                :id="`audio-format-${format}`" 
                :value="format" 
                v-model="outputFormat" 
                :name="'audioFormat'" 
              />
              <label :for="`audio-format-${format}`" class="format-label">{{ name }}</label>
            </div>
          </div>
        </div>
        
        <!-- 转换按钮 -->
        <div class="action-area">
          <button 
            @click="startConversion" 
            class="btn btn-success conversion-btn" 
            :disabled="!selectedFile || isConverting"
          >
            <i class="bi" :class="isConverting ? 'bi-arrow-repeat spin' : 'bi-arrow-right-circle'"></i>
            {{ isConverting ? $t('common.converting') : $t('common.convert') }}
          </button>
        </div>
        
        <!-- 转换状态 -->
        <div v-if="isConverting" class="status-card">
          <div class="converting-status">
            <div class="status-icon-wrapper">
              <i class="bi bi-arrow-repeat spin"></i>
            </div>
            <h3>{{ $t('common.converting') }}</h3>
            <div class="progress-container">
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: `${progress}%` }"></div>
              </div>
              <div class="progress-text">{{ progress }}%</div>
            </div>
            <button 
              @click="cancelConversion" 
              class="btn btn-outline-danger cancel-btn"
            >
              <i class="bi bi-x-circle"></i> {{ $t('common.cancel') }}
            </button>
          </div>
        </div>
        
        <!-- 转换成功结果 -->
        <div v-if="conversionCompleted && !conversionError" class="result-card success">
          <div class="result-header">
            <div class="result-icon-wrapper success">
              <i class="bi bi-check-circle-fill"></i>
            </div>
            <h4>{{ $t('common.completed') }}</h4>
          </div>
          <div class="result-content">
            <div class="output-path">
              <i class="bi bi-file-earmark-music"></i> 
              <span>{{ $t('common.outputLocation') }}:</span>
              <code class="path-text">{{ outputPath }}</code>
            </div>
            <button @click="openOutputFolder" class="btn btn-outline-primary open-btn">
              <i class="bi bi-folder2-open"></i> {{ $t('common.open') }}
            </button>
          </div>
        </div>
        
        <!-- 转换失败结果 -->
        <div v-if="conversionError" class="result-card error">
          <div class="result-header">
            <div class="result-icon-wrapper error">
              <i class="bi bi-exclamation-triangle-fill"></i>
            </div>
            <h4>{{ $t('common.failed') }}</h4>
          </div>
          <div class="result-content">
            <div class="error-message">{{ conversionError }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { openPath } from '@tauri-apps/plugin-opener';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { dirname } from '@tauri-apps/api/path';

const selectedFile = ref<string | null>(null);
const outputFormat = ref('mp3');
const isConverting = ref(false);
const progress = ref(0);
const conversionCompleted = ref(false);
const conversionError = ref<string | null>(null);
const outputPath = ref<string>('');

// 定义音频格式选项
const audioFormats = {
  mp3: 'MP3',
  wav: 'WAV',
  ogg: 'OGG',
  flac: 'FLAC',
  aac: 'AAC'
};

// 打开文件选择对话框
const openFileDialog = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Video',
        extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'wmv', 'mpg', 'mpeg']
      }]
    });
    
    if (selected && !Array.isArray(selected)) {
      selectedFile.value = selected;
      conversionCompleted.value = false;
      conversionError.value = null;
      progress.value = 0;
    }
  } catch (error) {
    console.error('Error selecting file:', error);
  }
};

// 开始转换
const startConversion = async () => {
  if (!selectedFile.value) return;
  
  isConverting.value = true;
  conversionCompleted.value = false;
  conversionError.value = null;
  progress.value = 0;
  
  try {
    // 监听进度事件
    const unlisten = await listen<number>('conversion-progress', (event: { payload: number }) => {
      progress.value = Math.round(event.payload);
    });
    
    // 调用Rust函数进行转换
    outputPath.value = await invoke('extract_audio_from_video', {
      inputPath: selectedFile.value,
      outputFormat: outputFormat.value
    }) as string;
    
    conversionCompleted.value = true;
    unlisten();
  } catch (error) {
    conversionError.value = String(error);
  } finally {
    isConverting.value = false;
  }
};

// 取消转换
const cancelConversion = async () => {
  try {
    await invoke('cancel_conversion');
    isConverting.value = false;
  } catch (error) {
    console.error('Error canceling conversion:', error);
  }
};

// 打开输出文件夹
const openOutputFolder = async () => {
  if (outputPath.value) {
    try {
      // 获取文件所在的目录路径
      const folderPath = await dirname(outputPath.value);
      console.log('Opening folder path:', folderPath);
      // 直接打开目录
      await openPath(folderPath);
    } catch (error) {
      console.error('Error opening folder:', error);
    }
  } else {
    console.error('No output path available');
  }
};
</script>

<style scoped>
.conversion-container {
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
}

.card {
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  margin-bottom: 20px;
}

.main-card {
  border: 1px solid #e0e0e0;
}

.card-header {
  background-color: #f8f9fa;
  padding: 15px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.header-content {
  display: flex;
  align-items: center;
}

.header-icon {
  font-size: 2rem;
  margin-right: 15px;
  color: #4a86e8;
}

.card-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #333;
}

.text-muted {
  color: #6c757d;
}

.card-body {
  padding: 20px;
}

.form-group {
  margin-bottom: 20px;
}

.form-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #333;
}

.form-label i {
  margin-right: 5px;
  color: #4a86e8;
}

.file-selector {
  display: flex;
  align-items: center;
  gap: 10px;
}

.file-input-wrapper {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.file-status-icon {
  position: absolute;
  left: 10px;
  z-index: 1;
  color: #4a86e8;
}

.form-control {
  width: 100%;
  padding: 10px 10px 10px 35px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 0.9rem;
  background-color: #f8f9fa;
  cursor: default;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  gap: 5px;
}

.btn i {
  font-size: 1rem;
}

.btn-primary {
  background-color: #4a86e8;
  color: white;
}

.btn-primary:hover {
  background-color: #3a76d8;
}

.btn-success {
  background-color: #34a853;
  color: white;
}

.btn-success:hover {
  background-color: #2d9249;
}

.btn-success:disabled {
  background-color: #a8d5b5;
  cursor: not-allowed;
}

.btn-outline-danger {
  background-color: transparent;
  border: 1px solid #ea4335;
  color: #ea4335;
}

.btn-outline-danger:hover {
  background-color: #ea4335;
  color: white;
}

.btn-outline-primary {
  background-color: transparent;
  border: 1px solid #4a86e8;
  color: #4a86e8;
}

.btn-outline-primary:hover {
  background-color: #4a86e8;
  color: white;
}

.format-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.format-option {
  position: relative;
}

.format-option input[type="radio"] {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.format-label {
  display: inline-block;
  padding: 8px 16px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  background-color: #f8f9fa;
}

.format-option input[type="radio"]:checked + .format-label {
  background-color: #4a86e8;
  color: white;
  border-color: #4a86e8;
}

.action-area {
  margin-top: 25px;
  display: flex;
  justify-content: center;
}

.conversion-btn {
  padding: 10px 25px;
  font-size: 1rem;
}

.status-card {
  margin-top: 20px;
  padding: 15px;
  border-radius: 8px;
  background-color: #f8f9fa;
  border: 1px solid #e0e0e0;
}

.converting-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.status-icon-wrapper {
  font-size: 2rem;
  color: #4a86e8;
  margin-bottom: 10px;
}

.progress-container {
  width: 100%;
  margin: 15px 0;
}

.progress-bar {
  height: 10px;
  background-color: #e0e0e0;
  border-radius: 5px;
  overflow: hidden;
  margin-bottom: 5px;
}

.progress-fill {
  height: 100%;
  background-color: #4a86e8;
  transition: width 0.3s ease;
}

.progress-text {
  text-align: center;
  font-size: 0.9rem;
  color: #666;
}

.cancel-btn {
  margin-top: 10px;
}

.result-card {
  margin-top: 20px;
  border-radius: 8px;
  overflow: hidden;
}

.result-card.success {
  border: 1px solid #34a853;
  background-color: #e6f4ea;
}

.result-card.error {
  border: 1px solid #ea4335;
  background-color: #fce8e6;
}

.result-header {
  padding: 15px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.result-icon-wrapper {
  font-size: 1.5rem;
}

.result-icon-wrapper.success {
  color: #34a853;
}

.result-icon-wrapper.error {
  color: #ea4335;
}

.result-header h4 {
  margin: 0;
  font-size: 1.2rem;
}

.result-content {
  padding: 0 15px 15px;
}

.output-path {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 15px;
  flex-wrap: wrap;
}

.path-text {
  background-color: rgba(0, 0, 0, 0.05);
  padding: 4px 8px;
  border-radius: 4px;
  font-family: monospace;
  word-break: break-all;
}

.error-message {
  color: #ea4335;
  font-family: monospace;
  white-space: pre-wrap;
  background-color: rgba(234, 67, 53, 0.05);
  padding: 10px;
  border-radius: 4px;
}

.open-btn {
  margin-top: 5px;
}

.spin {
  animation: spin 1.5s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 响应式调整 */
@media (max-width: 768px) {
  .file-selector {
    flex-direction: column;
  }
  
  .select-btn {
    width: 100%;
  }
  
  .format-selector {
    justify-content: center;
  }
}
</style>