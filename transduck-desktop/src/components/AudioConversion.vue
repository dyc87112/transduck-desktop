<template>
  <div class="conversion-container">
    <!-- 任务配置区 -->
    <div class="config-section">
      <h2>{{ $t('audio.title') }}</h2>
      <p>{{ $t('audio.description') }}</p>
      
      <div class="form-group">
        <label>{{ $t('common.selectFile') }}</label>
        <div class="file-input-container">
          <input type="text" readonly :value="selectedFile || ''" class="form-control" />
          <button @click="openFileDialog" class="btn btn-primary">{{ $t('common.selectFile') }}</button>
        </div>
      </div>
      
      <div class="form-group">
        <label>{{ $t('common.outputFormat') }}</label>
        <div class="radio-group">
          <div v-for="(name, format) in audioFormats" :key="format" class="radio-option">
            <input type="radio" :id="`audio-format-${format}`" :value="format" v-model="outputFormat" :name="'audioFormat'" />
            <label :for="`audio-format-${format}`">{{ name }}</label>
          </div>
        </div>
      </div>
      
      <button 
        @click="startConversion" 
        class="btn btn-success" 
        :disabled="!selectedFile || isConverting"
      >
        {{ isConverting ? $t('common.converting') : $t('common.convert') }}
      </button>
    </div>
    
    <!-- 任务执行进度和结果显示区 -->
    <div class="result-section">
      <div v-if="isConverting || conversionCompleted" class="progress-container">
        <h3>{{ $t('common.progress') }}</h3>
        <div class="progress">
          <div 
            class="progress-bar" 
            :style="{ width: `${progress}%` }" 
            :class="{ 'bg-success': conversionCompleted }"
          >
            {{ progress }}%
          </div>
        </div>
        
        <button 
          v-if="isConverting" 
          @click="cancelConversion" 
          class="btn btn-danger mt-2"
        >
          {{ $t('common.cancel') }}
        </button>
      </div>
      
      <div v-if="conversionCompleted && !conversionError" class="result-info">
        <div class="alert alert-success">
          <h4>{{ $t('common.completed') }}</h4>
          <p>{{ $t('common.outputLocation') }}: {{ outputPath }}</p>
          <button @click="openOutputFolder" class="btn btn-info">
            {{ $t('common.open') }}
          </button>
        </div>
      </div>
      
      <div v-if="conversionError" class="result-info">
        <div class="alert alert-danger">
          <h4>{{ $t('common.failed') }}</h4>
          <p>{{ conversionError }}</p>
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
.conversion-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.config-section {
  padding: 20px;
  border-bottom: 1px solid #dee2e6;
}

.result-section {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
}

.file-input-container {
  display: flex;
  gap: 10px;
}

.file-input-container input {
  flex: 1;
}

.progress-container {
  margin-bottom: 20px;
}

.progress {
  height: 25px;
  border-radius: 5px;
  overflow: hidden;
  background-color: #e9ecef;
}

.progress-bar {
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #007bff;
  color: white;
  transition: width 0.3s;
}

.result-info {
  margin-top: 20px;
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
  border-radius: 0.25rem;
  transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}

.btn {
  display: inline-block;
  font-weight: 400;
  text-align: center;
  white-space: nowrap;
  vertical-align: middle;
  user-select: none;
  border: 1px solid transparent;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  border-radius: 0.25rem;
  transition: color 0.15s ease-in-out, background-color 0.15s ease-in-out, border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
  cursor: pointer;
}

.btn-primary {
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
}

.btn-success {
  color: #fff;
  background-color: #28a745;
  border-color: #28a745;
}

.btn-danger {
  color: #fff;
  background-color: #dc3545;
  border-color: #dc3545;
}

.btn-info {
  color: #fff;
  background-color: #17a2b8;
  border-color: #17a2b8;
}

.alert {
  position: relative;
  padding: 0.75rem 1.25rem;
  margin-bottom: 1rem;
  border: 1px solid transparent;
  border-radius: 0.25rem;
}

.alert-success {
  color: #155724;
  background-color: #d4edda;
  border-color: #c3e6cb;
}

.alert-danger {
  color: #721c24;
  background-color: #f8d7da;
  border-color: #f5c6cb;
}

.mt-2 {
  margin-top: 0.5rem;
}

.radio-group {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 5px;
}

.radio-option {
  display: flex;
  align-items: center;
  margin-right: 15px;
}

.radio-option input[type="radio"] {
  margin-right: 5px;
}

.radio-option label {
  margin-bottom: 0;
  cursor: pointer;
}
</style>