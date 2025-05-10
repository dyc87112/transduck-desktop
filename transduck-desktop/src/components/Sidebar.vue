<template>
  <div class="sidebar">
    <div class="logo-container">
      <img src="../assets/transduck-logo.png" alt="TransDuck Logo" class="logo-image" />
      <h3>TransDuck</h3>
    </div>
    <div class="menu">
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'audio' }"
        @click="$emit('menu-change', 'audio')"
      >
        <i class="bi bi-file-earmark-music menu-icon"></i>
        <span>{{ $t('app.menu.audioConversion') }}</span>
      </div>
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'video' }"
        @click="$emit('menu-change', 'video')"
      >
        <i class="bi bi-film menu-icon"></i>
        <span>{{ $t('app.menu.videoConversion') }}</span>
      </div>
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'about' }"
        @click="$emit('menu-change', 'about')"
      >
        <i class="bi bi-info-circle menu-icon"></i>
        <span>{{ $t('app.menu.about') }}</span>
      </div>
    </div>
    <div class="language-selector">
      <div class="language-container">
        <i class="bi bi-globe language-icon"></i>
        <select v-model="currentLocale" @change="changeLocale" class="form-select">
          <option value="en">English</option>
          <option value="zh">中文</option>
        </select>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{
  activeMenu: string;
}>();

const emit = defineEmits<{
  (e: 'menu-change', menu: string): void;
}>();

// 使用变量以避免 TypeScript 警告
void props;
void emit;

const { locale } = useI18n();
const currentLocale = ref(locale.value);

const changeLocale = () => {
  locale.value = currentLocale.value;
  localStorage.setItem('locale', currentLocale.value);
};

watch(() => locale.value, (newLocale) => {
  currentLocale.value = newLocale;
});
</script>

<style scoped>
.sidebar {
  width: 220px;
  height: 100%;
  background-color: #f8f9fa;
  border-right: 1px solid #dee2e6;
  display: flex;
  flex-direction: column;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.logo-container {
  padding: 20px;
  text-align: center;
  border-bottom: 1px solid #dee2e6;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.logo-image {
  width: 32px;
  height: 32px;
  object-fit: contain;
}

.menu {
  flex: 1;
  padding: 15px 0;
}

.menu-item {
  padding: 12px 20px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  border-left: 4px solid transparent;
}

.menu-item:hover {
  background-color: #e9ecef;
}

.menu-item.active {
  background-color: #e9ecef;
  font-weight: bold;
  border-left: 4px solid #0d6efd;
}

.menu-icon {
  margin-right: 10px;
  font-size: 1.2rem;
  width: 24px;
  text-align: center;
}

.language-selector {
  padding: 20px;
  border-top: 1px solid #dee2e6;
}

.language-container {
  display: flex;
  align-items: center;
  gap: 10px;
}

.language-icon {
  font-size: 1.2rem;
  color: #6c757d;
}

.form-select {
  flex: 1;
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #ced4da;
  background-color: white;
  cursor: pointer;
}

.form-select:focus {
  border-color: #86b7fe;
  outline: 0;
  box-shadow: 0 0 0 0.25rem rgba(13, 110, 253, 0.25);
}
</style>