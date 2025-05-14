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
        
        <!-- 转换状态和结果 -->
        <div class="status-container">
          <!-- 转换中状态 -->
          <div v-if="isConverting" class="status-inline converting">
            <i class="bi bi-arrow-repeat spin status-icon"></i>
            <div class="status-info">
              <div class="status-text">{{ $t('common.converting') }} ({{ progress }}%)</div>
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: `${progress}%` }"></div>
              </div>
            </div>
            <button @click="cancelConversion" class="btn btn-sm btn-outline-danger">
              <i class="bi bi-x-circle"></i>
            </button>
          </div>

          <!-- 转换成功状态 -->
          <div v-if="conversionCompleted && !conversionError" class="status-inline success">
            <i class="bi bi-check-circle-fill status-icon"></i>
            <div class="status-info">
              <div class="status-text">{{ $t('common.completed') }}</div>
              <div class="output-info">
                <!-- <i class="bi bi-file-earmark-video"></i> -->
                <span class="path-text" :title="outputPath">{{ outputPath }}</span>
              </div>
            </div>
            <button @click="openOutputFolder" class="btn btn-sm btn-outline-primary">
              <i class="bi bi-folder2-open"></i>
            </button>
          </div>

          <!-- 转换失败状态 -->
          <div v-if="conversionError" class="status-inline error">
            <i class="bi bi-exclamation-triangle-fill status-icon"></i>
            <div class="status-info">
              <div class="status-text">{{ $t('common.failed') }}</div>
              <div class="error-text">{{ conversionError }}</div>
            </div>
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

/* 状态容器样式 */
.status-container {
  margin-top: 20px;
}

.status-inline {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  background-color: #fff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.status-icon {
  font-size: 1.25rem;
  width: 24px;
  text-align: center;
}

.status-info {
  flex: 1;
  min-width: 0;
}

.status-text {
  font-weight: 500;
  margin-bottom: 4px;
}

.progress-bar {
  height: 4px;
  background-color: #e2e8f0;
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: #4361ee;
  transition: width 0.3s ease;
}

.output-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
  color: #64748b;
}

.path-text {
  font-family: monospace;
  max-width: 100%;
  display: inline-block;
  cursor: pointer;
}

.error-text {
  font-size: 0.9rem;
  color: #b91c1c;
}

/* 状态变体样式 */
.status-inline.converting {
  border: 1px solid #4361ee;
}

.status-inline.converting .status-icon {
  color: #4361ee;
}

.status-inline.success {
  border: 1px solid #10b981;
}

.status-inline.success .status-icon {
  color: #10b981;
}

.status-inline.error {
  border: 1px solid #ef4444;
}

.status-inline.error .status-icon {
  color: #ef4444;
}

/* 按钮样式调整 */
.btn-sm {
  padding: 4px 8px;
  font-size: 0.875rem;
}

.btn-sm i {
  font-size: 1rem;
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