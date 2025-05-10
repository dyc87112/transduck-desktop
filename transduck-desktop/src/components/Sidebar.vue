<template>
  <div class="sidebar">
    <div class="logo-container">
      <h3>TransDuck</h3>
    </div>
    <div class="menu">
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'audio' }"
        @click="$emit('menu-change', 'audio')"
      >
        <span>{{ $t('app.menu.audioConversion') }}</span>
      </div>
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'video' }"
        @click="$emit('menu-change', 'video')"
      >
        <span>{{ $t('app.menu.videoConversion') }}</span>
      </div>
    </div>
    <div class="language-selector">
      <select v-model="currentLocale" @change="changeLocale">
        <option value="en">English</option>
        <option value="zh">中文</option>
      </select>
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
  width: 200px;
  height: 100%;
  background-color: #f8f9fa;
  border-right: 1px solid #dee2e6;
  display: flex;
  flex-direction: column;
}

.logo-container {
  padding: 20px;
  text-align: center;
  border-bottom: 1px solid #dee2e6;
}

.menu {
  flex: 1;
  padding: 10px 0;
}

.menu-item {
  padding: 12px 20px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.menu-item:hover {
  background-color: #e9ecef;
}

.menu-item.active {
  background-color: #e9ecef;
  font-weight: bold;
}

.language-selector {
  padding: 20px;
  border-top: 1px solid #dee2e6;
}

.language-selector select {
  width: 100%;
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #ced4da;
}
</style>