<template>
  <div class="conversion-container">
    <div class="card main-card">
      <!-- 页面标题 -->
      <div class="card-header">
        <div class="header-content">
          <i class="bi bi-film header-icon"></i>
          <div>
            <h2>{{ $t('video.title') }}</h2>
            <p class="text-muted mb-0">{{ $t('video.description') }}</p>
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
            <div v-for="(name, format) in videoFormats" :key="format" class="format-option">
              <input 
                type="radio" 
                :id="`video-format-${format}`" 
                :value="format" 
                v-model="outputFormat" 
                :name="'videoFormat'" 
              />
              <label :for="`video-format-${format}`" class="format-label">{{ name }}</label>
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
const outputFormat = ref('mp4');
const isConverting = ref(false);
const progress = ref(0);
const conversionCompleted = ref(false);
const conversionError = ref<string | null>(null);
const outputPath = ref<string>('');

// 定义视频格式选项
const videoFormats = {
  mp4: 'MP4',
  mkv: 'MKV',
  avi: 'AVI',
  mov: 'MOV',
  webm: 'WebM',
  flv: 'FLV',
  wmv: 'WMV'
};

// 打开文件选择对话框
const openFileDialog = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Video',
        extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'wmv']
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
    outputPath.value = await invoke('convert_video', {
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
@import '../styles/common.css';

/* 主容器样式 */
.conversion-container {
  padding: 20px;
  height: 100%;
  background-color: #f9fafb;
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
  padding: 12px 24px;
  font-size: 1rem;
}

.conversion-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 状态卡片样式 */
.converting-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  text-align: center;
}

.status-icon-wrapper {
  font-size: 2.5rem;
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
  font-weight: 600;
}

/* 结果卡片样式 */
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

/* 响应式调整 */
@media (max-width: 768px) {
  .conversion-container {
    padding: 15px;
  }

  .file-selector {
    flex-direction: column;
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