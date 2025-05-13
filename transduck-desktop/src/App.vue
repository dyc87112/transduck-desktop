<script setup lang="ts">
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import AudioConversion from "./components/AudioConversion.vue";
import VideoConversion from "./components/VideoConversion.vue";
import VideoAudioExtraction from "./components/VideoAudioExtraction.vue";
import About from "./components/About.vue";

// 当前选择的功能
const activeMenu = ref("audio");

// 切换功能
const handleMenuChange = (menu: string) => {
  activeMenu.value = menu;
};
</script>

<template>
  <div class="app-container">
    <!-- 左侧侧边栏 -->
    <Sidebar :activeMenu="activeMenu" @menu-change="handleMenuChange" />
    
    <!-- 右侧内容区 -->
    <div class="main-content">
      <!-- 根据当前选择的功能显示对应的组件 -->
      <AudioConversion v-if="activeMenu === 'audio'" />
      <VideoConversion v-if="activeMenu === 'video'" />
      <VideoAudioExtraction v-if="activeMenu === 'videoAudio'" />
      <About v-if="activeMenu === 'about'" />
    </div>
  </div>
</template>

<style>
/* 全局样式 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  width: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

/* 应用容器样式 */
.app-container {
  display: flex;
  height: 100%;
  width: 100%;
}

/* 主内容区样式 */
.main-content {
  flex: 1;
  overflow: auto;
  padding: 20px;
}

/* 表单样式 */
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

.progress {
  display: flex;
  height: 1rem;
  overflow: hidden;
  font-size: 0.75rem;
  background-color: #e9ecef;
  border-radius: 0.25rem;
}

.progress-bar {
  display: flex;
  flex-direction: column;
  justify-content: center;
  color: #fff;
  text-align: center;
  white-space: nowrap;
  background-color: #007bff;
  transition: width 0.6s ease;
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
</style>