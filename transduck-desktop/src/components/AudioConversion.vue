<template>
  <div class="conversion-container">
    <div class="card main-card">
      <!-- 页面标题 -->
      <div class="card-header">
        <div class="header-content">
          <i class="bi bi-music-note-beamed header-icon"></i>
          <div>
            <h2>{{ $t('audio.title') }}</h2>
            <p class="text-muted mb-0">{{ $t('audio.description') }}</p>
          </div>
        </div>
      </div>
      
      <div class="card-body">
        <!-- 文件选择 -->
        <div class="form-group">
          <label class="form-label">
            <i class="bi bi-file-earmark-music"></i> {{ $t('common.selectFile') }}
          </label>
          <div class="file-selector">
            <div class="file-input-wrapper">
              <div class="file-status-icon">
                <i v-if="selectedFile" class="bi bi-check-circle-fill text-success"></i>
                <i v-else class="bi bi-music-note text-muted"></i>
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
              <i class="bi bi-file-earmark-check"></i> 
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
        name: 'Audio',
        extensions: ['mp3', 'wav', 'ogg', 'flac', 'aac', 'm4a']
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
    outputPath.value = await invoke('convert_audio', {
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
/* 主容器样式 */
.conversion-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 20px;
  background-color: #f9fafb;
}

.main-card {
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  background-color: #fff;
  overflow: hidden;
  transition: all 0.3s ease;
  border: none;
}

/* 卡片头部样式 */
.card-header {
  background-color: #f8f9fa;
  border-bottom: 1px solid #eaedf0;
  padding: 20px;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  font-size: 2rem;
  color: #4361ee;
  background-color: rgba(67, 97, 238, 0.1);
  padding: 12px;
  border-radius: 12px;
}

.card-header h2 {
  margin: 0 0 5px 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #2d3748;
}

.text-muted {
  color: #6c757d;
}

.mb-0 {
  margin-bottom: 0;
}

/* 卡片内容区域 */
.card-body {
  padding: 24px;
}

/* 表单元素样式 */
.form-group {
  margin-bottom: 24px;
}

.form-label {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
  font-weight: 500;
  color: #4a5568;
}

.form-label i {
  margin-right: 8px;
  color: #4361ee;
}

/* 文件选择器样式 */
.file-selector {
  display: flex;
  gap: 12px;
}

.file-input-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 0 12px;
  background-color: #fff;
  transition: all 0.2s ease;
}

.file-input-wrapper:focus-within {
  border-color: #4361ee;
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.15);
}

.file-status-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  color: #4361ee;
}

.text-success {
  color: #10b981;
}

.text-muted {
  color: #9ca3af;
}

.form-control {
  flex: 1;
  border: none;
  padding: 12px 0;
  font-size: 0.95rem;
  background: transparent;
  color: #4a5568;
}

.form-control:focus {
  outline: none;
}

.select-btn {
  white-space: nowrap;
  padding: 10px 16px;
  border-radius: 8px;
}

/* 格式选择器样式 */
.format-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 8px;
}

.format-option {
  position: relative;
}

.format-option input[type="radio"] {
  position: absolute;
  opacity: 0;
}

.format-label {
  display: block;
  padding: 10px 16px;
  border-radius: 8px;
  background-color: #f1f5f9;
  color: #4a5568;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: center;
  margin: 0;
  min-width: 60px;
}

.format-option input[type="radio"]:checked + .format-label {
  background-color: #4361ee;
  color: white;
  box-shadow: 0 2px 10px rgba(67, 97, 238, 0.3);
}

.format-option input[type="radio"]:focus + .format-label {
  box-shadow: 0 0 0 3px rgba(67, 97, 238, 0.15);
}

/* 操作区域样式 */
.action-area {
  display: flex;
  justify-content: center;
  margin: 24px 0;
}

.conversion-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  font-weight: 500;
  font-size: 1rem;
  border-radius: 8px;
  background-color: #4361ee;
  border-color: #4361ee;
  transition: all 0.2s ease;
}

.conversion-btn:hover:not(:disabled) {
  background-color: #3a56d4;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(67, 97, 238, 0.25);
}

.conversion-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 状态卡片样式 */
.status-card {
  background-color: #f8fafc;
  border-radius: 12px;
  padding: 24px;
  margin-top: 20px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
  text-align: center;
}

.converting-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.status-icon-wrapper {
  font-size: 2.5rem;
  color: #4361ee;
  height: 70px;
  width: 70px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(67, 97, 238, 0.1);
  border-radius: 50%;
  margin-bottom: 8px;
}

.converting-status h3 {
  margin: 0;
  color: #4a5568;
  font-weight: 600;
}

.cancel-btn {
  margin-top: 8px;
  padding: 8px 16px;
  border-radius: 8px;
}

/* 结果卡片样式 */
.result-card {
  margin-top: 24px;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
}

.result-card.success {
  background-color: #f0fdf4;
  border: 1px solid #dcfce7;
}

.result-card.error {
  background-color: #fef2f2;
  border: 1px solid #fee2e2;
}

.result-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.result-icon-wrapper {
  height: 50px;
  width: 50px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 1.5rem;
}

.result-icon-wrapper.success {
  background-color: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.result-icon-wrapper.error {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.result-header h4 {
  margin: 0;
  font-weight: 600;
  color: #4a5568;
}

.result-content {
  padding: 16px 0;
}

.output-path {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: rgba(0, 0, 0, 0.02);
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.output-path i {
  color: #10b981;
}

.path-text {
  font-family: monospace;
  word-break: break-all;
  background-color: rgba(0, 0, 0, 0.03);
  padding: 4px 8px;
  border-radius: 4px;
  margin-left: 4px;
  flex: 1;
}

.error-message {
  background-color: rgba(239, 68, 68, 0.05);
  padding: 12px 16px;
  border-radius: 8px;
  color: #b91c1c;
  font-size: 0.95rem;
}

.open-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 8px;
}

/* 按钮样式 */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-weight: 500;
  border: 1px solid transparent;
  padding: 0.5rem 1rem;
  font-size: 0.95rem;
  border-radius: 0.375rem;
  transition: all 0.2s ease;
  cursor: pointer;
}

.btn i {
  margin-right: 6px;
}

.btn-primary {
  color: #fff;
  background-color: #4361ee;
  border-color: #4361ee;
}

.btn-primary:hover:not(:disabled) {
  background-color: #3a56d4;
  border-color: #3a56d4;
}

.btn-outline-danger {
  color: #ef4444;
  background-color: transparent;
  border-color: #ef4444;
}

.btn-outline-danger:hover {
  color: #fff;
  background-color: #ef4444;
}

.btn-outline-primary {
  color: #4361ee;
  background-color: transparent;
  border-color: #4361ee;
}

.btn-outline-primary:hover {
  color: #fff;
  background-color: #4361ee;
}

/* 动画效果 */
.spin {
  animation: spin 1.2s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
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
    justify-content: space-between;
  }
  
  .format-label {
    min-width: auto;
    padding: 8px 12px;
    font-size: 0.9rem;
  }
}
</style>