<template>
  <div class="conversion-container">
    <!-- 任务配置区 -->
    <div class="config-section">
      <div class="section-header">
        <i class="bi bi-film section-icon"></i>
        <div>
          <h2>{{ $t('video.title') }}</h2>
          <p>{{ $t('video.description') }}</p>
        </div>
      </div>
      
      <div class="form-group">
        <label><i class="bi bi-camera-video"></i> {{ $t('common.selectFile') }}</label>
        <div class="file-input-container">
          <div class="input-group">
            <span class="input-group-text" v-if="selectedFile">
              <i class="bi bi-check-circle-fill text-success"></i>
            </span>
            <span class="input-group-text" v-else>
              <i class="bi bi-film"></i>
            </span>
            <input type="text" readonly :value="selectedFile || ''" class="form-control" placeholder="未选择文件" />
            <button @click="openFileDialog" class="btn btn-primary">
              <i class="bi bi-folder2-open"></i> {{ $t('common.selectFile') }}
            </button>
          </div>
        </div>
      </div>
      
      <div class="form-group">
        <label><i class="bi bi-arrow-left-right"></i> {{ $t('common.outputFormat') }}</label>
        <div class="radio-group">
          <div v-for="(name, format) in videoFormats" :key="format" class="radio-option">
            <input type="radio" :id="`video-format-${format}`" :value="format" v-model="outputFormat" :name="'videoFormat'" />
            <label :for="`video-format-${format}`" class="format-label">{{ name }}</label>
          </div>
        </div>
      </div>
      
      <button 
        @click="startConversion" 
        class="btn btn-success conversion-btn" 
        :disabled="!selectedFile || isConverting"
      >
        <i class="bi" :class="isConverting ? 'bi-arrow-repeat spin' : 'bi-arrow-right-circle'"></i>
        {{ isConverting ? $t('common.converting') : $t('common.convert') }}
      </button>
    </div>
    
    <!-- 任务执行结果显示区 -->
    <div class="result-section">
      <div v-if="isConverting" class="status-container">
        <div class="converting-status">
          <i class="bi bi-arrow-repeat spin status-icon"></i>
          <h3>{{ $t('common.converting') }}</h3>
        </div>
        
        <button 
          @click="cancelConversion" 
          class="btn btn-danger mt-3"
        >
          <i class="bi bi-x-circle"></i> {{ $t('common.cancel') }}
        </button>
      </div>
      
      <div v-if="conversionCompleted && !conversionError" class="result-info">
        <div class="alert alert-success result-card">
          <div class="result-header">
            <i class="bi bi-check-circle-fill result-icon"></i>
            <h4>{{ $t('common.completed') }}</h4>
          </div>
          <p class="output-path">
            <i class="bi bi-file-earmark-check"></i> 
            {{ $t('common.outputLocation') }}: <span class="path-text">{{ outputPath }}</span>
          </p>
          <button @click="openOutputFolder" class="btn btn-info">
            <i class="bi bi-folder2-open"></i> {{ $t('common.open') }}
          </button>
        </div>
      </div>
      
      <div v-if="conversionError" class="result-info">
        <div class="alert alert-danger result-card">
          <div class="result-header">
            <i class="bi bi-exclamation-triangle-fill result-icon"></i>
            <h4>{{ $t('common.failed') }}</h4>
          </div>
          <p class="error-message">{{ conversionError }}</p>
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
.conversion-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.section-header {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
  gap: 15px;
}

.section-icon {
  font-size: 2rem;
  color: #0d6efd;
}

.config-section {
  padding: 25px;
  border-bottom: 1px solid #dee2e6;
  background-color: #f8f9fa;
  border-radius: 8px 8px 0 0;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
}

.result-section {
  flex: 1;
  padding: 25px;
  overflow-y: auto;
  background-color: #fff;
  border-radius: 0 0 8px 8px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  font-weight: 500;
  color: #495057;
}

.form-group label i {
  margin-right: 8px;
  color: #0d6efd;
}

.file-input-container {
  display: flex;
  gap: 10px;
}

.file-input-container input {
  flex: 1;
}

.input-group {
  display: flex;
  flex-wrap: nowrap;
}

.input-group-text {
  display: flex;
  align-items: center;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  font-weight: 400;
  line-height: 1.5;
  color: #212529;
  text-align: center;
  white-space: nowrap;
  background-color: #f8f9fa;
  border: 1px solid #ced4da;
  border-radius: 0.25rem 0 0 0.25rem;
}

.status-container {
  margin-bottom: 25px;
  background-color: #f8f9fa;
  padding: 25px;
  border-radius: 8px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
  text-align: center;
}

.converting-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 15px;
  margin-bottom: 15px;
}

.status-icon {
  font-size: 3rem;
  color: #0d6efd;
}

.converting-status h3 {
  color: #495057;
  margin: 0;
}

.mt-3 {
  margin-top: 1rem;
}

.result-info {
  margin-top: 20px;
}

.result-card {
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.result-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 15px;
}

.result-icon {
  font-size: 1.5rem;
}

.output-path {
  background-color: rgba(0, 0, 0, 0.03);
  padding: 12px;
  border-radius: 5px;
  margin: 15px 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-text {
  font-family: monospace;
  word-break: break-all;
  margin-left: 5px;
}

.error-message {
  background-color: rgba(0, 0, 0, 0.03);
  padding: 12px;
  border-radius: 5px;
  color: #842029;
}

.form-control {
  display: block;
  width: 100%;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  color: #495057;
  background-color: #fff;
  background-clip: padding-box;
  border: 1px solid #ced4da;
  border-radius: 0 0.25rem 0.25rem 0;
  transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}

.btn {
  display: inline-block;
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
  vertical-align: middle;
  user-select: none;
  border: 1px solid transparent;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  border-radius: 0.25rem;
  transition: all 0.15s ease-in-out;
  cursor: pointer;
}

.btn i {
  margin-right: 5px;
}

.btn-primary {
  color: #fff;
  background-color: #0d6efd;
  border-color: #0d6efd;
}

.btn-primary:hover {
  background-color: #0b5ed7;
  border-color: #0a58ca;
}

.btn-success {
  color: #fff;
  background-color: #198754;
  border-color: #198754;
}

.btn-success:hover {
  background-color: #157347;
  border-color: #146c43;
}

.btn-danger {
  color: #fff;
  background-color: #dc3545;
  border-color: #dc3545;
}

.btn-danger:hover {
  background-color: #bb2d3b;
  border-color: #b02a37;
}

.btn-info {
  color: #fff;
  background-color: #0dcaf0;
  border-color: #0dcaf0;
}

.btn-info:hover {
  background-color: #31d2f2;
  border-color: #25cff2;
}

.alert {
  position: relative;
  padding: 1rem 1.25rem;
  margin-bottom: 1rem;
  border: 1px solid transparent;
  border-radius: 0.25rem;
}

.alert-success {
  color: #0f5132;
  background-color: #d1e7dd;
  border-color: #badbcc;
}

.alert-danger {
  color: #842029;
  background-color: #f8d7da;
  border-color: #f5c2c7;
}

.mt-2 {
  margin-top: 0.5rem;
}

.radio-group {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 10px;
}

.radio-option {
  display: flex;
  align-items: center;
}

.radio-option input[type="radio"] {
  position: absolute;
  opacity: 0;
}

.format-label {
  margin-bottom: 0;
  cursor: pointer;
  padding: 8px 15px;
  border-radius: 20px;
  background-color: #e9ecef;
  transition: all 0.2s;
  font-weight: 500;
}

.radio-option input[type="radio"]:checked + .format-label {
  background-color: #0d6efd;
  color: white;
  box-shadow: 0 2px 5px rgba(13, 110, 253, 0.3);
}

.conversion-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  font-weight: 500;
}

.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>